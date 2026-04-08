use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- GroupId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupId(Uuid);

impl Default for GroupId {
    fn default() -> Self {
        Self::new()
    }
}

impl GroupId {
    pub fn new() -> Self {
        GroupId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        GroupId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- GroupType (FR-007: Economic Grouping) ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupType {
    Family,
    BusinessGroup,
    Partnership,
    Syndicate,
    Other,
}

impl GroupType {
    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "family" => Ok(GroupType::Family),
            "businessgroup" | "business_group" => Ok(GroupType::BusinessGroup),
            "partnership" => Ok(GroupType::Partnership),
            "syndicate" => Ok(GroupType::Syndicate),
            "other" => Ok(GroupType::Other),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown group type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            GroupType::Family => "Family",
            GroupType::BusinessGroup => "BusinessGroup",
            GroupType::Partnership => "Partnership",
            GroupType::Syndicate => "Syndicate",
            GroupType::Other => "Other",
        }
    }
}

impl std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- CustomerGroup (FR-007: Link related customers/families/enterprises) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerGroup {
    group_id: GroupId,
    group_name: String,
    group_type: GroupType,
    members: Vec<Uuid>,  // Customer IDs
    parent_customer_id: Option<Uuid>,  // Optional primary customer
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CustomerGroup {
    /// Create a new customer group with at least one member.
    pub fn new(
        group_name: &str,
        group_type: GroupType,
        members: Vec<Uuid>,
    ) -> Result<Self, DomainError> {
        let group_name = group_name.trim().to_string();
        if group_name.is_empty() {
            return Err(DomainError::ValidationError(
                "Group name cannot be empty".to_string(),
            ));
        }

        if members.is_empty() {
            return Err(DomainError::ValidationError(
                "Group must have at least one member".to_string(),
            ));
        }

        // Check for duplicates
        let mut unique_members = members.clone();
        unique_members.sort();
        unique_members.dedup();
        if unique_members.len() != members.len() {
            return Err(DomainError::ValidationError(
                "Group members must be unique".to_string(),
            ));
        }

        let now = Utc::now();
        Ok(CustomerGroup {
            group_id: GroupId::new(),
            group_name,
            group_type,
            members,
            parent_customer_id: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        group_id: GroupId,
        group_name: String,
        group_type: GroupType,
        members: Vec<Uuid>,
        parent_customer_id: Option<Uuid>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        CustomerGroup {
            group_id,
            group_name,
            group_type,
            members,
            parent_customer_id,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn group_id(&self) -> &GroupId {
        &self.group_id
    }

    pub fn group_name(&self) -> &str {
        &self.group_name
    }

    pub fn group_type(&self) -> GroupType {
        self.group_type
    }

    pub fn members(&self) -> &[Uuid] {
        &self.members
    }

    pub fn parent_customer_id(&self) -> Option<Uuid> {
        self.parent_customer_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    /// Set the parent/primary customer for this group.
    pub fn set_parent(&mut self, customer_id: Uuid) -> Result<(), DomainError> {
        if !self.members.contains(&customer_id) {
            return Err(DomainError::ValidationError(
                "Parent customer must be a member of the group".to_string(),
            ));
        }
        self.parent_customer_id = Some(customer_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Add a member to the group.
    pub fn add_member(&mut self, customer_id: Uuid) -> Result<(), DomainError> {
        if self.members.contains(&customer_id) {
            return Err(DomainError::ValidationError(
                "Customer is already a member of this group".to_string(),
            ));
        }
        self.members.push(customer_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a member from the group. Group must have at least one member remaining.
    pub fn remove_member(&mut self, customer_id: Uuid) -> Result<(), DomainError> {
        if self.members.len() <= 1 {
            return Err(DomainError::ValidationError(
                "Group must have at least one member".to_string(),
            ));
        }

        let initial_len = self.members.len();
        self.members.retain(|id| id != &customer_id);

        if self.members.len() == initial_len {
            return Err(DomainError::ValidationError(
                "Customer is not a member of this group".to_string(),
            ));
        }

        // Clear parent if they were removed
        if self.parent_customer_id == Some(customer_id) {
            self.parent_customer_id = None;
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if a customer is a member of this group.
    pub fn is_member(&self, customer_id: Uuid) -> bool {
        self.members.contains(&customer_id)
    }

    /// Get member count.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_customer_id() -> Uuid {
        Uuid::new_v4()
    }

    #[test]
    fn test_customer_group_new_valid() {
        let id1 = sample_customer_id();
        let id2 = sample_customer_id();
        let group =
            CustomerGroup::new("Family Smith", GroupType::Family, vec![id1, id2]).unwrap();

        assert_eq!(group.group_name(), "Family Smith");
        assert_eq!(group.group_type(), GroupType::Family);
        assert_eq!(group.member_count(), 2);
        assert!(group.is_member(id1));
        assert!(group.is_member(id2));
    }

    #[test]
    fn test_customer_group_empty_name() {
        let id = sample_customer_id();
        let result = CustomerGroup::new("", GroupType::Family, vec![id]);
        assert!(result.is_err());
    }

    #[test]
    fn test_customer_group_no_members() {
        let result = CustomerGroup::new("Family", GroupType::Family, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_customer_group_duplicate_members() {
        let id = sample_customer_id();
        let result = CustomerGroup::new("Family", GroupType::Family, vec![id, id]);
        assert!(result.is_err());
    }

    #[test]
    fn test_customer_group_set_parent() {
        let id1 = sample_customer_id();
        let id2 = sample_customer_id();
        let mut group = CustomerGroup::new("Family", GroupType::Family, vec![id1, id2]).unwrap();

        assert!(group.set_parent(id1).is_ok());
        assert_eq!(group.parent_customer_id(), Some(id1));
    }

    #[test]
    fn test_customer_group_set_parent_non_member() {
        let id1 = sample_customer_id();
        let id2 = sample_customer_id();
        let id3 = sample_customer_id();
        let mut group = CustomerGroup::new("Family", GroupType::Family, vec![id1, id2]).unwrap();

        let result = group.set_parent(id3);
        assert!(result.is_err());
    }

    #[test]
    fn test_customer_group_add_member() {
        let id1 = sample_customer_id();
        let id2 = sample_customer_id();
        let mut group = CustomerGroup::new("Family", GroupType::Family, vec![id1]).unwrap();

        assert!(group.add_member(id2).is_ok());
        assert_eq!(group.member_count(), 2);
        assert!(group.is_member(id2));
    }

    #[test]
    fn test_customer_group_add_duplicate_member() {
        let id1 = sample_customer_id();
        let id2 = sample_customer_id();
        let mut group = CustomerGroup::new("Family", GroupType::Family, vec![id1, id2]).unwrap();

        let result = group.add_member(id2);
        assert!(result.is_err());
    }

    #[test]
    fn test_customer_group_remove_member() {
        let id1 = sample_customer_id();
        let id2 = sample_customer_id();
        let mut group = CustomerGroup::new("Family", GroupType::Family, vec![id1, id2]).unwrap();

        assert!(group.remove_member(id1).is_ok());
        assert_eq!(group.member_count(), 1);
        assert!(!group.is_member(id1));
    }

    #[test]
    fn test_customer_group_cannot_remove_last_member() {
        let id = sample_customer_id();
        let mut group = CustomerGroup::new("Family", GroupType::Family, vec![id]).unwrap();

        let result = group.remove_member(id);
        assert!(result.is_err());
    }

    #[test]
    fn test_customer_group_remove_clears_parent() {
        let id1 = sample_customer_id();
        let id2 = sample_customer_id();
        let mut group = CustomerGroup::new("Family", GroupType::Family, vec![id1, id2]).unwrap();
        group.set_parent(id1).unwrap();

        assert!(group.remove_member(id1).is_ok());
        assert_eq!(group.parent_customer_id(), None);
    }

    #[test]
    fn test_customer_group_remove_nonexistent_member() {
        let id1 = sample_customer_id();
        let id2 = sample_customer_id();
        let id3 = sample_customer_id();
        let mut group = CustomerGroup::new("Family", GroupType::Family, vec![id1, id2]).unwrap();

        let result = group.remove_member(id3);
        assert!(result.is_err());
    }

    #[test]
    fn test_group_type_from_str() {
        assert_eq!(
            GroupType::from_str_type("Family").unwrap(),
            GroupType::Family
        );
        assert_eq!(
            GroupType::from_str_type("business_group").unwrap(),
            GroupType::BusinessGroup
        );
        assert_eq!(
            GroupType::from_str_type("Partnership").unwrap(),
            GroupType::Partnership
        );
    }

    #[test]
    fn test_group_type_invalid() {
        assert!(GroupType::from_str_type("unknown").is_err());
    }
}
