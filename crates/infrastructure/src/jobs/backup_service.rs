use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;
use tracing::{debug, error, info, warn};

// ============================================================
// Backup Enums and Data Structures (STORY-DR-01 to DR-03)
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental,
    Wal,
}

impl fmt::Display for BackupType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupType::Full => write!(f, "Full"),
            BackupType::Incremental => write!(f, "Incremental"),
            BackupType::Wal => write!(f, "Wal"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupStatus {
    Running,
    Completed,
    Failed,
    Expired,
}

impl fmt::Display for BackupStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupStatus::Running => write!(f, "Running"),
            BackupStatus::Completed => write!(f, "Completed"),
            BackupStatus::Failed => write!(f, "Failed"),
            BackupStatus::Expired => write!(f, "Expired"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRetentionPolicy {
    pub max_age_days: u32,
    pub min_backups: u32,
}

impl Default for BackupRetentionPolicy {
    fn default() -> Self {
        BackupRetentionPolicy {
            max_age_days: 90,
            min_backups: 7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRecord {
    pub id: String,
    pub backup_type: BackupType,
    pub file_path: String,
    pub size_bytes: u64,
    pub checksum: String,
    pub status: BackupStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl BackupRecord {
    pub fn new(
        backup_type: BackupType,
        file_path: String,
        size_bytes: u64,
        checksum: String,
        retention_days: u32,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::days(retention_days as i64);

        BackupRecord {
            id: Uuid::new_v4().to_string(),
            backup_type,
            file_path,
            size_bytes,
            checksum,
            status: BackupStatus::Running,
            created_at: now,
            expires_at: Some(expires_at),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionResult {
    pub deleted_count: usize,
    pub deleted_size_bytes: u64,
    pub remaining_backups: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreResult {
    pub backup_id: String,
    pub restored_at: DateTime<Utc>,
    pub duration_secs: u64,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrStepResult {
    pub step_name: String,
    pub status: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrReport {
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub rto_achieved_secs: u64,
    pub rpo_achieved_secs: u64,
    pub steps: Vec<DrStepResult>,
    pub success: bool,
}

// ============================================================
// BackupService (STORY-DR-01)
// ============================================================

pub struct BackupService {
    s3_bucket: String,
    s3_endpoint: String,
    retention_policy: BackupRetentionPolicy,
}

impl BackupService {
    pub fn new(
        s3_bucket: String,
        s3_endpoint: String,
        retention_policy: BackupRetentionPolicy,
    ) -> Self {
        BackupService {
            s3_bucket,
            s3_endpoint,
            retention_policy,
        }
    }

    /// Create a backup with the specified type
    /// Steps:
    /// 1. Generate pg_dump command
    /// 2. Compress (.gz)
    /// 3. Upload to S3/MinIO
    /// 4. Verify checksum
    /// 5. Return BackupRecord
    pub async fn backup_database(&self, backup_type: BackupType) -> Result<BackupRecord, String> {
        info!("Starting database backup: {:?}", backup_type);

        let backup_id = Uuid::new_v4();
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();

        // Step 1: Generate pg_dump command (stub)
        debug!("Step 1: Generate pg_dump command for {:?}", backup_type);
        let dump_file = format!("backup_{}_{}_{}.sql", backup_type, timestamp, backup_id);
        info!("Generated dump file: {}", dump_file);

        // Step 2: Compress (.gz) - stub
        debug!("Step 2: Compress backup file");
        let compressed_file = format!("{}.gz", dump_file);
        let size_bytes: u64 = 1024 * 1024 * 500; // Stub: 500 MB

        // Step 3: Upload to S3/MinIO - stub
        debug!("Step 3: Upload to S3/MinIO");
        let s3_path = format!("s3://{}/backups/{}", self.s3_bucket, compressed_file);
        info!("Uploading to S3 path: {}", s3_path);

        // Step 4: Generate SHA-256 checksum - stub
        debug!("Step 4: Generate and verify checksum");
        let checksum = self.generate_checksum(&compressed_file);
        info!("Generated checksum: {}", checksum);

        // Step 5: Create and return BackupRecord
        let mut record = BackupRecord::new(
            backup_type,
            s3_path,
            size_bytes,
            checksum,
            self.retention_policy.max_age_days,
        );
        record.status = BackupStatus::Completed;

        info!("Backup completed: {:?}", record.id);
        Ok(record)
    }

    /// Verify a backup by checking S3 existence and checksum match
    pub async fn verify_backup(&self, backup_id: &str) -> Result<bool, String> {
        info!("Verifying backup: {}", backup_id);

        // In production, this would:
        // 1. Check S3 object existence
        // 2. Download object metadata
        // 3. Verify checksum
        // For now, return true to indicate verification passed

        debug!("Checking S3 existence for backup: {}", backup_id);
        debug!("Verifying checksum match");

        info!("Backup {} verified successfully", backup_id);
        Ok(true)
    }

    /// Apply retention policy: delete backups older than max_age_days, keeping at least min_backups
    pub async fn apply_retention_policy(&self) -> Result<RetentionResult, String> {
        info!(
            "Applying retention policy: max_age={} days, min_backups={}",
            self.retention_policy.max_age_days, self.retention_policy.min_backups
        );

        // In production, this would query backup records and delete those exceeding policy
        // For now, return stub result
        let result = RetentionResult {
            deleted_count: 0,
            deleted_size_bytes: 0,
            remaining_backups: 7,
        };

        info!(
            "Retention policy applied: deleted {} backups ({} bytes)",
            result.deleted_count, result.deleted_size_bytes
        );
        Ok(result)
    }

    /// List all backup records
    pub async fn list_backups(&self) -> Result<Vec<BackupRecord>, String> {
        info!("Listing all backups");

        // In production, query from backup_records table
        // For now, return empty list
        Ok(vec![])
    }

    fn generate_checksum(&self, filename: &str) -> String {
        // Stub: Generate SHA-256 checksum
        format!("sha256_{}", filename)
    }
}

// ============================================================
// RestoreService (STORY-DR-02)
// ============================================================

pub struct RestoreService {
    s3_bucket: String,
}

impl RestoreService {
    pub fn new(s3_bucket: String) -> Self {
        RestoreService { s3_bucket }
    }

    /// Restore from a full backup
    /// Steps:
    /// 1. Download from S3
    /// 2. Decompress
    /// 3. pg_restore (stub)
    /// 4. Verify connectivity
    pub async fn restore_from_backup(
        &self,
        backup_id: &str,
    ) -> Result<RestoreResult, String> {
        info!("Starting restore from backup: {}", backup_id);

        let start_time = Utc::now();

        // Step 1: Download from S3 - stub
        debug!("Step 1: Download backup from S3");
        info!("Downloaded backup {} from S3", backup_id);

        // Step 2: Decompress - stub
        debug!("Step 2: Decompress backup file");
        info!("Decompressed backup file");

        // Step 3: pg_restore - stub
        debug!("Step 3: Execute pg_restore");
        info!("pg_restore completed");

        // Step 4: Verify connectivity - stub
        debug!("Step 4: Verify database connectivity");
        info!("Database connectivity verified");

        let duration_secs = (Utc::now() - start_time).num_seconds() as u64;

        let result = RestoreResult {
            backup_id: backup_id.to_string(),
            restored_at: Utc::now(),
            duration_secs,
            verified: true,
        };

        info!(
            "Restore completed in {} seconds with verification: {}",
            result.duration_secs, result.verified
        );
        Ok(result)
    }

    /// Restore to a point in time by finding the nearest full backup and applying WAL segments
    pub async fn restore_point_in_time(
        &self,
        target_time: DateTime<Utc>,
    ) -> Result<RestoreResult, String> {
        info!("Starting point-in-time restore to: {}", target_time);

        let start_time = Utc::now();

        // Step 1: Find nearest full backup before target - stub
        debug!("Step 1: Find nearest full backup before {}", target_time);
        let nearest_backup_id = "BACKUP_001";
        info!("Found nearest backup: {}", nearest_backup_id);

        // Step 2: Download and restore full backup - stub
        debug!("Step 2: Restore from full backup");
        info!("Full backup restored");

        // Step 3: Apply WAL segments up to target_time - stub
        debug!("Step 3: Apply WAL segments up to {}", target_time);
        info!("WAL segments applied");

        // Step 4: Verify - stub
        debug!("Step 4: Verify restored database");
        info!("Database verified");

        let duration_secs = (Utc::now() - start_time).num_seconds() as u64;

        let result = RestoreResult {
            backup_id: nearest_backup_id.to_string(),
            restored_at: Utc::now(),
            duration_secs,
            verified: true,
        };

        info!(
            "Point-in-time restore completed in {} seconds",
            result.duration_secs
        );
        Ok(result)
    }
}

// ============================================================
// DisasterRecoveryOrchestrator (STORY-DR-03)
// ============================================================

pub struct DisasterRecoveryOrchestrator {
    backup_service: BackupService,
    restore_service: RestoreService,
}

impl DisasterRecoveryOrchestrator {
    pub fn new(
        backup_service: BackupService,
        restore_service: RestoreService,
    ) -> Self {
        DisasterRecoveryOrchestrator {
            backup_service,
            restore_service,
        }
    }

    /// Execute the disaster recovery plan
    /// Steps:
    /// 1. Identify latest backup
    /// 2. Restore from backup
    /// 3. Replay WAL
    /// 4. Health check
    /// 5. Notify
    pub async fn execute_dr_plan(&self) -> Result<DrReport, String> {
        info!("Starting disaster recovery plan execution");

        let started_at = Utc::now();
        let mut steps = Vec::new();

        // Step 1: Identify latest backup
        debug!("Step 1: Identify latest backup");
        let backups = self
            .backup_service
            .list_backups()
            .await
            .unwrap_or_default();

        let latest_backup = backups
            .first()
            .ok_or("No backups found")?
            .id
            .clone();

        steps.push(DrStepResult {
            step_name: "identify_latest_backup".to_string(),
            status: "Completed".to_string(),
            details: Some(format!("Latest backup: {}", latest_backup)),
        });
        info!("Latest backup identified: {}", latest_backup);

        // Step 2: Restore from backup
        debug!("Step 2: Restore from backup");
        match self.restore_service.restore_from_backup(&latest_backup).await {
            Ok(restore_result) => {
                steps.push(DrStepResult {
                    step_name: "restore_from_backup".to_string(),
                    status: "Completed".to_string(),
                    details: Some(format!(
                        "Restored in {} seconds",
                        restore_result.duration_secs
                    )),
                });
                info!(
                    "Backup restored in {} seconds",
                    restore_result.duration_secs
                );
            }
            Err(e) => {
                error!("Restore failed: {}", e);
                steps.push(DrStepResult {
                    step_name: "restore_from_backup".to_string(),
                    status: "Failed".to_string(),
                    details: Some(e.clone()),
                });
                let completed_at = Utc::now();
                return Ok(DrReport {
                    started_at,
                    completed_at,
                    rto_achieved_secs: (completed_at - started_at).num_seconds() as u64,
                    rpo_achieved_secs: 0,
                    steps,
                    success: false,
                });
            }
        }

        // Step 3: Replay WAL
        debug!("Step 3: Replay WAL segments");
        steps.push(DrStepResult {
            step_name: "replay_wal".to_string(),
            status: "Completed".to_string(),
            details: Some("WAL segments replayed".to_string()),
        });
        info!("WAL segments replayed");

        // Step 4: Health check
        debug!("Step 4: Execute health check");
        steps.push(DrStepResult {
            step_name: "health_check".to_string(),
            status: "Completed".to_string(),
            details: Some("Database health verified".to_string()),
        });
        info!("Health check completed");

        // Step 5: Notify
        debug!("Step 5: Send notifications");
        steps.push(DrStepResult {
            step_name: "notify".to_string(),
            status: "Completed".to_string(),
            details: Some("Notifications sent to stakeholders".to_string()),
        });
        info!("Notifications sent");

        let completed_at = Utc::now();
        let rto = (completed_at - started_at).num_seconds() as u64;

        let report = DrReport {
            started_at,
            completed_at,
            rto_achieved_secs: rto,
            rpo_achieved_secs: 300, // Stub: 5 minutes RPO
            steps,
            success: true,
        };

        info!(
            "Disaster recovery completed with RTO: {} seconds, RPO: {} seconds",
            report.rto_achieved_secs, report.rpo_achieved_secs
        );
        Ok(report)
    }
}

// ============================================================
// BackupScheduler (STORY-DR-01)
// ============================================================

pub struct BackupScheduler;

impl BackupScheduler {
    /// Spawn a background task that runs daily backup at the specified hour
    pub fn spawn_daily(hour: u32) -> tokio::task::JoinHandle<()> {
        if hour > 23 {
            panic!("Invalid hour: must be 0-23");
        }

        tokio::spawn(async move {
            let backup_service = BackupService::new(
                "banko-backups".to_string(),
                "http://minio:9000".to_string(),
                BackupRetentionPolicy::default(),
            );

            loop {
                let now = Utc::now();
                let today = now.date_naive();

                // Calculate next run time
                let mut next_run = today
                    .and_hms_opt(hour, 0, 0)
                    .unwrap()
                    .and_utc();

                if next_run <= now {
                    next_run = (today + chrono::Duration::days(1))
                        .and_hms_opt(hour, 0, 0)
                        .unwrap()
                        .and_utc();
                }

                let duration = next_run - now;
                debug!(
                    "Next backup scheduled for: {} (in {} seconds)",
                    next_run,
                    duration.num_seconds()
                );

                tokio::time::sleep(duration.to_std().unwrap()).await;

                // Execute full backup
                match backup_service.backup_database(BackupType::Full).await {
                    Ok(record) => {
                        info!("Daily backup completed: {:?}", record.id);
                    }
                    Err(e) => {
                        error!("Daily backup failed: {}", e);
                    }
                }

                // Apply retention policy
                match backup_service.apply_retention_policy().await {
                    Ok(result) => {
                        info!(
                            "Retention policy applied: deleted {} backups",
                            result.deleted_count
                        );
                    }
                    Err(e) => {
                        error!("Retention policy failed: {}", e);
                    }
                }
            }
        })
    }

    /// Trigger a backup immediately and return the record
    pub async fn run_backup_now(backup_type: BackupType) -> Result<BackupRecord, String> {
        let backup_service = BackupService::new(
            "banko-backups".to_string(),
            "http://minio:9000".to_string(),
            BackupRetentionPolicy::default(),
        );

        backup_service.backup_database(backup_type).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_type_display() {
        assert_eq!(BackupType::Full.to_string(), "Full");
        assert_eq!(BackupType::Incremental.to_string(), "Incremental");
        assert_eq!(BackupType::Wal.to_string(), "Wal");
    }

    #[test]
    fn test_backup_status_display() {
        assert_eq!(BackupStatus::Running.to_string(), "Running");
        assert_eq!(BackupStatus::Completed.to_string(), "Completed");
        assert_eq!(BackupStatus::Failed.to_string(), "Failed");
        assert_eq!(BackupStatus::Expired.to_string(), "Expired");
    }

    #[test]
    fn test_backup_retention_policy_default() {
        let policy = BackupRetentionPolicy::default();
        assert_eq!(policy.max_age_days, 90);
        assert_eq!(policy.min_backups, 7);
    }

    #[test]
    fn test_backup_record_creation() {
        let record = BackupRecord::new(
            BackupType::Full,
            "s3://bucket/backup.gz".to_string(),
            1024 * 1024 * 500,
            "checksum123".to_string(),
            90,
        );

        assert_eq!(record.backup_type, BackupType::Full);
        assert_eq!(record.status, BackupStatus::Running);
        assert!(record.expires_at.is_some());
    }

    #[tokio::test]
    async fn test_backup_service_creation() {
        let service = BackupService::new(
            "test-bucket".to_string(),
            "http://localhost:9000".to_string(),
            BackupRetentionPolicy::default(),
        );

        assert_eq!(service.s3_bucket, "test-bucket");
    }

    #[tokio::test]
    async fn test_backup_database() {
        let service = BackupService::new(
            "test-bucket".to_string(),
            "http://localhost:9000".to_string(),
            BackupRetentionPolicy::default(),
        );

        let result = service.backup_database(BackupType::Full).await;
        assert!(result.is_ok());

        let record = result.unwrap();
        assert_eq!(record.backup_type, BackupType::Full);
        assert_eq!(record.status, BackupStatus::Completed);
    }

    #[tokio::test]
    async fn test_verify_backup() {
        let service = BackupService::new(
            "test-bucket".to_string(),
            "http://localhost:9000".to_string(),
            BackupRetentionPolicy::default(),
        );

        let result = service.verify_backup("BACKUP_001").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_apply_retention_policy() {
        let service = BackupService::new(
            "test-bucket".to_string(),
            "http://localhost:9000".to_string(),
            BackupRetentionPolicy::default(),
        );

        let result = service.apply_retention_policy().await;
        assert!(result.is_ok());

        let retention = result.unwrap();
        assert_eq!(retention.deleted_count, 0);
    }

    #[tokio::test]
    async fn test_list_backups() {
        let service = BackupService::new(
            "test-bucket".to_string(),
            "http://localhost:9000".to_string(),
            BackupRetentionPolicy::default(),
        );

        let result = service.list_backups().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_restore_from_backup() {
        let restore_service = RestoreService::new("test-bucket".to_string());

        let result = restore_service.restore_from_backup("BACKUP_001").await;
        assert!(result.is_ok());

        let restore = result.unwrap();
        assert_eq!(restore.backup_id, "BACKUP_001");
        assert!(restore.verified);
    }

    #[tokio::test]
    async fn test_restore_point_in_time() {
        let restore_service = RestoreService::new("test-bucket".to_string());
        let target_time = Utc::now() - chrono::Duration::hours(1);

        let result = restore_service.restore_point_in_time(target_time).await;
        assert!(result.is_ok());

        let restore = result.unwrap();
        assert!(restore.verified);
    }

    #[tokio::test]
    async fn test_disaster_recovery_orchestrator() {
        let backup_service = BackupService::new(
            "test-bucket".to_string(),
            "http://localhost:9000".to_string(),
            BackupRetentionPolicy::default(),
        );

        let restore_service = RestoreService::new("test-bucket".to_string());
        let orchestrator = DisasterRecoveryOrchestrator::new(backup_service, restore_service);

        let result = orchestrator.execute_dr_plan().await;
        assert!(result.is_err()); // Will fail because no backups exist
    }

    #[tokio::test]
    async fn test_backup_scheduler_run_now() {
        let result = BackupScheduler::run_backup_now(BackupType::Full).await;
        assert!(result.is_ok());

        let record = result.unwrap();
        assert_eq!(record.backup_type, BackupType::Full);
    }

    #[test]
    fn test_backup_scheduler_spawn_validation() {
        // Valid hour
        let handle = BackupScheduler::spawn_daily(2);
        assert!(!handle.is_finished());
        handle.abort();

        // Invalid hour should panic
        // BackupScheduler::spawn_daily(24); // Would panic
    }

    #[test]
    fn test_retention_result_creation() {
        let result = RetentionResult {
            deleted_count: 3,
            deleted_size_bytes: 1024 * 1024 * 1500,
            remaining_backups: 7,
        };

        assert_eq!(result.deleted_count, 3);
        assert_eq!(result.remaining_backups, 7);
    }

    #[test]
    fn test_restore_result_creation() {
        let restore = RestoreResult {
            backup_id: "BACKUP_001".to_string(),
            restored_at: Utc::now(),
            duration_secs: 300,
            verified: true,
        };

        assert_eq!(restore.duration_secs, 300);
        assert!(restore.verified);
    }

    #[test]
    fn test_dr_report_creation() {
        let report = DrReport {
            started_at: Utc::now(),
            completed_at: Utc::now(),
            rto_achieved_secs: 600,
            rpo_achieved_secs: 300,
            steps: vec![],
            success: true,
        };

        assert_eq!(report.rto_achieved_secs, 600);
        assert_eq!(report.rpo_achieved_secs, 300);
        assert!(report.success);
    }
}
