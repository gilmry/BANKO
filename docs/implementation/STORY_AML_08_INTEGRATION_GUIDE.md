# STORY-AML-08: Infrastructure Integration Guide

## Overview

STORY-AML-08 has implemented the application layer for AML account freezes. This guide shows how to integrate it at the infrastructure layer by implementing the `IAccountFreezePort` trait.

## Architecture Context

```
┌─────────────────────────────────────────────────────────┐
│               DOMAIN LAYER                              │
│  AssetFreeze, Account (entities with business logic)   │
└─────────────────────────────────────────────────────────┘
                        ↑
┌─────────────────────────────────────────────────────────┐
│            APPLICATION LAYER (AML)                      │
│  AssetFreezeService                                    │
│  Ports: IAssetFreezeRepository                         │
│         IAccountFreezePort ← NEW (abstraction)         │
└─────────────────────────────────────────────────────────┘
                        ↑
┌─────────────────────────────────────────────────────────┐
│          APPLICATION LAYER (Account)                    │
│  AccountService (called via port)                      │
│  Port: IAccountRepository                              │
└─────────────────────────────────────────────────────────┘
                        ↑
┌─────────────────────────────────────────────────────────┐
│         INFRASTRUCTURE LAYER (Adapters)                │
│  AccountFreezeAdapter ← NEW (implements port)          │
│  PostgreSQL Repositories                               │
│  HTTP Handlers                                         │
└─────────────────────────────────────────────────────────┘
```

## Step 1: Create the Account Freeze Adapter

**File to create**: `/crates/infrastructure/src/aml/account_freeze_adapter.rs`

```rust
use std::sync::Arc;
use uuid::Uuid;

use banko_application::aml::IAccountFreezePort;
use banko_domain::account::{Account, AccountId};

use crate::account::AccountService;

/// Adapter that implements IAccountFreezePort by delegating to AccountService.
/// This allows AML context to freeze/unfreeze accounts without direct dependency.
pub struct AccountFreezeAdapter {
    account_service: Arc<AccountService>,
}

impl AccountFreezeAdapter {
    pub fn new(account_service: Arc<AccountService>) -> Self {
        AccountFreezeAdapter { account_service }
    }
}

#[async_trait::async_trait]
impl IAccountFreezePort for AccountFreezeAdapter {
    async fn find_account_by_id(&self, account_id: Uuid) -> Result<Option<Account>, String> {
        let account_id = AccountId::from_uuid(account_id);
        match self.account_service.find_by_id(&account_id).await {
            Ok(account) => Ok(Some(account)),
            Err(_e) => {
                // AccountServiceError::AccountNotFound — return Ok(None) for "not found"
                // Other errors should bubble up as Err
                Ok(None)
            }
        }
    }

    async fn freeze_account(&self, account_id: Uuid) -> Result<Account, String> {
        let account_id = AccountId::from_uuid(account_id);

        // Find the account
        let mut account = self
            .account_service
            .find_by_id(&account_id)
            .await
            .map_err(|e| format!("Failed to find account: {}", e))?;

        // Freeze it (sets status=Suspended, available_balance=0)
        account.freeze();

        // Save the frozen account
        self.account_service
            .update_account(&account)
            .await
            .map_err(|e| format!("Failed to freeze account: {}", e))?;

        Ok(account)
    }

    async fn unfreeze_account(&self, account_id: Uuid) -> Result<Account, String> {
        let account_id = AccountId::from_uuid(account_id);

        // Find the account
        let mut account = self
            .account_service
            .find_by_id(&account_id)
            .await
            .map_err(|e| format!("Failed to find account: {}", e))?;

        // Unfreeze it (sets status=Active, restores available_balance)
        account.unfreeze();

        // Save the unfrozen account
        self.account_service
            .update_account(&account)
            .await
            .map_err(|e| format!("Failed to unfreeze account: {}", e))?;

        Ok(account)
    }
}
```

**Note**: This assumes `AccountService` has an `update_account()` method. If not, use the appropriate method to persist the updated account.

## Step 2: Extend AccountService with update_account()

If `AccountService::update_account()` doesn't exist, add it to:
**File**: `/crates/application/src/account/service.rs`

```rust
    /// Update an existing account (used by other services like AML).
    pub async fn update_account(&self, account: &Account) -> Result<(), AccountServiceError> {
        self.account_repo
            .save(account)
            .await
            .map_err(AccountServiceError::Internal)
    }
```

## Step 3: Wire Up the Adapter in DI Container

**File**: `/crates/infrastructure/src/mod.rs` or wherever DI is initialized

```rust
use std::sync::Arc;
use banko_application::aml::AssetFreezeService;
use crate::aml::AccountFreezeAdapter;
use crate::account::AccountService;

pub struct AmlServices {
    pub asset_freeze_service: Arc<AssetFreezeService>,
}

impl AmlServices {
    pub fn new(
        account_service: Arc<AccountService>,
        freeze_repo: Arc<dyn IAssetFreezeRepository>,
    ) -> Self {
        // Create the adapter
        let account_freeze_adapter = Arc::new(AccountFreezeAdapter::new(account_service));

        // Create service with full integration
        let asset_freeze_service = Arc::new(
            AssetFreezeService::with_account_port(freeze_repo, account_freeze_adapter)
        );

        AmlServices {
            asset_freeze_service,
        }
    }
}
```

## Step 4: Add to Module Exports

**File**: `/crates/infrastructure/src/aml/mod.rs`

```rust
mod account_freeze_adapter;
mod repositories;  // existing
// ... other modules

pub use account_freeze_adapter::AccountFreezeAdapter;
pub use repositories::*;
// ... other exports
```

## Step 5: Update HTTP Handlers

**File**: `/crates/infrastructure/src/web/handlers/aml.rs` (or similar)

```rust
use actix_web::{web, HttpResponse, Result as ActixResult};
use uuid::Uuid;

use banko_application::aml::AssetFreezeService;

#[post("/api/v1/aml/freezes")]
pub async fn freeze_account(
    aml_service: web::Data<Arc<AssetFreezeService>>,
    payload: web::Json<FreezeAccountRequest>,
) -> ActixResult<HttpResponse> {
    // Add permission check here — only compliance officers
    // let user = extract_and_verify_user(req)?;
    // user.has_permission("aml.freeze")?;

    let account_id = Uuid::parse_str(&payload.account_id)
        .map_err(|_| HttpError::InvalidInput("Invalid account_id".to_string()))?;

    match aml_service
        .freeze_account(
            account_id,
            payload.reason.clone(),
            "current_user_id".to_string(),  // Get from auth context
        )
        .await
    {
        Ok(freeze) => {
            // Log the action to audit trail
            // audit_logger.log_aml_freeze(&freeze);

            Ok(HttpResponse::Created().json(freeze))
        }
        Err(e) => {
            error!("Failed to freeze account: {}", e);
            Ok(HttpResponse::InternalServerError().json(
                json!({"error": "Failed to freeze account"}),
            ))
        }
    }
}

#[patch("/api/v1/aml/freezes/{id}/lift")]
pub async fn lift_freeze(
    aml_service: web::Data<Arc<AssetFreezeService>>,
    path: web::Path<String>,
    payload: web::Json<LiftFreezeRequest>,
) -> ActixResult<HttpResponse> {
    // Add permission check here — only CTAF officers
    // let user = extract_and_verify_user(req)?;
    // user.has_permission("aml.lift_freeze")?;

    let freeze_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| HttpError::InvalidInput("Invalid freeze_id".to_string()))?;

    match aml_service.lift_freeze(freeze_id, payload.lifted_by.clone()).await {
        Ok(freeze) => {
            // Log the action to audit trail
            // audit_logger.log_aml_lift(&freeze);

            Ok(HttpResponse::Ok().json(freeze))
        }
        Err(e) => {
            error!("Failed to lift freeze: {}", e);
            Ok(HttpResponse::InternalServerError().json(
                json!({"error": "Failed to lift freeze"}),
            ))
        }
    }
}
```

## Step 6: Add Audit Logging

Create an audit trail entry for all freeze/lift operations.

**In freeze_account handler:**
```rust
// After successful freeze
governance_service.audit_log(
    AuditEvent {
        action: "ACCOUNT_FREEZE",
        entity_type: "Account",
        entity_id: account_id.to_string(),
        user_id: current_user_id,
        details: json!({
            "reason": payload.reason,
            "freeze_id": freeze.id,
        }),
        timestamp: Utc::now(),
    }
).await?;
```

**In lift_freeze handler:**
```rust
// After successful lift
governance_service.audit_log(
    AuditEvent {
        action: "ACCOUNT_UNFREEZE",
        entity_type: "Account",
        entity_id: freeze.account_id.to_string(),
        user_id: current_user_id,
        details: json!({
            "freeze_id": freeze.id,
        }),
        timestamp: Utc::now(),
    }
).await?;
```

## Step 7: Add Permission Guards

Update your authorization middleware to enforce freeze/lift permissions.

**Example with casbin or similar:**
```rust
// In middleware or handler guards
pub async fn require_freeze_permission(user: &User) -> Result<(), AuthError> {
    if !user.has_permission("aml.freeze") {
        return Err(AuthError::Forbidden);
    }
    Ok(())
}

pub async fn require_lift_permission(user: &User) -> Result<(), AuthError> {
    if !user.has_permission("aml.lift_freeze") {
        return Err(AuthError::Forbidden);
    }
    Ok(())
}
```

## Step 8: Integration Tests

Create integration tests to verify the full flow.

**File**: `/crates/infrastructure/tests/aml_freeze_integration.rs`

```rust
#[tokio::test]
async fn test_freeze_account_integration() {
    // Setup
    let db = setup_test_db().await;
    let account_repo = PostgresAccountRepository::new(db.clone());
    let freeze_repo = PostgresAssetFreezeRepository::new(db.clone());
    let account_service = Arc::new(AccountService::new(
        Arc::new(account_repo),
        Arc::new(MockKycVerifier::new()),
    ));
    let adapter = Arc::new(AccountFreezeAdapter::new(account_service.clone()));
    let aml_service = AssetFreezeService::with_account_port(
        Arc::new(freeze_repo),
        adapter,
    );

    // Create account
    let customer_id = CustomerId::new();
    let account = account_service
        .open_account(customer_id.clone(), AccountType::Current)
        .await
        .unwrap();
    account_service
        .deposit(&account.id(), tnd(1000.0), "Initial deposit")
        .await
        .unwrap();

    // Verify account is active with available balance
    let account = account_service.find_by_id(&account.id()).await.unwrap();
    assert_eq!(account.status(), AccountStatus::Active);
    assert_eq!(account.available_balance().amount(), 1000.0);

    // Freeze account
    let freeze = aml_service
        .freeze_account(
            account.id().as_uuid().clone(),
            "Suspicious activity".to_string(),
            "compliance_officer".to_string(),
        )
        .await
        .unwrap();

    assert_eq!(freeze.status, "Active");

    // Verify account is frozen
    let frozen_account = account_service.find_by_id(&account.id()).await.unwrap();
    assert_eq!(frozen_account.status(), AccountStatus::Suspended);
    assert!(frozen_account.available_balance().is_zero());

    // Try to withdraw — should fail
    let result = account_service
        .withdraw(&account.id(), tnd(100.0), "Withdrawal attempt")
        .await;
    assert!(result.is_err());

    // Lift freeze
    let lifted = aml_service
        .lift_freeze(
            Uuid::parse_str(&freeze.id).unwrap(),
            "CTAF_officer".to_string(),
        )
        .await
        .unwrap();

    assert_eq!(lifted.status, "Lifted");

    // Verify account is unfrozen
    let unfrozen_account = account_service.find_by_id(&account.id()).await.unwrap();
    assert_eq!(unfrozen_account.status(), AccountStatus::Active);
    assert_eq!(unfrozen_account.available_balance().amount(), 1000.0);

    // Withdraw should now succeed
    let result = account_service
        .withdraw(&account.id(), tnd(100.0), "Withdrawal after unfreeze")
        .await;
    assert!(result.is_ok());
}
```

## Checklist for Integration

- [ ] Create `AccountFreezeAdapter` in infrastructure
- [ ] Add `update_account()` to `AccountService` if needed
- [ ] Wire up DI to create `AssetFreezeService` with adapter
- [ ] Add freeze/lift HTTP endpoints
- [ ] Add permission checks (compliance officer can freeze, CTAF can lift)
- [ ] Add audit logging for all freeze/lift operations
- [ ] Update API documentation (OpenAPI/Swagger)
- [ ] Add integration tests
- [ ] Test error scenarios (frozen account withdrawal, etc.)
- [ ] Manual testing with Postman/curl
- [ ] Security review (who can freeze, who can lift)
- [ ] Documentation for operators

## Error Handling

Common error scenarios to handle:

### Account Not Found
```rust
if aml_service.freeze_account(account_id, ...).await.is_err() {
    // Account doesn't exist — return 404
}
```

### Account Already Frozen
```rust
// Check before freezing
if aml_service.is_account_frozen(account_id).await? {
    return Err(HttpError::BadRequest("Account is already frozen".to_string()));
}
```

### Permission Denied
```rust
if !user.has_permission("aml.freeze") {
    return Err(HttpError::Forbidden("Insufficient permissions".to_string()));
}
```

## Performance Considerations

1. **Freeze Operation**:
   - Creates AssetFreeze record (1 INSERT)
   - Calls account_port.freeze_account() (1 SELECT + 1 UPDATE)
   - Total: 2 queries (should be <10ms)

2. **Lift Operation**: Same as freeze (2 queries)

3. **is_account_frozen Check**: 1 SELECT (should be <5ms)

4. **Caching**: Consider caching active freezes if checking frequently

## Deployment Notes

1. **Backward Compatibility**: Old code using `AssetFreezeService::new()` will still work
2. **Gradual Rollout**: Deploy adapter first without using it, then activate when ready
3. **Database**: Ensure PostgreSQL has appropriate indexes on account_id, status
4. **Migration**: If adding new columns/tables, create migrations first

## Monitoring & Alerts

Add metrics/alerts for:
- Number of active freezes
- Freeze/lift operation latency
- Failed freeze attempts (permissions, account not found, etc.)
- Unusual patterns (mass freezes)

```rust
// Prometheus metrics
lazy_static! {
    static ref ACCOUNT_FREEZES: Counter = Counter::new("aml_account_freezes_total", "Total account freezes").unwrap();
    static ref ACCOUNT_UNFREEZES: Counter = Counter::new("aml_account_unfreezes_total", "Total account unfreezes").unwrap();
    static ref FREEZE_OPERATION_DURATION: Histogram = Histogram::new("aml_freeze_operation_duration_seconds", "Freeze operation duration").unwrap();
}

// In freeze handler
let start = Instant::now();
let result = aml_service.freeze_account(...).await;
FREEZE_OPERATION_DURATION.observe(start.elapsed().as_secs_f64());
if result.is_ok() {
    ACCOUNT_FREEZES.inc();
}
```

## Questions & Troubleshooting

**Q: What if account_port errors?**
A: The error is caught and returned as a service error. Freeze operation fails and returns error to caller.

**Q: Can multiple freezes happen on same account?**
A: Only one freeze can be active at a time (per domain model). Multiple frozen freezes can exist (historical records).

**Q: What happens to in-flight transactions when freeze happens?**
A: Depends on timing. Best practice: check is_account_frozen() before processing any withdrawal/transfer.

**Q: Can account hold still work while frozen?**
A: No — frozen account can't do any operations (deposit, withdraw, apply hold, etc.).

**Q: How do I know who froze/lifted?**
A: Check freeze.ordered_by (who created) and freeze.lifted_by (who lifted).
