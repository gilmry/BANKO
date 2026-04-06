use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Configuration for rate limiting behavior
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_requests_per_minute: u32,
    pub window_seconds: u64,
    pub whitelist_ips: Vec<String>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_minute: 60,
            window_seconds: 60,
            whitelist_ips: vec!["127.0.0.1".to_string(), "::1".to_string()],
        }
    }
}

impl RateLimitConfig {
    pub fn new(max_requests_per_minute: u32, window_seconds: u64) -> Self {
        Self {
            max_requests_per_minute,
            window_seconds,
            whitelist_ips: vec!["127.0.0.1".to_string(), "::1".to_string()],
        }
    }

    pub fn with_whitelist(mut self, ips: Vec<String>) -> Self {
        self.whitelist_ips = ips;
        self
    }
}

/// Errors that can occur during rate limiting checks
#[derive(Debug, Clone)]
pub enum RateLimitError {
    TooManyRequests { retry_after_secs: u64 },
}

/// In-memory store for tracking requests using sliding window algorithm
#[derive(Debug)]
pub struct RateLimitStore {
    requests: Arc<RwLock<HashMap<String, Vec<Instant>>>>,
}

impl RateLimitStore {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if a request should be allowed for the given key (IP or user_id)
    pub async fn check_rate_limit(
        &self,
        key: &str,
        config: &RateLimitConfig,
    ) -> Result<(), RateLimitError> {
        if self.is_whitelisted(key, config) {
            debug!("IP/key {} is whitelisted, allowing request", key);
            return Ok(());
        }

        let mut requests = self.requests.write().await;
        let now = Instant::now();
        let window_duration = Duration::from_secs(config.window_seconds);

        // Get or create entry for this key
        let entry = requests.entry(key.to_string()).or_insert_with(Vec::new);

        // Remove expired requests (outside the sliding window)
        entry.retain(|instant| now.duration_since(*instant) < window_duration);

        // Check if limit is exceeded
        if entry.len() >= config.max_requests_per_minute as usize {
            let oldest = entry.first().copied();
            let retry_after = if let Some(oldest_instant) = oldest {
                let elapsed = now.duration_since(oldest_instant);
                let remaining = window_duration.saturating_sub(elapsed);
                remaining.as_secs().max(1)
            } else {
                1
            };

            warn!(
                "Rate limit exceeded for key: {}, retry after: {} secs",
                key, retry_after
            );
            return Err(RateLimitError::TooManyRequests {
                retry_after_secs: retry_after,
            });
        }

        // Add current request timestamp
        entry.push(now);
        debug!(
            "Request allowed for key: {}, count: {}",
            key,
            entry.len()
        );

        Ok(())
    }

    /// Check if an IP address is whitelisted
    pub fn is_whitelisted(&self, ip: &str, config: &RateLimitConfig) -> bool {
        // Try exact match first
        if config.whitelist_ips.contains(&ip.to_string()) {
            return true;
        }

        // Try parsing as IP for comparison (handles both IPv4 and IPv6)
        if let Ok(ip_addr) = IpAddr::from_str(ip) {
            for whitelist_entry in &config.whitelist_ips {
                if let Ok(whitelist_addr) = IpAddr::from_str(whitelist_entry) {
                    if ip_addr == whitelist_addr {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Remove expired entries from the store (for maintenance)
    pub async fn cleanup_expired(&self, config: &RateLimitConfig) {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        let window_duration = Duration::from_secs(config.window_seconds);

        for entry in requests.values_mut() {
            entry.retain(|instant| now.duration_since(*instant) < window_duration);
        }

        // Remove empty entries
        requests.retain(|_, v| !v.is_empty());

        debug!("Rate limit store cleanup completed");
    }

    /// Get the current number of requests for a key (for testing/monitoring)
    pub async fn get_request_count(&self, key: &str) -> usize {
        let requests = self.requests.read().await;
        requests.get(key).map(|v| v.len()).unwrap_or(0)
    }

    /// Clear all entries (for testing)
    pub async fn clear(&self) {
        self.requests.write().await.clear();
    }
}

impl Default for RateLimitStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_under_limit_allows_request() {
        let store = RateLimitStore::new();
        let config = RateLimitConfig::new(5, 60);

        // First 5 requests should succeed
        for i in 0..5 {
            let result = store.check_rate_limit(&format!("user_{}", i), &config).await;
            assert!(result.is_ok(), "Request {} should be allowed", i);
        }
    }

    #[tokio::test]
    async fn test_rate_limit_over_limit_rejects_request() {
        let store = RateLimitStore::new();
        let config = RateLimitConfig::new(2, 60);

        // First 2 requests should succeed
        assert!(store.check_rate_limit("test_key", &config).await.is_ok());
        assert!(store.check_rate_limit("test_key", &config).await.is_ok());

        // 3rd request should fail
        let result = store.check_rate_limit("test_key", &config).await;
        assert!(matches!(
            result,
            Err(RateLimitError::TooManyRequests {
                retry_after_secs: _
            })
        ));
    }

    #[tokio::test]
    async fn test_whitelist_ip_bypass_rate_limit() {
        let store = RateLimitStore::new();
        let config = RateLimitConfig::new(1, 60)
            .with_whitelist(vec!["192.168.1.1".to_string()]);

        // Even with limit of 1, whitelisted IP should allow multiple requests
        assert!(store
            .check_rate_limit("192.168.1.1", &config)
            .await
            .is_ok());
        assert!(store
            .check_rate_limit("192.168.1.1", &config)
            .await
            .is_ok());
        assert!(store
            .check_rate_limit("192.168.1.1", &config)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_whitelist_loopback_addresses() {
        let store = RateLimitStore::new();
        let config = RateLimitConfig::default();

        // Localhost addresses should be whitelisted by default
        assert!(store.is_whitelisted("127.0.0.1", &config));
        assert!(store.is_whitelisted("::1", &config));
    }

    #[tokio::test]
    async fn test_cleanup_expired_requests() {
        let store = RateLimitStore::new();
        let config = RateLimitConfig::new(10, 1); // 1 second window

        // Add request
        assert!(store.check_rate_limit("test_key", &config).await.is_ok());

        // Verify request was recorded
        let count_before = store.get_request_count("test_key").await;
        assert_eq!(count_before, 1);

        // Wait for window to expire
        tokio::time::sleep(Duration::from_millis(1100)).await;

        // Cleanup
        store.cleanup_expired(&config).await;

        // Verify old requests were removed
        let count_after = store.get_request_count("test_key").await;
        assert_eq!(count_after, 0);
    }

    #[tokio::test]
    async fn test_sliding_window_accuracy() {
        let store = RateLimitStore::new();
        let config = RateLimitConfig::new(3, 1); // 3 requests per second

        // Use same key for all requests
        let key = "sliding_window_test";

        // First 3 requests should succeed
        assert!(store.check_rate_limit(key, &config).await.is_ok());
        assert!(store.check_rate_limit(key, &config).await.is_ok());
        assert!(store.check_rate_limit(key, &config).await.is_ok());

        // 4th request should fail
        assert!(store.check_rate_limit(key, &config).await.is_err());

        // Wait for window to partially expire (500ms)
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Still should fail because window hasn't fully expired
        assert!(store.check_rate_limit(key, &config).await.is_err());

        // Wait for full window to expire
        tokio::time::sleep(Duration::from_millis(600)).await;

        // Now should succeed (old requests outside window)
        assert!(store.check_rate_limit(key, &config).await.is_ok());
    }

    #[tokio::test]
    async fn test_different_keys_independent() {
        let store = RateLimitStore::new();
        let config = RateLimitConfig::new(2, 60);

        // First key hits limit
        assert!(store.check_rate_limit("key_1", &config).await.is_ok());
        assert!(store.check_rate_limit("key_1", &config).await.is_ok());
        assert!(store.check_rate_limit("key_1", &config).await.is_err());

        // Second key should still work (independent bucket)
        assert!(store.check_rate_limit("key_2", &config).await.is_ok());
        assert!(store.check_rate_limit("key_2", &config).await.is_ok());
        assert!(store.check_rate_limit("key_2", &config).await.is_err());
    }

    #[tokio::test]
    async fn test_clear_resets_store() {
        let store = RateLimitStore::new();
        let config = RateLimitConfig::new(1, 60);

        // Add a request
        assert!(store.check_rate_limit("test", &config).await.is_ok());
        assert_eq!(store.get_request_count("test").await, 1);

        // Clear store
        store.clear().await;

        // Should be empty
        assert_eq!(store.get_request_count("test").await, 0);

        // Should allow new request
        assert!(store.check_rate_limit("test", &config).await.is_ok());
    }
}
