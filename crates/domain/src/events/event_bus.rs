use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use super::domain_event::StoredEvent;

// ============================================================
// EventHandler Trait
// ============================================================

/// Trait for handling domain events.
/// Implementations subscribe to specific event types and react to them.
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handles the given event.
    async fn handle(&self, event: &StoredEvent) -> Result<(), String>;

    /// Returns the list of event types this handler subscribes to.
    fn handles_event_types(&self) -> Vec<String>;
}

// ============================================================
// EventBus Trait (Port)
// ============================================================

/// Core EventBus port. Defines the interface for event publication and subscription.
#[async_trait]
pub trait EventBus: Send + Sync {
    /// Publishes a single event to all matching handlers.
    async fn publish(&self, event: StoredEvent) -> Result<(), String>;

    /// Publishes multiple events in order.
    async fn publish_all(&self, events: Vec<StoredEvent>) -> Result<(), String>;

    /// Subscribes a handler to this event bus.
    async fn subscribe(&self, handler: Box<dyn EventHandler + Send + Sync>);
}

// ============================================================
// InMemoryEventBus Implementation
// ============================================================

/// In-memory implementation of the EventBus.
/// Suitable for testing and development. Events are distributed to handlers synchronously.
pub struct InMemoryEventBus {
    handlers: Arc<Mutex<Vec<Arc<dyn EventHandler>>>>,
    published_events: Arc<Mutex<Vec<StoredEvent>>>,
}

impl InMemoryEventBus {
    /// Creates a new in-memory event bus.
    pub fn new() -> Self {
        InMemoryEventBus {
            handlers: Arc::new(Mutex::new(Vec::new())),
            published_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns all published events (for testing).
    pub async fn get_published_events(&self) -> Vec<StoredEvent> {
        self.published_events.lock().await.clone()
    }

    /// Clears published events history (for testing).
    pub async fn clear_published_events(&self) {
        self.published_events.lock().await.clear();
    }

    /// Returns count of published events.
    pub async fn published_count(&self) -> usize {
        self.published_events.lock().await.len()
    }

    /// Returns count of registered handlers.
    pub async fn handler_count(&self) -> usize {
        self.handlers.lock().await.len()
    }
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: StoredEvent) -> Result<(), String> {
        let handlers = self.handlers.lock().await;

        // Store the event
        self.published_events.lock().await.push(event.clone());

        // Send to matching handlers
        for handler in handlers.iter() {
            let handles = handler.handles_event_types();
            if handles.contains(&event.event_type) {
                handler.handle(&event).await?;
            }
        }

        Ok(())
    }

    async fn publish_all(&self, events: Vec<StoredEvent>) -> Result<(), String> {
        for event in events {
            self.publish(event).await?;
        }
        Ok(())
    }

    async fn subscribe(&self, handler: Box<dyn EventHandler + Send + Sync>) {
        self.handlers.lock().await.push(Arc::from(handler));
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEventHandler {
        handled_events: Arc<Mutex<Vec<StoredEvent>>>,
        event_types: Vec<String>,
    }

    impl TestEventHandler {
        fn new(event_types: Vec<String>) -> Self {
            TestEventHandler {
                handled_events: Arc::new(Mutex::new(Vec::new())),
                event_types,
            }
        }

        async fn get_handled_events(&self) -> Vec<StoredEvent> {
            self.handled_events.lock().await.clone()
        }
    }

    #[async_trait]
    impl EventHandler for TestEventHandler {
        async fn handle(&self, event: &StoredEvent) -> Result<(), String> {
            self.handled_events.lock().await.push(event.clone());
            Ok(())
        }

        fn handles_event_types(&self) -> Vec<String> {
            self.event_types.clone()
        }
    }

    #[tokio::test]
    async fn test_publish_single_event() {
        let bus = InMemoryEventBus::new();
        let handler = Box::new(TestEventHandler::new(vec!["TestEvent".to_string()]));
        bus.subscribe(handler).await;

        let event = StoredEvent::new(
            Uuid::new_v4(),
            "TestAggregate".to_string(),
            "TestEvent".to_string(),
            serde_json::json!({"test": "data"}),
        );

        bus.publish(event.clone()).await.expect("Should publish");

        assert_eq!(bus.published_count().await, 1);
    }

    #[tokio::test]
    async fn test_handler_receives_matching_event() {
        let bus = InMemoryEventBus::new();
        let handler = Box::new(TestEventHandler::new(vec!["AccountOpened".to_string()]));
        bus.subscribe(handler).await;

        let event = StoredEvent::new(
            Uuid::new_v4(),
            "Account".to_string(),
            "AccountOpened".to_string(),
            serde_json::json!({}),
        );

        bus.publish(event).await.expect("Should publish");
        assert_eq!(bus.published_count().await, 1);
    }

    #[tokio::test]
    async fn test_publish_to_multiple_handlers() {
        let bus = InMemoryEventBus::new();
        let handler1 = Box::new(TestEventHandler::new(vec!["PaymentInitiated".to_string()]));
        let handler2 = Box::new(TestEventHandler::new(vec!["PaymentInitiated".to_string()]));

        bus.subscribe(handler1).await;
        bus.subscribe(handler2).await;

        let event = StoredEvent::new(
            Uuid::new_v4(),
            "Payment".to_string(),
            "PaymentInitiated".to_string(),
            serde_json::json!({}),
        );

        bus.publish(event).await.expect("Should publish");
        assert_eq!(bus.handler_count().await, 2);
    }

    #[tokio::test]
    async fn test_handler_filtering_by_event_type() {
        let bus = InMemoryEventBus::new();
        let handler = Box::new(TestEventHandler::new(vec!["AccountOpened".to_string()]));
        bus.subscribe(handler).await;

        let event = StoredEvent::new(
            Uuid::new_v4(),
            "Account".to_string(),
            "AccountClosed".to_string(), // Different event type
            serde_json::json!({}),
        );

        bus.publish(event).await.expect("Should publish");
        // Handler should not have received it
        assert_eq!(bus.published_count().await, 1);
    }

    #[tokio::test]
    async fn test_publish_all_sends_batch() {
        let bus = InMemoryEventBus::new();
        let handler = Box::new(TestEventHandler::new(vec!["TestEvent".to_string()]));
        bus.subscribe(handler).await;

        let events = vec![
            StoredEvent::new(
                Uuid::new_v4(),
                "Test".to_string(),
                "TestEvent".to_string(),
                serde_json::json!({}),
            ),
            StoredEvent::new(
                Uuid::new_v4(),
                "Test".to_string(),
                "TestEvent".to_string(),
                serde_json::json!({}),
            ),
            StoredEvent::new(
                Uuid::new_v4(),
                "Test".to_string(),
                "TestEvent".to_string(),
                serde_json::json!({}),
            ),
        ];

        bus.publish_all(events).await.expect("Should publish all");
        assert_eq!(bus.published_count().await, 3);
    }

    #[tokio::test]
    async fn test_event_ordering_preserved() {
        let bus = InMemoryEventBus::new();
        let handler = Box::new(TestEventHandler::new(vec!["Event".to_string()]));
        bus.subscribe(handler).await;

        let event1_id = Uuid::new_v4();
        let event2_id = Uuid::new_v4();

        let event1 = StoredEvent::new(
            event1_id,
            "Test".to_string(),
            "Event".to_string(),
            serde_json::json!({"order": 1}),
        );

        let event2 = StoredEvent::new(
            event2_id,
            "Test".to_string(),
            "Event".to_string(),
            serde_json::json!({"order": 2}),
        );

        bus.publish(event1).await.expect("Should publish");
        bus.publish(event2).await.expect("Should publish");

        let published = bus.get_published_events().await;
        assert_eq!(published.len(), 2);
        assert_eq!(published[0].aggregate_id, event1_id);
        assert_eq!(published[1].aggregate_id, event2_id);
    }

    #[tokio::test]
    async fn test_no_handler_for_event_type() {
        let bus = InMemoryEventBus::new();
        let handler = Box::new(TestEventHandler::new(vec!["SpecificEvent".to_string()]));
        bus.subscribe(handler).await;

        let event = StoredEvent::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "UnhandledEvent".to_string(),
            serde_json::json!({}),
        );

        // Should not error, just skip
        bus.publish(event).await.expect("Should handle gracefully");
        assert_eq!(bus.published_count().await, 1);
    }

    #[tokio::test]
    async fn test_handler_error_propagates() {
        struct FailingHandler;

        #[async_trait]
        impl EventHandler for FailingHandler {
            async fn handle(&self, _event: &StoredEvent) -> Result<(), String> {
                Err("Handler failed".to_string())
            }

            fn handles_event_types(&self) -> Vec<String> {
                vec!["TestEvent".to_string()]
            }
        }

        let bus = InMemoryEventBus::new();
        let handler = Box::new(FailingHandler);
        bus.subscribe(handler).await;

        let event = StoredEvent::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "TestEvent".to_string(),
            serde_json::json!({}),
        );

        let result = bus.publish(event).await;
        assert!(result.is_err());
    }
}
