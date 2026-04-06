use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

/// A single entry in the hash chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashChainEntry {
    pub sequence: u64,
    pub previous_hash: String,
    pub current_hash: String,
    pub operation_type: String,
    pub operation_id: Uuid,
    pub data_hash: String,
    pub timestamp: DateTime<Utc>,
}

impl HashChainEntry {
    pub fn new(
        sequence: u64,
        previous_hash: String,
        operation_type: &str,
        operation_id: Uuid,
        data_hash: String,
        current_hash: String,
    ) -> Self {
        Self {
            sequence,
            previous_hash,
            current_hash,
            operation_type: operation_type.to_string(),
            operation_id,
            data_hash,
            timestamp: Utc::now(),
        }
    }
}

/// Errors that can occur in hash chain operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainError {
    InvalidHash {
        sequence: u64,
        expected: String,
        actual: String,
    },
    MissingEntry {
        sequence: u64,
    },
    BrokenLink {
        sequence: u64,
    },
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainError::InvalidHash {
                sequence,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Invalid hash at sequence {}: expected {}, got {}",
                    sequence, expected, actual
                )
            }
            ChainError::MissingEntry { sequence } => {
                write!(f, "Missing entry at sequence {}", sequence)
            }
            ChainError::BrokenLink { sequence } => {
                write!(f, "Broken link at sequence {}: previous_hash mismatch", sequence)
            }
        }
    }
}

/// Immutable hash chain for audit trails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashChain {
    entries: Vec<HashChainEntry>,
}

impl HashChain {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Append a new operation to the chain
    pub fn append(
        &mut self,
        operation_type: &str,
        operation_id: Uuid,
        data: &str,
    ) -> Result<HashChainEntry, DomainError> {
        let sequence = self.entries.len() as u64;

        // Calculate data hash using SHA-256
        let data_hash = Self::compute_sha256(data);

        // Determine previous hash
        let previous_hash = if let Some(last_entry) = self.entries.last() {
            last_entry.current_hash.clone()
        } else {
            "GENESIS".to_string()
        };

        // Compute current hash: SHA256(previous_hash + data_hash + timestamp)
        let timestamp = Utc::now();
        let chain_input = format!(
            "{}{}{}",
            previous_hash,
            data_hash,
            timestamp.timestamp()
        );
        let current_hash = Self::compute_sha256(&chain_input);

        let entry = HashChainEntry::new(
            sequence,
            previous_hash,
            operation_type,
            operation_id,
            data_hash,
            current_hash,
        );

        self.entries.push(entry.clone());
        Ok(entry)
    }

    /// Verify the entire chain integrity
    pub fn verify_chain(&self) -> Result<(), Vec<ChainError>> {
        let mut errors = Vec::new();

        for (idx, entry) in self.entries.iter().enumerate() {
            // Check sequence
            if entry.sequence != idx as u64 {
                errors.push(ChainError::MissingEntry {
                    sequence: entry.sequence,
                });
                continue;
            }

            // Verify hash
            let expected_hash = if idx == 0 {
                // For first entry, previous should be GENESIS
                if entry.previous_hash != "GENESIS" {
                    errors.push(ChainError::BrokenLink {
                        sequence: entry.sequence,
                    });
                    continue;
                }
                let chain_input = format!(
                    "{}{}{}",
                    entry.previous_hash, entry.data_hash, entry.timestamp.timestamp()
                );
                Self::compute_sha256(&chain_input)
            } else {
                // For subsequent entries, verify previous_hash matches
                if let Some(prev_entry) = self.entries.get(idx - 1) {
                    if prev_entry.current_hash != entry.previous_hash {
                        errors.push(ChainError::BrokenLink {
                            sequence: entry.sequence,
                        });
                        continue;
                    }
                    let chain_input = format!(
                        "{}{}{}",
                        entry.previous_hash, entry.data_hash, entry.timestamp.timestamp()
                    );
                    Self::compute_sha256(&chain_input)
                } else {
                    errors.push(ChainError::MissingEntry {
                        sequence: entry.sequence - 1,
                    });
                    continue;
                }
            };

            if expected_hash != entry.current_hash {
                errors.push(ChainError::InvalidHash {
                    sequence: entry.sequence,
                    expected: expected_hash,
                    actual: entry.current_hash.clone(),
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Verify a single entry at a specific sequence
    pub fn verify_entry(&self, sequence: u64) -> bool {
        if let Some(entry) = self.entries.get(sequence as usize) {
            let expected_hash = if sequence == 0 {
                if entry.previous_hash != "GENESIS" {
                    return false;
                }
                let chain_input = format!(
                    "{}{}{}",
                    entry.previous_hash, entry.data_hash, entry.timestamp.timestamp()
                );
                Self::compute_sha256(&chain_input)
            } else if let Some(prev_entry) = self.entries.get((sequence - 1) as usize) {
                if prev_entry.current_hash != entry.previous_hash {
                    return false;
                }
                let chain_input = format!(
                    "{}{}{}",
                    entry.previous_hash, entry.data_hash, entry.timestamp.timestamp()
                );
                Self::compute_sha256(&chain_input)
            } else {
                return false;
            };

            expected_hash == entry.current_hash
        } else {
            false
        }
    }

    /// Get proof of a specific operation (chain from genesis to that operation)
    pub fn get_proof(&self, operation_id: Uuid) -> Option<Vec<HashChainEntry>> {
        self.entries
            .iter()
            .position(|e| e.operation_id == operation_id)
            .map(|idx| self.entries[0..=idx].to_vec())
    }

    /// Get the number of entries in the chain
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get entry at sequence
    pub fn get_entry(&self, sequence: u64) -> Option<&HashChainEntry> {
        self.entries.get(sequence as usize)
    }

    /// Get all entries
    pub fn entries(&self) -> &[HashChainEntry] {
        &self.entries
    }

    /// Compute SHA-256 hash of input
    fn compute_sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl Default for HashChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_genesis_entry() {
        let mut chain = HashChain::new();
        let operation_id = Uuid::new_v4();
        let result = chain.append("CREATE_ACCOUNT", operation_id, "initial_data");

        assert!(result.is_ok());
        let entry = result.unwrap();
        assert_eq!(entry.sequence, 0);
        assert_eq!(entry.previous_hash, "GENESIS");
        assert_eq!(entry.operation_type, "CREATE_ACCOUNT");
    }

    #[test]
    fn test_append_multiple_entries() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let entry1 = chain.append("CREATE", id1, "data1").unwrap();
        let entry2 = chain.append("UPDATE", id2, "data2").unwrap();

        assert_eq!(entry1.sequence, 0);
        assert_eq!(entry2.sequence, 1);
        assert_eq!(entry2.previous_hash, entry1.current_hash);
    }

    #[test]
    fn test_verify_valid_chain() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();

        let _ = chain.append("CREATE", id1, "data1");
        let _ = chain.append("UPDATE", id2, "data2");
        let _ = chain.append("DELETE", id3, "data3");

        let result = chain.verify_chain();
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_single_entry() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let _ = chain.append("CREATE", id1, "data1");
        let _ = chain.append("UPDATE", id2, "data2");

        assert!(chain.verify_entry(0));
        assert!(chain.verify_entry(1));
    }

    #[test]
    fn test_verify_entry_nonexistent() {
        let chain = HashChain::new();
        assert!(!chain.verify_entry(0));
        assert!(!chain.verify_entry(100));
    }

    #[test]
    fn test_detect_tampered_hash() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();

        let _ = chain.append("CREATE", id1, "data1");

        // Manually tamper with the entry
        if let Some(entry) = chain.entries.get_mut(0) {
            entry.current_hash = "tampered_hash".to_string();
        }

        let result = chain.verify_chain();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
        assert!(matches!(errors[0], ChainError::InvalidHash { .. }));
    }

    #[test]
    fn test_detect_broken_link_in_middle() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();

        let _ = chain.append("CREATE", id1, "data1");
        let _ = chain.append("UPDATE", id2, "data2");
        let _ = chain.append("DELETE", id3, "data3");

        // Tamper with middle entry's previous_hash
        if let Some(entry) = chain.entries.get_mut(1) {
            entry.previous_hash = "broken_link".to_string();
        }

        let result = chain.verify_chain();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_get_proof_for_operation() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();

        let _ = chain.append("CREATE", id1, "data1");
        let _ = chain.append("UPDATE", id2, "data2");
        let _ = chain.append("DELETE", id3, "data3");

        // Get proof for second operation
        let proof = chain.get_proof(id2);
        assert!(proof.is_some());

        let proof_entries = proof.unwrap();
        assert_eq!(proof_entries.len(), 2); // Should include genesis and the operation
        assert_eq!(proof_entries[0].operation_id, id1);
        assert_eq!(proof_entries[1].operation_id, id2);
    }

    #[test]
    fn test_get_proof_nonexistent_operation() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let nonexistent = Uuid::new_v4();

        let _ = chain.append("CREATE", id1, "data1");
        let _ = chain.append("UPDATE", id2, "data2");

        let proof = chain.get_proof(nonexistent);
        assert!(proof.is_none());
    }

    #[test]
    fn test_chain_length() {
        let mut chain = HashChain::new();
        assert_eq!(chain.len(), 0);

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let _ = chain.append("CREATE", id1, "data1");
        assert_eq!(chain.len(), 1);

        let _ = chain.append("UPDATE", id2, "data2");
        assert_eq!(chain.len(), 2);
    }

    #[test]
    fn test_hash_chain_immutability() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let _ = chain.append("CREATE", id1, "data1");
        let hash1 = chain.entries[0].current_hash.clone();

        let _ = chain.append("UPDATE", id2, "data2");

        // First entry's hash should not change
        assert_eq!(chain.entries[0].current_hash, hash1);
    }

    #[test]
    fn test_empty_chain_verification() {
        let chain = HashChain::new();
        let result = chain.verify_chain();
        assert!(result.is_ok()); // Empty chain is valid
    }

    #[test]
    fn test_genesis_entry_detection() {
        let mut chain = HashChain::new();
        let id1 = Uuid::new_v4();
        let _ = chain.append("CREATE", id1, "data1");

        // First entry should always have GENESIS as previous
        assert_eq!(chain.entries[0].previous_hash, "GENESIS");
    }
}
