use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::governance::{AuditFilter, IAuditRepository, ICommitteeRepository, IControlCheckRepository};
use banko_domain::governance::*;

// ============================================================
// AuditRepository
// ============================================================

pub struct AuditRepository {
    pool: PgPool,
}

impl AuditRepository {
    pub fn new(pool: PgPool) -> Self {
        AuditRepository { pool }
    }
}

#[async_trait]
impl IAuditRepository for AuditRepository {
    async fn append(&self, entry: &AuditTrailEntry) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO governance.audit_trail (id, timestamp, user_id, action, resource_type, resource_id, changes, ip_address, previous_hash, hash)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(entry.entry_id().as_uuid())
        .bind(entry.timestamp())
        .bind(entry.user_id())
        .bind(entry.action().as_str())
        .bind(entry.resource_type().as_str())
        .bind(entry.resource_id())
        .bind(entry.changes().map(|s| serde_json::Value::String(s.to_string())))
        .bind(entry.ip_address())
        .bind(entry.previous_hash())
        .bind(entry.hash())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: &AuditEntryId) -> Result<Option<AuditTrailEntry>, String> {
        let row = sqlx::query_as::<_, AuditRow>(
            "SELECT id, timestamp, user_id, action, resource_type, resource_id, changes::text, ip_address, previous_hash, hash FROM governance.audit_trail WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_entity()?)),
            None => Ok(None),
        }
    }

    async fn find_latest(&self) -> Result<Option<AuditTrailEntry>, String> {
        let row = sqlx::query_as::<_, AuditRow>(
            "SELECT id, timestamp, user_id, action, resource_type, resource_id, changes::text, ip_address, previous_hash, hash FROM governance.audit_trail ORDER BY timestamp DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_entity()?)),
            None => Ok(None),
        }
    }

    async fn find_all(
        &self,
        filters: &AuditFilter,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditTrailEntry>, String> {
        let rows = sqlx::query_as::<_, AuditRow>(
            r#"
            SELECT id, timestamp, user_id, action, resource_type, resource_id, changes::text, ip_address, previous_hash, hash
            FROM governance.audit_trail
            WHERE ($1::uuid IS NULL OR user_id = $1)
              AND ($2::text IS NULL OR action = $2)
              AND ($3::text IS NULL OR resource_type = $3)
              AND ($4::uuid IS NULL OR resource_id = $4)
              AND ($5::timestamptz IS NULL OR timestamp >= $5)
              AND ($6::timestamptz IS NULL OR timestamp <= $6)
            ORDER BY timestamp DESC
            LIMIT $7 OFFSET $8
            "#,
        )
        .bind(filters.user_id)
        .bind(filters.action.as_deref())
        .bind(filters.resource_type.as_deref())
        .bind(filters.resource_id)
        .bind(filters.from)
        .bind(filters.to)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_entity()).collect()
    }

    async fn count_all(&self, filters: &AuditFilter) -> Result<i64, String> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM governance.audit_trail
            WHERE ($1::uuid IS NULL OR user_id = $1)
              AND ($2::text IS NULL OR action = $2)
              AND ($3::text IS NULL OR resource_type = $3)
              AND ($4::uuid IS NULL OR resource_id = $4)
              AND ($5::timestamptz IS NULL OR timestamp >= $5)
              AND ($6::timestamptz IS NULL OR timestamp <= $6)
            "#,
        )
        .bind(filters.user_id)
        .bind(filters.action.as_deref())
        .bind(filters.resource_type.as_deref())
        .bind(filters.resource_id)
        .bind(filters.from)
        .bind(filters.to)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row.0)
    }

    async fn find_chain(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<AuditTrailEntry>, String> {
        let rows = sqlx::query_as::<_, AuditRow>(
            r#"
            SELECT id, timestamp, user_id, action, resource_type, resource_id, changes::text, ip_address, previous_hash, hash
            FROM governance.audit_trail
            WHERE timestamp >= $1 AND timestamp <= $2
            ORDER BY timestamp ASC
            "#,
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_entity()).collect()
    }
}

#[derive(sqlx::FromRow)]
struct AuditRow {
    id: Uuid,
    timestamp: DateTime<Utc>,
    user_id: Uuid,
    action: String,
    resource_type: String,
    resource_id: Uuid,
    changes: Option<String>,
    ip_address: Option<String>,
    previous_hash: String,
    hash: String,
}

impl AuditRow {
    fn into_entity(self) -> Result<AuditTrailEntry, String> {
        let action = AuditAction::from_str_type(&self.action).map_err(|e| e.to_string())?;
        let resource_type =
            ResourceType::from_str_type(&self.resource_type).map_err(|e| e.to_string())?;

        Ok(AuditTrailEntry::from_raw(
            AuditEntryId::from_uuid(self.id),
            self.timestamp,
            self.user_id,
            action,
            resource_type,
            self.resource_id,
            self.changes,
            self.ip_address,
            self.previous_hash,
            self.hash,
        ))
    }
}

// ============================================================
// CommitteeRepository
// ============================================================

pub struct CommitteeRepository {
    pool: PgPool,
}

impl CommitteeRepository {
    pub fn new(pool: PgPool) -> Self {
        CommitteeRepository { pool }
    }
}

#[async_trait]
impl ICommitteeRepository for CommitteeRepository {
    async fn save_committee(&self, committee: &Committee) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO governance.committees (id, name, committee_type, members, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET name = $2, committee_type = $3, members = $4
            "#,
        )
        .bind(committee.id())
        .bind(committee.name())
        .bind(committee.committee_type().as_str())
        .bind(committee.members())
        .bind(committee.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_committee_by_id(&self, id: Uuid) -> Result<Option<Committee>, String> {
        let row = sqlx::query_as::<_, CommitteeRow>(
            "SELECT id, name, committee_type, members, created_at FROM governance.committees WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_entity()?)),
            None => Ok(None),
        }
    }

    async fn find_all_committees(&self) -> Result<Vec<Committee>, String> {
        let rows = sqlx::query_as::<_, CommitteeRow>(
            "SELECT id, name, committee_type, members, created_at FROM governance.committees ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_entity()).collect()
    }

    async fn save_decision(&self, decision: &CommitteeDecision) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO governance.committee_decisions (id, committee_id, subject, decision, justification, decided_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(decision.id())
        .bind(decision.committee_id())
        .bind(decision.subject())
        .bind(decision.decision().as_str())
        .bind(decision.justification())
        .bind(decision.decided_at())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        for vote in decision.votes() {
            sqlx::query(
                r#"
                INSERT INTO governance.decision_votes (decision_id, member_id, vote)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(decision.id())
            .bind(vote.member_id)
            .bind(vote.vote.as_str())
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_decisions_by_committee(
        &self,
        committee_id: Uuid,
    ) -> Result<Vec<CommitteeDecision>, String> {
        let rows = sqlx::query_as::<_, DecisionRow>(
            "SELECT id, committee_id, subject, decision, justification, decided_at FROM governance.committee_decisions WHERE committee_id = $1 ORDER BY decided_at DESC"
        )
        .bind(committee_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut decisions = Vec::new();
        for row in rows {
            let vote_rows = sqlx::query_as::<_, VoteRow>(
                "SELECT member_id, vote FROM governance.decision_votes WHERE decision_id = $1"
            )
            .bind(row.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let votes: Result<Vec<Vote>, String> = vote_rows
                .into_iter()
                .map(|v| {
                    let vc = VoteChoice::from_str_type(&v.vote).map_err(|e| e.to_string())?;
                    Ok(Vote {
                        member_id: v.member_id,
                        vote: vc,
                    })
                })
                .collect();

            let outcome =
                DecisionOutcome::from_str_type(&row.decision).map_err(|e| e.to_string())?;

            decisions.push(CommitteeDecision::from_raw(
                row.id,
                row.committee_id,
                row.subject,
                outcome,
                votes?,
                row.justification,
                row.decided_at,
            ));
        }

        Ok(decisions)
    }
}

#[derive(sqlx::FromRow)]
struct CommitteeRow {
    id: Uuid,
    name: String,
    committee_type: String,
    members: Vec<Uuid>,
    created_at: DateTime<Utc>,
}

impl CommitteeRow {
    fn into_entity(self) -> Result<Committee, String> {
        let ct =
            CommitteeType::from_str_type(&self.committee_type).map_err(|e| e.to_string())?;
        Ok(Committee::from_raw(
            self.id,
            self.name,
            ct,
            self.members,
            self.created_at,
        ))
    }
}

#[derive(sqlx::FromRow)]
struct DecisionRow {
    id: Uuid,
    committee_id: Uuid,
    subject: String,
    decision: String,
    justification: Option<String>,
    decided_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct VoteRow {
    member_id: Uuid,
    vote: String,
}

// ============================================================
// ControlCheckRepository
// ============================================================

pub struct ControlCheckRepository {
    pool: PgPool,
}

impl ControlCheckRepository {
    pub fn new(pool: PgPool) -> Self {
        ControlCheckRepository { pool }
    }
}

#[async_trait]
impl IControlCheckRepository for ControlCheckRepository {
    async fn save(&self, check: &ControlCheck) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO governance.control_checks (id, operation_type, operation_id, checker_id, status, comments, checked_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET checker_id = $4, status = $5, comments = $6, checked_at = $7
            "#,
        )
        .bind(check.id())
        .bind(check.operation_type())
        .bind(check.operation_id())
        .bind(check.checker_id())
        .bind(check.status().as_str())
        .bind(check.comments())
        .bind(check.checked_at())
        .bind(check.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ControlCheck>, String> {
        let row = sqlx::query_as::<_, ControlCheckRow>(
            "SELECT id, operation_type, operation_id, checker_id, status, comments, checked_at, created_at FROM governance.control_checks WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_entity()?)),
            None => Ok(None),
        }
    }

    async fn find_all(
        &self,
        status: Option<ControlStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ControlCheck>, String> {
        let status_str = status.map(|s| s.as_str().to_string());
        let rows = sqlx::query_as::<_, ControlCheckRow>(
            r#"
            SELECT id, operation_type, operation_id, checker_id, status, comments, checked_at, created_at
            FROM governance.control_checks
            WHERE ($1::text IS NULL OR status = $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(status_str.as_deref())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_entity()).collect()
    }

    async fn count_all(&self, status: Option<ControlStatus>) -> Result<i64, String> {
        let status_str = status.map(|s| s.as_str().to_string());
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM governance.control_checks
            WHERE ($1::text IS NULL OR status = $1)
            "#,
        )
        .bind(status_str.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row.0)
    }
}

#[derive(sqlx::FromRow)]
struct ControlCheckRow {
    id: Uuid,
    operation_type: String,
    operation_id: Uuid,
    checker_id: Option<Uuid>,
    status: String,
    comments: Option<String>,
    checked_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl ControlCheckRow {
    fn into_entity(self) -> Result<ControlCheck, String> {
        let status = ControlStatus::from_str_type(&self.status).map_err(|e| e.to_string())?;
        Ok(ControlCheck::from_raw(
            self.id,
            self.operation_type,
            self.operation_id,
            self.checker_id,
            status,
            self.comments,
            self.checked_at,
            self.created_at,
        ))
    }
}
