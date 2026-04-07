# STORY-AML-08: Quick Reference

## What Changed

### Added: `IAccountFreezePort` trait
**Location**: `/crates/application/src/aml/ports.rs` (lines 104-120)

A new port interface for managing account freezes in response to AML actions. This decouples the AML context from the Account context per DDD principles.

```rust
#[async_trait]
pub trait IAccountFreezePort: Send + Sync {
    async fn find_account_by_id(&self, account_id: Uuid) -> Result<Option<Account>, String>;
    async fn freeze_account(&self, account_id: Uuid) -> Result<Account, String>;
    async fn unfreeze_account(&self, account_id: Uuid) -> Result<Account, String>;
}
```

### Enhanced: `AssetFreezeService`
**Location**: `/crates/application/src/aml/service.rs` (lines 504-641)

#### New Field
```rust
pub struct AssetFreezeService {
    freeze_repo: Arc<dyn IAssetFreezeRepository>,
    account_port: Option<Arc<dyn IAccountFreezePort>>,  // NEW
}
```

#### New Constructor
```rust
pub fn with_account_port(
    freeze_repo: Arc<dyn IAssetFreezeRepository>,
    account_port: Arc<dyn IAccountFreezePort>,
) -> Self
```

#### Updated Methods
- **`freeze_account()`** (line 534): Now calls `account_port.freeze_account()` to set available_balance to 0
- **`lift_freeze()`** (line 561): Now calls `account_port.unfreeze_account()` to restore available_balance
- **`is_account_frozen()`** (line 596): Already existed, unchanged

### Added Tests
**Location**: `/crates/application/src/aml/service.rs` (lines 1093-1191)

- `test_freeze_account_immediate()` - Basic freeze without port
- `test_freeze_and_lift()` - Full lifecycle
- `test_freeze_account_with_port_integration()` - With port integration

## How to Use

### Without Account Port (Basic - Backward Compatible)
```rust
let service = AssetFreezeService::new(Arc::new(freeze_repo));
let freeze = service.freeze_account(account_id, reason, ordered_by).await?;
// Account freeze not applied (no port)
```

### With Account Port (Full Integration)
```rust
let service = AssetFreezeService::with_account_port(
    Arc::new(freeze_repo),
    Arc::new(account_port_impl),
);
let freeze = service.freeze_account(account_id, reason, ordered_by).await?;
// Account status set to Suspended, available_balance set to 0
```

## What Gets Frozen vs What Doesn't

When `freeze_account()` is called:

**Gets Frozen** (with account port):
- Account status → `Suspended`
- available_balance → `0` (prevents new withdrawals/transfers)

**Stays the Same**:
- Total balance (for accounting audit trail)
- All movements/history (for compliance records)

## Domain Model Flow

```
Investigation concludes → AML Officer freezes account
                              ↓
                AssetFreezeService.freeze_account()
                              ↓
        ┌─────────────────────┴─────────────────────┐
        ↓                                           ↓
Create AssetFreeze entity            Call IAccountFreezePort
(domain: AML context)                (Application layer)
        ↓                                           ↓
Save to freeze_repo                  Account.freeze()
(infrastructure)                      ↓
                              Set status=Suspended
                              Set available_balance=0
                              (domain: Account context)

---

Time passes... CTAF authorizes lift
                              ↓
                AssetFreezeService.lift_freeze()
                              ↓
        ┌─────────────────────┴─────────────────────┐
        ↓                                           ↓
Update AssetFreeze entity            Call IAccountFreezePort
(domain: AML context)                (Application layer)
        ↓                                           ↓
Save to freeze_repo                  Account.unfreeze()
(infrastructure)                      ↓
                              Set status=Active
                              Restore available_balance
                              (domain: Account context)
```

## Key Properties

| Aspect | Detail |
|--------|--------|
| **Freeze Type** | Immediate (INV-09) — no pending/approval state |
| **Who Can Freeze** | Any authenticated user with permission (enforced at API layer) |
| **Who Can Lift** | Only CTAF officers (must pass `lifted_by` parameter) |
| **Effect on Transactions** | Withdraw/Transfer fail with `InsufficientFunds` or `AccountSuspended` |
| **Effect on Deposits** | Also fail (account suspended prevents all movements) |
| **Accounting Impact** | Balance unchanged; only available_balance is 0 |
| **Audit Trail** | frozen_at, frozen_reason, ordered_by, lifted_at, lifted_by all recorded |
| **Concurrent Freezes** | Each freeze is independent; only one can be active at a time |

## Integration Checklist

To fully integrate AML freezes with Accounts:

- [ ] **Infrastructure**: Create adapter implementing `IAccountFreezePort`
  - Location: `/crates/infrastructure/src/aml/account_freeze_adapter.rs`
  - Must call `AccountService.freeze_account()` and `.unfreeze_account()`

- [ ] **DI Container**: Register the adapter in application startup
  - Inject into `AssetFreezeService::with_account_port()`

- [ ] **API Handlers**: Add freeze/lift endpoints
  - `POST /api/v1/aml/freezes` to create
  - `PATCH /api/v1/aml/freezes/{id}/lift` to lift

- [ ] **Permission Checks**:
  - Only compliance officers can freeze
  - Only CTAF can lift
  - Add `@RequirePermission` guards

- [ ] **Audit Logging**: Log all freeze/lift operations
  - Use governance context audit trail

- [ ] **Integration Tests**: Test with real Account service
  - Verify withdraw fails when frozen
  - Verify withdraw succeeds when unfrozen

## Common Patterns

### Check Before Transaction
```rust
if aml_service.is_account_frozen(account_id).await? {
    return Err(TransactionError::AccountFrozen);
}
```

### Graceful Degradation
```rust
// Port integration is optional — service works without it
let service = AssetFreezeService::new(freeze_repo);
// Freezes are recorded but account.available_balance not updated
// Later, upgrade to with_account_port() for full integration
```

### Error Handling
```rust
match aml_service.freeze_account(account_id, reason, ordered_by).await {
    Ok(freeze) => {
        // Freeze successful; account is now suspended
        // All withdrawals/transfers will fail
    }
    Err(e) => {
        // Domain error (invalid reason/ordered_by) or internal error
        // Freeze was not created
    }
}
```

## Testing Strategy

1. **Unit Tests** (included): Test freeze/lift lifecycle without account service
2. **Integration Tests** (todo): Test with real Account service and repos
3. **E2E Tests** (todo): Test API endpoints and permission checks
4. **Regression Tests** (needed): Ensure Account operations fail when frozen

## Files Modified

| File | Change | Lines |
|------|--------|-------|
| `crates/application/src/aml/ports.rs` | Added `IAccountFreezePort` trait | 104-120 |
| `crates/application/src/aml/service.rs` | Enhanced `AssetFreezeService` with account integration | 6, 510, 522-530, 534-557, 561-593, 1093-1191 |

## Backward Compatibility

✓ `AssetFreezeService::new()` still works (account_port is optional)
✓ Existing services unaffected
✓ No changes to domain entities
✓ No breaking API changes
