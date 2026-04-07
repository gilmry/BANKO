use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::compliance::{
    BiometricVerificationDto, DataPortabilityRequest, ErasureRequest, IBiometricRepository,
    IBreachNotificationRepository, IDataPortabilityRepository, IDpiaRepository,
    IErasureRepository, IInpdpConsentRepository, ISmsiRepository, ITokenVaultRepository,
    RiskEntry, SmsiControl, TokenVaultEntry,
};
use banko_domain::compliance::{
    BreachNotification, BreachNotificationId, BreachStatus, Dpia, DpiaId, DpiaStatus,
    InpdpConsent, InpdpConsentId, LegalBasis, ConsentPurpose,
};

// ============================================================
// SMSI Controls Repository
// ============================================================

pub struct PgSmsiRepository {
    pool: PgPool,
}

impl PgSmsiRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SmsiControlRow {
    id: Uuid,
    control_ref: String,
    title: String,
    theme: String,
    description: Option<String>,
    status: String,
    #[sqlx(rename = "responsible")]
    _responsible: Option<String>,
    evidence: Option<String>,
    #[sqlx(rename = "last_audit_date")]
    _last_audit_date: Option<DateTime<Utc>>,
    #[sqlx(rename = "next_audit_date")]
    _next_audit_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SmsiControlRow {
    fn into_domain(self) -> Result<SmsiControl, String> {
        Ok(SmsiControl {
            id: self.id,
            control_code: self.control_ref,
            name: self.title,
            description: self.description.unwrap_or_default(),
            theme: self.theme,
            status: self.status,
            evidence: self.evidence,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
struct RiskEntryRow {
    id: Uuid,
    risk_ref: String,
    #[sqlx(rename = "title")]
    _title: String,
    description: Option<String>,
    #[sqlx(rename = "category")]
    _category: String,
    #[sqlx(rename = "likelihood")]
    _likelihood: i16,
    impact: i16,
    inherent_score: i16,
    #[sqlx(rename = "residual_score")]
    _residual_score: i16,
    mitigations: Option<Vec<String>>,
    #[sqlx(rename = "owner")]
    _owner: Option<String>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RiskEntryRow {
    fn into_domain(self) -> Result<RiskEntry, String> {
        Ok(RiskEntry {
            id: self.id,
            identifier: self.risk_ref,
            description: self.description.unwrap_or_default(),
            score: (self.inherent_score * self.impact) as i32,
            status: self.status,
            mitigations: self.mitigations.map(|m| m.join(", ")),
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

#[async_trait]
impl ISmsiRepository for PgSmsiRepository {
    async fn save_control(&self, control: &SmsiControl) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO compliance.smsi_controls (id, control_ref, title, theme, description, status, responsible, evidence, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                evidence = EXCLUDED.evidence,
                responsible = EXCLUDED.responsible,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(control.id)
        .bind(&control.control_code)
        .bind(&control.name)
        .bind(&control.theme)
        .bind(&control.description)
        .bind(&control.status)
        .bind("")
        .bind(&control.evidence)
        .bind(control.created_at)
        .bind(control.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save control error: {e}"))?;

        Ok(())
    }

    async fn find_control_by_id(&self, id: Uuid) -> Result<Option<SmsiControl>, String> {
        let row: Option<SmsiControlRow> = sqlx::query_as(
            "SELECT id, control_ref, title, theme, description, status, responsible, evidence, last_audit_date, next_audit_date, created_at, updated_at FROM compliance.smsi_controls WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_control_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_controls_by_theme(&self, theme: &str) -> Result<Vec<SmsiControl>, String> {
        let rows: Vec<SmsiControlRow> = sqlx::query_as(
            "SELECT id, control_ref, title, theme, description, status, responsible, evidence, last_audit_date, next_audit_date, created_at, updated_at FROM compliance.smsi_controls WHERE theme = $1 ORDER BY control_ref ASC",
        )
        .bind(theme)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_controls_by_theme error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn list_all_controls(&self) -> Result<Vec<SmsiControl>, String> {
        let rows: Vec<SmsiControlRow> = sqlx::query_as(
            "SELECT id, control_ref, title, theme, description, status, responsible, evidence, last_audit_date, next_audit_date, created_at, updated_at FROM compliance.smsi_controls ORDER BY control_ref ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_all_controls error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn count_by_status(&self, status: &str) -> Result<i64, String> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM compliance.smsi_controls WHERE status = $1"
        )
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB count_by_status error: {e}"))?;

        Ok(result.0)
    }

    async fn save_risk(&self, risk: &RiskEntry) -> Result<(), String> {
        let mitigations: Option<Vec<String>> = risk.mitigations.as_ref().map(|m| m.split(',').map(|s| s.trim().to_string()).collect());

        sqlx::query(
            r#"
            INSERT INTO compliance.risk_entries (id, risk_ref, title, description, category, likelihood, impact, inherent_score, residual_score, mitigations, owner, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                residual_score = EXCLUDED.residual_score,
                mitigations = EXCLUDED.mitigations,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(risk.id)
        .bind(&risk.identifier)
        .bind("")
        .bind(&risk.description)
        .bind("Operational")
        .bind(1i16)
        .bind(1i16)
        .bind((risk.score / 5) as i16)
        .bind((risk.score / 5) as i16)
        .bind(&mitigations)
        .bind("")
        .bind(&risk.status)
        .bind(risk.created_at)
        .bind(risk.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_risk error: {e}"))?;

        Ok(())
    }

    async fn find_risk_by_id(&self, id: Uuid) -> Result<Option<RiskEntry>, String> {
        let row: Option<RiskEntryRow> = sqlx::query_as(
            "SELECT id, risk_ref, title, description, category, likelihood, impact, inherent_score, residual_score, mitigations, owner, status, created_at, updated_at FROM compliance.risk_entries WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_risk_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn list_all_risks(&self) -> Result<Vec<RiskEntry>, String> {
        let rows: Vec<RiskEntryRow> = sqlx::query_as(
            "SELECT id, risk_ref, title, description, category, likelihood, impact, inherent_score, residual_score, mitigations, owner, status, created_at, updated_at FROM compliance.risk_entries ORDER BY inherent_score DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_all_risks error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn list_high_risks(&self) -> Result<Vec<RiskEntry>, String> {
        let rows: Vec<RiskEntryRow> = sqlx::query_as(
            "SELECT id, risk_ref, title, description, category, likelihood, impact, inherent_score, residual_score, mitigations, owner, status, created_at, updated_at FROM compliance.risk_entries WHERE inherent_score >= 15 ORDER BY inherent_score DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_high_risks error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }
}

// ============================================================
// Token Vault Repository (PCI DSS)
// ============================================================

pub struct PgTokenVaultRepository {
    pool: PgPool,
}

impl PgTokenVaultRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct TokenVaultRow {
    id: Uuid,
    token: String,
    masked_pan: String,
    #[sqlx(rename = "card_holder_encrypted")]
    _card_holder_encrypted: Option<Vec<u8>>,
    #[sqlx(rename = "expiry_month")]
    _expiry_month: i16,
    #[sqlx(rename = "expiry_year")]
    _expiry_year: i16,
    token_status: String,
    #[sqlx(rename = "encryption_key_id")]
    _encryption_key_id: String,
    created_at: DateTime<Utc>,
    #[sqlx(rename = "expires_at")]
    _expires_at: Option<DateTime<Utc>>,
}

impl TokenVaultRow {
    fn into_domain(self) -> Result<TokenVaultEntry, String> {
        Ok(TokenVaultEntry {
            id: self.id,
            token: self.token,
            masked_pan: self.masked_pan,
            is_active: self.token_status == "Active",
            created_at: self.created_at,
            revoked_at: None,
        })
    }
}

#[async_trait]
impl ITokenVaultRepository for PgTokenVaultRepository {
    async fn save_token(&self, entry: &TokenVaultEntry) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO compliance.token_vault (id, token, masked_pan, expiry_month, expiry_year, token_status, encryption_key_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (token) DO UPDATE SET
                token_status = EXCLUDED.token_status
            "#,
        )
        .bind(entry.id)
        .bind(&entry.token)
        .bind(&entry.masked_pan)
        .bind(12i16)
        .bind(2026i16)
        .bind(if entry.is_active { "Active" } else { "Revoked" })
        .bind("default-key")
        .bind(entry.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_token error: {e}"))?;

        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<TokenVaultEntry>, String> {
        let row: Option<TokenVaultRow> = sqlx::query_as(
            "SELECT id, token, masked_pan, card_holder_encrypted, expiry_month, expiry_year, token_status, encryption_key_id, created_at, expires_at FROM compliance.token_vault WHERE token = $1",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_token error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn revoke_token(&self, token_id: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE compliance.token_vault SET token_status = $1 WHERE id = $2"
        )
        .bind("Revoked")
        .bind(token_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB revoke_token error: {e}"))?;

        Ok(())
    }

    async fn list_active_tokens(&self) -> Result<Vec<TokenVaultEntry>, String> {
        let rows: Vec<TokenVaultRow> = sqlx::query_as(
            "SELECT id, token, masked_pan, card_holder_encrypted, expiry_month, expiry_year, token_status, encryption_key_id, created_at, expires_at FROM compliance.token_vault WHERE token_status = $1 ORDER BY created_at DESC",
        )
        .bind("Active")
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_active_tokens error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn count_tokens(&self) -> Result<i64, String> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM compliance.token_vault"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB count_tokens error: {e}"))?;

        Ok(result.0)
    }
}

// ============================================================
// INPDP Consent Repository
// ============================================================

pub struct PgInpdpConsentRepository {
    pool: PgPool,
}

impl PgInpdpConsentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct InpdpConsentRow {
    id: Uuid,
    customer_id: Uuid,
    purpose: String,
    granted: bool,
    granted_at: Option<DateTime<Utc>>,
    revoked_at: Option<DateTime<Utc>>,
    expiry_date: Option<DateTime<Utc>>,
    legal_basis: String,
    data_categories: Option<Vec<String>>,
    #[sqlx(rename = "created_at")]
    _created_at: DateTime<Utc>,
}

impl InpdpConsentRow {
    fn into_domain(self) -> Result<InpdpConsent, String> {
        let consent_id = InpdpConsentId::from_uuid(self.id);
        let purpose = ConsentPurpose::from_str(&self.purpose)
            .map_err(|e| format!("Invalid purpose: {e}"))?;
        let legal_basis = LegalBasis::from_str(&self.legal_basis)
            .map_err(|e| format!("Invalid legal basis: {e}"))?;

        let consent = InpdpConsent::reconstitute(
            consent_id,
            self.customer_id,
            purpose,
            self.granted,
            self.granted_at,
            self.revoked_at,
            self.expiry_date,
            legal_basis,
            self.data_categories.unwrap_or_default(),
        );
        Ok(consent)
    }
}

#[async_trait]
impl IInpdpConsentRepository for PgInpdpConsentRepository {
    async fn save_consent(&self, consent: &InpdpConsent) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO compliance.inpdp_consents (id, customer_id, purpose, granted, granted_at, revoked_at, expiry_date, legal_basis, data_categories, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                granted = EXCLUDED.granted,
                granted_at = EXCLUDED.granted_at,
                revoked_at = EXCLUDED.revoked_at,
                expiry_date = EXCLUDED.expiry_date
            "#,
        )
        .bind(consent.id().as_uuid())
        .bind(consent.customer_id())
        .bind(consent.purpose().as_str())
        .bind(consent.is_valid())
        .bind(consent.granted_at())
        .bind(consent.revoked_at())
        .bind(consent.expiry_date())
        .bind(consent.legal_basis().as_str())
        .bind(consent.data_categories().to_vec())
        .bind(consent.granted_at().unwrap_or_else(Utc::now))
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_consent error: {e}"))?;

        Ok(())
    }

    async fn find_consent_by_id(
        &self,
        id: &InpdpConsentId,
    ) -> Result<Option<InpdpConsent>, String> {
        let row: Option<InpdpConsentRow> = sqlx::query_as(
            "SELECT id, customer_id, purpose, granted, granted_at, revoked_at, expiry_date, legal_basis, data_categories, created_at FROM compliance.inpdp_consents WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_consent_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_consents_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<InpdpConsent>, String> {
        let rows: Vec<InpdpConsentRow> = sqlx::query_as(
            "SELECT id, customer_id, purpose, granted, granted_at, revoked_at, expiry_date, legal_basis, data_categories, created_at FROM compliance.inpdp_consents WHERE customer_id = $1 ORDER BY created_at DESC",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_consents_by_customer error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_active_consents_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<InpdpConsent>, String> {
        let rows: Vec<InpdpConsentRow> = sqlx::query_as(
            "SELECT id, customer_id, purpose, granted, granted_at, revoked_at, expiry_date, legal_basis, data_categories, created_at FROM compliance.inpdp_consents WHERE customer_id = $1 AND granted = true AND revoked_at IS NULL ORDER BY created_at DESC",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_active_consents_by_customer error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn count_by_purpose(
        &self,
        customer_id: Uuid,
        purpose: &str,
    ) -> Result<i64, String> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM compliance.inpdp_consents WHERE customer_id = $1 AND purpose = $2"
        )
        .bind(customer_id)
        .bind(purpose)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB count_by_purpose error: {e}"))?;

        Ok(result.0)
    }
}

// ============================================================
// DPIA (Data Protection Impact Assessment) Repository
// ============================================================

pub struct PgDpiaRepository {
    pool: PgPool,
}

impl PgDpiaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct DpiaRow {
    id: Uuid,
    title: String,
    description: Option<String>,
    processing_activity: String,
    risk_assessment: Option<String>,
    mitigations: Option<Vec<String>>,
    status: String,
    created_at: DateTime<Utc>,
    approved_by: Option<String>,
    approved_at: Option<DateTime<Utc>>,
}

impl DpiaRow {
    fn into_domain(self) -> Result<Dpia, String> {
        let dpia_id = DpiaId::from_uuid(self.id);
        let status = DpiaStatus::from_str(&self.status)
            .map_err(|e| format!("Invalid DPIA status: {e}"))?;

        let dpia = Dpia::reconstitute(
            dpia_id,
            self.title,
            self.description.unwrap_or_default(),
            self.processing_activity,
            self.risk_assessment.unwrap_or_default(),
            self.mitigations.unwrap_or_default(),
            status,
            self.created_at,
            self.approved_by,
            self.approved_at,
        );
        Ok(dpia)
    }
}

#[async_trait]
impl IDpiaRepository for PgDpiaRepository {
    async fn save_dpia(&self, dpia: &Dpia) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO compliance.dpias (id, title, description, processing_activity, risk_assessment, mitigations, status, created_at, approved_by, approved_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                risk_assessment = EXCLUDED.risk_assessment,
                mitigations = EXCLUDED.mitigations,
                approved_by = EXCLUDED.approved_by,
                approved_at = EXCLUDED.approved_at
            "#,
        )
        .bind(dpia.id().as_uuid())
        .bind(dpia.title())
        .bind(dpia.description())
        .bind(dpia.processing_activity())
        .bind(dpia.risk_assessment())
        .bind(dpia.mitigations().to_vec())
        .bind(dpia.status().as_str())
        .bind(dpia.created_at())
        .bind(dpia.approved_by())
        .bind(dpia.approved_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_dpia error: {e}"))?;

        Ok(())
    }

    async fn find_dpia_by_id(&self, id: &DpiaId) -> Result<Option<Dpia>, String> {
        let row: Option<DpiaRow> = sqlx::query_as(
            "SELECT id, title, description, processing_activity, risk_assessment, mitigations, status, created_at, approved_by, approved_at FROM compliance.dpias WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_dpia_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn list_all_dpias(&self) -> Result<Vec<Dpia>, String> {
        let rows: Vec<DpiaRow> = sqlx::query_as(
            "SELECT id, title, description, processing_activity, risk_assessment, mitigations, status, created_at, approved_by, approved_at FROM compliance.dpias ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_all_dpias error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn list_by_status(&self, status: &str) -> Result<Vec<Dpia>, String> {
        let rows: Vec<DpiaRow> = sqlx::query_as(
            "SELECT id, title, description, processing_activity, risk_assessment, mitigations, status, created_at, approved_by, approved_at FROM compliance.dpias WHERE status = $1 ORDER BY created_at DESC",
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_by_status error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn count_by_status(&self, status: &str) -> Result<i64, String> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM compliance.dpias WHERE status = $1"
        )
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB count_by_status error: {e}"))?;

        Ok(result.0)
    }
}

// ============================================================
// Breach Notification Repository
// ============================================================

pub struct PgBreachNotificationRepository {
    pool: PgPool,
}

impl PgBreachNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BreachNotificationRow {
    id: Uuid,
    breach_type: String,
    description: String,
    affected_data: Option<Vec<String>>,
    affected_count: i32,
    detected_at: DateTime<Utc>,
    notified_authority_at: Option<DateTime<Utc>>,
    notified_subjects_at: Option<DateTime<Utc>>,
    status: String,
    #[sqlx(rename = "created_at")]
    _created_at: DateTime<Utc>,
}

impl BreachNotificationRow {
    fn into_domain(self) -> Result<BreachNotification, String> {
        let breach_id = BreachNotificationId::from_uuid(self.id);
        let status = BreachStatus::from_str(&self.status)
            .map_err(|e| format!("Invalid breach status: {e}"))?;

        let breach = BreachNotification::reconstitute(
            breach_id,
            self.breach_type,
            self.description,
            self.affected_data.unwrap_or_default(),
            self.affected_count as u32,
            self.detected_at,
            self.notified_authority_at,
            self.notified_subjects_at,
            status,
        );
        Ok(breach)
    }
}

#[async_trait]
impl IBreachNotificationRepository for PgBreachNotificationRepository {
    async fn save_breach(&self, breach: &BreachNotification) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO compliance.breach_notifications (id, breach_type, description, affected_data, affected_count, detected_at, notified_authority_at, notified_subjects_at, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                notified_authority_at = EXCLUDED.notified_authority_at,
                notified_subjects_at = EXCLUDED.notified_subjects_at
            "#,
        )
        .bind(breach.id().as_uuid())
        .bind(breach.breach_type())
        .bind(breach.description())
        .bind(breach.affected_data().to_vec())
        .bind(breach.affected_count() as i32)
        .bind(breach.detected_at())
        .bind(breach.notified_authority_at())
        .bind(breach.notified_subjects_at())
        .bind(breach.status().as_str())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_breach error: {e}"))?;

        Ok(())
    }

    async fn find_breach_by_id(
        &self,
        id: &BreachNotificationId,
    ) -> Result<Option<BreachNotification>, String> {
        let row: Option<BreachNotificationRow> = sqlx::query_as(
            "SELECT id, breach_type, description, affected_data, affected_count, detected_at, notified_authority_at, notified_subjects_at, status, created_at FROM compliance.breach_notifications WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_breach_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn list_all_breaches(&self) -> Result<Vec<BreachNotification>, String> {
        let rows: Vec<BreachNotificationRow> = sqlx::query_as(
            "SELECT id, breach_type, description, affected_data, affected_count, detected_at, notified_authority_at, notified_subjects_at, status, created_at FROM compliance.breach_notifications ORDER BY detected_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_all_breaches error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn list_by_status(&self, status: &str) -> Result<Vec<BreachNotification>, String> {
        let rows: Vec<BreachNotificationRow> = sqlx::query_as(
            "SELECT id, breach_type, description, affected_data, affected_count, detected_at, notified_authority_at, notified_subjects_at, status, created_at FROM compliance.breach_notifications WHERE status = $1 ORDER BY detected_at DESC",
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_by_status error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn count_pending_authority_notification(&self) -> Result<i64, String> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM compliance.breach_notifications WHERE notified_authority_at IS NULL AND detected_at < NOW() - INTERVAL '72 hours'"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB count_pending_authority_notification error: {e}"))?;

        Ok(result.0)
    }
}

// ============================================================
// Data Portability Repository
// ============================================================

pub struct PgDataPortabilityRepository {
    pool: PgPool,
}

impl PgDataPortabilityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct DataPortabilityRequestRow {
    id: Uuid,
    customer_id: Uuid,
    #[sqlx(rename = "request_type")]
    _request_type: String,
    status: String,
    reason: Option<String>,
    requested_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

impl DataPortabilityRequestRow {
    fn into_domain(self) -> Result<DataPortabilityRequest, String> {
        Ok(DataPortabilityRequest {
            id: self.id,
            customer_id: self.customer_id,
            status: self.status,
            reason: self.reason,
            requested_at: self.requested_at,
            completed_at: self.completed_at,
        })
    }
}

#[async_trait]
impl IDataPortabilityRepository for PgDataPortabilityRepository {
    async fn save_request(&self, request: &DataPortabilityRequest) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO compliance.data_rights_requests (id, customer_id, request_type, status, reason, requested_at, completed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                completed_at = EXCLUDED.completed_at
            "#,
        )
        .bind(request.id)
        .bind(request.customer_id)
        .bind("Portability")
        .bind(&request.status)
        .bind(&request.reason)
        .bind(request.requested_at)
        .bind(request.completed_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_request (portability) error: {e}"))?;

        Ok(())
    }

    async fn find_request_by_id(&self, id: Uuid) -> Result<Option<DataPortabilityRequest>, String> {
        let row: Option<DataPortabilityRequestRow> = sqlx::query_as(
            "SELECT id, customer_id, request_type, status, reason, requested_at, completed_at FROM compliance.data_rights_requests WHERE id = $1 AND request_type = $2",
        )
        .bind(id)
        .bind("Portability")
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_request_by_id (portability) error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<DataPortabilityRequest>, String> {
        let rows: Vec<DataPortabilityRequestRow> = sqlx::query_as(
            "SELECT id, customer_id, request_type, status, reason, requested_at, completed_at FROM compliance.data_rights_requests WHERE customer_id = $1 AND request_type = $2 ORDER BY requested_at DESC",
        )
        .bind(customer_id)
        .bind("Portability")
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_customer (portability) error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }
}

// ============================================================
// Erasure Request Repository
// ============================================================

pub struct PgErasureRepository {
    pool: PgPool,
}

impl PgErasureRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ErasureRequestRow {
    id: Uuid,
    customer_id: Uuid,
    #[sqlx(rename = "request_type")]
    _request_type: String,
    status: String,
    reason: Option<String>,
    requested_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

impl ErasureRequestRow {
    fn into_domain(self) -> Result<ErasureRequest, String> {
        Ok(ErasureRequest {
            id: self.id,
            customer_id: self.customer_id,
            status: self.status,
            reason: self.reason,
            requested_at: self.requested_at,
            scheduled_for: self.completed_at,
        })
    }
}

#[async_trait]
impl IErasureRepository for PgErasureRepository {
    async fn save_request(&self, request: &ErasureRequest) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO compliance.data_rights_requests (id, customer_id, request_type, status, reason, requested_at, completed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                completed_at = EXCLUDED.completed_at
            "#,
        )
        .bind(request.id)
        .bind(request.customer_id)
        .bind("Erasure")
        .bind(&request.status)
        .bind(&request.reason)
        .bind(request.requested_at)
        .bind(request.scheduled_for)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_request (erasure) error: {e}"))?;

        Ok(())
    }

    async fn find_request_by_id(&self, id: Uuid) -> Result<Option<ErasureRequest>, String> {
        let row: Option<ErasureRequestRow> = sqlx::query_as(
            "SELECT id, customer_id, request_type, status, reason, requested_at, completed_at FROM compliance.data_rights_requests WHERE id = $1 AND request_type = $2",
        )
        .bind(id)
        .bind("Erasure")
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_request_by_id (erasure) error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<ErasureRequest>, String> {
        let rows: Vec<ErasureRequestRow> = sqlx::query_as(
            "SELECT id, customer_id, request_type, status, reason, requested_at, completed_at FROM compliance.data_rights_requests WHERE customer_id = $1 AND request_type = $2 ORDER BY requested_at DESC",
        )
        .bind(customer_id)
        .bind("Erasure")
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_customer (erasure) error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }
}

// ============================================================
// Biometric Verification Repository (e-KYC)
// ============================================================

pub struct PgBiometricRepository {
    pool: PgPool,
}

impl PgBiometricRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BiometricVerificationRow {
    id: Uuid,
    customer_id: Uuid,
    verification_type: String,
    status: String,
    confidence_score: f64,
    liveness_check: bool,
    document_type: Option<String>,
    document_number: Option<String>,
    verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl BiometricVerificationRow {
    fn into_domain(self) -> Result<BiometricVerificationDto, String> {
        Ok(BiometricVerificationDto {
            id: self.id,
            customer_id: self.customer_id,
            verification_type: self.verification_type,
            status: self.status,
            confidence_score: self.confidence_score,
            liveness_check: self.liveness_check,
            document_type: self.document_type,
            document_number: self.document_number,
            verified_at: self.verified_at,
            created_at: self.created_at,
            expires_at: self.expires_at,
        })
    }
}

#[async_trait]
impl IBiometricRepository for PgBiometricRepository {
    async fn save_verification(&self, verification: &BiometricVerificationDto) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO compliance.biometric_verifications (id, customer_id, verification_type, status, confidence_score, liveness_check, document_type, document_number, verified_at, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                confidence_score = EXCLUDED.confidence_score,
                verified_at = EXCLUDED.verified_at
            "#,
        )
        .bind(verification.id)
        .bind(verification.customer_id)
        .bind(&verification.verification_type)
        .bind(&verification.status)
        .bind(verification.confidence_score)
        .bind(verification.liveness_check)
        .bind(&verification.document_type)
        .bind(&verification.document_number)
        .bind(verification.verified_at)
        .bind(verification.created_at)
        .bind(verification.expires_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_verification error: {e}"))?;

        Ok(())
    }

    async fn find_verification_by_id(&self, id: Uuid) -> Result<Option<BiometricVerificationDto>, String> {
        let row: Option<BiometricVerificationRow> = sqlx::query_as(
            "SELECT id, customer_id, verification_type, status, confidence_score, liveness_check, document_type, document_number, verified_at, created_at, expires_at FROM compliance.biometric_verifications WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_verification_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_latest_by_customer_and_type(
        &self,
        customer_id: Uuid,
        verification_type: &str,
    ) -> Result<Option<BiometricVerificationDto>, String> {
        let row: Option<BiometricVerificationRow> = sqlx::query_as(
            "SELECT id, customer_id, verification_type, status, confidence_score, liveness_check, document_type, document_number, verified_at, created_at, expires_at FROM compliance.biometric_verifications WHERE customer_id = $1 AND verification_type = $2 ORDER BY created_at DESC LIMIT 1",
        )
        .bind(customer_id)
        .bind(verification_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_latest_by_customer_and_type error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<BiometricVerificationDto>, String> {
        let rows: Vec<BiometricVerificationRow> = sqlx::query_as(
            "SELECT id, customer_id, verification_type, status, confidence_score, liveness_check, document_type, document_number, verified_at, created_at, expires_at FROM compliance.biometric_verifications WHERE customer_id = $1 ORDER BY created_at DESC",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_customer error: {e}"))?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_verified_by_customer(&self, customer_id: Uuid) -> Result<Option<BiometricVerificationDto>, String> {
        let row: Option<BiometricVerificationRow> = sqlx::query_as(
            "SELECT id, customer_id, verification_type, status, confidence_score, liveness_check, document_type, document_number, verified_at, created_at, expires_at FROM compliance.biometric_verifications WHERE customer_id = $1 AND status = $2 AND expires_at > NOW() ORDER BY created_at DESC LIMIT 1",
        )
        .bind(customer_id)
        .bind("verified")
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_verified_by_customer error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn count_by_status(&self, status: &str) -> Result<i64, String> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM compliance.biometric_verifications WHERE status = $1"
        )
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB count_by_status error: {e}"))?;

        Ok(result.0)
    }
}
