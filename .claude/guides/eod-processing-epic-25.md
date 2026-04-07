# EPIC-25: End-of-Day Processing for BANKO

## Overview

EPIC-25 implements comprehensive End-of-Day (EOD) processing for the BANKO banking platform. The implementation follows hexagonal architecture principles and is organized into three main components:

1. **EOD Orchestrator** — Coordinates execution of all EOD steps
2. **Interest Accrual Service** — Calculates and records daily interest
3. **Reconciliation Service** — Validates general ledger account balances

## Architecture

### Component Structure

```
Domain Layer (Entities & Services)
    ↓
Application Layer (Use Cases & Ports)
    ├── InterestAccrualService
    ├── ReconciliationService
    └── Ports (IAccrualRepository, IInterestAccountProvider, IReconciliationRepository)
    ↓
Infrastructure Layer (Adapters & Implementations)
    ├── EodOrchestrator (coordinator)
    ├── EodScheduler (background scheduler)
    └── Database Repositories (eod schema)
```

### Database Schema

All EOD-related tables are in the `eod` schema:

- **`eod.runs`** — Metadata for each EOD execution
- **`eod.step_results`** — Results for each step within a run
- **`eod.interest_accruals_daily`** — Daily interest accrual records
- **`eod.reconciliation_reports`** — GL reconciliation reports

See migration file: `/sessions/nice-vigilant-rubin/mnt/BANKO/migrations/20260406000024_eod_schema.sql`

## Components

### 1. EOD Orchestrator (`infrastructure/src/jobs/eod_orchestrator.rs`)

**Responsibility:** Orchestrate the sequential execution of all EOD steps with failure handling and rollback.

#### Key Types

- **`EodStepStatus`** — Enum: `Pending | Running | Completed | Failed(String) | Skipped`
- **`EodContext`** — Contains `date`, `started_at`, `dry_run` flag
- **`EodStepResult`** — Result from a single step execution
- **`EodReport`** — Complete report of an EOD run

#### EodStep Trait

All EOD steps must implement this trait:

```rust
#[async_trait]
pub trait EodStep: Send + Sync {
    fn name(&self) -> &str;
    fn is_critical(&self) -> bool;
    async fn execute(&self, context: &EodContext) -> Result<EodStepResult, String>;
    async fn rollback(&self, context: &EodContext) -> Result<(), String>;
}
```

#### Default Steps (in order)

1. **InterestAccrualStep** (Critical)
2. **ReconciliationStep** (Critical)
3. **FeeCalculationStep** (Non-critical)
4. **ChequeCompensationStep** (Non-critical)
5. **CardSpendingResetStep** (Non-critical)
6. **ReportingSnapshotStep** (Non-critical)

#### Execution Logic

1. Execute each step sequentially
2. **On critical failure:** Rollback all completed critical steps in reverse order, then stop
3. **On non-critical failure:** Log, skip, continue with next step
4. **On retry-able failure:** Retry up to `max_retries` (default: 3) with `retry_delay_secs` (default: 300 = 5 min)
5. Return comprehensive `EodReport`

#### Usage

```rust
// Run full EOD processing
let orchestrator = EodOrchestrator::new();
let report = orchestrator.run(NaiveDate::from_ymd_opt(2026, 4, 6).unwrap()).await;

// Rerun single step
let result = orchestrator.run_single_step("interest_accrual", date).await;

// Spawn daily scheduler (runs at 23:00 UTC every day)
let _handle = EodScheduler::spawn(23, 0);

// Manual trigger
let report = EodScheduler::run_now(date).await;
```

#### Tests

Comprehensive tests included:

- Full success run
- Critical failure triggers rollback
- Non-critical failure continues
- Rollback logic verification
- Dry run mode
- Single step rerun
- Report generation
- Retry logic
- Status display

### 2. Interest Accrual Service (`application/src/accounting/interest_accrual_service.rs`)

**Responsibility:** Calculate and record daily interest accrual for all interest-bearing accounts.

#### Key Types

- **`AccrualMethod`** — `Simple | Compound`
- **`AccrualType`** — `Credit` (earns interest) | `Debit` (pays interest)
- **`AccountType`** — `Savings | TermDeposit | Loan`
- **`CapitalizationFrequency`** — `Monthly | Quarterly | Annually | None`
- **`AccrualEntry`** — Represents a single daily accrual
- **`AccrualBatchResult`** — Statistics from a batch accrual run

#### Ports

Two main port interfaces must be implemented:

**`IAccrualRepository`** — Persists and retrieves accrual entries:
```rust
async fn save(&self, entry: &AccrualEntry) -> Result<(), String>;
async fn find_by_account_and_date(...) -> Result<Option<AccrualEntry>, String>;
async fn find_by_account(account_id, from, to) -> Result<Vec<AccrualEntry>, String>;
async fn sum_accrued(account_id, from, to) -> Result<Decimal, String>;
```

**`IInterestAccountProvider`** — Provides list of interest-bearing accounts:
```rust
async fn list_interest_bearing_accounts(&self) -> Result<Vec<InterestAccountInfo>, String>;
```

#### Key Methods

**`accrue_daily(date)`** — Calculates daily interest for all accounts
- For each account: `daily_interest = principal × annual_rate / 365`
- Determines accrual type based on account type
- Tracks credit and debit totals separately
- Returns batch statistics

**`get_accrued_interest(account_id, from, to)`** — Retrieves total accrued interest over a date range

**`capitalize_monthly(date)`** — Capitalizes interest (adds to principal) for monthly accounts at month-end

**`capitalize_quarterly(date)`** — Capitalizes quarterly accruals

**`capitalize_annually(date)`** — Capitalizes annual accruals

#### Example

```rust
let service = InterestAccrualService::new(repo, provider);

// Run daily accrual
let result = service.accrue_daily(date).await?;
println!("Processed {}, credit: {}, debit: {}",
    result.accounts_processed,
    result.total_credit_interest,
    result.total_debit_interest);

// Get total interest for an account over a period
let total = service.get_accrued_interest(account_id, start_date, end_date).await?;

// Capitalize monthly interest
let capitalized = service.capitalize_monthly(date).await?;
```

#### Tests

8 comprehensive tests:
- Simple interest calculation
- Loan interest calculation
- Zero-balance account skipping
- Batch processing
- Accrued interest retrieval
- Monthly capitalization
- Credit vs. debit accrual types
- Empty account list

### 3. Reconciliation Service (`application/src/accounting/reconciliation_service.rs`)

**Responsibility:** Reconcile general ledger accounts and produce reconciliation reports.

#### Key Types

- **`ReconciliationStatus`** — `Balanced | Variance | AutoResolved | ManualReviewRequired`
- **`AccountReconciliation`** — Details for a single GL account
- **`AutoResolution`** — Details of auto-resolved variance
- **`ReconciliationReport`** — Complete reconciliation report

#### Ports

**`IReconciliationRepository`** — Persists and retrieves reports:
```rust
async fn save(&self, report: &ReconciliationReport) -> Result<(), String>;
async fn find_by_date(date: NaiveDate) -> Result<Option<ReconciliationReport>, String>;
async fn find_all(offset, limit) -> Result<Vec<ReconciliationReport>, String>;
```

Uses existing **`ILedgerRepository`** to fetch account balances.

#### Key Methods

**`reconcile(date)`** — Reconciles all GL accounts for a date
1. Retrieves all account balances from ledger
2. For each account: computes `variance = |debits - credits|`
3. Determines status:
   - `variance == 0` → Balanced
   - `variance <= 1.0 TND` → AutoResolved (rounding tolerance)
   - `variance > 1.0 TND` → ManualReviewRequired
4. Computes overall status (priority: ManualReviewRequired > AutoResolved > Balanced)
5. Persists report
6. Returns complete report

**`get_report(date)`** — Retrieves previously generated report

**`list_reports(from, to, limit)`** — Retrieves reports within a date range

#### Example

```rust
let service = ReconciliationService::new(ledger_repo, reconciliation_repo);

// Run reconciliation
let report = service.reconcile(date).await?;
println!("Status: {}, Variance: {}",
    report.overall_status,
    report.total_variance);

for account in &report.accounts {
    if account.status != ReconciliationStatus::Balanced {
        println!("Account {} needs review: {}",
            account.account_code, account.variance);
    }
}

// Retrieve historical report
if let Some(historical) = service.get_report(date).await? {
    println!("Historical variance: {}", historical.total_variance);
}
```

#### Tests

8 comprehensive tests:
- Balanced accounts
- Variance detection
- Small variance auto-resolution (rounding)
- Multiple accounts with mixed status
- Empty ledger handling
- Report retrieval
- Report listing with date range
- Status display

## Usage Examples

### Running Full EOD Processing

```rust
// In a background job or API endpoint
let orchestrator = EodOrchestrator::new();
let eod_date = Utc::now().date_naive() - chrono::Duration::days(1);

let report = orchestrator.run(eod_date).await;

match report.overall_status {
    EodOverallStatus::Completed => {
        info!("EOD completed successfully");
    }
    EodOverallStatus::PartiallyCompleted => {
        warn!("EOD partially completed, some steps failed");
    }
    EodOverallStatus::Failed => {
        error!("EOD failed completely");
    }
}

// Log individual step results
for step_result in report.steps {
    info!("Step '{}': {} ({} records in {}ms)",
        step_result.step_name,
        step_result.status,
        step_result.records_processed,
        step_result.duration_ms);
}
```

### Running Scheduled EOD

```rust
// In application startup
let _eod_handle = EodScheduler::spawn(23, 0); // 23:00 UTC daily
```

### Adding Custom EOD Step

```rust
struct MyCustomStep;

#[async_trait::async_trait]
impl EodStep for MyCustomStep {
    fn name(&self) -> &str { "my_custom_step" }
    fn is_critical(&self) -> bool { false }

    async fn execute(&self, context: &EodContext) -> Result<EodStepResult, String> {
        // Your implementation
        Ok(EodStepResult {
            step_name: self.name().to_string(),
            status: EodStepStatus::Completed,
            records_processed: 100,
            duration_ms: 250,
            details: Some("Custom step completed".to_string()),
        })
    }

    async fn rollback(&self, _context: &EodContext) -> Result<(), String> {
        // Rollback logic if critical
        Ok(())
    }
}

// Use in orchestrator
let steps: Vec<Box<dyn EodStep + Send + Sync>> = vec![
    Box::new(InterestAccrualStep),
    Box::new(MyCustomStep),
];
let orchestrator = EodOrchestrator::with_steps(steps);
```

## Integration Points

### 1. Accounting Context
- Uses existing `IJournalRepository` for posting accrual entries
- Uses `ILedgerRepository` for balance information
- Stores results in `eod.interest_accruals_daily` and `eod.reconciliation_reports`

### 2. Reporting Context
- EOD results feed into regulatory reporting
- Interest accruals affect IFRS 9 ECL calculations
- Reconciliation status impacts closing procedures

### 3. Front-end Integration
- EOD reports can be exposed via REST API
- Monitoring dashboard can show step-by-step progress
- Manual step reruns for operational recovery

## Testing

### Unit Tests
Each service includes 8-10 comprehensive unit tests with mock repositories:

```bash
# Run EOD orchestrator tests
cargo test eod_orchestrator --lib

# Run interest accrual tests
cargo test interest_accrual_service --lib

# Run reconciliation tests
cargo test reconciliation_service --lib
```

### Integration Tests
When database infrastructure is implemented, add integration tests:

```bash
cargo test --test eod_integration
```

## Performance Considerations

- **Batch Processing:** All accounts accrued in single transaction
- **Indexing:** Database indexes on date, account_id, status for fast queries
- **Retry Logic:** Exponential backoff with configurable delay (default 300s)
- **Dry Run Mode:** Execute without persisting changes (for validation)
- **Parallel Steps:** Non-critical steps could run in parallel (future optimization)

## Error Handling

- **Critical Step Failure:** Triggers immediate rollback and stops processing
- **Non-Critical Failure:** Logged as warning, processing continues
- **Transient Errors:** Automatic retry up to max_retries
- **Rollback Failures:** Logged as critical but doesn't fail entire EOD

## Future Enhancements

1. **Parallel Execution:** Run non-critical steps concurrently
2. **Distributed Processing:** Scale EOD across multiple nodes
3. **Incremental Accrual:** Only process changed accounts
4. **Performance Metrics:** Track step duration and optimize
5. **Alternative Accounting:** Support multiple accounting standards
6. **Audit Trail:** Immutable log of all changes
7. **Compensating Transactions:** Better rollback mechanisms
8. **Configurable Rounding Tolerance:** Make 1.0 TND threshold configurable

## Files Created

1. **Infrastructure Layer:**
   - `/crates/infrastructure/src/jobs/eod_orchestrator.rs` — Orchestrator and scheduler
   - `/crates/infrastructure/src/jobs/mod.rs` — Updated with exports

2. **Application Layer:**
   - `/crates/application/src/accounting/interest_accrual_service.rs` — Interest service
   - `/crates/application/src/accounting/reconciliation_service.rs` — Reconciliation service
   - `/crates/application/src/accounting/mod.rs` — Updated with exports

3. **Database:**
   - `/migrations/20260406000024_eod_schema.sql` — Database schema

## Running Tests

```bash
# Run all EOD tests
cargo test eod --lib

# Run specific test
cargo test test_full_success_run -- --nocapture

# Run with output
cargo test -- --nocapture --test-threads=1
```

---

For questions or contributions, refer to CONTRIBUTING.md and SECURITY.md.
