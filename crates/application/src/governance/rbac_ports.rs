use async_trait::async_trait;
use uuid::Uuid;

use banko_domain::governance::{
    Role, RoleId, Permission, PermissionId, SegregationOfDuties, UserRoleAssignment,
};

// ============================================================
// Role Repository Port
// ============================================================

#[async_trait]
pub trait IRoleRepository: Send + Sync {
    /// Save or update a role
    async fn save(&self, role: &Role) -> Result<(), String>;

    /// Find role by ID
    async fn find_by_id(&self, id: &RoleId) -> Result<Option<Role>, String>;

    /// Find all roles
    async fn find_all(&self) -> Result<Vec<Role>, String>;

    /// Find active roles only
    async fn find_active(&self) -> Result<Vec<Role>, String>;

    /// Find roles by parent role ID (for role hierarchy)
    async fn find_by_parent(&self, parent_id: &RoleId) -> Result<Vec<Role>, String>;

    /// Delete a role (soft delete or archive)
    async fn delete(&self, id: &RoleId) -> Result<(), String>;

    /// Count total roles
    async fn count_all(&self) -> Result<i64, String>;
}

// ============================================================
// Permission Repository Port
// ============================================================

#[async_trait]
pub trait IPermissionRepository: Send + Sync {
    /// Save or update a permission
    async fn save(&self, permission: &Permission) -> Result<(), String>;

    /// Find permission by ID
    async fn find_by_id(&self, id: &PermissionId) -> Result<Option<Permission>, String>;

    /// Find all permissions
    async fn find_all(&self) -> Result<Vec<Permission>, String>;

    /// Find permissions by resource type
    async fn find_by_resource(&self, resource: &str) -> Result<Vec<Permission>, String>;

    /// Find permissions by action type
    async fn find_by_action(&self, action: &str) -> Result<Vec<Permission>, String>;

    /// Find permissions by resource and action
    async fn find_by_resource_and_action(
        &self,
        resource: &str,
        action: &str,
    ) -> Result<Vec<Permission>, String>;

    /// Delete a permission
    async fn delete(&self, id: &PermissionId) -> Result<(), String>;

    /// Count total permissions
    async fn count_all(&self) -> Result<i64, String>;
}

// ============================================================
// Segregation of Duties Repository Port
// ============================================================

#[async_trait]
pub trait ISegregationOfDutiesRepository: Send + Sync {
    /// Save or update SoD rules
    async fn save(&self, sod: &SegregationOfDuties) -> Result<(), String>;

    /// Find all SoD rules
    async fn find_rules(&self) -> Result<SegregationOfDuties, String>;

    /// Check if user has conflicting roles
    async fn check_user_conflict(&self, user_id: Uuid) -> Result<bool, String>;

    /// Get incompatible role pairs
    async fn get_incompatible_pairs(&self) -> Result<Vec<(RoleId, RoleId)>, String>;
}

// ============================================================
// User Role Assignment Repository Port
// ============================================================

#[async_trait]
pub trait IUserRoleRepository: Send + Sync {
    /// Save or update user role assignment
    async fn save(&self, assignment: &UserRoleAssignment) -> Result<(), String>;

    /// Find assignment by user ID
    async fn find_by_user(&self, user_id: Uuid) -> Result<Option<UserRoleAssignment>, String>;

    /// Find all users with a specific role
    async fn find_users_by_role(&self, role_id: &RoleId) -> Result<Vec<Uuid>, String>;

    /// Check if user has a specific role
    async fn user_has_role(&self, user_id: Uuid, role_id: &RoleId) -> Result<bool, String>;

    /// Get all roles for a user (including inherited)
    async fn get_effective_roles(&self, user_id: Uuid) -> Result<Vec<RoleId>, String>;

    /// Get all permissions for a user (effective)
    async fn get_effective_permissions(&self, user_id: Uuid) -> Result<Vec<PermissionId>, String>;

    /// Delete user role assignment
    async fn delete(&self, user_id: Uuid) -> Result<(), String>;

    /// Audit: find role change history
    async fn find_role_history(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<RoleChangeRecord>, String>;
}

#[derive(Debug, Clone)]
pub struct RoleChangeRecord {
    pub user_id: Uuid,
    pub role_id: RoleId,
    pub change_type: String, // "ASSIGNED" or "REVOKED"
    pub changed_by: Uuid,
    pub changed_at: chrono::DateTime<chrono::Utc>,
}
