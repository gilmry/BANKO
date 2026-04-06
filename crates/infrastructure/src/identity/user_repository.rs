use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::identity::IUserRepository;
use banko_domain::identity::{PasswordHash, Role, User, UserId};
use banko_domain::shared::EmailAddress;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        PgUserRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    roles: Vec<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserRow {
    fn into_domain(self) -> Result<User, String> {
        let id = UserId::from_uuid(self.id);
        let email = EmailAddress::new(&self.email).map_err(|e| e.to_string())?;
        let password_hash = PasswordHash::new(self.password_hash).map_err(|e| e.to_string())?;
        let roles: Result<Vec<Role>, _> = self
            .roles
            .iter()
            .map(|r| Role::from_str_role(r).map_err(|e| e.to_string()))
            .collect();
        let roles = roles?;

        Ok(User::reconstitute(
            id,
            email,
            password_hash,
            roles,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IUserRepository for PgUserRepository {
    async fn save(&self, user: &User) -> Result<(), String> {
        let roles: Vec<String> = user
            .roles()
            .iter()
            .map(|r| r.as_str().to_string())
            .collect();

        sqlx::query(
            r#"
            INSERT INTO identity.users (id, email, password_hash, roles, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                roles = EXCLUDED.roles,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.email().as_str())
        .bind(user.password_hash().as_str())
        .bind(&roles)
        .bind(user.is_active())
        .bind(user.created_at())
        .bind(user.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save error: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, String> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, email, password_hash, roles, is_active, created_at, updated_at FROM identity.users WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>, String> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, email, password_hash, roles, is_active, created_at, updated_at FROM identity.users WHERE email = $1",
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_email error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn exists_by_email(&self, email: &EmailAddress) -> Result<bool, String> {
        let result: (bool,) =
            sqlx::query_as("SELECT EXISTS(SELECT 1 FROM identity.users WHERE email = $1)")
                .bind(email.as_str())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| format!("DB exists_by_email error: {e}"))?;

        Ok(result.0)
    }

    async fn delete(&self, id: &UserId) -> Result<(), String> {
        sqlx::query("DELETE FROM identity.users WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB delete error: {e}"))?;

        Ok(())
    }
}
