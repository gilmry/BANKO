# BANKO Reporting Advanced Services - Developer Guide

## Quick Start

### Installation & Setup

1. **Domain Layer** imports from `banko_domain::reporting::*`
2. **Application Layer** imports from `banko_application::reporting::*`
3. All services are dependency-injected via trait objects

### Basic Usage

#### Creating a Scheduled Report

```rust
use banko_domain::reporting::{ScheduleFrequency, ScheduledReportId};
use banko_application::reporting::ScheduledReportService;
use std::sync::Arc;
use uuid::Uuid;

let service = ScheduledReportService::new(repo); // Arc<dyn IScheduledReportRepository>

let report = service.create_scheduled_report(
    "Daily Prudential Report".to_string(),
    Some("Daily BCT prudential report".to_string()),
    "Prudential".to_string(),
    ScheduleFrequency::Daily,
    Some("0 9 * * *".to_string()), // Cron: daily at 9 AM
    None,
    Uuid::new_v4(),
).await?;

println!("Created: {}", report.name());
```

#### Setting Up Distribution

```rust
use banko_domain::reporting::{DistributionChannel};

let dist = service.create_distribution(
    report.report_id().to_string(),
    DistributionChannel::Email,
    vec!["compliance@bank.tn".to_string(), "audit@bank.tn".to_string()],
).await?;

// Add more recipients dynamically
distribution_service.add_recipient(&dist.distribution_id(), "cfo@bank.tn".to_string()).await?;
```

#### Archiving Reports (7-Year Retention)

```rust
use banko_domain::reporting::ReportFormatType;

let archive = service.archive_report(
    "report-001".to_string(),
    "s3://banko-reports/2026-03/report-001.xbrl".to_string(),
    "sha256_hash_here".to_string(),
    ReportFormatType::Xbrl,
    102400, // size in bytes
    7, // retention years (BCT standard)
).await?;

// Check expiry
if archive.is_expired() {
    println!("Report ready for deletion");
} else {
    println!("Days until expiry: {}", archive.days_until_expiry());
}
```

#### Creating Ad-Hoc Reports

```rust
use banko_domain::reporting::ReportFormatType;

let adhoc = service.create_adhoc_report(
    "Customer Transaction Summary".to_string(),
    Some("Monthly transactions by account type".to_string()),
    serde_json::json!({
        "account_type": "Checking",
        "min_amount": 1000.0
    }),
    vec![
        "account_id".to_string(),
        "customer_name".to_string(),
        "total_amount".to_string(),
        "transaction_count".to_string(),
    ],
    ReportFormatType::Csv,
    Uuid::new_v4(),
).await?;

// Execute the report
adhoc_service.execute_adhoc_report(&adhoc.adhoc_report_id()).await?;
```

#### Tax Reporting (TVA)

```rust
use banko_domain::reporting::TaxReportType;
use chrono::NaiveDate;

let tax_report = service.generate_tax_report(
    TaxReportType::Tva,
    NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
    NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
    1_000_000.0, // Total taxable amount
    180_000.0,   // VAT amount (18%)
    serde_json::json!({
        "vat_rate": 0.18,
        "monthly_breakdown": [
            {"month": 1, "amount": 300_000.0, "tax": 54_000.0},
            {"month": 2, "amount": 350_000.0, "tax": 63_000.0},
            {"month": 3, "amount": 350_000.0, "tax": 63_000.0},
        ]
    }),
    Uuid::new_v4(),
).await?;
```

#### IFRS 9 Reporting (ECL Staging)

```rust
use banko_domain::reporting::{Ifrs9Report, StagingAnalysis, CreditStage, TransitionMatrix};
use chrono::NaiveDate;

let staging = vec![
    StagingAnalysis {
        stage: CreditStage::Stage1,
        loan_count: 500,
        ecl_amount: 250_000.0,
        probability_of_default: 0.01,
        loss_given_default: 0.4,
        exposure_at_default: 25_000_000.0,
    },
    StagingAnalysis {
        stage: CreditStage::Stage2,
        loan_count: 80,
        ecl_amount: 200_000.0,
        probability_of_default: 0.05,
        loss_given_default: 0.5,
        exposure_at_default: 8_000_000.0,
    },
    StagingAnalysis {
        stage: CreditStage::Stage3,
        loan_count: 20,
        ecl_amount: 500_000.0,
        probability_of_default: 0.25,
        loss_given_default: 0.7,
        exposure_at_default: 2_857_000.0,
    },
];

let transitions = vec![
    TransitionMatrix {
        from_stage: CreditStage::Stage1,
        to_stage: CreditStage::Stage2,
        count: 10,
        percentage: 2.0, // 2% of Stage 1
    },
];

let ifrs9 = service.generate_ifrs9_report(
    NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
    staging,
    transitions,
    950_000.0, // Total ECL
    Uuid::new_v4(),
).await?;

// Query by stage
let stage1_ecl = ifrs9.get_stage_ecl(CreditStage::Stage1);
let stage1_count = ifrs9.get_stage_count(CreditStage::Stage1);
```

## Entity Relationships

```
ScheduledReport
  ├─ name, description
  ├─ frequency (Daily, Weekly, Monthly, etc.)
  ├─ cron_expression (optional, for advanced scheduling)
  └─ lifecycle: active → execute → deactivate

ReportDistribution
  ├─ report_id (reference to RegulatoryReport)
  ├─ channel (Email, Portal, SFTP, API)
  ├─ recipients (dynamic list)
  └─ lifecycle: active → add/remove recipients → deactivate

ReportArchive
  ├─ report_id (audit trail reference)
  ├─ storage_path (S3/MinIO compatible)
  ├─ content_hash (SHA256 integrity check)
  ├─ retention_until (7 years for BCT)
  └─ format (JSON, CSV, XBRL, XML, Excel, PDF)

AdHocReport
  ├─ name, description
  ├─ filters (JSON configuration)
  ├─ columns (user-selected)
  ├─ format (output format)
  └─ created_by (user audit trail)

TaxReport
  ├─ tax_type (TVA, WithholdingTax, AnnualTaxSummary)
  ├─ period (period_start, period_end)
  ├─ total_amount, tax_amount
  └─ details (JSON breakdown)

Ifrs9Report
  ├─ as_of (point-in-time date)
  ├─ staging_analysis (Stage 1/2/3 breakdown)
  ├─ transition_matrices (stage migration tracking)
  └─ total_ecl (sum of stage ECLs)
```

## Error Handling

All services return `Result<T, ReportingServiceError>`:

```rust
use banko_application::reporting::ReportingServiceError;

match service.create_scheduled_report(...).await {
    Ok(report) => println!("Created: {}", report.name()),
    Err(ReportingServiceError::InvalidInput(msg)) => eprintln!("Bad input: {}", msg),
    Err(ReportingServiceError::DomainError(msg)) => eprintln!("Domain rule violation: {}", msg),
    Err(ReportingServiceError::Internal(msg)) => eprintln!("Database error: {}", msg),
    Err(ReportingServiceError::ReportNotFound) => eprintln!("Report not found"),
    Err(_) => eprintln!("Other error"),
}
```

## Testing

### Unit Tests

```bash
# Domain tests
cargo test --lib reporting::advanced::tests

# Application tests
cargo test --lib application::reporting::advanced_service::tests
```

### Integration Tests

```bash
# Full reporting flow
cargo test --test reporting_integration
```

### Mocking Repositories

```rust
use async_trait::async_trait;
use banko_application::reporting::IScheduledReportRepository;

struct MockScheduledReportRepository {
    reports: Arc<tokio::sync::RwLock<Vec<ScheduledReport>>>,
}

#[async_trait]
impl IScheduledReportRepository for MockScheduledReportRepository {
    async fn save(&self, report: &ScheduledReport) -> Result<(), String> {
        self.reports.write().await.push(report.clone());
        Ok(())
    }
    // ... implement other methods
}

#[tokio::test]
async fn test_service() {
    let repo = Arc::new(MockScheduledReportRepository {
        reports: Arc::new(tokio::sync::RwLock::new(Vec::new())),
    });
    let service = ScheduledReportService::new(repo);
    // ... test logic
}
```

## Scheduling Integration

### With External Schedulers

```rust
// Example: APScheduler (Python) or Quartz (Java) integration point

// 1. List due reports
let due = service.list_due_scheduled_reports().await?;

// 2. For each due report, call generate_report() on ReportingService

// 3. After generation, mark as executed
service.mark_executed(&report.scheduled_report_id()).await?;

// 4. If distribution set, call distribute()
for dist in distribution_service.get_report_distributions(report_id).await? {
    distribute_via_channel(dist.channel(), dist.recipients(), report_data).await?;
}

// 5. Archive the report
archive_service.archive_report(
    report_id,
    storage_path,
    content_hash,
    ReportFormatType::Xbrl,
    size_bytes,
    7, // BCT retention
).await?;
```

## Persistence Layer Notes

When implementing repositories, follow these patterns:

### ScheduledReportRepository

```sql
CREATE TABLE scheduled_reports (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    report_type VARCHAR(100) NOT NULL,
    frequency VARCHAR(50) NOT NULL,
    cron_expression VARCHAR(50),
    next_run TIMESTAMP WITH TIME ZONE,
    last_run TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT true,
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    INDEX idx_active (is_active),
    INDEX idx_next_run (next_run)
);
```

### ReportArchiveRepository

```sql
CREATE TABLE report_archives (
    id UUID PRIMARY KEY,
    report_id VARCHAR(255) NOT NULL,
    storage_path VARCHAR(1000) NOT NULL,
    content_hash VARCHAR(256) NOT NULL,
    format VARCHAR(50) NOT NULL,
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    retention_until TIMESTAMP WITH TIME ZONE NOT NULL,
    INDEX idx_report_id (report_id),
    INDEX idx_retention_until (retention_until),
    UNIQUE (report_id, created_at)
);
```

## Performance Considerations

1. **Batch Operations**: Use bulk insert for multiple distributions
2. **Archival Cleanup**: Schedule daily job to purge expired archives
3. **Indexing**: Index `next_run`, `retention_until`, `report_id`
4. **Caching**: Cache active schedules (update frequency: ~1min)
5. **Query Optimization**: Use appropriate `limit` and `offset` for listing

## Compliance Checklist

- [x] 7-year retention for BCT reports
- [x] Audit trail for all operations
- [x] Content integrity via hashing
- [x] XBRL export support
- [x] IFRS 9 staging compliance
- [x] Tax reporting integration
- [x] Schedule-based automation
- [x] Multi-channel distribution

## FAQ

**Q: How do I handle report generation failures?**
A: Catch `ReportingServiceError::Internal`, log the error, and retry with exponential backoff. Mark scheduled report as failed if max retries exceeded.

**Q: Can reports be scheduled based on external events?**
A: Yes, set `next_run` to trigger on custom events. Update via service after event occurs.

**Q: How long does archival take?**
A: Depends on report size and S3 latency (~100-500ms typical). Consider async background job.

**Q: Can I modify archived reports?**
A: No. Archive is immutable by design. Create new version and archive separately.

**Q: How to generate IFRS 9 from upstream services?**
A: Use `IEclDataProvider` from existing Ifrs9ReportService, but aggregate via new `AdvancedIfrs9ReportService` for full staging analysis.

---

**Version**: 1.0
**Last Updated**: 2026-04-07
**Compliance Level**: BMAD v4.0.1 (100%)
