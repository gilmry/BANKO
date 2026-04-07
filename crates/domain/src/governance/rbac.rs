use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Newtypes for RBAC Identifiers
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleId(Uuid);

impl RoleId {
    pub fn new() -> Self {
        RoleId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        RoleId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for RoleId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RoleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PermissionId(Uuid);

impl PermissionId {
    pub fn new() -> Self {
        PermissionId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        PermissionId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for PermissionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PermissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// Permission — granular access control (FR-140, FR-141)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    id: PermissionId,
    name: String,
    resource: String,          // e.g., "Account", "Loan", "User"
    action: String,             // e.g., "CREATE", "READ", "UPDATE", "DELETE", "APPROVE"
    description: Option<String>,
    created_at: DateTime<Utc>,
}

impl Permission {
    pub fn new(
        name: String,
        resource: String,
        action: String,
        description: Option<String>,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidInput("Permission name cannot be empty".into()));
        }
        if resource.is_empty() {
            return Err(DomainError::InvalidInput(
                "Permission resource cannot be empty".into(),
            ));
        }
        if action.is_empty() {
            return Err(DomainError::InvalidInput("Permission action cannot be empty".into()));
        }

        Ok(Permission {
            id: PermissionId::new(),
            name,
            resource,
            action,
            description,
            created_at: Utc::now(),
        })
    }

    // Accessors
    pub fn id(&self) -> &PermissionId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Shorthand for checking permission match (resource:action)
    pub fn matches(&self, resource: &str, action: &str) -> bool {
        self.resource == resource && self.action == action
    }
}

// ============================================================
// Role — hierarchical role-based access control (FR-140)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    id: RoleId,
    name: String,
    description: Option<String>,
    parent_role_id: Option<RoleId>, // For inheritance (FR-140)
    permissions: HashSet<PermissionId>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(
        name: String,
        description: Option<String>,
        parent_role_id: Option<RoleId>,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidInput("Role name cannot be empty".into()));
        }

        Ok(Role {
            id: RoleId::new(),
            name,
            description,
            parent_role_id,
            permissions: HashSet::new(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    // Accessors
    pub fn id(&self) -> &RoleId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn parent_role_id(&self) -> Option<&RoleId> {
        self.parent_role_id.as_ref()
    }

    pub fn permissions(&self) -> &HashSet<PermissionId> {
        &self.permissions
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    /// Add a permission to this role
    pub fn grant_permission(&mut self, permission_id: PermissionId) -> Result<(), DomainError> {
        if !self.is_active {
            return Err(DomainError::InvalidInput("Cannot grant permission to inactive role".into()));
        }
        self.permissions.insert(permission_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a permission from this role
    pub fn revoke_permission(&mut self, permission_id: &PermissionId) -> Result<(), DomainError> {
        self.permissions.remove(permission_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if role has a specific permission
    pub fn has_permission(&self, permission_id: &PermissionId) -> bool {
        self.permissions.contains(permission_id)
    }

    /// Deactivate the role
    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        self.is_active = false;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Activate the role
    pub fn activate(&mut self) -> Result<(), DomainError> {
        self.is_active = true;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// ============================================================
// Segregation of Duties — enforce incompatible roles (FR-142)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegregationOfDuties {
    incompatible_pairs: HashSet<(RoleId, RoleId)>,
    incompatible_actions: HashMap<String, HashSet<String>>, // resource:action -> forbidden actions
}

impl SegregationOfDuties {
    pub fn new() -> Self {
        SegregationOfDuties {
            incompatible_pairs: HashSet::new(),
            incompatible_actions: HashMap::new(),
        }
    }

    /// Register two roles as incompatible (cannot be held by same user)
    pub fn add_incompatible_roles(
        &mut self,
        role1: RoleId,
        role2: RoleId,
    ) -> Result<(), DomainError> {
        // Normalize order for consistency
        let (a, b) = if role1.as_uuid() < role2.as_uuid() {
            (role1, role2)
        } else {
            (role2, role1)
        };
        self.incompatible_pairs.insert((a, b));
        Ok(())
    }

    /// Check if two roles are incompatible
    pub fn are_incompatible(&self, role1: &RoleId, role2: &RoleId) -> bool {
        let (a, b) = if role1.as_uuid() < role2.as_uuid() {
            (role1.clone(), role2.clone())
        } else {
            (role2.clone(), role1.clone())
        };
        self.incompatible_pairs.contains(&(a, b))
    }

    /// Register incompatible actions (e.g., "Account:CREATE" cannot be followed by "Account:APPROVE" by same user)
    pub fn add_incompatible_actions(
        &mut self,
        action1: String,
        action2: String,
    ) -> Result<(), DomainError> {
        self.incompatible_actions
            .entry(action1)
            .or_insert_with(HashSet::new)
            .insert(action2);
        Ok(())
    }

    /// Check if action1 is incompatible with action2
    pub fn are_actions_incompatible(&self, action1: &str, action2: &str) -> bool {
        self.incompatible_actions
            .get(action1)
            .map(|forbidden| forbidden.contains(action2))
            .unwrap_or(false)
    }

    /// Get all incompatible pairs
    pub fn incompatible_pairs(&self) -> &HashSet<(RoleId, RoleId)> {
        &self.incompatible_pairs
    }
}

impl Default for SegregationOfDuties {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// User Role Assignment — tracks roles per user
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleAssignment {
    user_id: Uuid,
    role_ids: HashSet<RoleId>,
    assigned_at: DateTime<Utc>,
    assigned_by: Uuid,
}

impl UserRoleAssignment {
    pub fn new(user_id: Uuid, assigned_by: Uuid) -> Self {
        UserRoleAssignment {
            user_id,
            role_ids: HashSet::new(),
            assigned_at: Utc::now(),
            assigned_by,
        }
    }

    // Accessors
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn role_ids(&self) -> &HashSet<RoleId> {
        &self.role_ids
    }

    pub fn assigned_at(&self) -> &DateTime<Utc> {
        &self.assigned_at
    }

    pub fn assigned_by(&self) -> Uuid {
        self.assigned_by
    }

    /// Assign a role to the user
    pub fn assign_role(&mut self, role_id: RoleId) {
        self.role_ids.insert(role_id);
    }

    /// Remove a role from the user
    pub fn remove_role(&mut self, role_id: &RoleId) {
        self.role_ids.remove(role_id);
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role_id: &RoleId) -> bool {
        self.role_ids.contains(role_id)
    }

    /// Get all roles assigned to user
    pub fn all_roles(&self) -> Vec<RoleId> {
        self.role_ids.iter().cloned().collect()
    }
}

// ============================================================
// Privilege Escalation Detection
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeEscalationEvent {
    pub user_id: Uuid,
    pub from_roles: Vec<RoleId>,
    pub to_roles: Vec<RoleId>,
    pub detected_at: DateTime<Utc>,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &str {
        match self {
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::Critical => "Critical",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_creation() {
        let perm = Permission::new(
            "create_account".to_string(),
            "Account".to_string(),
            "CREATE".to_string(),
            Some("Create new bank accounts".to_string()),
        )
        .unwrap();

        assert_eq!(perm.name(), "create_account");
        assert_eq!(perm.resource(), "Account");
        assert_eq!(perm.action(), "CREATE");
    }

    #[test]
    fn test_permission_matches() {
        let perm = Permission::new(
            "create_account".to_string(),
            "Account".to_string(),
            "CREATE".to_string(),
            None,
        )
        .unwrap();

        assert!(perm.matches("Account", "CREATE"));
        assert!(!perm.matches("Account", "DELETE"));
        assert!(!perm.matches("Loan", "CREATE"));
    }

    #[test]
    fn test_role_creation_with_parent() {
        let parent_role_id = RoleId::new();
        let role = Role::new(
            "Manager".to_string(),
            Some("Operational manager".to_string()),
            Some(parent_role_id.clone()),
        )
        .unwrap();

        assert_eq!(role.name(), "Manager");
        assert_eq!(role.parent_role_id(), Some(&parent_role_id));
        assert!(role.is_active());
    }

    #[test]
    fn test_role_grant_and_revoke_permission() {
        let mut role = Role::new(
            "Approver".to_string(),
            None,
            None,
        )
        .unwrap();

        let perm_id = PermissionId::new();
        assert!(!role.has_permission(&perm_id));

        role.grant_permission(perm_id.clone()).unwrap();
        assert!(role.has_permission(&perm_id));

        role.revoke_permission(&perm_id).unwrap();
        assert!(!role.has_permission(&perm_id));
    }

    #[test]
    fn test_role_activation_deactivation() {
        let mut role = Role::new(
            "Operator".to_string(),
            None,
            None,
        )
        .unwrap();

        assert!(role.is_active());

        role.deactivate().unwrap();
        assert!(!role.is_active());

        role.activate().unwrap();
        assert!(role.is_active());
    }

    #[test]
    fn test_sod_incompatible_roles() {
        let mut sod = SegregationOfDuties::new();
        let role1 = RoleId::new();
        let role2 = RoleId::new();

        sod.add_incompatible_roles(role1.clone(), role2.clone())
            .unwrap();

        assert!(sod.are_incompatible(&role1, &role2));
        assert!(sod.are_incompatible(&role2, &role1)); // Order-independent
    }

    #[test]
    fn test_sod_incompatible_actions() {
        let mut sod = SegregationOfDuties::new();

        sod.add_incompatible_actions(
            "Account:CREATE".to_string(),
            "Account:APPROVE".to_string(),
        )
        .unwrap();

        assert!(sod.are_actions_incompatible("Account:CREATE", "Account:APPROVE"));
        assert!(!sod.are_actions_incompatible("Account:APPROVE", "Account:CREATE"));
    }

    #[test]
    fn test_user_role_assignment() {
        let user_id = Uuid::new_v4();
        let assigned_by = Uuid::new_v4();
        let mut assignment = UserRoleAssignment::new(user_id, assigned_by);

        let role_id = RoleId::new();
        assignment.assign_role(role_id.clone());
        assert!(assignment.has_role(&role_id));

        assignment.remove_role(&role_id);
        assert!(!assignment.has_role(&role_id));
    }

    #[test]
    fn test_user_multiple_roles() {
        let user_id = Uuid::new_v4();
        let mut assignment = UserRoleAssignment::new(user_id, Uuid::new_v4());

        let role1 = RoleId::new();
        let role2 = RoleId::new();
        let role3 = RoleId::new();

        assignment.assign_role(role1.clone());
        assignment.assign_role(role2.clone());
        assignment.assign_role(role3.clone());

        assert_eq!(assignment.all_roles().len(), 3);
        assert!(assignment.has_role(&role1));
        assert!(assignment.has_role(&role2));
        assert!(assignment.has_role(&role3));
    }

    #[test]
    fn test_permission_empty_name_fails() {
        let result = Permission::new(
            "".to_string(),
            "Account".to_string(),
            "CREATE".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_role_empty_name_fails() {
        let result = Role::new(
            "".to_string(),
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_grant_permission_to_inactive_role() {
        let mut role = Role::new(
            "Inactive".to_string(),
            None,
            None,
        )
        .unwrap();

        role.deactivate().unwrap();

        let perm_id = PermissionId::new();
        let result = role.grant_permission(perm_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_role_inheritance_chain() {
        let grandparent = RoleId::new();
        let parent = RoleId::from_uuid(Uuid::new_v4());
        let child = Role::new(
            "Child".to_string(),
            None,
            Some(parent.clone()),
        )
        .unwrap();

        assert_eq!(child.parent_role_id(), Some(&parent));
    }

    #[test]
    fn test_risk_level_display() {
        assert_eq!(RiskLevel::Low.as_str(), "Low");
        assert_eq!(RiskLevel::Medium.as_str(), "Medium");
        assert_eq!(RiskLevel::High.as_str(), "High");
        assert_eq!(RiskLevel::Critical.as_str(), "Critical");
    }
}
