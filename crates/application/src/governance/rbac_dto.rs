use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================
// Permission DTOs
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub resource: String,       // e.g., "Account"
    pub action: String,          // e.g., "CREATE"
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResponse {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ============================================================
// Role DTOs
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
    pub parent_role_id: Option<String>,  // Parent role UUID (for hierarchy)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_role_id: Option<String>,
    pub permissions: Vec<String>,  // List of permission IDs
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantPermissionRequest {
    pub role_id: String,
    pub permission_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokePermissionRequest {
    pub role_id: String,
    pub permission_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleHierarchyResponse {
    pub role_id: String,
    pub name: String,
    pub parent_role_id: Option<String>,
    pub children: Vec<RoleHierarchyResponse>,
    pub all_permissions: Vec<PermissionResponse>,  // Effective permissions including inherited
}

// ============================================================
// Segregation of Duties DTOs
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddIncompatibleRolesRequest {
    pub role1_id: String,
    pub role2_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoDAuditResponse {
    pub user_id: String,
    pub assigned_roles: Vec<String>,
    pub conflicts: Vec<RoleConflict>,
    pub compliant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConflict {
    pub role1: String,
    pub role2: String,
    pub reason: String,
}

// ============================================================
// User Role Assignment DTOs
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRoleRequest {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRolesResponse {
    pub user_id: String,
    pub assigned_roles: Vec<RoleResponse>,
    pub effective_permissions: Vec<PermissionResponse>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessResponse {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<PermissionResponse>,
    pub delegated_permissions: Vec<DelegatedPermissionInfo>,
    pub access_compliance: AccessComplianceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedPermissionInfo {
    pub scope: String,
    pub delegated_by: String,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessComplianceStatus {
    pub compliant: bool,
    pub violations: Vec<String>,
    pub last_reviewed: Option<DateTime<Utc>>,
}

// ============================================================
// Role Change Audit DTOs
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleChangeRequest {
    pub user_id: String,
    pub role_id: String,
    pub operation: String,  // "ASSIGN" or "REVOKE"
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleChangeResponse {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub operation: String,
    pub reason: String,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
    pub status: String,  // "SUCCESS" or "REJECTED"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleChangeHistoryResponse {
    pub user_id: String,
    pub changes: Vec<RoleChangeAuditEntry>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleChangeAuditEntry {
    pub role_id: String,
    pub operation: String,  // "ASSIGNED" or "REVOKED"
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
}

// ============================================================
// Bulk Operations DTOs
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRoleAssignmentRequest {
    pub user_ids: Vec<String>,
    pub role_id: String,
    pub reason: String,
    pub require_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRoleAssignmentResponse {
    pub total_users: usize,
    pub successful: usize,
    pub failed: usize,
    pub details: Vec<BulkOperationDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationDetail {
    pub user_id: String,
    pub status: String,  // "SUCCESS" or "FAILED"
    pub error: Option<String>,
}

// ============================================================
// Dashboard/Reporting DTOs
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacDashboardResponse {
    pub total_roles: i64,
    pub active_roles: i64,
    pub total_permissions: i64,
    pub users_with_roles: i64,
    pub users_with_conflicts: i64,
    pub privileged_users: Vec<PrivilegedUserInfo>,
    pub last_sod_check: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegedUserInfo {
    pub user_id: String,
    pub role_count: i64,
    pub permission_count: i64,
    pub highest_privilege_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMatrixResponse {
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub matrix: Vec<Vec<bool>>,  // [role_index][permission_index]
}
