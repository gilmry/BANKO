use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;
use super::{PasswordHash, UserId};

const MAX_PASSWORD_HISTORY: usize = 12;

/// Password history entry — prevents reuse of recent passwords
/// FR-159: Password policy with history tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHistoryEntry {
    id: Uuid,
    user_id: UserId,
    password_hash: PasswordHash,
    created_at: DateTime<Utc>,
}

impl PasswordHistoryEntry {
    pub fn new(user_id: UserId, password_hash: PasswordHash) -> Self {
        PasswordHistoryEntry {
            id: Uuid::new_v4(),
            user_id,
            password_hash,
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

/// Password history aggregate — manages user's password history
/// Prevents password reuse within a configurable window (default: last 12 passwords)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHistory {
    user_id: UserId,
    entries: Vec<PasswordHistoryEntry>,
}

impl PasswordHistory {
    /// Create a new password history for a user
    pub fn new(user_id: UserId) -> Self {
        PasswordHistory {
            user_id,
            entries: Vec::new(),
        }
    }

    /// Reconstitute from stored entries
    pub fn reconstitute(user_id: UserId, entries: Vec<PasswordHistoryEntry>) -> Self {
        PasswordHistory { user_id, entries }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn entries(&self) -> &[PasswordHistoryEntry] {
        &self.entries
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Add a new password to history
    pub fn add_password(&mut self, password_hash: PasswordHash) {
        let entry = PasswordHistoryEntry::new(self.user_id.clone(), password_hash);
        self.entries.push(entry);

        // Keep only the last MAX_PASSWORD_HISTORY entries
        if self.entries.len() > MAX_PASSWORD_HISTORY {
            self.entries.remove(0);
        }
    }

    /// Check if a password has been used before (in history)
    pub fn has_been_used(&self, password_hash: &PasswordHash) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.password_hash() == password_hash)
    }

    /// Get the most recent password from history
    pub fn last_password(&self) -> Option<&PasswordHistoryEntry> {
        self.entries.last()
    }

    /// Clear all history (for administrative purposes)
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get password history within a certain time window
    pub fn get_recent_passwords(&self, days: u32) -> Vec<&PasswordHistoryEntry> {
        let cutoff = Utc::now()
            - chrono::Duration::days(days as i64);

        self.entries
            .iter()
            .filter(|entry| entry.created_at() > cutoff)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_hash(suffix: &str) -> PasswordHash {
        PasswordHash::new(
            format!("$2b$12$LJ3m4ys3Lg2HEOjdLNRsWuBNRZLJDhG5JQqJK9qJKj3K4hNqXKw{}", suffix)
        )
        .unwrap()
    }

    #[test]
    fn test_password_history_entry_new() {
        let user_id = UserId::new();
        let hash = valid_hash("1");
        let entry = PasswordHistoryEntry::new(user_id.clone(), hash.clone());

        assert_eq!(entry.user_id(), &user_id);
        assert_eq!(entry.password_hash(), &hash);
        assert!(entry.created_at() <= Utc::now());
    }

    #[test]
    fn test_password_history_new() {
        let user_id = UserId::new();
        let history = PasswordHistory::new(user_id.clone());

        assert_eq!(history.user_id(), &user_id);
        assert_eq!(history.count(), 0);
        assert!(history.entries().is_empty());
    }

    #[test]
    fn test_add_password() {
        let user_id = UserId::new();
        let mut history = PasswordHistory::new(user_id);

        let hash1 = valid_hash("1");
        history.add_password(hash1);
        assert_eq!(history.count(), 1);

        let hash2 = valid_hash("2");
        history.add_password(hash2);
        assert_eq!(history.count(), 2);
    }

    #[test]
    fn test_password_history_max_entries() {
        let user_id = UserId::new();
        let mut history = PasswordHistory::new(user_id);

        // Add more than MAX_PASSWORD_HISTORY entries
        for i in 0..20 {
            let hash = valid_hash(&i.to_string());
            history.add_password(hash);
        }

        assert_eq!(history.count(), MAX_PASSWORD_HISTORY);
    }

    #[test]
    fn test_password_history_oldest_removed() {
        let user_id = UserId::new();
        let mut history = PasswordHistory::new(user_id);

        let hash1 = valid_hash("1");
        history.add_password(hash1.clone());

        // Add MAX_PASSWORD_HISTORY more passwords
        for i in 1..=MAX_PASSWORD_HISTORY {
            let hash = valid_hash(&i.to_string());
            history.add_password(hash);
        }

        // First hash should be gone
        assert!(!history.has_been_used(&hash1));
    }

    #[test]
    fn test_has_been_used() {
        let user_id = UserId::new();
        let mut history = PasswordHistory::new(user_id);

        let hash1 = valid_hash("1");
        let hash2 = valid_hash("2");

        history.add_password(hash1.clone());

        assert!(history.has_been_used(&hash1));
        assert!(!history.has_been_used(&hash2));
    }

    #[test]
    fn test_last_password() {
        let user_id = UserId::new();
        let mut history = PasswordHistory::new(user_id);

        assert!(history.last_password().is_none());

        let hash1 = valid_hash("1");
        history.add_password(hash1.clone());
        assert_eq!(history.last_password().unwrap().password_hash(), &hash1);

        let hash2 = valid_hash("2");
        history.add_password(hash2.clone());
        assert_eq!(history.last_password().unwrap().password_hash(), &hash2);
    }

    #[test]
    fn test_clear_history() {
        let user_id = UserId::new();
        let mut history = PasswordHistory::new(user_id);

        for i in 0..5 {
            let hash = valid_hash(&i.to_string());
            history.add_password(hash);
        }

        assert_eq!(history.count(), 5);
        history.clear();
        assert_eq!(history.count(), 0);
    }

    #[test]
    fn test_get_recent_passwords() {
        let user_id = UserId::new();
        let mut history = PasswordHistory::new(user_id);

        let hash = valid_hash("1");
        history.add_password(hash);

        let recent = history.get_recent_passwords(1); // last 1 day
        assert_eq!(recent.len(), 1);

        let very_old = history.get_recent_passwords(0); // only from today onwards
        // May be 0 or 1 depending on exact timing
        assert!(very_old.is_empty() || very_old.len() == 1);
    }

    #[test]
    fn test_reconstitute() {
        let user_id = UserId::new();
        let hash1 = valid_hash("1");
        let entry = PasswordHistoryEntry::new(user_id.clone(), hash1);
        let entries = vec![entry];

        let history = PasswordHistory::reconstitute(user_id.clone(), entries);
        assert_eq!(history.user_id(), &user_id);
        assert_eq!(history.count(), 1);
    }

    #[test]
    fn test_password_reuse_prevention() {
        let user_id = UserId::new();
        let mut history = PasswordHistory::new(user_id);

        let old_password = valid_hash("old");
        history.add_password(old_password.clone());

        // Try to reuse the old password
        assert!(history.has_been_used(&old_password));

        // New password should not be in history
        let new_password = valid_hash("new");
        assert!(!history.has_been_used(&new_password));
        history.add_password(new_password.clone());

        // Both should now be in history
        assert!(history.has_been_used(&old_password));
        assert!(history.has_been_used(&new_password));
    }
}
