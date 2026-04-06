use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use banko_domain::events::StoredEvent;

// ============================================================
// Event Store Repository Port
// ============================================================

/// Repository port for persisting and retrieving domain events.
#[async_trait]
pub trait IEventStoreRepository: Send + Sync {
    /// Appends an event to the store. Returns the assigned sequence number.
    async fn append(&self, event: &StoredEvent) -> Result<i64, String>;

    /// Retrieves all events for a specific aggregate.
    async fn get_events_for_aggregate(&self, aggregate_id: Uuid) -> Result<Vec<StoredEvent>, String>;

    /// Retrieves all events since a given sequence number (inclusive).
    async fn get_events_since(&self, sequence_number: i64) -> Result<Vec<StoredEvent>, String>;

    /// Retrieves events by type, optionally since a given timestamp.
    async fn get_events_by_type(
        &self,
        event_type: &str,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<StoredEvent>, String>;

    /// Retrieves all events with pagination.
    async fn get_all_events(&self, offset: i64, limit: i64) -> Result<Vec<StoredEvent>, String>;

    /// Returns the total count of events.
    async fn count_events(&self) -> Result<i64, String>;
}

// ============================================================
// Aggregate Snapshot
// ============================================================

/// Represents a snapshot of an aggregate state at a specific version.
/// Used for optimization during event replay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AggregateSnapshot {
    /// Unique ID of the snapshot
    pub id: Uuid,
    /// ID of the aggregate
    pub aggregate_id: Uuid,
    /// Type of aggregate (e.g., "Account", "Payment")
    pub aggregate_type: String,
    /// The snapshot of the aggregate state
    pub state: serde_json::Value,
    /// The version/sequence number of the aggregate at snapshot time
    pub version: i64,
    /// When the snapshot was created
    pub created_at: DateTime<Utc>,
}

impl AggregateSnapshot {
    /// Creates a new aggregate snapshot.
    pub fn new(
        aggregate_id: Uuid,
        aggregate_type: String,
        state: serde_json::Value,
        version: i64,
    ) -> Self {
        AggregateSnapshot {
            id: Uuid::new_v4(),
            aggregate_id,
            aggregate_type,
            state,
            version,
            created_at: Utc::now(),
        }
    }
}

// ============================================================
// Snapshot Repository Port
// ============================================================

/// Repository port for storing and retrieving aggregate snapshots.
#[async_trait]
pub trait ISnapshotRepository: Send + Sync {
    /// Saves a snapshot of an aggregate.
    async fn save_snapshot(
        &self,
        aggregate_id: Uuid,
        aggregate_type: &str,
        state: serde_json::Value,
        version: i64,
    ) -> Result<(), String>;

    /// Retrieves the latest snapshot for an aggregate.
    async fn get_latest_snapshot(&self, aggregate_id: Uuid) -> Result<Option<AggregateSnapshot>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_snapshot_creation() {
        let aggregate_id = Uuid::new_v4();
        let state = serde_json::json!({"balance": 1000});

        let snapshot = AggregateSnapshot::new(
            aggregate_id,
            "Account".to_string(),
            state.clone(),
            5,
        );

        assert_eq!(snapshot.aggregate_id, aggregate_id);
        assert_eq!(snapshot.aggregate_type, "Account");
        assert_eq!(snapshot.state, state);
        assert_eq!(snapshot.version, 5);
    }

    #[test]
    fn test_aggregate_snapshot_serialization() {
        let snapshot = AggregateSnapshot::new(
            Uuid::new_v4(),
            "Test".to_string(),
            serde_json::json!({"test": "data"}),
            1,
        );

        let json = serde_json::to_string(&snapshot).expect("Should serialize");
        let deserialized: AggregateSnapshot = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(snapshot, deserialized);
    }
}
