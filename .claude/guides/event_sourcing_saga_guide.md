# Event Sourcing & Saga Pattern Developer Guide

## Quick Start

### Publishing Events

```rust
use banko_domain::events::{DomainEvent, StoredEvent, AccountOpenedEvent};
use uuid::Uuid;

// Create a domain event
let account_id = Uuid::new_v4();
let customer_id = Uuid::new_v4();
let event = AccountOpenedEvent::new(
    account_id,
    customer_id,
    "Checking".to_string(),
);

// Convert to StoredEvent for persistence
let stored = StoredEvent::new(
    event.aggregate_id(),
    event.aggregate_type().to_string(),
    event.event_type().to_string(),
    event.payload(),
);

// Publish via event store service
let sequence_number = event_store_service.append_event(stored).await?;
```

### Creating Custom Domain Events

```rust
use banko_domain::events::DomainEvent;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPostedEvent {
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub amount: String,
    pub timestamp: DateTime<Utc>,
}

impl DomainEvent for TransactionPostedEvent {
    fn event_type(&self) -> &str {
        "TransactionPosted"
    }

    fn aggregate_id(&self) -> Uuid {
        self.transaction_id
    }

    fn aggregate_type(&self) -> &str {
        "Transaction"
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Serialization failed")
    }

    fn event_id(&self) -> Uuid {
        Uuid::new_v5(&self.transaction_id, b"TransactionPosted")
    }
}
```

### Creating Event Handlers

```rust
use banko_domain::events::{EventHandler, StoredEvent};
use async_trait::async_trait;

struct AccountOpeningNotificationHandler;

#[async_trait]
impl EventHandler for AccountOpeningNotificationHandler {
    async fn handle(&self, event: &StoredEvent) -> Result<(), String> {
        if event.event_type == "AccountOpened" {
            let customer_id: Option<Uuid> = serde_json::from_value(
                event.payload.get("customer_id").cloned().unwrap_or_default()
            ).ok();

            if let Some(customer_id) = customer_id {
                println!("Sending welcome email to customer {}", customer_id);
                // Send notification logic here
            }
        }
        Ok(())
    }

    fn handles_event_types(&self) -> Vec<String> {
        vec!["AccountOpened".to_string()]
    }
}

// Subscribe handler
event_bus.subscribe(Box::new(AccountOpeningNotificationHandler)).await;
```

### Replaying Events

```rust
// Replay all events from sequence number 100
let result = event_store_service.replay_from(100).await?;
println!("Replayed {} events (seq {} to {})",
    result.events_replayed,
    result.from_sequence,
    result.to_sequence
);

// Get aggregate's complete history
let events = event_store_service.get_aggregate_events(account_id).await?;
println!("Found {} events for account", events.len());

// Rebuild aggregate (uses snapshots if available)
let events = event_store_service.rebuild_aggregate(account_id).await?;
```

## Saga Pattern

### Creating a Saga Step

```rust
use banko_application::saga::{SagaStep, SagaContext, SagaError};
use async_trait::async_trait;

struct DebitAccountStep {
    account_service: Arc<dyn IAccountService>,
}

#[async_trait]
impl SagaStep for DebitAccountStep {
    fn name(&self) -> &str {
        "DebitAccount"
    }

    async fn execute(&self, context: &mut SagaContext) -> Result<(), SagaError> {
        let account_id: Uuid = context.get("account_id")
            .ok_or_else(|| SagaError::ContextError("account_id not found".to_string()))?;
        let amount: String = context.get("amount")
            .ok_or_else(|| SagaError::ContextError("amount not found".to_string()))?;

        self.account_service.debit(account_id, &amount).await
            .map_err(|e| SagaError::StepFailed {
                step_name: self.name().to_string(),
                reason: e.to_string(),
            })?;

        // Store transaction ID for compensation
        context.set("transaction_id", Uuid::new_v4()).ok();

        Ok(())
    }

    async fn compensate(&self, context: &mut SagaContext) -> Result<(), SagaError> {
        let transaction_id: Uuid = context.get("transaction_id")
            .ok_or_else(|| SagaError::ContextError("transaction_id not found".to_string()))?;

        self.account_service.reverse_transaction(transaction_id).await
            .map_err(|e| SagaError::CompensationFailed {
                step_name: self.name().to_string(),
                reason: e.to_string(),
            })?;

        Ok(())
    }
}
```

### Executing a Saga

```rust
use banko_application::saga::{SagaOrchestrator, SagaContext};
use uuid::Uuid;

// Create saga steps
let step1 = Box::new(DebitAccountStep { /* ... */ });
let step2 = Box::new(CreditAccountStep { /* ... */ });
let step3 = Box::new(RecordTransactionStep { /* ... */ });

// Create orchestrator
let mut orchestrator = SagaOrchestrator::new(vec![step1, step2, step3]);
let saga_id = orchestrator.saga_id();

// Create context and set initial data
let mut context = SagaContext::with_idempotency_key(
    saga_id,
    format!("transfer-{}-{}", from_account, to_account)
);
context.set("account_id", from_account).ok();
context.set("amount", "1000.00").ok();

// Execute saga
match orchestrator.execute(&mut context).await {
    Ok(result) => {
        println!("Saga completed: {:?}", result.status);
        println!("Completed steps: {:?}", result.completed_steps);
        println!("Duration: {:?}", result.duration);
    }
    Err(e) => {
        eprintln!("Saga failed: {}", e);
        // Compensation has already been triggered automatically
    }
}
```

## Architecture Patterns

### Pattern 1: Event-Driven Updates

```
Domain Event Published
    ↓
Event Bus Routes to Handlers
    ↓
Handler 1 (Notification Service)
Handler 2 (Analytics)
Handler 3 (Compliance Check)
```

Use this when you need multiple services to react to business events without coupling.

### Pattern 2: Aggregate Reconstruction

```
Event Store Query
    ↓
Check for Latest Snapshot
    ↓
If Snapshot Found:
    Load Events Since Snapshot
Else:
    Load All Events for Aggregate
    ↓
Replay Events to Current State
```

Use this to reconstruct aggregate state for analysis or recovery.

### Pattern 3: Saga for Distributed Transactions

```
Step 1: DebitAccount
    ├─ Success → Continue
    └─ Failure → Compensate (Skip, no reversal)

Step 2: CreditAccount
    ├─ Success → Continue
    └─ Failure → Compensate Step 1

Step 3: RecordTransaction
    ├─ Success → Saga Complete
    └─ Failure → Compensate Step 2 and 1 (reverse order)
```

Use this for multi-step operations that need atomicity guarantees.

## Performance Considerations

### Snapshot Strategy

- Default threshold: 100 events
- Adjust for your aggregate size:
  - Small aggregates (simple data): 200-500 events
  - Large aggregates (complex state): 50-100 events
- Snapshots saved at threshold intervals only

```rust
let service = EventStoreService::new(
    event_repo,
    snapshot_repo,
    event_bus,
    200, // Custom threshold
);
```

### Event Replay Optimization

1. **Queries by aggregate ID**: Fast (indexed on aggregate_id)
2. **Queries by event type**: Fast (indexed on event_type)
3. **Time-range queries**: Use `get_events_by_type` with timestamp range
4. **Full replay**: Use pagination with `get_all_events`

```rust
// Good: Uses index on aggregate_id
let events = repo.get_events_for_aggregate(account_id).await?;

// Good: Uses index on event_type
let events = repo.get_events_by_type("AccountOpened", Some(start_time)).await?;

// Good: Paginated to avoid memory issues
let events = repo.get_all_events(0, 1000).await?;
```

## Error Handling

### Event Bus Errors

```rust
match event_bus.publish(event).await {
    Ok(_) => println!("Event published and all handlers succeeded"),
    Err(e) => eprintln!("Handler failed: {}", e),
}
```

When a handler fails, the error propagates. Implement retry logic in your handler or publish layer as needed.

### Saga Errors

```rust
match orchestrator.execute(&mut context).await {
    Ok(result) => { /* Saga succeeded */ }
    Err(SagaError::StepFailed { step_name, reason }) => {
        eprintln!("Step {} failed: {}", step_name, reason);
        // Compensation has already run
    }
    Err(SagaError::CompensationFailed { step_name, reason }) => {
        eprintln!("Could not compensate step {}: {}", step_name, reason);
        // Manual intervention required
    }
    Err(SagaError::ContextError(e)) => {
        eprintln!("Context error: {}", e);
    }
}
```

## Testing

### Unit Testing Events

```rust
#[test]
fn test_account_opened_event() {
    let event = AccountOpenedEvent::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        "Checking".to_string(),
    );

    assert_eq!(event.event_type(), "AccountOpened");
    assert_eq!(event.aggregate_type(), "Account");
    assert_eq!(event.version(), 1);
}
```

### Unit Testing Sagas

```rust
#[tokio::test]
async fn test_saga_compensation_on_failure() {
    let mut orchestrator = SagaOrchestrator::new(vec![
        Box::new(SuccessfulStep),
        Box::new(FailingStep),
        Box::new(UnreachedStep),
    ]);

    let mut context = SagaContext::new(orchestrator.saga_id());
    let result = orchestrator.execute(&mut context).await;

    assert!(result.is_err());
    // Verify compensation was triggered
}
```

## Database Queries

### Check Event Count

```sql
SELECT COUNT(*) FROM event_store;
```

### Find Events for Aggregate

```sql
SELECT * FROM event_store
WHERE aggregate_id = 'uuid-here'
ORDER BY sequence_number;
```

### Find Latest Snapshot

```sql
SELECT * FROM aggregate_snapshots
WHERE aggregate_id = 'uuid-here'
ORDER BY version DESC
LIMIT 1;
```

### Replay Progress

```sql
SELECT event_type, COUNT(*) as count
FROM event_store
WHERE timestamp >= '2026-04-06'
GROUP BY event_type
ORDER BY count DESC;
```

## Common Pitfalls

### 1. Forgetting Idempotency
Always include an idempotency key in sagas:
```rust
let context = SagaContext::with_idempotency_key(
    saga_id,
    format!("unique-key-for-this-operation")
);
```

### 2. Not Compensating Early Enough
Compensation runs in reverse order. Design steps so they can be safely reversed:
```rust
// Good: Reversible
1. Debit Account
2. Credit Account
3. Record Entry

// Bad: Hard to reverse
1. Complex computation
2. External API call
3. Update multiple tables
```

### 3. Storing Sensitive Data
Don't store passwords or secrets in events:
```rust
// Bad
PaymentEvent {
    account_number: "1234-5678-9012-3456",
}

// Good
PaymentEvent {
    account_id: Uuid,
    masked_account: "****-****-****-3456",
}
```

### 4. Unbounded Event Replays
Always use pagination for large replays:
```rust
// Bad: Could load millions of events
let all_events = repo.get_all_events(0, i64::MAX).await?;

// Good: Process in batches
for offset in (0..total_count).step_by(1000) {
    let batch = repo.get_all_events(offset, 1000).await?;
    // Process batch
}
```

## Integration Checklist

- [ ] Create domain events for your bounded context
- [ ] Implement EventHandler for key event types
- [ ] Subscribe handlers to event bus during initialization
- [ ] Implement IEventStoreRepository for your store
- [ ] Implement ISnapshotRepository if using snapshots
- [ ] Create SagaStep implementations for distributed operations
- [ ] Add integration tests for saga flows
- [ ] Set up monitoring for event bus throughput
- [ ] Create dashboard for replay progress
- [ ] Document event schema for API consumers

## References

- **Domain Events**: See `banko-domain::events::domain_event`
- **Event Bus**: See `banko-domain::events::event_bus`
- **Event Store**: See `banko-application::events::event_store`
- **Saga Pattern**: See `banko-application::saga::orchestrator`
- **Ports**: See `banko-application::events::ports`
