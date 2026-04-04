use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;
use crate::shared::value_objects::EmailAddress;

// --- UserId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        UserId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(UserId)
            .map_err(|e| DomainError::InvalidUser(format!("Invalid UserId UUID: {e}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- PasswordHash ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash {
    hash: String,
}

impl PasswordHash {
    pub fn new(hash: String) -> Result<Self, DomainError> {
        if hash.is_empty() {
            return Err(DomainError::InvalidPasswordHash(
                "Password hash cannot be empty".to_string(),
            ));
        }
        if hash.len() < 20 {
            return Err(DomainError::InvalidPasswordHash(
                "Password hash is too short to be a valid hash".to_string(),
            ));
        }
        Ok(PasswordHash { hash })
    }

    pub fn as_str(&self) -> &str {
        &self.hash
    }
}

impl fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***")
    }
}

// --- Role ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
    SuperAdmin,
    Analyst,
    Compliance,
    CRO,
}

impl Role {
    pub fn from_str_role(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Role::User),
            "admin" => Ok(Role::Admin),
            "superadmin" | "super_admin" => Ok(Role::SuperAdmin),
            "analyst" => Ok(Role::Analyst),
            "compliance" => Ok(Role::Compliance),
            "cro" => Ok(Role::CRO),
            _ => Err(DomainError::InvalidRole(format!("Unknown role: {s}"))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Role::User => "user",
            Role::Admin => "admin",
            Role::SuperAdmin => "superadmin",
            Role::Analyst => "analyst",
            Role::Compliance => "compliance",
            Role::CRO => "cro",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- Permission ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    CreateAccount,
    ViewAccount,
    ManageUsers,
    ViewAudit,
    ManageRoles,
    ViewReports,
    ManageCredit,
    ManageCompliance,
    ManageSanctions,
    ViewDashboard,
}

impl Permission {
    pub fn as_str(&self) -> &str {
        match self {
            Permission::CreateAccount => "create_account",
            Permission::ViewAccount => "view_account",
            Permission::ManageUsers => "manage_users",
            Permission::ViewAudit => "view_audit",
            Permission::ManageRoles => "manage_roles",
            Permission::ViewReports => "view_reports",
            Permission::ManageCredit => "manage_credit",
            Permission::ManageCompliance => "manage_compliance",
            Permission::ManageSanctions => "manage_sanctions",
            Permission::ViewDashboard => "view_dashboard",
        }
    }
}

impl Role {
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::User => vec![Permission::ViewAccount, Permission::ViewDashboard],
            Role::Analyst => vec![
                Permission::ViewAccount,
                Permission::ViewReports,
                Permission::ViewDashboard,
            ],
            Role::Compliance => vec![
                Permission::ViewAccount,
                Permission::ViewAudit,
                Permission::ManageCompliance,
                Permission::ManageSanctions,
                Permission::ViewDashboard,
            ],
            Role::CRO => vec![
                Permission::ViewAccount,
                Permission::ViewAudit,
                Permission::ViewReports,
                Permission::ManageCredit,
                Permission::ViewDashboard,
            ],
            Role::Admin => vec![
                Permission::CreateAccount,
                Permission::ViewAccount,
                Permission::ManageUsers,
                Permission::ViewAudit,
                Permission::ViewReports,
                Permission::ViewDashboard,
            ],
            Role::SuperAdmin => vec![
                Permission::CreateAccount,
                Permission::ViewAccount,
                Permission::ManageUsers,
                Permission::ViewAudit,
                Permission::ManageRoles,
                Permission::ViewReports,
                Permission::ManageCredit,
                Permission::ManageCompliance,
                Permission::ManageSanctions,
                Permission::ViewDashboard,
            ],
        }
    }
}

// --- User (Aggregate Root) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    email: EmailAddress,
    password_hash: PasswordHash,
    roles: Vec<Role>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        email: EmailAddress,
        password_hash: PasswordHash,
    ) -> Result<Self, DomainError> {
        let now = Utc::now();
        Ok(User {
            id: UserId::new(),
            email,
            password_hash,
            roles: vec![Role::User],
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: UserId,
        email: EmailAddress,
        password_hash: PasswordHash,
        roles: Vec<Role>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        User {
            id,
            email,
            password_hash,
            roles,
            is_active,
            created_at,
            updated_at,
        }
    }

    // --- Getters (immutable access) ---

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn roles(&self) -> &[Role] {
        &self.roles
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Role::Admin) || self.roles.contains(&Role::SuperAdmin)
    }

    pub fn is_super_admin(&self) -> bool {
        self.roles.contains(&Role::SuperAdmin)
    }

    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }

    pub fn has_permission(&self, permission: Permission) -> bool {
        self.roles
            .iter()
            .any(|role| role.permissions().contains(&permission))
    }

    pub fn assign_role(&mut self, role: Role) {
        if !self.roles.contains(&role) {
            self.roles.push(role);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_role(&mut self, role: &Role) -> Result<(), DomainError> {
        if *role == Role::User && self.roles.len() == 1 {
            return Err(DomainError::InvalidUser(
                "Cannot remove the last role from a user".to_string(),
            ));
        }
        self.roles.retain(|r| r != role);
        if self.roles.is_empty() {
            self.roles.push(Role::User);
        }
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_roles(&mut self, roles: Vec<Role>) -> Result<(), DomainError> {
        if roles.is_empty() {
            return Err(DomainError::InvalidUser(
                "User must have at least one role".to_string(),
            ));
        }
        self.roles = roles;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn update_password(&mut self, new_hash: PasswordHash) {
        self.password_hash = new_hash;
        self.updated_at = Utc::now();
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_email() -> EmailAddress {
        EmailAddress::new("test@banko.tn").unwrap()
    }

    fn valid_hash() -> PasswordHash {
        PasswordHash::new(
            "$2b$12$LJ3m4ys3Lg2HEOjdLNRsWuBNRZLJDhG5JQqJK9qJKj3K4hNqXKwu".to_string(),
        )
        .unwrap()
    }

    // --- UserId tests ---

    #[test]
    fn test_user_id_new_generates_unique() {
        let id1 = UserId::new();
        let id2 = UserId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_user_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = UserId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_user_id_parse_valid() {
        let id = UserId::parse("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(id.to_string(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_user_id_parse_invalid() {
        assert!(UserId::parse("not-a-uuid").is_err());
    }

    // --- PasswordHash tests ---

    #[test]
    fn test_password_hash_valid() {
        let hash = PasswordHash::new(
            "$2b$12$LJ3m4ys3Lg2HEOjdLNRsWuBNRZLJDhG5JQqJK9qJKj3K4hNqXKwu".to_string(),
        );
        assert!(hash.is_ok());
    }

    #[test]
    fn test_password_hash_empty_rejected() {
        let hash = PasswordHash::new(String::new());
        assert!(hash.is_err());
    }

    #[test]
    fn test_password_hash_too_short_rejected() {
        let hash = PasswordHash::new("short".to_string());
        assert!(hash.is_err());
    }

    #[test]
    fn test_password_hash_not_displayed() {
        let hash = valid_hash();
        assert_eq!(format!("{hash}"), "***");
    }

    #[test]
    fn test_password_hash_not_equal_to_plain() {
        let hash = valid_hash();
        assert_ne!(hash.as_str(), "password");
    }

    // --- Role tests ---

    #[test]
    fn test_role_from_str_valid() {
        assert_eq!(Role::from_str_role("user").unwrap(), Role::User);
        assert_eq!(Role::from_str_role("Admin").unwrap(), Role::Admin);
        assert_eq!(Role::from_str_role("SuperAdmin").unwrap(), Role::SuperAdmin);
        assert_eq!(Role::from_str_role("analyst").unwrap(), Role::Analyst);
        assert_eq!(Role::from_str_role("Compliance").unwrap(), Role::Compliance);
        assert_eq!(Role::from_str_role("CRO").unwrap(), Role::CRO);
    }

    #[test]
    fn test_role_from_str_invalid() {
        assert!(Role::from_str_role("unknown").is_err());
    }

    #[test]
    fn test_role_permissions_user() {
        let perms = Role::User.permissions();
        assert!(perms.contains(&Permission::ViewAccount));
        assert!(perms.contains(&Permission::ViewDashboard));
        assert!(!perms.contains(&Permission::ManageUsers));
    }

    #[test]
    fn test_role_permissions_admin() {
        let perms = Role::Admin.permissions();
        assert!(perms.contains(&Permission::ManageUsers));
        assert!(perms.contains(&Permission::ViewAudit));
    }

    #[test]
    fn test_role_permissions_super_admin_has_all() {
        let perms = Role::SuperAdmin.permissions();
        assert!(perms.contains(&Permission::ManageRoles));
        assert!(perms.contains(&Permission::ManageUsers));
        assert!(perms.contains(&Permission::ManageCompliance));
    }

    // --- User aggregate tests ---

    #[test]
    fn test_user_new_success() {
        let user = User::new(valid_email(), valid_hash()).unwrap();
        assert_eq!(user.email().as_str(), "test@banko.tn");
        assert!(user.is_active());
        assert!(!user.id().as_uuid().is_nil());
    }

    #[test]
    fn test_user_default_role_is_user() {
        let user = User::new(valid_email(), valid_hash()).unwrap();
        assert_eq!(user.roles(), &[Role::User]);
    }

    #[test]
    fn test_user_is_not_admin_by_default() {
        let user = User::new(valid_email(), valid_hash()).unwrap();
        assert!(!user.is_admin());
    }

    #[test]
    fn test_user_assign_role_admin() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        user.assign_role(Role::Admin);
        assert!(user.roles().contains(&Role::Admin));
        assert!(user.is_admin());
    }

    #[test]
    fn test_user_assign_duplicate_role_no_change() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        user.assign_role(Role::User);
        assert_eq!(user.roles().len(), 1);
    }

    #[test]
    fn test_user_has_permission() {
        let user = User::new(valid_email(), valid_hash()).unwrap();
        assert!(user.has_permission(Permission::ViewAccount));
        assert!(!user.has_permission(Permission::ManageUsers));
    }

    #[test]
    fn test_user_admin_has_manage_users_permission() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        user.assign_role(Role::Admin);
        assert!(user.has_permission(Permission::ManageUsers));
    }

    #[test]
    fn test_user_deactivate() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        user.deactivate();
        assert!(!user.is_active());
    }

    #[test]
    fn test_user_activate() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        user.deactivate();
        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_user_set_roles_empty_rejected() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        assert!(user.set_roles(vec![]).is_err());
    }

    #[test]
    fn test_user_set_roles() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        user.set_roles(vec![Role::Admin, Role::Analyst]).unwrap();
        assert_eq!(user.roles().len(), 2);
        assert!(user.has_role(&Role::Admin));
        assert!(user.has_role(&Role::Analyst));
    }

    #[test]
    fn test_user_remove_last_role_keeps_user() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        let result = user.remove_role(&Role::User);
        assert!(result.is_err());
    }

    #[test]
    fn test_user_remove_role_admin() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        user.assign_role(Role::Admin);
        user.remove_role(&Role::Admin).unwrap();
        assert!(!user.is_admin());
        assert!(user.has_role(&Role::User));
    }

    #[test]
    fn test_user_update_password() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        let new_hash = PasswordHash::new(
            "$2b$12$NEWHASHNEWHASHNEWHASHNEWHASHNEWHASHNEWHASHNEWHASHNEW".to_string(),
        )
        .unwrap();
        let old_updated = user.updated_at();
        user.update_password(new_hash.clone());
        assert_eq!(user.password_hash(), &new_hash);
        assert!(user.updated_at() >= old_updated);
    }

    #[test]
    fn test_user_password_hash_is_not_plain_text() {
        let user = User::new(valid_email(), valid_hash()).unwrap();
        assert!(user.password_hash().as_str().starts_with("$2b$"));
    }

    #[test]
    fn test_user_is_super_admin() {
        let mut user = User::new(valid_email(), valid_hash()).unwrap();
        assert!(!user.is_super_admin());
        user.assign_role(Role::SuperAdmin);
        assert!(user.is_super_admin());
        assert!(user.is_admin()); // SuperAdmin implies admin
    }

    #[test]
    fn test_user_reconstitute() {
        let id = UserId::new();
        let email = valid_email();
        let hash = valid_hash();
        let now = Utc::now();
        let user = User::reconstitute(
            id.clone(),
            email.clone(),
            hash.clone(),
            vec![Role::Admin, Role::User],
            true,
            now,
            now,
        );
        assert_eq!(user.id(), &id);
        assert_eq!(user.email(), &email);
        assert!(user.is_admin());
        assert_eq!(user.roles().len(), 2);
    }

    #[test]
    fn test_user_has_role() {
        let user = User::new(valid_email(), valid_hash()).unwrap();
        assert!(user.has_role(&Role::User));
        assert!(!user.has_role(&Role::Admin));
    }

    #[test]
    fn test_email_validation_reused_from_shared() {
        assert!(EmailAddress::new("invalid").is_err());
        assert!(EmailAddress::new("valid@banko.tn").is_ok());
    }
}
