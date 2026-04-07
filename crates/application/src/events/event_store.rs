use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use banko_domain::events::{EventBus, StoredEvent};

use super::ports::{IEventStoreRepository, ISnapshotRepository};

// ============================================================
// Replay Result
// ============================================================

/// Result of replaying events from the event store.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplayResult {
    /// Number of events replayed
    pub events_replayed: usize,
    /// Starting sequence number of replay
    pub from_sequence: i64,
    /// Ending sequence number of replay
    pub to_sequence: i64,
}

// ============================================================
// Event Store Service
// ============================================================

/// Service for managing the event store, snapshots, and event replays.
/// Coordinates between the event store repository, snapshot repository, and event bus.
pub struct EventStoreService {
    event_repo: Arc<dyn IEventStoreRepository>,
    snapshot_repo: Arc<dyn ISnapshotRepository>,
    event_bus: Arc<dyn EventBus>,
    snapshot_threshold: usize,
}

impl EventStoreService {
    /// Creates a new event store service.
    ///
    /// # Arguments
    /// * `event_repo` - Repository for storing/retrieving events
    /// * `snapshot_repo` - Repository for storing/retrieving snapshots
    /// * `event_bus` - Event bus for publishing events
    /// * `snapshot_threshold` - Number of events before creating a snapshot (default: 100)
    pub fn new(
        event_repo: Arc<dyn IEventStoreRepository>,
        snapshot_repo: Arc<dyn ISnapshotRepository>,
        event_bus: Arc<dyn EventBus>,
        snapshot_threshold: usize,
    ) -> Self {
        EventStoreService {
            event_repo,
            snapshot_repo,
            event_bus,
            snapshot_threshold,
        }
    }

    /// Creates a new event store service with default snapshot threshold (100).
    pub fn with_defaults(
        event_repo: Arc<dyn IEventStoreRepository>,
        snapshot_repo: Arc<dyn ISnapshotRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self::new(event_repo, snapshot_repo, event_bus, 100)
    }

    /// Appends a single event to the store.
    /// 1. Persists the event in the event store
    /// 2. Publishes the event to the event bus
    /// 3. Checks if a snapshot is needed
    /// 4. Returns the assigned sequence number
    pub async fn append_event(&self, mut event: StoredEvent) -> Result<i64, String> {
        // Persist in event store
        let sequence_number = self.event_repo.append(&event).await?;
        event = event.with_sequence_number(sequence_number);

        // Publish on event bus
        self.event_bus.publish(event.clone()).await?;

        // Check if snapshot is needed
        self.check_and_create_snapshot(&event).await?;

        Ok(sequence_number)
    }

    /// Appends multiple events in order.
    pub async fn append_events(&self, events: Vec<StoredEvent>) -> Result<Vec<i64>, String> {
        let mut sequence_numbers = Vec::new();

        for event in events {
            let seq = self.append_event(event).await?;
            sequence_numbers.push(seq);
        }

        Ok(sequence_numbers)
    }

    /// Retrieves all events for a specific aggregate.
    pub async fn get_aggregate_events(&self, aggregate_id: Uuid) -> Result<Vec<StoredEvent>, String> {
        self.event_repo.get_events_for_aggregate(aggregate_id).await
    }

    /// Replays events starting from a specific sequence number.
    /// 1. Retrieves all events since the sequence number
    /// 2. Re-publishes each event to the event bus in order
    /// 3. Returns the replay result
    pub async fn replay_from(&self, sequence_number: i64) -> Result<ReplayResult, String> {
        let events = self.event_repo.get_events_since(sequence_number).await?;

        if events.is_empty() {
            return Ok(ReplayResult {
                events_replayed: 0,
                from_sequence: sequence_number,
                to_sequence: sequence_number,
            });
        }

        let count = events.len();
        let last_sequence = events.last().map(|e| e.sequence_number).unwrap_or(sequence_number);

        // Re-publish each event
        for event in events {
            self.event_bus.publish(event).await?;
        }

        Ok(ReplayResult {
            events_replayed: count,
            from_sequence: sequence_number,
            to_sequence: last_sequence,
        })
    }

    /// Rebuilds an aggregate by retrieving its events, optionally from a snapshot.
    /// 1. Checks for the latest snapshot
    /// 2. Retrieves events since the snapshot version
    /// 3. Returns the combined events
    pub async fn rebuild_aggregate(&self, aggregate_id: Uuid) -> Result<Vec<StoredEvent>, String> {
        // Try to get latest snapshot
        let snapshot = self.snapshot_repo.get_latest_snapshot(aggregate_id).await?;

        let events = if let Some(snapshot) = snapshot {
            // Get events since snapshot
            self.event_repo
                .get_events_since(snapshot.version)
                .await?
                .into_iter()
                .filter(|e| e.aggregate_id == aggregate_id)
                .collect()
        } else {
            // Get all events for aggregate
            self.event_repo.get_events_for_aggregate(aggregate_id).await?
        };

        Ok(events)
    }

    /// Checks if a snapshot should be created and creates it if needed.
    /// A snapshot is created every `snapshot_threshold` events.
    async fn check_and_create_snapshot(&self, event: &StoredEvent) -> Result<(), String> {
        // Check if we should create a snapshot
        // For simplicity, we check every snapshot_threshold sequence number
        if event.sequence_number > 0 && (event.sequence_number as usize).is_multiple_of(self.snapshot_threshold) {
            // Create a snapshot - in this simple implementation, just store the event state
            self.snapshot_repo
                .save_snapshot(
                    event.aggregate_id,
                    &event.aggregate_type,
                    event.payload.clone(),
                    event.sequence_number,
                )
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    // Mock repositories for testing
    struct MockEventStoreRepository {
        events: Mutex<Vec<StoredEvent>>,
    }

    impl MockEventStoreRepository {
        fn new() -> Self {
            MockEventStoreRepository {
                events: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IEventStoreRepository for MockEventStoreRepository {
        async fn append(&self, event: &StoredEvent) -> Result<i64, String> {
            let mut events = self.events.lock().unwrap();
            let sequence_number = events.len() as i64 + 1;
            let mut event = event.clone();
            event = event.with_sequence_number(sequence_number);
            events.push(event);
            Ok(sequence_number)
        }

        async fn get_events_for_aggregate(&self, aggregate_id: Uuid) -> Result<Vec<StoredEvent>, String> {
            let events = self.events.lock().unwrap();
            Ok(events.iter().filter(|e| e.aggregate_id == aggregate_id).cloned().collect())
        }

        async fn get_events_since(&self, sequence_number: i64) -> Result<Vec<StoredEvent>, String> {
            let events = self.events.lock().unwrap();
            Ok(events.iter().filter(|e| e.sequence_number >= sequence_number).cloned().collect())
        }

        async fn get_events_by_type(
            &self,
            event_type: &str,
            _since: Option<DateTime<Utc>>,
        ) -> Result<Vec<StoredEvent>, String> {
            let events = self.events.lock().unwrap();
            Ok(events.iter().filter(|e| e.event_type == event_type).cloned().collect())
        }

        async fn get_all_events(&self, offset: i64, limit: i64) -> Result<Vec<StoredEvent>, String> {
            let events = self.events.lock().unwrap();
            Ok(events
                .iter()
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }

        async fn count_events(&self) -> Result<i64, String> {
            Ok(self.events.lock().unwrap().len() as i64)
        }
    }

    struct MockSnapshotRepository {
        snapshots: Mutex<Vec<AggregateSnapshot>>,
    }

    impl MockSnapshotRepository {
        fn new() -> Self {
            MockSnapshotRepository {
                snapshots: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ISnapshotRepository for MockSnapshotRepository {
        async fn save_snapshot(
            &self,
            aggregate_id: Uuid,
            aggregate_type: &str,
            state: serde_json::Value,
            version: i64,
        ) -> Result<(), String> {
            let mut snapshots = self.snapshots.lock().unwrap();
            snapshots.push(AggregateSnapshot::new(
                aggregate_id,
                aggregate_type.to_string(),
                state,
                version,
            ));
            Ok(())
        }

        async fn get_latest_snapshot(&self, aggregate_id: Uuid) -> Result<Option<AggregateSnapshot>, String> {
            let snapshots = self.snapshots.lock().unwrap();
            Ok(snapshots
                .iter()
                .filter(|s| s.aggregate_id == aggregate_id)
                .last()
                .cloned())
        }
    }

    // Simple in-memory event bus for testing
    struct MockEventBus {
        published: Mutex<Vec<StoredEvent>>,
    }

    impl MockEventBus {
        fn new() -> Self {
            MockEventBus {
                published: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl EventBus for MockEventBus {
        async fn publish(&self, event: StoredEvent) -> Result<(), String> {
            self.published.lock().unwrap().push(event);
            Ok(())
        }

        async fn publish_all(&self, events: Vec<StoredEvent>) -> Result<(), String> {
            for event in events {
                self.publish(event).await?;
            }
            Ok(())
        }

        async fn subscribe(&self, _handler: Box<dyn banko_domain::events::EventHandler + Send + Sync>) {
            // No-op for mock
        }
    }

    #[tokio::test]
    async fn test_append_single_event() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(event_repo.clone(), snapshot_repo, event_bus);

        let event = StoredEvent::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "TestEvent".to_string(),
            serde_json::json!({}),
        );

        let seq = service.append_event(event).await.expect("Should append");
        assert_eq!(seq, 1);
    }

    #[tokio::test]
    async fn test_append_multiple_events() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(event_repo, snapshot_repo, event_bus);

        let events = vec![
            StoredEvent::new(Uuid::new_v4(), "Test".to_string(), "Event1".to_string(), serde_json::json!({})),
            StoredEvent::new(Uuid::new_v4(), "Test".to_string(), "Event2".to_string(), serde_json::json!({})),
            StoredEvent::new(Uuid::new_v4(), "Test".to_string(), "Event3".to_string(), serde_json::json!({})),
        ];

        let seqs = service.append_events(events).await.expect("Should append all");
        assert_eq!(seqs.len(), 3);
    }

    #[tokio::test]
    async fn test_get_aggregate_events() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(event_repo, snapshot_repo, event_bus);

        let aggregate_id = Uuid::new_v4();
        let event = StoredEvent::new(
            aggregate_id,
            "Test".to_string(),
            "Event".to_string(),
            serde_json::json!({}),
        );

        service.append_event(event).await.expect("Should append");

        let retrieved = service.get_aggregate_events(aggregate_id).await.expect("Should get");
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].aggregate_id, aggregate_id);
    }

    #[tokio::test]
    async fn test_replay_from_sequence() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(event_repo, snapshot_repo, event_bus.clone());

        // Append some events
        for i in 0..3 {
            let event = StoredEvent::new(
                Uuid::new_v4(),
                "Test".to_string(),
                format!("Event{}", i),
                serde_json::json!({}),
            );
            service.append_event(event).await.expect("Should append");
        }

        // Replay from sequence 2
        let result = service.replay_from(2).await.expect("Should replay");
        assert_eq!(result.events_replayed, 2); // Should get events 2 and 3
    }

    #[tokio::test]
    async fn test_rebuild_aggregate_without_snapshot() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(event_repo, snapshot_repo, event_bus);

        let aggregate_id = Uuid::new_v4();
        for i in 0..3 {
            let event = StoredEvent::new(
                aggregate_id,
                "Test".to_string(),
                format!("Event{}", i),
                serde_json::json!({}),
            );
            service.append_event(event).await.expect("Should append");
        }

        let events = service.rebuild_aggregate(aggregate_id).await.expect("Should rebuild");
        assert_eq!(events.len(), 3);
    }

    #[tokio::test]
    async fn test_rebuild_aggregate_with_snapshot() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(
            event_repo.clone(),
            snapshot_repo.clone(),
            event_bus,
        );

        let aggregate_id = Uuid::new_v4();

        // Manually create a snapshot at version 2
        snapshot_repo
            .save_snapshot(
                aggregate_id,
                "Test",
                serde_json::json!({"state": "snapshot"}),
                2,
            )
            .await
            .expect("Should save snapshot");

        // Append events after snapshot
        for i in 0..2 {
            let event = StoredEvent::new(
                aggregate_id,
                "Test".to_string(),
                format!("Event{}", i),
                serde_json::json!({}),
            )
            .with_sequence_number((i + 3) as i64);
            event_repo
                .append(&event)
                .await
                .expect("Should append");
        }

        // Rebuild should include events from snapshot onwards
        let events = service.rebuild_aggregate(aggregate_id).await.expect("Should rebuild");
        // Should get events from sequence 2 onwards
        assert!(events.len() > 0);
    }

    #[tokio::test]
    async fn test_event_ordering_in_replay() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(
            event_repo.clone(),
            snapshot_repo,
            event_bus.clone(),
        );

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();

        let event1 = StoredEvent::new(id1, "Test".to_string(), "E1".to_string(), serde_json::json!({}));
        let event2 = StoredEvent::new(id2, "Test".to_string(), "E2".to_string(), serde_json::json!({}));
        let event3 = StoredEvent::new(id3, "Test".to_string(), "E3".to_string(), serde_json::json!({}));

        service.append_event(event1).await.expect("Should append");
        service.append_event(event2).await.expect("Should append");
        service.append_event(event3).await.expect("Should append");

        // Verify order in bus
        let published: Vec<_> = event_bus
            .published
            .lock()
            .unwrap()
            .iter()
            .map(|e| e.aggregate_id)
            .collect();

        assert_eq!(published[0], id1);
        assert_eq!(published[1], id2);
        assert_eq!(published[2], id3);
    }

    #[tokio::test]
    async fn test_empty_event_store() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(event_repo, snapshot_repo, event_bus);

        let result = service.replay_from(1).await.expect("Should handle empty");
        assert_eq!(result.events_replayed, 0);
    }

    #[tokio::test]
    async fn test_replay_result_structure() {
        let event_repo = Arc::new(MockEventStoreRepository::new());
        let snapshot_repo = Arc::new(MockSnapshotRepository::new());
        let event_bus = Arc::new(MockEventBus::new());

        let service = EventStoreService::with_defaults(event_repo, snapshot_repo, event_bus);

        for i in 0..5 {
            let event = StoredEvent::new(
                Uuid::new_v4(),
                "Test".to_string(),
                format!("Event{}", i),
                serde_json::json!({}),
            );
            service.append_event(event).await.expect("Should append");
        }

        let result = service.replay_from(2).await.expect("Should replay");
        assert!(result.to_sequence >= result.from_sequence);
        assert_eq!(result.events_replayed, 4); // Events 2-5
    }
}
