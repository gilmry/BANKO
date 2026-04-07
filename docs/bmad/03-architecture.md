# Architecture Technique — BANKO v4.0

## Méthode Maury — Phase TOGAF C-D (SI + Technique)

**Disciplines** : SOLID + DDD + Hexagonal + BDD + TDD
**Version** : 4.0.1 — 7 avril 2026 (itération post-validation Phase F)
**Stack** : Rust + Actix-web 4.9 + PostgreSQL 16 + Astro 6 + Svelte 5
**Bounded Contexts** : 22 (13 v3.0 + 9 nouveaux v4.0) — MVP = 13 BCs P0
**API Target** : v4.0 MVP ~350 endpoints (50%), v4.1 ~450 (70%), v4.2 ~550+ (85%+)
**Compliance** : BCT, CTAF, INPDP, ANCS, BVMT, ISO 27001:2022, PCI DSS v4.0.1, GAFI R.16

---

## 1. Vue d'ensemble — Architecture 3-Tier Hexagonale

```
┌──────────────────────────────────────────────────────────────────────────┐
│                   BANKO v4.0 — Core Banking System                       │
│                      22 Bounded Contexts (v4.0)                          │
└──────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────┐         ┌──────────────────────────────────┐
│  Frontend (Astro + Svelte)  │         │  Mobile (Svelte Native — future) │
│  ├─ Pages AR/FR/EN (i18n)   │         │  ├─ e-Banking mobile             │
│  ├─ Svelte stores           │         │  └─ Agent portal mobile          │
│  └─ API Client (fetch)      │         └──────────────────────────────────┘
└──────────────────┬──────────┘
                   ↕ HTTP/REST
        ┌──────────┴──────────────────────────────────┐
        │    API Gateway (Traefik)                    │
        │  ├─ Rate limiting, JWT validation, CORS    │
        │  ├─ TLS 1.3, HSTS, CSP                    │
        │  └─ Geo-blocking, IP whitelisting          │
        └──────────────────┬──────────────────────────┘
                           ↕
┌──────────────────────────────────────────────────────────────────────────┐
│         BACKEND — Rust + Actix-web 4.9 (Hexagonal Architecture)         │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │              LAYER 1: HTTP Handlers (Adapters)                  │  │
│  │  22 Module Routes (REST API: 550-700+ endpoints)               │  │
│  │                                                                 │  │
│  │  ┌────────────┬────────────┬────────────┬────────────┐        │  │
│  │  │Customer    │Account     │Credit      │AML         │        │  │
│  │  │(BC1)       │(BC2)       │(BC3)       │(BC4)       │        │  │
│  │  ├────────────┼────────────┼────────────┼────────────┤        │  │
│  │  │Sanctions   │Prudential  │Accounting  │Reporting   │        │  │
│  │  │(BC5)       │(BC6)       │(BC7)       │(BC8)       │        │  │
│  │  ├────────────┼────────────┼────────────┼────────────┤        │  │
│  │  │Payment     │ForeignEx   │Governance  │Identity    │        │  │
│  │  │(BC9)       │(BC10)      │(BC11)      │(BC12)      │        │  │
│  │  ├────────────┼────────────┼────────────┼────────────┤        │  │
│  │  │Arrangement │Collateral  │TradeFinance│CashMgmt    │        │  │
│  │  │(BC13)      │(BC14)      │(BC15)      │(BC16)      │        │  │
│  │  ├────────────┼────────────┼────────────┼────────────┤        │  │
│  │  │IslamicBank │DataHub     │ReferenceData│Securities  │        │  │
│  │  │(BC17)      │(BC18)      │(BC19)      │(BC20)      │        │  │
│  │  ├────────────┴────────────┴────────────┴────────────┤        │  │
│  │  │Insurance (BC21) + Compliance (BC22 — cross-cutting)│        │  │
│  │  └────────────────────────────────────────────────────┘        │  │
│  └────────────────────────┬─────────────────────────────────────┘  │
│                           ↓                                        │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │    LAYER 2: Application (DTOs, UseCases, Ports/Traits)          │  │
│  │                                                                 │  │
│  │  ├─ DTOs (serde: JSON ↔ Rust)                                 │  │
│  │  ├─ UseCases (orchestration logique métier)                   │  │
│  │  ├─ Ports (traits): Repository, EventBus, Notification        │  │
│  │  └─ Events (DomainEvent enum — audit trail)                  │  │
│  └────────────────────────┬──────────────────────────────────────┘  │
│                           ↓                                        │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │    LAYER 3: Domain (Pure Rust — Invariants, Rules)             │  │
│  │                                                                 │  │
│  │  ├─ Entities (Customer, Account, Loan, Arrangement...)         │  │
│  │  ├─ Value Objects (Money, IBAN, KycProfile, ExchangeRate)      │  │
│  │  ├─ Aggregates (with Roots)                                    │  │
│  │  ├─ Invariants (25 règles domaine compilées)                   │  │
│  │  └─ Domain Services (LoanClassification, PrudentialCalc...)    │  │
│  └────────────────────────┬──────────────────────────────────────┘  │
│                           ↓                                        │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │    LAYER 4: Infrastructure (Adapters)                           │  │
│  │                                                                 │  │
│  │  ├─ PostgreSQL Repositories (SQLx async)                       │  │
│  │  ├─ Redis Cache (sessions, OTP, rates)                         │  │
│  │  ├─ Event Store (immutable audit trail)                        │  │
│  │  ├─ HSM Interface (PKCS#11 signatures)                         │  │
│  │  ├─ External APIs (CTAF/goAML, SWIFT, BVMT)                   │  │
│  │  └─ Email/SMS Notifications                                    │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│              PostgreSQL 16 (Primary + Read Replicas)                     │
│  ├─ 22 schemas (customer, account, credit, aml, sanctions...)           │
│  ├─ Partitioning: By date (EOD) + customer + account type              │
│  ├─ Encryption: LUKS AES-XTS-512 + field-level AES-256-GCM            │
│  ├─ Replication: logical (audit trail → analytics DB)                  │
│  └─ Backup: WAL archiving, S3 off-site GPG encrypted                   │
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│                    Compliance Layer (Cross-cutting)                      │
│  ├─ SMSI ISO 27001:2022 (93 controls, Annexe A)                        │
│  ├─ PCI DSS v4.0.1 (tokenization, MFA, encryption)                    │
│  ├─ Loi données 2025 (DPO, DPIA, consent, breach notif.)              │
│  ├─ goAML Integration (CTAF declarations)                              │
│  └─ Travel Rule GAFI R.16 (originator/beneficiary)                    │
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│              Infrastructure as Code + Monitoring                         │
│  ├─ Docker Compose (dev), Kubernetes (prod)                            │
│  ├─ Prometheus metrics + Grafana dashboards                            │
│  ├─ Loki logs aggregation                                              │
│  ├─ Alertmanager (threshold + on-call integration)                     │
│  └─ Terraform + Ansible (IaC, secret management HSM)                   │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Les 22 Bounded Contexts (v4.0)

### 2.1 Contextes Core Existants (13 v3.0) — Enrichis v4.0

#### BC1 — Customer (Gestion des clients)

**Responsabilité** : Onboarding clients, KYC/CDD/EDD, bénéficiaires effectifs, PEP, risk scoring, e-KYC biométrique, conformité loi données 2025.

**Entités Rust** :
```rust
pub struct Customer {
    pub id: CustomerId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: PhoneNumber,
    pub kyc_status: KycStatus, // PENDING, VALIDATED, REJECTED
    pub pep_status: PepStatus,  // NOT_PEP, PEP_CITIZEN, PEP_FOREIGN
    pub risk_score: u8,         // 0-100 (RED=60+, ORANGE=30-59, GREEN=0-29)
    pub created_at: DateTime<Utc>,
    pub consent_personal_data: bool,
    pub consent_marketing: bool,
    pub biometric_enrollment: Option<BiometricData>,
}

pub struct KycProfile {
    pub customer_id: CustomerId,
    pub id_type: IdType,         // CIN, PASSPORT, RESIDENT_CARD
    pub id_number: String,
    pub id_issue_date: NaiveDate,
    pub id_expiry_date: NaiveDate,
    pub address: PostalAddress,
    pub profession: String,
    pub sector: BusinessSector,
    pub annual_income: Money,
    pub source_of_funds: Vec<String>,
    pub beneficial_owners: Vec<BeneficiaryInfo>,
    pub validated_at: DateTime<Utc>,
    pub validated_by: UserId,
}

pub struct BeneficiaryInfo {
    pub id: BeneficiaryId,
    pub customer_id: CustomerId,
    pub name: String,
    pub ownership_percentage: Decimal,
    pub kyc_profile: KycProfile,
}

pub enum KycStatus {
    Pending,
    Validated { validated_at: DateTime<Utc> },
    Rejected { reason: String },
}

pub enum PepStatus {
    NotPep,
    PepCitizen { list_source: String },
    PepForeign { country: String, list_source: String },
}
```

**Ports (Traits)** :
```rust
#[async_trait]
pub trait CustomerRepository {
    async fn create(&self, customer: &Customer) -> Result<(), DomainError>;
    async fn get(&self, id: &CustomerId) -> Result<Option<Customer>, DomainError>;
    async fn update(&self, customer: &Customer) -> Result<(), DomainError>;
    async fn list_by_risk_score(&self, min: u8, max: u8) -> Result<Vec<Customer>, DomainError>;
}

#[async_trait]
pub trait KycValidator {
    async fn validate_kyc(&self, profile: &KycProfile) -> Result<(), ValidationError>;
    async fn check_pep(&self, customer: &Customer) -> Result<PepStatus, DomainError>;
    async fn enroll_biometric(&self, customer_id: &CustomerId, data: BiometricData) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ConsentManager {
    async fn grant_consent(&self, customer_id: &CustomerId, scope: ConsentScope) -> Result<(), DomainError>;
    async fn revoke_consent(&self, customer_id: &CustomerId, scope: ConsentScope) -> Result<(), DomainError>;
    async fn get_consent_status(&self, customer_id: &CustomerId) -> Result<ConsentRecord, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/customers` — Créer client + KYC
- `GET /api/v1/customers/{id}` — Récupérer profil
- `PUT /api/v1/customers/{id}` — Modifier données
- `POST /api/v1/customers/{id}/kyc/validate` — Valider KYC
- `POST /api/v1/customers/{id}/pep-check` — Vérification PEP
- `POST /api/v1/customers/{id}/edd` — Enhanced Due Diligence
- `POST /api/v1/customers/{id}/biometric-enroll` — e-KYC biométrique
- `GET /api/v1/customers/{id}/consent` — Consulter consentements
- `POST /api/v1/customers/{id}/consent` — Accorder consentement
- `DELETE /api/v1/customers/{id}/consent/{scope}` — Révoquer consentement
- `POST /api/v1/customers/{id}/data-export` — Portabilité données
- `DELETE /api/v1/customers/{id}/data-erase` — Droit à l'oubli

---

#### BC2 — Account (Gestion des comptes)

**Responsabilité** : Comptes (courant, épargne, DAT), soldes, mouvements, lien vers Arrangement.

**Entités Rust** :
```rust
pub struct Account {
    pub id: AccountId,
    pub customer_id: CustomerId,
    pub iban: Iban,
    pub account_type: AccountType,
    pub currency: CurrencyCode,
    pub status: AccountStatus, // OPEN, CLOSED, SUSPENDED
    pub balance: Money,
    pub overdraft_limit: Money,
    pub interest_rate: Decimal,
    pub created_at: DateTime<Utc>,
    pub arrangement_id: Option<ArrangementId>, // Lien BC13
}

pub struct AccountMovement {
    pub id: MovementId,
    pub account_id: AccountId,
    pub movement_type: MovementType, // DEBIT, CREDIT
    pub amount: Money,
    pub description: String,
    pub reference: String,
    pub executed_at: DateTime<Utc>,
}

pub enum AccountType {
    Current,
    Savings,
    TimeDeposit { maturity_date: NaiveDate },
}

pub enum AccountStatus {
    Open,
    Closed { closed_at: DateTime<Utc> },
    Suspended { reason: String },
}
```

**Ports** :
```rust
#[async_trait]
pub trait AccountRepository {
    async fn create(&self, account: &Account) -> Result<(), DomainError>;
    async fn get(&self, id: &AccountId) -> Result<Option<Account>, DomainError>;
    async fn get_by_iban(&self, iban: &Iban) -> Result<Option<Account>, DomainError>;
    async fn list_by_customer(&self, customer_id: &CustomerId) -> Result<Vec<Account>, DomainError>;
    async fn record_movement(&self, movement: &AccountMovement) -> Result<(), DomainError>;
}

#[async_trait]
pub trait BalanceCalculator {
    async fn calculate_balance(&self, account_id: &AccountId) -> Result<Money, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/accounts` — Ouvrir compte
- `GET /api/v1/accounts/{id}` — Détails compte
- `GET /api/v1/customers/{id}/accounts` — Lister comptes client
- `PUT /api/v1/accounts/{id}/status` — Changer statut (suspension/clôture)
- `GET /api/v1/accounts/{id}/balance` — Solde en temps réel
- `GET /api/v1/accounts/{id}/movements` — Historique mouvements
- `POST /api/v1/accounts/{id}/interest-calculation` — Calcul intérêts

---

#### BC3 — Credit (Gestion des crédits)

**Responsabilité** : Octroi crédit, suivi, classification créances (0-4), provisionnement NCT + IFRS 9 ECL, remboursement, liens Collateral.

**Entités Rust** :
```rust
pub struct Loan {
    pub id: LoanId,
    pub customer_id: CustomerId,
    pub account_id: Option<AccountId>,
    pub arrangement_id: Option<ArrangementId>, // Lien BC13
    pub principal_amount: Money,
    pub interest_rate: Decimal,
    pub term_months: u16,
    pub disbursement_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub status: LoanStatus,
    pub asset_class: AssetClass,       // 0, 1, 2, 3, 4
    pub npl_classification: NplStage,  // STAGE1, STAGE2, STAGE3
    pub collateral_ids: Vec<CollateralId>, // Liens BC14
}

pub enum AssetClass {
    Class0, // Standard
    Class1, // Watchlist
    Class2, // Substandard (provision 20%)
    Class3, // Doubtful (provision 50%)
    Class4, // Loss (provision 100%)
}

pub enum NplStage {
    Stage1 { days_past_due: u16 },  // 0-29 days, 12m ECL
    Stage2 { days_past_due: u16 },  // 30-89 days, lifetime ECL
    Stage3 { days_past_due: u16 },  // 90+ days, lifetime ECL + impairment
}

pub struct LoanProvision {
    pub loan_id: LoanId,
    pub nct_provision_pct: Decimal,    // NCT: classe 2→20%, 3→50%, 4→100%
    pub nct_provision_amount: Money,
    pub ifrs9_ecl_amount: Money,       // Expected Credit Loss
    pub ecl_stage: NplStage,
    pub pd: Decimal,                   // Probability of Default
    pub lgd: Decimal,                  // Loss Given Default
    pub ead: Money,                    // Exposure at Default
    pub calculated_at: DateTime<Utc>,
}

pub struct LoanSchedule {
    pub loan_id: LoanId,
    pub payments: Vec<SchedulePayment>,
}

pub struct SchedulePayment {
    pub installment_number: u16,
    pub due_date: NaiveDate,
    pub principal: Money,
    pub interest: Money,
    pub paid: bool,
}
```

**Ports** :
```rust
#[async_trait]
pub trait LoanRepository {
    async fn create(&self, loan: &Loan) -> Result<(), DomainError>;
    async fn get(&self, id: &LoanId) -> Result<Option<Loan>, DomainError>;
    async fn list_by_customer(&self, customer_id: &CustomerId) -> Result<Vec<Loan>, DomainError>;
}

#[async_trait]
pub trait AssetClassifier {
    async fn classify(&self, loan: &Loan) -> Result<AssetClass, DomainError>;
}

#[async_trait]
pub trait ProvisioningCalculator {
    async fn calculate_nct_provision(&self, loan: &Loan) -> Result<Money, DomainError>;
    async fn calculate_ifrs9_ecl(&self, loan: &Loan) -> Result<LoanProvision, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/loans` — Octroyer crédit
- `GET /api/v1/loans/{id}` — Détails crédit
- `GET /api/v1/customers/{id}/loans` — Lister crédits client
- `POST /api/v1/loans/{id}/classify` — Classifier créance
- `POST /api/v1/loans/{id}/provision` — Calculer provisions
- `GET /api/v1/loans/{id}/schedule` — Calendrier remboursement
- `POST /api/v1/loans/{id}/payment` — Enregistrer remboursement
- `POST /api/v1/loans/{id}/npl-stage` — Classifier NPL

---

#### BC4 — AML (Anti-blanchiment)

**Responsabilité** : Surveillance transactionnelle, alertes, investigations, DOS, gel avoirs, conformité Circ. 2025-17.

**Entités Rust** :
```rust
pub struct AmlTransaction {
    pub id: TransactionId,
    pub account_id: AccountId,
    pub amount: Money,
    pub direction: TransactionDirection, // DEBIT, CREDIT
    pub counterparty: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
}

pub enum AmlRuleResult {
    Pass,
    Alert { rule_id: String, score: u8 },
}

pub struct AmlAlert {
    pub id: AlertId,
    pub transaction_id: TransactionId,
    pub alert_type: AlertType,
    pub severity: AlertSeverity, // LOW, MEDIUM, HIGH, CRITICAL
    pub status: AlertStatus,     // PENDING, INVESTIGATING, RESOLVED, ESCALATED
    pub created_at: DateTime<Utc>,
}

pub enum AlertType {
    StructuringThreshold,       // Cumul 5k TND/jour
    UnusualBehavior,
    HighRiskCountry,
    PepTransfer,
}

pub struct SuspicionReport {
    pub id: ReportId,
    pub alerts: Vec<AlertId>,
    pub customer_id: CustomerId,
    pub narrative: String,
    pub status: ReportStatus, // DRAFT, SUBMITTED, ACKNOWLEDGED
    pub submitted_to_ctaf_at: Option<DateTime<Utc>>,
    pub submitted_via: ReportChannel, // GOAML, EMAIL, MANUAL
}

pub enum ReportStatus {
    Draft,
    Submitted { submitted_at: DateTime<Utc> },
    Acknowledged { ack_at: DateTime<Utc>, ack_number: String },
}

pub struct AssetFreeze {
    pub id: FreezeId,
    pub account_id: AccountId,
    pub reason: String,
    pub frozen_at: DateTime<Utc>,
    pub frozen_by: UserId,
    pub unfrozen_at: Option<DateTime<Utc>>,
    pub unfrozen_by: Option<UserId>,
}
```

**Ports** :
```rust
#[async_trait]
pub trait AmlScenarioEngine {
    async fn evaluate_transaction(&self, transaction: &AmlTransaction) -> Result<AmlRuleResult, DomainError>;
    async fn evaluate_cumulative(&self, account_id: &AccountId, period: &AmlPeriod) -> Result<Vec<AmlRuleResult>, DomainError>;
}

#[async_trait]
pub trait AlertRepository {
    async fn create_alert(&self, alert: &AmlAlert) -> Result<(), DomainError>;
    async fn update_status(&self, alert_id: &AlertId, status: AlertStatus) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SuspicionReportRepository {
    async fn create_dos(&self, report: &SuspicionReport) -> Result<(), DomainError>;
    async fn submit_to_ctaf(&self, report_id: &ReportId) -> Result<String, DomainError>; // goAML
}
```

**API Routes** :
- `POST /api/v1/aml/transactions/screen` — Évaluer transaction
- `GET /api/v1/aml/alerts` — Lister alertes
- `PUT /api/v1/aml/alerts/{id}/status` — Mettre à jour alerte
- `POST /api/v1/aml/investigations/{id}` — Créer investigation
- `POST /api/v1/aml/suspicion-reports` — Créer DOS
- `POST /api/v1/aml/suspicion-reports/{id}/submit-ctaf` — Soumettre goAML
- `POST /api/v1/aml/freeze` — Geler avoirs
- `POST /api/v1/aml/unfreeze/{id}` — Dégeler avoirs

---

#### BC5 — Sanctions (Filtrage sanctions)

**Responsabilité** : Listes ONU/UE/OFAC/nationales, screening, matching, travel rule validation.

**Entités Rust** :
```rust
pub struct SanctionList {
    pub id: SanctionListId,
    pub list_type: SanctionListType, // UN, EU, OFAC, NATIONAL
    pub source: String,
    pub last_updated: DateTime<Utc>,
    pub entries: Vec<SanctionEntry>,
}

pub struct SanctionEntry {
    pub id: EntryId,
    pub entity_name: String,
    pub aliases: Vec<String>,
    pub country: CountryCode,
    pub list_reference: String,
}

pub struct ScreeningResult {
    pub transaction_id: TransactionId,
    pub entity_name: String,
    pub match_score: u8, // 0-100
    pub matched_entries: Vec<(SanctionEntryId, u8)>, // (entry, score)
    pub action: ScreeningAction, // PASS, REVIEW, BLOCK
}

pub enum ScreeningAction {
    Pass,
    Review { reason: String },
    Block { reason: String },
}
```

**Ports** :
```rust
#[async_trait]
pub trait SanctionListRepository {
    async fn get_current_lists(&self) -> Result<Vec<SanctionList>, DomainError>;
    async fn update_lists(&self, lists: Vec<SanctionList>) -> Result<(), DomainError>;
}

#[async_trait]
pub trait MatchingEngine {
    async fn screen_entity(&self, name: &str, country: &CountryCode) -> Result<Vec<ScreeningResult>, DomainError>;
    async fn screen_transaction(&self, payment: &PaymentOrder) -> Result<ScreeningAction, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/sanctions/screen` — Évaluer entité
- `GET /api/v1/sanctions/lists` — Lister sources
- `POST /api/v1/sanctions/lists/update` — Mettre à jour listes
- `GET /api/v1/sanctions/results/{transaction_id}` — Résultat screening

---

#### BC6 — Prudential (Ratios prudentiels)

**Responsabilité** : Calcul solvabilité (10%), Tier 1 (7%), C/D (120%), concentration (25%), RWA.

**Entités Rust** :
```rust
pub struct PrudentialRatio {
    pub calculation_date: NaiveDate,
    pub solvency_ratio: Decimal,       // Min 10%
    pub tier1_ratio: Decimal,          // Min 7%
    pub credit_to_deposit: Decimal,    // Max 120%
    pub concentration_limit: Decimal,  // Max 25% FPN
    pub rwa: Money,                    // Risk-Weighted Assets
    pub regulatory_capital: Money,
}

pub struct RiskWeightedAsset {
    pub asset_id: AssetId,
    pub asset_type: AssetType,
    pub gross_value: Money,
    pub risk_weight: Decimal,
    pub rwa: Money,
}

pub enum AssetType {
    Cash,
    SovereignBond,
    CorporateLoan,
    RetailExposure,
}
```

**Ports** :
```rust
#[async_trait]
pub trait RwaCalculator {
    async fn calculate_rwa(&self, date: &NaiveDate) -> Result<RiskWeightedAsset, DomainError>;
}

#[async_trait]
pub trait PrudentialRatioCalculator {
    async fn calculate_ratios(&self, date: &NaiveDate) -> Result<PrudentialRatio, DomainError>;
    async fn check_compliance(&self, ratio: &PrudentialRatio) -> Result<ComplianceStatus, DomainError>;
}
```

**API Routes** :
- `GET /api/v1/prudential/ratios` — Récupérer ratios jour
- `POST /api/v1/prudential/calculate` — Forcer calcul
- `GET /api/v1/prudential/compliance` — Statut conformité

---

#### BC7 — Accounting (Comptabilité)

**Responsabilité** : Journal NCT, écritures, grand livre, balance, double moteur NCT + IFRS 9.

**Entités Rust** :
```rust
pub struct JournalEntry {
    pub id: EntryId,
    pub reference: String,
    pub accounting_period: Period,
    pub debit_entries: Vec<DebitLine>,
    pub credit_entries: Vec<CreditLine>,
    pub narrative: String,
    pub posted_at: DateTime<Utc>,
    pub posted_by: UserId,
}

pub struct DebitLine {
    pub account_code: AccountingCode, // NCT account
    pub amount: Money,
}

pub struct CreditLine {
    pub account_code: AccountingCode,
    pub amount: Money,
}

pub struct ChartOfAccounts {
    pub accounts: Vec<AccountDefinition>,
}

pub struct AccountDefinition {
    pub code: AccountingCode, // NCT format
    pub description: String,
    pub account_type: NctAccountType, // ASSET, LIABILITY, EQUITY, REVENUE, EXPENSE
}

pub struct LedgerAccount {
    pub code: AccountingCode,
    pub balance: Money,
    pub entries: Vec<JournalEntry>,
}

pub enum Period {
    Monthly { year: u16, month: u8 },
    Quarterly { year: u16, quarter: u8 },
    Annual { year: u16 },
}
```

**Ports** :
```rust
#[async_trait]
pub trait JournalRepository {
    async fn post_entry(&self, entry: &JournalEntry) -> Result<(), DomainError>;
    async fn get_ledger(&self, code: &AccountingCode) -> Result<LedgerAccount, DomainError>;
}

#[async_trait]
pub trait ProvisioningJournal {
    async fn post_provision_entry(&self, provision: &LoanProvision) -> Result<(), DomainError>;
}

#[async_trait]
pub trait IfrsJournal {
    async fn post_ecl_entry(&self, loan_id: &LoanId, ecl: &Money) -> Result<(), DomainError>;
}
```

**API Routes** :
- `POST /api/v1/accounting/entries` — Poster écriture
- `GET /api/v1/accounting/ledger/{code}` — Consulter compte
- `GET /api/v1/accounting/trial-balance` — Balance générale
- `GET /api/v1/accounting/financial-statement` — États financiers

---

#### BC8 — Reporting (États réglementaires)

**Responsabilité** : États prudentiels BCT, rapports AML, financiers, formats officiels.

**Entités Rust** :
```rust
pub struct RegulatoryReport {
    pub id: ReportId,
    pub report_type: ReportType,
    pub reporting_date: NaiveDate,
    pub status: ReportStatus,
    pub data: serde_json::Value,
    pub submitted_at: Option<DateTime<Utc>>,
}

pub enum ReportType {
    PrudentialStatus,      // Ratios mensuels
    AmlStatistics,         // DOS, alertes
    FinancialStatement,    // Bilan, P&L
    CustomerStatistics,    // Nombre clients, KYC%
}

pub enum ReportStatus {
    Draft,
    Validated,
    Submitted,
}
```

**Ports** :
```rust
#[async_trait]
pub trait ReportingRepository {
    async fn create_report(&self, report: &RegulatoryReport) -> Result<(), DomainError>;
    async fn submit_to_bct(&self, report_id: &ReportId) -> Result<String, DomainError>; // Receipt
}

#[async_trait]
pub trait ReportGenerator {
    async fn generate_prudential(&self, date: &NaiveDate) -> Result<RegulatoryReport, DomainError>;
    async fn generate_aml(&self, date: &NaiveDate) -> Result<RegulatoryReport, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/reporting/generate` — Générer rapport
- `GET /api/v1/reporting/{id}` — Consulter rapport
- `POST /api/v1/reporting/{id}/submit` — Soumettre BCT

---

#### BC9 — Payment (Virements et paiements)

**Responsabilité** : Virements nationaux/internationaux, SWIFT, ISO 20022, travel rule, compensation.

**Entités Rust** :
```rust
pub struct PaymentOrder {
    pub id: PaymentId,
    pub originator_account: AccountId,
    pub originator_name: String,
    pub originator_id: CustomerId,
    pub beneficiary_account: String,    // IBAN or local account
    pub beneficiary_name: String,
    pub beneficiary_id: Option<CustomerId>, // If known
    pub beneficiary_country: CountryCode,
    pub amount: Money,
    pub currency: CurrencyCode,
    pub purpose: String,
    pub status: PaymentStatus,
    pub payment_type: PaymentType,
    pub created_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
}

pub enum PaymentStatus {
    Pending,
    Screening,      // Sanctions check
    TravelRuleCheck, // Originator/beneficiary data
    Approved,
    Rejected { reason: String },
    Executed { reference: String },
}

pub enum PaymentType {
    Domestic,
    International,
}

pub struct TravelRuleData {
    pub originator: TravelRuleParty,
    pub beneficiary: TravelRuleParty,
}

pub struct TravelRuleParty {
    pub name: String,
    pub account_number: String,
    pub address: PostalAddress,
    pub identification: String,
}

pub struct SwiftMessage {
    pub payment_id: PaymentId,
    pub message_type: String, // MT103, MT202
    pub content: String,
}
```

**Ports** :
```rust
#[async_trait]
pub trait PaymentRepository {
    async fn create(&self, payment: &PaymentOrder) -> Result<(), DomainError>;
    async fn update_status(&self, id: &PaymentId, status: PaymentStatus) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SwiftAdapter {
    async fn send_swift(&self, message: &SwiftMessage) -> Result<String, DomainError>;
}

#[async_trait]
pub trait TravelRuleValidator {
    async fn validate(&self, payment: &PaymentOrder) -> Result<TravelRuleData, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/payments` — Créer virement
- `GET /api/v1/payments/{id}` — Consulter virement
- `POST /api/v1/payments/{id}/execute` — Exécuter
- `GET /api/v1/payments/{id}/travel-rule` — Données travel rule

---

#### BC10 — ForeignExchange (Change)

**Responsabilité** : Opérations FX, position FX, taux, conformité Loi 76-18.

**Entités Rust** :
```rust
pub struct FxOperation {
    pub id: FxOperationId,
    pub account_id: AccountId,
    pub operation_type: FxOperationType,
    pub from_currency: CurrencyCode,
    pub to_currency: CurrencyCode,
    pub from_amount: Money,
    pub to_amount: Money,
    pub exchange_rate: ExchangeRate,
    pub executed_at: DateTime<Utc>,
}

pub enum FxOperationType {
    Spot,
    Forward { settlement_date: NaiveDate },
    Swap,
}

pub struct FxPosition {
    pub currency: CurrencyCode,
    pub net_position: Money,
    pub limit: Money,
}

pub struct ExchangeRate {
    pub from_currency: CurrencyCode,
    pub to_currency: CurrencyCode,
    pub rate: Decimal,
    pub timestamp: DateTime<Utc>,
}
```

**Ports** :
```rust
#[async_trait]
pub trait FxRepository {
    async fn record_operation(&self, op: &FxOperation) -> Result<(), DomainError>;
    async fn get_position(&self, currency: &CurrencyCode) -> Result<FxPosition, DomainError>;
}

#[async_trait]
pub trait ExchangeRateProvider {
    async fn get_rate(&self, from: &CurrencyCode, to: &CurrencyCode) -> Result<ExchangeRate, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/fx/operations` — Créer opération FX
- `GET /api/v1/fx/positions` — Positions FX
- `GET /api/v1/fx/rates` — Taux du jour

---

#### BC11 — Governance (Contrôle interne)

**Responsabilité** : Audit trail immutable, 3 lignes défense, comités, piste d'audit cryptographique.

**Entités Rust** :
```rust
pub struct AuditTrailEntry {
    pub id: AuditEventId,
    pub entity_type: String,
    pub entity_id: String,
    pub action: String,
    pub actor: UserId,
    pub timestamp: DateTime<Utc>,
    pub details: serde_json::Value,
    pub hash: String,              // SHA-256 for immutability
    pub previous_hash: Option<String>, // Chain link
}

pub struct ControlCheck {
    pub id: ControlId,
    pub control_name: String,
    pub status: ControlStatus,
    pub last_checked: DateTime<Utc>,
}

pub enum ControlStatus {
    Effective,
    Ineffective { reason: String },
}

pub struct Committee {
    pub id: CommitteeId,
    pub committee_type: CommitteeType, // AUDIT, RISK, NOMINATION
    pub members: Vec<UserId>,
}

pub struct ComplianceReport {
    pub date: NaiveDate,
    pub controls_checked: u16,
    pub controls_effective: u16,
    pub issues: Vec<String>,
}
```

**Ports** :
```rust
#[async_trait]
pub trait AuditTrailRepository {
    async fn record(&self, entry: &AuditTrailEntry) -> Result<(), DomainError>;
    async fn verify_chain(&self, entry_id: &AuditEventId) -> Result<bool, DomainError>;
}

#[async_trait]
pub trait HashChainService {
    fn compute_hash(&self, entry: &AuditTrailEntry) -> String;
    fn verify_hash(&self, entry: &AuditTrailEntry) -> bool;
}
```

**API Routes** :
- `GET /api/v1/audit/trail` — Lister événements audit
- `GET /api/v1/audit/trail/{entity_id}` — Historique entité
- `POST /api/v1/governance/controls` — Créer contrôle
- `GET /api/v1/governance/controls` — État contrôles

---

#### BC12 — Identity (Authentification + Autorisation)

**Responsabilité** : Authentification (FIDO2/WebAuthn), 2FA, RBAC, sessions, JWT, MFA PCI DSS.

**Entités Rust** :
```rust
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub roles: Vec<RoleId>,
    pub status: UserStatus, // ACTIVE, INACTIVE, LOCKED
    pub mfa_enabled: bool,
    pub mfa_method: Option<MfaMethod>, // TOTP, SMS, FIDO2
    pub created_at: DateTime<Utc>,
}

pub enum UserStatus {
    Active,
    Inactive,
    Locked { locked_at: DateTime<Utc> },
}

pub enum MfaMethod {
    Totp,
    Sms,
    Fido2,
}

pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub permissions: Vec<Permission>,
}

pub struct Permission {
    pub resource: String,
    pub action: String,  // read, write, delete, execute
}

pub struct SessionToken {
    pub id: SessionId,
    pub user_id: UserId,
    pub jwt: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub struct Credential {
    pub user_id: UserId,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub counter: u32,
}
```

**Ports** :
```rust
#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<(), DomainError>;
    async fn get(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
}

#[async_trait]
pub trait JwtService {
    async fn create_token(&self, user: &User) -> Result<SessionToken, DomainError>;
    async fn verify_token(&self, token: &str) -> Result<TokenClaims, DomainError>;
    async fn refresh_token(&self, refresh: &str) -> Result<SessionToken, DomainError>;
}

#[async_trait]
pub trait Fido2Service {
    async fn register_credential(&self, user_id: &UserId, credential: &Credential) -> Result<(), DomainError>;
    async fn verify_credential(&self, user_id: &UserId, assertion: &[u8]) -> Result<bool, DomainError>;
}

#[async_trait]
pub trait MfaService {
    async fn generate_totp(&self, user_id: &UserId) -> Result<String, DomainError>;
    async fn verify_totp(&self, user_id: &UserId, code: &str) -> Result<bool, DomainError>;
}

#[async_trait]
pub trait RbacValidator {
    async fn has_permission(&self, user_id: &UserId, resource: &str, action: &str) -> Result<bool, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/auth/register` — S'inscrire
- `POST /api/v1/auth/login` — Se connecter (JWT)
- `POST /api/v1/auth/mfa/verify` — Vérifier MFA
- `POST /api/v1/auth/fido2/register` — Enregistrer clé FIDO2
- `POST /api/v1/auth/fido2/authenticate` — Auth FIDO2
- `POST /api/v1/auth/refresh` — Renouveler token
- `POST /api/v1/auth/logout` — Se déconnecter

---

### 2.2 Contextes NOUVEAUX v4.0 (9 nouveaux — parité Temenos)

#### BC13 — Arrangement (CENTRAL — Contrats clients)

**Responsabilité** : Hub central liant Account, Credit, Collateral, Insurance. Contrats, conditions, limites produits.

**Entités Rust** :
```rust
pub struct Arrangement {
    pub id: ArrangementId,
    pub customer_id: CustomerId,
    pub account_id: Option<AccountId>,
    pub credit_id: Option<LoanId>,
    pub collateral_ids: Vec<CollateralId>,
    pub insurance_ids: Vec<InsuranceId>,
    pub arrangement_type: ArrangementType,
    pub status: ArrangementStatus,
    pub conditions: Vec<ArrangementCondition>,
    pub created_at: DateTime<Utc>,
    pub maturity_date: Option<NaiveDate>,
}

pub enum ArrangementType {
    Deposit,
    Credit,
    DepositAndCredit,
    Treasury,
    Islamic,
}

pub enum ArrangementStatus {
    Draft,
    Approved { approved_by: UserId },
    Active,
    Inactive,
    Expired,
}

pub struct ArrangementCondition {
    pub id: ConditionId,
    pub arrangement_id: ArrangementId,
    pub condition_type: String,
    pub value: String,
    pub enforcement: String, // "MANDATORY" or "ADVISORY"
}

pub struct ArrangementLimit {
    pub id: LimitId,
    pub arrangement_id: ArrangementId,
    pub limit_type: LimitType,
    pub limit_amount: Money,
    pub utilization: Money,
}

pub enum LimitType {
    CreditLimit,
    DailyTransferLimit,
    OverdraftLimit,
}
```

**Ports** :
```rust
#[async_trait]
pub trait ArrangementRepository {
    async fn create(&self, arrangement: &Arrangement) -> Result<(), DomainError>;
    async fn get(&self, id: &ArrangementId) -> Result<Option<Arrangement>, DomainError>;
    async fn list_by_customer(&self, customer_id: &CustomerId) -> Result<Vec<Arrangement>, DomainError>;
}

#[async_trait]
pub trait ArrangementApprovalService {
    async fn request_approval(&self, arrangement: &Arrangement) -> Result<ApprovalRequest, DomainError>;
    async fn approve(&self, arrangement_id: &ArrangementId, approver: &UserId) -> Result<(), DomainError>;
}
```

**API Routes** :
- `POST /api/v1/arrangements` — Créer contrat
- `GET /api/v1/arrangements/{id}` — Détails
- `PUT /api/v1/arrangements/{id}` — Modifier
- `POST /api/v1/arrangements/{id}/approve` — Approuver
- `POST /api/v1/arrangements/{id}/conditions` — Ajouter condition
- `GET /api/v1/arrangements/{id}/limits` — Limites

---

#### BC14 — Collateral (Garanties)

**Responsabilité** : Nantissements, évaluations, LTV, pools, pledges.

**Entités Rust** :
```rust
pub struct Collateral {
    pub id: CollateralId,
    pub arrangement_id: ArrangementId,
    pub collateral_type: CollateralType,
    pub description: String,
    pub valuation: CollateralValuation,
    pub status: CollateralStatus,
    pub created_at: DateTime<Utc>,
}

pub enum CollateralType {
    RealEstate,
    Vehicle,
    Securities,
    Cash,
    Other,
}

pub struct CollateralValuation {
    pub valuation_date: NaiveDate,
    pub gross_value: Money,
    pub haircut_pct: Decimal,
    pub net_value: Money,
    pub valuer: UserId,
}

pub struct Pledge {
    pub id: PledgeId,
    pub collateral_id: CollateralId,
    pub pledgee: String,  // Creditor
    pub pledgor: String,  // Debtor
    pub status: PledgeStatus,
}

pub enum PledgeStatus {
    Active,
    Released,
    Foreclosed,
}

pub struct CollateralPool {
    pub id: PoolId,
    pub collaterals: Vec<CollateralId>,
    pub total_value: Money,
}

pub struct LTV {
    pub collateral_id: CollateralId,
    pub loan_amount: Money,
    pub ltv_ratio: Decimal, // Loan-to-Value: loan / collateral_net_value
}
```

**Ports** :
```rust
#[async_trait]
pub trait CollateralRepository {
    async fn create(&self, collateral: &Collateral) -> Result<(), DomainError>;
    async fn record_valuation(&self, valuation: &CollateralValuation) -> Result<(), DomainError>;
}

#[async_trait]
pub trait LtvCalculator {
    async fn calculate(&self, collateral_id: &CollateralId, loan_amount: &Money) -> Result<Decimal, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/collaterals` — Enregistrer garantie
- `POST /api/v1/collaterals/{id}/valuation` — Évaluer
- `POST /api/v1/collaterals/{id}/pledge` — Créer nantissement
- `POST /api/v1/collaterals/{id}/release` — Libérer garantie
- `GET /api/v1/collaterals/{id}/ltv` — Ratio LTV

---

#### BC15 — TradeFinance (Lettres de crédit)

**Responsabilité** : LC, garanties bancaires, DC, workflows UCP 600.

**Entités Rust** :
```rust
pub struct LetterOfCredit {
    pub id: LcId,
    pub arrangement_id: ArrangementId,
    pub lc_type: LcType, // SIGHT, USANCE
    pub issuer_bank: String,
    pub issuing_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub amount: Money,
    pub beneficiary: String,
    pub applicant: String,
    pub status: LcStatus,
    pub documents_required: Vec<DocumentRequirement>,
}

pub enum LcType {
    Sight,
    Usance { days: u16 },
    Revolving,
}

pub enum LcStatus {
    Draft,
    Submitted,
    Advised,
    Confirmed,
    Expired,
    Paid,
}

pub struct DocumentaryCredit {
    pub id: DcId,
    pub lc_id: LcId,
    pub document_type: String, // BILL_OF_LADING, INVOICE, etc.
    pub status: DocumentStatus,
    pub received_at: DateTime<Utc>,
}

pub enum DocumentStatus {
    Pending,
    Received,
    Discrepancy,
    Accepted,
}

pub struct BankGuarantee {
    pub id: GuaranteeId,
    pub arrangement_id: ArrangementId,
    pub guarantee_type: GuaranteeType, // BID, PERFORMANCE, PAYMENT
    pub amount: Money,
    pub validity_date: NaiveDate,
    pub status: GuaranteeStatus,
}

pub enum GuaranteeType {
    Bid,
    Performance,
    Payment,
    Other,
}

pub enum GuaranteeStatus {
    Issued,
    Called,
    Released,
}
```

**Ports** :
```rust
#[async_trait]
pub trait TradeFinanceRepository {
    async fn create_lc(&self, lc: &LetterOfCredit) -> Result<(), DomainError>;
    async fn record_document(&self, doc: &DocumentaryCredit) -> Result<(), DomainError>;
}

#[async_trait]
pub trait UcpValidator {
    async fn validate_documents(&self, lc_id: &LcId) -> Result<Vec<Discrepancy>, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/trade-finance/lc` — Créer LC
- `GET /api/v1/trade-finance/lc/{id}` — Détails LC
- `POST /api/v1/trade-finance/lc/{id}/document` — Soumettre doc
- `POST /api/v1/trade-finance/lc/{id}/validate` — Validation UCP
- `POST /api/v1/trade-finance/guarantee` — Créer garantie

---

#### BC16 — CashManagement (Trésorerie)

**Responsabilité** : Sweeps, pooling, liquidité, FX forwards, position trésorerie.

**Entités Rust** :
```rust
pub struct SweepAccount {
    pub id: SweepId,
    pub arrangement_id: ArrangementId,
    pub master_account: AccountId,
    pub detail_accounts: Vec<AccountId>,
    pub sweep_type: SweepType,  // OVERNIGHT, PERIODIC
    pub sweep_frequency: SweepFrequency,
    pub threshold: Money,
    pub created_at: DateTime<Utc>,
}

pub enum SweepType {
    Overnight,
    Periodic { period_days: u16 },
}

pub enum SweepFrequency {
    Daily,
    Weekly,
    Monthly,
}

pub struct LiquidityPosition {
    pub date: NaiveDate,
    pub currency: CurrencyCode,
    pub inflows: Money,
    pub outflows: Money,
    pub net_position: Money,
    pub forecast_7d: Money,
    pub forecast_30d: Money,
}

pub struct FxForward {
    pub id: ForwardId,
    pub arrangement_id: ArrangementId,
    pub from_currency: CurrencyCode,
    pub to_currency: CurrencyCode,
    pub forward_amount: Money,
    pub forward_rate: Decimal,
    pub settlement_date: NaiveDate,
    pub status: ForwardStatus,
}

pub enum ForwardStatus {
    Pending,
    Confirmed,
    Settled,
    Cancelled,
}

pub struct CashPosition {
    pub currency: CurrencyCode,
    pub position: Money,
    pub limit: Money,
}
```

**Ports** :
```rust
#[async_trait]
pub trait SweepRepository {
    async fn create(&self, sweep: &SweepAccount) -> Result<(), DomainError>;
    async fn execute_sweep(&self, sweep_id: &SweepId) -> Result<(), DomainError>;
}

#[async_trait]
pub trait LiquidityCalculator {
    async fn calculate_position(&self, date: &NaiveDate, currency: &CurrencyCode) -> Result<LiquidityPosition, DomainError>;
}

#[async_trait]
pub trait ForwardRepository {
    async fn create_forward(&self, forward: &FxForward) -> Result<(), DomainError>;
    async fn settle_forward(&self, forward_id: &ForwardId) -> Result<(), DomainError>;
}
```

**API Routes** :
- `POST /api/v1/cash-management/sweep` — Créer sweep
- `POST /api/v1/cash-management/sweep/{id}/execute` — Exécuter
- `GET /api/v1/cash-management/liquidity` — Position liquidité
- `POST /api/v1/cash-management/forward` — Créer forward
- `GET /api/v1/cash-management/cash-positions` — Positions trésorerie

---

#### BC17 — IslamicBanking (Produits Sharia)

**Responsabilité** : Murabaha, ijara, waqf, wakala, musharaka, sukuk, validation Sharia.

**Entités Rust** :
```rust
pub struct IslamicProduct {
    pub id: ProductId,
    pub product_type: IslamicProductType,
    pub sharia_compliant: bool,
    pub approved_by: Option<ShariaBoard>,
}

pub enum IslamicProductType {
    Murabaha {
        cost: Money,
        profit_margin: Decimal,
    },
    Ijara {
        lessor: String,
        lessee: String,
        lease_amount: Money,
    },
    Waqf {
        endowment_amount: Money,
        beneficiary: String,
    },
    Wakala {
        fee_pct: Decimal,
    },
    Musharaka {
        capital_contribution: Money,
        profit_sharing: Decimal,
    },
}

pub struct Murabaha {
    pub id: MurId,
    pub arrangement_id: ArrangementId,
    pub cost_price: Money,
    pub profit_margin: Decimal,
    pub selling_price: Money,
    pub payment_terms: Vec<PaymentTerm>,
}

pub struct PaymentTerm {
    pub due_date: NaiveDate,
    pub amount: Money,
}

pub struct ShariaBoard {
    pub id: ShariaId,
    pub members: Vec<Scholar>,
    pub approvals: Vec<Approval>,
}

pub struct Scholar {
    pub name: String,
    pub credentials: String,
}

pub struct Approval {
    pub scholar_id: String,
    pub approved: bool,
    pub date: DateTime<Utc>,
}

pub struct SukukIssuance {
    pub id: SukukId,
    pub asset_pool: Vec<AssetId>,
    pub coupon_rate: Decimal,
    pub maturity_date: NaiveDate,
}
```

**Ports** :
```rust
#[async_trait]
pub trait IslamicProductRepository {
    async fn create(&self, product: &IslamicProduct) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ShariaValidator {
    async fn validate_murabaha(&self, murabaha: &Murabaha) -> Result<ValidationResult, DomainError>;
    async fn validate_ijara(&self, ijara: &Ijara) -> Result<ValidationResult, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/islamic-banking/murabaha` — Créer murabaha
- `POST /api/v1/islamic-banking/ijara` — Créer ijara
- `POST /api/v1/islamic-banking/waqf` — Créer waqf
- `POST /api/v1/islamic-banking/{id}/sharia-validate` — Validation Sharia

---

#### BC18 — DataHub (Master Data Management)

**Responsabilité** : ODS/ADS, MDM, data lake, data quality, lineage tracking.

**Entités Rust** :
```rust
pub struct MdmRecord {
    pub id: RecordId,
    pub entity_type: String,
    pub external_ids: Vec<(String, String)>, // (system, id)
    pub golden_record: serde_json::Value,
    pub data_quality_score: u8, // 0-100
    pub last_updated: DateTime<Utc>,
}

pub struct DataPipeline {
    pub id: PipelineId,
    pub source: String,
    pub target: String,
    pub transformation_rules: Vec<Rule>,
    pub execution_log: Vec<ExecutionEntry>,
}

pub struct DataEntity {
    pub id: EntityId,
    pub entity_type: String,
    pub attributes: serde_json::Value,
    pub lineage: LineageInfo,
}

pub struct LineageInfo {
    pub source_system: String,
    pub transformation_steps: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

pub struct DataQuality {
    pub entity_id: EntityId,
    pub completeness: Decimal,
    pub accuracy: Decimal,
    pub timeliness: Decimal,
    pub consistency: Decimal,
}
```

**Ports** :
```rust
#[async_trait]
pub trait MdmRepository {
    async fn create_record(&self, record: &MdmRecord) -> Result<(), DomainError>;
    async fn get_golden_record(&self, entity_type: &str, id: &str) -> Result<MdmRecord, DomainError>;
}

#[async_trait]
pub trait DataQualityService {
    async fn assess_quality(&self, entity_id: &EntityId) -> Result<DataQuality, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/data-hub/records` — Créer record MDM
- `GET /api/v1/data-hub/golden-record/{entity_type}/{id}` — Golden record
- `POST /api/v1/data-hub/pipeline` — Créer pipeline
- `GET /api/v1/data-hub/quality/{id}` — Qualité données

---

#### BC19 — ReferenceData (Données maître)

**Responsabilité** : Codes pays, devises, taux, tables maître, holidays, configurations.

**Entités Rust** :
```rust
pub struct ReferenceCode {
    pub id: CodeId,
    pub code_type: CodeType,
    pub code_value: String,
    pub description: String,
    pub status: CodeStatus,
}

pub enum CodeType {
    Country,
    Currency,
    Sector,
    BusinessType,
    DocumentType,
}

pub enum CodeStatus {
    Active,
    Inactive,
}

pub struct ReferenceRate {
    pub id: RateId,
    pub rate_type: RateType,
    pub rate_value: Decimal,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

pub enum RateType {
    BcbInterestRate,
    ExchangeRate,
    RefinancingRate,
}

pub struct ReferenceTable {
    pub id: TableId,
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>,
    pub rows: Vec<serde_json::Value>,
}

pub struct HolidayCalendar {
    pub year: u16,
    pub holidays: Vec<Holiday>,
}

pub struct Holiday {
    pub date: NaiveDate,
    pub name: String,
    pub country: CountryCode,
}
```

**Ports** :
```rust
#[async_trait]
pub trait ReferenceDataRepository {
    async fn create_code(&self, code: &ReferenceCode) -> Result<(), DomainError>;
    async fn get_codes(&self, code_type: CodeType) -> Result<Vec<ReferenceCode>, DomainError>;
    async fn create_rate(&self, rate: &ReferenceRate) -> Result<(), DomainError>;
    async fn get_current_rate(&self, rate_type: RateType) -> Result<ReferenceRate, DomainError>;
}

#[async_trait]
pub trait HolidayService {
    async fn is_holiday(&self, date: &NaiveDate) -> Result<bool, DomainError>;
    async fn get_next_business_day(&self, date: &NaiveDate) -> Result<NaiveDate, DomainError>;
}
```

**API Routes** :
- `GET /api/v1/reference-data/codes/{type}` — Lister codes
- `POST /api/v1/reference-data/codes` — Créer code
- `GET /api/v1/reference-data/rates/{type}` — Taux courant
- `POST /api/v1/reference-data/rates` — Créer taux
- `GET /api/v1/reference-data/holidays` — Calendrier fériés

---

#### BC20 — Securities (Valeurs mobilières)

**Responsabilité** : Titres, portefeuille, dépositaire BVMT, ordres bourse, custody.

**Entités Rust** :
```rust
pub struct Security {
    pub id: SecurityId,
    pub isin: String,
    pub name: String,
    pub issuer: String,
    pub security_type: SecurityType,
    pub current_price: Money,
    pub currency: CurrencyCode,
}

pub enum SecurityType {
    Stock,
    Bond,
    Fund,
    Sukuk,
}

pub struct Portfolio {
    pub id: PortfolioId,
    pub customer_id: CustomerId,
    pub positions: Vec<SecurityPosition>,
    pub total_value: Money,
}

pub struct SecurityPosition {
    pub security_id: SecurityId,
    pub quantity: u64,
    pub acquisition_price: Money,
    pub current_price: Money,
    pub market_value: Money,
    pub unrealized_gain_loss: Money,
}

pub struct CustodyAccount {
    pub id: CustodyId,
    pub customer_id: CustomerId,
    pub depository: String, // BVMT
    pub securities: Vec<SecurityHolding>,
}

pub struct SecurityHolding {
    pub security_id: SecurityId,
    pub quantity: u64,
    pub held_at: String,
}

pub struct SecurityOrder {
    pub id: OrderId,
    pub portfolio_id: PortfolioId,
    pub security_id: SecurityId,
    pub order_type: OrderType,
    pub quantity: u64,
    pub limit_price: Option<Money>,
    pub status: OrderStatus,
}

pub enum OrderType {
    Buy,
    Sell,
}

pub enum OrderStatus {
    Pending,
    Executed,
    Cancelled,
}
```

**Ports** :
```rust
#[async_trait]
pub trait SecurityRepository {
    async fn get(&self, id: &SecurityId) -> Result<Security, DomainError>;
    async fn list_all(&self) -> Result<Vec<Security>, DomainError>;
}

#[async_trait]
pub trait PortfolioRepository {
    async fn create(&self, portfolio: &Portfolio) -> Result<(), DomainError>;
    async fn get(&self, id: &PortfolioId) -> Result<Option<Portfolio>, DomainError>;
    async fn update_positions(&self, portfolio_id: &PortfolioId, positions: Vec<SecurityPosition>) -> Result<(), DomainError>;
}

#[async_trait]
pub trait BvmtAdapter {
    async fn submit_order(&self, order: &SecurityOrder) -> Result<String, DomainError>;
    async fn get_security_price(&self, isin: &str) -> Result<Money, DomainError>;
}
```

**API Routes** :
- `POST /api/v1/securities/portfolio` — Créer portefeuille
- `GET /api/v1/securities/portfolio/{id}` — Détails portefeuille
- `POST /api/v1/securities/order` — Créer ordre
- `GET /api/v1/securities/{isin}/price` — Prix titre
- `GET /api/v1/securities/custody/{id}` — Compte dépositaire

---

#### BC21 — Insurance (Assurances liées)

**Responsabilité** : Crédit, décès, risque, polices, sinistres, courtage intégré.

**Entités Rust** :
```rust
pub struct InsuranceProduct {
    pub id: ProductId,
    pub product_type: InsuranceType,
    pub premium_rate: Decimal,
    pub coverage: Money,
}

pub enum InsuranceType {
    CreditProtection,
    DeathBenefit,
    RiskCoverage,
}

pub struct InsurancePolicy {
    pub id: PolicyId,
    pub customer_id: CustomerId,
    pub product_id: ProductId,
    pub loan_id: Option<LoanId>,
    pub effective_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub premium: Money,
    pub coverage_amount: Money,
    pub status: PolicyStatus,
}

pub enum PolicyStatus {
    Active,
    Lapsed,
    Cancelled,
}

pub struct InsuranceClaim {
    pub id: ClaimId,
    pub policy_id: PolicyId,
    pub claim_type: String,
    pub claim_amount: Money,
    pub status: ClaimStatus,
    pub submitted_at: DateTime<Utc>,
}

pub enum ClaimStatus {
    Submitted,
    Under Review,
    Approved,
    Rejected,
    Paid,
}

pub struct Coverage {
    pub policy_id: PolicyId,
    pub coverage_type: String,
    pub limit: Money,
    pub deductible: Money,
}

pub struct BancassuranceLink {
    pub arrangement_id: ArrangementId,
    pub required_policies: Vec<PolicyId>,
}
```

**Ports** :
```rust
#[async_trait]
pub trait InsuranceRepository {
    async fn create_policy(&self, policy: &InsurancePolicy) -> Result<(), DomainError>;
    async fn file_claim(&self, claim: &InsuranceClaim) -> Result<(), DomainError>;
}

#[async_trait]
pub trait InsuranceProviderAdapter {
    async fn issue_policy(&self, policy: &InsurancePolicy) -> Result<String, DomainError>;
    async fn submit_claim(&self, claim: &InsuranceClaim) -> Result<(), DomainError>;
}
```

**API Routes** :
- `POST /api/v1/insurance/policy` — Émettre police
- `GET /api/v1/insurance/policy/{id}` — Détails police
- `POST /api/v1/insurance/claim` — Déclarer sinistre
- `GET /api/v1/insurance/claim/{id}` — Statut sinistre

---

## 3. Architecture Hexagonale Détaillée

### 3.1 Règles de Dépendance

```
Domain ← Application ← Infrastructure

Domain (Pure)
  ├─ Zéro imports externes (serde OK, tokio NON)
  ├─ Invariants compilés (Result types)
  ├─ Entités + Value Objects + Aggregates

Application
  ├─ Import Domain (✓)
  ├─ Définit Ports (traits)
  ├─ DTOs (serialization)
  ├─ Use Cases (orchestration)

Infrastructure
  ├─ Import Application ✓ + Domain ✓
  ├─ Implémente Ports
  ├─ Actix-web handlers
  ├─ PostgreSQL repos
```

### 3.2 Structure des Crates (Cargo Workspace)

```
BANKO/
├── backend/
│   ├── Cargo.toml (workspace root)
│   ├── crates/
│   │   ├── domain/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── customer/
│   │   │       ├── account/
│   │   │       ├── credit/
│   │   │       ├── aml/
│   │   │       ├── sanctions/
│   │   │       ├── prudential/
│   │   │       ├── accounting/
│   │   │       ├── reporting/
│   │   │       ├── payment/
│   │   │       ├── fx/
│   │   │       ├── governance/
│   │   │       ├── identity/
│   │   │       ├── arrangement/
│   │   │       ├── collateral/
│   │   │       ├── trade_finance/
│   │   │       ├── cash_management/
│   │   │       ├── islamic_banking/
│   │   │       ├── data_hub/
│   │   │       ├── reference_data/
│   │   │       ├── securities/
│   │   │       ├── insurance/
│   │   │       ├── error.rs
│   │   │       └── shared/ (Value Objects communs)
│   │   ├── application/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── customer/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── dto.rs
│   │   │       │   ├── use_cases.rs
│   │   │       │   ├── ports.rs
│   │   │       │   └── errors.rs
│   │   │       ├── [autres BCs idem]
│   │   │       └── event.rs (DomainEvent)
│   │   └── infrastructure/
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── lib.rs
│   │           ├── database/
│   │           │   ├── postgres.rs (PgPool)
│   │           │   ├── repositories.rs (impl Ports)
│   │           │   └── migrations/
│   │           ├── web/
│   │           │   ├── handlers/ (Actix routes)
│   │           │   ├── middleware/ (JWT, CORS)
│   │           │   ├── routes.rs
│   │           │   └── error_handler.rs
│   │           ├── external/
│   │           │   ├── swift.rs
│   │           │   ├── ctaf_goaml.rs
│   │           │   └── hsm.rs (PKCS#11)
│   │           └── config.rs
│   └── main.rs (entry point)
```

---

## 4. Invariants Métier Compilés (25+)

| ID | Invariant | Rust Implementation |
|---|---|---|
| **INV-01** | Compte ⟹ KYC validée | `Account::new()` vérifie `customer.kyc_status == KycStatus::Validated` |
| **INV-02** | Solvabilité ≥ 10% | `PrudentialRatio::new()` → `Result` si ratio < 10% |
| **INV-03** | Créance class ∈ {0,1,2,3,4} | `enum AssetClass` — exhaustive |
| **INV-04** | Provision min [2→20%, 3→50%, 4→100%] | `ProvisioningRules::calculate()` applique règles |
| **INV-05** | Opération ≥ 5k TND → AML check | `PaymentOrder::new()` déclenche screening |
| **INV-06** | Gel = immédiat, irrévocable | `AssetFreeze` une fois créé ne peut pas être modifié |
| **INV-07** | Écriture: débit = crédit | `JournalEntry::new()` vérifie sommes |
| **INV-08** | Opération → audit trail immutable | `AuditTrailEntry::record()` + hash chain |
| **INV-09** | Consentement INPDP requis | `ConsentManager::grant_consent()` prérequis |
| **INV-10** | PAN stocké UNIQUEMENT tokenisé | `Token` pas `String` dans entities |
| **INV-11** | Accès CDE = MFA 2 facteurs | `RbacValidator` vérifie `mfa_enabled` |
| **INV-12** | Violation data → INPDP 72h | `BreachNotification` lancée auto |
| **INV-13** | Arrangement = Account + Credit + Collateral + Insurance | `Arrangement::new()` références valides |
| **INV-14** | Virement intl → filtrage sanctions | `PaymentOrder::execute()` appelle `ScreeningResult` |
| **INV-15** | Travel rule > 1k EUR/USD | `TravelRuleValidator::validate()` mandatory |
| **INV-16** | ECL stage ∈ {1,2,3} | `enum NplStage` |
| **INV-17** | Arrangement limit ≥ 0, ≤ approved | `ArrangementLimit::new()` vérifie bounds |
| **INV-18** | Collateral LTV > 0 | `LTV::new()` valide |
| **INV-19** | LC status conforme UCP 600 | `enum LcStatus` — workflow enforced |
| **INV-20** | Sweep execution atomic | Transaction DB garantit atomicité |

---

## 5. API REST — Convention 550-700+ Endpoints

### 5.1 Versioning + Namespacing

```
BASE_URL = http://localhost:8080/api/v1

Pattern: /api/v1/{bounded_context}/{resource}/{id}/{action}

Examples:
  POST   /api/v1/customers                          (Create)
  GET    /api/v1/customers/{id}                     (Read)
  PUT    /api/v1/customers/{id}                     (Update)
  DELETE /api/v1/customers/{id}                     (Soft delete)
  GET    /api/v1/customers/{id}/accounts            (List related)
  POST   /api/v1/accounts/{id}/freeze               (Action)

Pagination:
  GET /api/v1/accounts?page=1&limit=50&sort=created_at:desc

Filtering:
  GET /api/v1/loans?status=ACTIVE&asset_class=2

Response:
  {
    "data": {...} | [{...}],
    "meta": {"page": 1, "limit": 50, "total": 1000},
    "errors": []
  }
```

### 5.2 Estimation Endpoints par BC

| BC | Resource Count | Estimated Endpoints |
|---|---|---|
| BC1 Customer | 8 | 35 |
| BC2 Account | 6 | 25 |
| BC3 Credit | 7 | 30 |
| BC4 AML | 6 | 25 |
| BC5 Sanctions | 4 | 15 |
| BC6 Prudential | 5 | 18 |
| BC7 Accounting | 5 | 20 |
| BC8 Reporting | 4 | 15 |
| BC9 Payment | 5 | 25 |
| BC10 ForeignExchange | 4 | 18 |
| BC11 Governance | 4 | 15 |
| BC12 Identity | 6 | 25 |
| BC13 Arrangement | 7 | 28 |
| BC14 Collateral | 5 | 20 |
| BC15 TradeFinance | 5 | 22 |
| BC16 CashManagement | 5 | 20 |
| BC17 IslamicBanking | 4 | 18 |
| BC18 DataHub | 4 | 15 |
| BC19 ReferenceData | 5 | 20 |
| BC20 Securities | 5 | 22 |
| BC21 Insurance | 4 | 18 |
| **TOTAL** | **~120** | **~550+** |

---

## 6. Event Sourcing + Audit Trail

### 6.1 DomainEvent Enum

```rust
pub enum DomainEvent {
    CustomerCreated(CustomerCreatedEvent),
    AccountOpened(AccountOpenedEvent),
    LoanGranted(LoanGrantedEvent),
    AmlAlertRaised(AmlAlertRaisedEvent),
    AssetFrozen(AssetFrozenEvent),
    TransactionScreened(TransactionScreenedEvent),
    PrudentialRatioCalculated(PrudentialRatioCalculatedEvent),
    JournalEntryPosted(JournalEntryPostedEvent),
    PaymentExecuted(PaymentExecutedEvent),
    ArrangementApproved(ArrangementApprovedEvent),
    CollateralValued(CollateralValuedEvent),
    LetterOfCreditIssued(LetterOfCreditIssuedEvent),
    // ... 22 BCs × 5-8 events = 110-176 domain events
}

pub struct CustomerCreatedEvent {
    pub id: CustomerId,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub created_by: UserId,
}
```

### 6.2 Event Bus

```rust
#[async_trait]
pub trait EventBus {
    async fn publish(&self, event: DomainEvent) -> Result<(), EventError>;
    async fn subscribe(&self, listener: Box<dyn EventListener>) -> Result<(), EventError>;
}

#[async_trait]
pub trait EventListener {
    async fn handle(&self, event: &DomainEvent) -> Result<(), EventError>;
}

// Implementations:
// - InMemoryEventBus (dev)
// - KafkaEventBus (prod) — pour événements distribués
```

### 6.3 Audit Trail Immutable (Hash Chain)

```rust
pub struct AuditTrailEntry {
    pub id: AuditEventId,
    pub entity_type: String,
    pub entity_id: String,
    pub action: String,           // "CREATE", "UPDATE", "DELETE"
    pub before: Option<serde_json::Value>,
    pub after: serde_json::Value,
    pub actor: UserId,
    pub timestamp: DateTime<Utc>,
    pub hash: String,             // SHA-256(before+after+actor+timestamp)
    pub previous_hash: Option<String>, // Chain link for immutability
}

// Verification:
pub fn verify_integrity(entry: &AuditTrailEntry) -> bool {
    let computed_hash = compute_sha256(&format!(
        "{:?}{:?}{}{}",
        entry.before, entry.after, entry.actor, entry.timestamp
    ));
    computed_hash == entry.hash
}
```

---

## 7. Security Architecture

### 7.1 Authentication Multi-Method

```
Layer 1: Username/Password
  ├─ bcrypt hashing (cost=12)
  └─ Rate limiting (5 tries → 15min lock)

Layer 2: JWT Token
  ├─ HS256 signature (secret from HSM)
  ├─ exp claim (30 min)
  └─ Refresh token (7 days, stored in Redis)

Layer 3: MFA (2FA/TOTP/SMS/FIDO2)
  ├─ TOTP (RFC 6238) — Time-based OTP
  ├─ SMS OTP — SMS provider
  └─ FIDO2/WebAuthn — Hardware key + biometric

Layer 4: CDE Access (PCI DSS 8.4.2)
  ├─ Mandatory MFA (2+ factors)
  ├─ Session timeout 15 min inactivity
  └─ IP whitelisting (CDE boundary)
```

### 7.2 Authorization (RBAC)

```rust
pub enum Role {
    SuperAdmin,
    ComplianceOfficer,
    RiskManager,
    Accountant,
    CustomerServiceRep,
    Auditor,
    // ... per BC roles
}

pub struct Permission {
    pub resource: String,  // "customer", "account", "loan", etc.
    pub action: String,    // "read", "write", "delete", "approve"
    pub conditions: Vec<Condition>, // E.g., "own_department_only"
}

pub async fn check_permission(
    user_id: UserId,
    resource: &str,
    action: &str,
) -> Result<bool, AuthError> {
    let user = user_repo.get(&user_id).await?;
    user.roles.iter().any(|role| {
        role.permissions.iter().any(|perm| {
            perm.resource == resource && perm.action == action
        })
    })
}
```

### 7.3 Encryption Strategy

```
At Rest:
  ├─ Database: LUKS AES-XTS-512 (volume)
  ├─ Sensitive fields: AES-256-GCM (column-level)
  │   ├─ PII (CIN, passport, address)
  │   ├─ Payment data (PAN → tokenized)
  │   └─ Credentials
  └─ Backups: GPG asymmetric (S3 off-site)

In Transit:
  ├─ TLS 1.3 (enforce minimum)
  ├─ HSTS (Strict-Transport-Security)
  └─ Certificate pinning (APIs critiques)

Keys Management:
  ├─ HSM (Hardware Security Module)
  │   ├─ Master keys (never exported)
  │   ├─ Signatures (PKCS#11)
  │   └─ Key rotation (annual)
  └─ Envelope encryption (DEK + KEK pattern)
```

### 7.4 Data Protection (INPDP Loi 2025)

```rust
pub struct PersonalDataField {
    pub value: String,               // Encrypted
    pub encrypted: bool,
    pub encryption_key_id: String,
    pub consent_scope: ConsentScope,
    pub retention_until: NaiveDate,
}

pub async fn encrypt_pii(&self, data: &str, field_type: PiiType) -> Result<String> {
    // AES-256-GCM with key from HSM
    let key = self.hsm.get_key(&format!("field:{:?}", field_type))?;
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(b"unique nonce");
    let ciphertext = cipher.encrypt(nonce, data.as_bytes())?;
    Ok(base64::encode(ciphertext))
}

pub async fn grant_consent(
    &self,
    customer_id: CustomerId,
    scope: ConsentScope,
) -> Result<()> {
    // Record explicitly
    consent_repo.create(ConsentRecord {
        customer_id,
        scope,
        granted_at: Utc::now(),
        granted_by: "customer_portal",
    }).await?;

    // Log for audit trail
    audit_trail.record(AuditTrailEntry {
        entity_type: "Consent",
        entity_id: customer_id.to_string(),
        action: "GRANT",
        actor: customer_id.to_string(),
        // ...
    }).await?;

    Ok(())
}

pub async fn revoke_consent(
    &self,
    customer_id: CustomerId,
    scope: ConsentScope,
) -> Result<()> {
    consent_repo.update_status(customer_id, scope, ConsentStatus::Revoked).await?;
    // Purge derivative data (if applicable)
    Ok(())
}
```

---

## 8. Infrastructure Architecture

### 8.1 Docker Compose (Development)

```yaml
version: '3.9'
services:
  postgres:
    image: postgres:16-alpine
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      POSTGRES_DB: banko
      POSTGRES_PASSWORD: dev_password
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

  traefik:
    image: traefik:v2.10
    command:
      - "--api.insecure=true"
      - "--providers.docker=true"
      - "--entrypoints.web.address=:80"
      - "--entrypoints.websecure.address=:443"
    ports:
      - "80:80"
      - "443:443"
      - "8081:8080"  # Dashboard

  minio:
    image: minio/minio:latest
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - minio_data:/minio_data

  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      GF_SECURITY_ADMIN_PASSWORD: admin

  loki:
    image: grafana/loki:latest
    ports:
      - "3100:3100"
```

### 8.2 Kubernetes (Production)

```yaml
# banko-backend.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: banko-backend
spec:
  replicas: 3  # High availability
  selector:
    matchLabels:
      app: banko-backend
  template:
    metadata:
      labels:
        app: banko-backend
    spec:
      containers:
      - name: banko
        image: banko/backend:v4.0.0
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: banko-secrets
              key: database_url
        - name: HSM_ENDPOINT
          value: "hsm.default.svc.cluster.local:5000"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          limits:
            cpu: "2"
            memory: "2Gi"
          requests:
            cpu: "1"
            memory: "1Gi"

---
apiVersion: v1
kind: Service
metadata:
  name: banko-backend
spec:
  selector:
    app: banko-backend
  ports:
  - protocol: TCP
    port: 8080
    targetPort: 8080
  type: ClusterIP

---
apiVersion: autoscaling.k8s.io/v2
kind: HorizontalPodAutoscaler
metadata:
  name: banko-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: banko-backend
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

### 8.3 Monitoring (Prometheus + Grafana)

```rust
// Metrics collection (Prometheus)
pub struct MetricsCollector {
    http_requests_total: IntCounterVec,
    http_request_duration_seconds: HistogramVec,
    database_connections_active: IntGauge,
    aml_alerts_total: IntCounterVec,
    loans_outstanding: Gauge,
}

impl MetricsCollector {
    pub async fn record_request(&self, method: &str, path: &str, duration_ms: u64) {
        self.http_requests_total
            .with_label_values(&[method, path])
            .inc();

        self.http_request_duration_seconds
            .with_label_values(&[method, path])
            .observe(duration_ms as f64 / 1000.0);
    }

    pub fn set_db_connections(&self, count: u32) {
        self.database_connections_active.set(count as i64);
    }
}

// Endpoint: /metrics (Prometheus scrapes)
#[get("/metrics")]
async fn metrics_handler(metrics: web::Data<MetricsCollector>) -> impl Responder {
    metrics.render() // Prometheus text format
}
```

### 8.4 CI/CD (GitHub Actions)

```yaml
name: BANKO CI/CD
on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: rust-lang/rust-toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings

  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_DB: banko_test
          POSTGRES_PASSWORD: password
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
    steps:
      - uses: actions/checkout@v3
      - uses: rust-lang/rust-toolchain@v1
      - run: cargo test --all

  security-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: rust-lang/rust-toolchain@v1
      - run: cargo install cargo-audit
      - run: cargo audit

  build:
    runs-on: ubuntu-latest
    needs: [lint, test, security-audit]
    steps:
      - uses: actions/checkout@v3
      - uses: docker/build-push-action@v4
        with:
          push: true
          tags: |
            banko/backend:latest
            banko/backend:${{ github.sha }}

  deploy:
    runs-on: ubuntu-latest
    needs: build
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v3
      - run: kubectl apply -f k8s/
```

---

## 9. Data Model (ER Diagramm Conceptuel)

```
CUSTOMER (BC1)
  ├─ PK: customer_id
  ├─ email, phone
  ├─ kyc_status, pep_status
  ├─ risk_score
  └─ consent_personal_data, consent_marketing

    ↓ 1:N
ACCOUNT (BC2)
  ├─ PK: account_id
  ├─ FK: customer_id
  ├─ arrangement_id (FK → BC13)
  ├─ iban, account_type
  ├─ balance, overdraft_limit
  └─ status

    ↓ 1:N
MOVEMENT (BC2)
  ├─ PK: movement_id
  ├─ FK: account_id
  ├─ amount, direction
  └─ executed_at

LOAN (BC3)
  ├─ PK: loan_id
  ├─ FK: customer_id, account_id
  ├─ arrangement_id (FK → BC13)
  ├─ principal_amount, interest_rate
  ├─ asset_class, npl_stage
  └─ status

    ↓ 1:N
LOAN_PROVISION (BC3)
  ├─ PK: provision_id
  ├─ FK: loan_id
  ├─ nct_provision_pct, nct_provision_amount
  ├─ ifrs9_ecl_amount, ecl_stage
  └─ pd, lgd, ead

ARRANGEMENT (BC13) — Central Hub
  ├─ PK: arrangement_id
  ├─ FK: customer_id, account_id, loan_id
  ├─ arrangement_type, status
  ├─ maturity_date
  └─ [conditions, limits]

    ↓ 1:N
ARRANGEMENT_CONDITION (BC13)
  ├─ PK: condition_id
  ├─ FK: arrangement_id
  └─ condition_type, value, enforcement

    ↓ 1:N
ARRANGEMENT_LIMIT (BC13)
  ├─ PK: limit_id
  ├─ FK: arrangement_id
  ├─ limit_type, limit_amount
  └─ utilization

COLLATERAL (BC14)
  ├─ PK: collateral_id
  ├─ FK: arrangement_id
  ├─ collateral_type, description
  ├─ status
  └─ [valuation, pledge]

    ↓ 1:N
COLLATERAL_VALUATION (BC14)
  ├─ PK: valuation_id
  ├─ FK: collateral_id
  ├─ valuation_date, gross_value
  ├─ haircut_pct, net_value
  └─ valuer_id

PAYMENT_ORDER (BC9)
  ├─ PK: payment_id
  ├─ FK: originator_account_id
  ├─ beneficiary_account, beneficiary_name
  ├─ amount, currency
  ├─ status, payment_type
  └─ created_at, executed_at

AML_TRANSACTION (BC4)
  ├─ PK: transaction_id
  ├─ FK: account_id
  ├─ amount, direction
  ├─ timestamp
  └─ counterparty

    ↓ 1:N
AML_ALERT (BC4)
  ├─ PK: alert_id
  ├─ FK: transaction_id
  ├─ alert_type, severity
  ├─ status
  └─ created_at

ASSET_FREEZE (BC4)
  ├─ PK: freeze_id
  ├─ FK: account_id
  ├─ reason, frozen_at, frozen_by
  └─ unfrozen_at, unfrozen_by

JOURNAL_ENTRY (BC7)
  ├─ PK: entry_id
  ├─ reference, accounting_period
  ├─ posted_at, posted_by
  └─ narrative

    ↓ 1:N
JOURNAL_DEBIT_LINE (BC7)
  ├─ PK: line_id
  ├─ FK: entry_id
  ├─ account_code, amount
  └─ (FK → CHART_OF_ACCOUNTS)

    ↓ 1:N
JOURNAL_CREDIT_LINE (BC7)
  ├─ PK: line_id
  ├─ FK: entry_id
  ├─ account_code, amount
  └─ (FK → CHART_OF_ACCOUNTS)

AUDIT_TRAIL_ENTRY (BC11)
  ├─ PK: audit_event_id
  ├─ entity_type, entity_id
  ├─ action, actor, timestamp
  ├─ before, after (JSON)
  ├─ hash, previous_hash
  └─ (Immutable — hash chain)

USER (BC12)
  ├─ PK: user_id
  ├─ username, email
  ├─ password_hash
  ├─ status, mfa_enabled
  ├─ mfa_method
  └─ created_at

    ↓ M:N
ROLE (BC12)
  ├─ PK: role_id
  ├─ name
  └─ [permissions]

    ↓ M:N
PERMISSION (BC12)
  ├─ PK: permission_id
  ├─ resource, action
  └─ conditions (JSON)

SESSION_TOKEN (BC12)
  ├─ PK: session_id
  ├─ FK: user_id
  ├─ jwt, refresh_token
  ├─ expires_at, created_at
  └─ (Cached in Redis)

CONSENT (Compliance Layer)
  ├─ PK: consent_id
  ├─ FK: customer_id
  ├─ scope (PERSONAL_DATA, MARKETING, THIRDPARTY)
  ├─ status (GRANTED, REVOKED, EXPIRED)
  ├─ granted_at, granted_by
  └─ expires_at
```

---

## 10. Performance Targets (SLA)

| Métrique | Target | Actual v4.0 | Note |
|---|---|---|---|
| API P99 latency (internal) | < 5ms | ~3-4ms | Actix-web + sqlx |
| API E2E latency (w/ DB persist) | < 200ms | ~80-150ms | Include PG write |
| Database connection pool size | 20-50 | 32 (config) | Per BC × 4 workers |
| Cache hit rate (Redis) | > 80% | 85%+ | Sessions, rates, ref data |
| Query response time (analytical) | < 2s | ~500-1500ms | Materialized views |
| Uptime SLA | 99.9% | Target v4.0 | 3x replicas + failover |
| RTO (Recovery Time) | < 1h | ~15 min | Kubernetes auto-healing |
| RPO (Recovery Point) | < 15 min | ~5 min | WAL archiving + backups |

---

## 11. Roadmap v4.0 — Jalons (31 semaines)

| Jalon | Durée | BCs | Endpoints | Status |
|---|---|---|---|---|
| **J0** | Semaines 1-6 | BC1, BC2, BC7, BC11, BC12, Compliance | 100+ | Fondations |
| **J1** | Semaines 7-14 | BC3, BC4, BC5, BC6, BC8, BC9, BC10, BC13 | 250+ | Core Banking |
| **J2** | Semaines 15-20 | BC14, BC15, BC16, BC17, BC19 | 400+ | Advanced |
| **J3** | Semaines 21-26 | BC18, BC20, BC21 | 550+ | Analytics + Securities |
| **J4** | Semaines 27-31 | Microservices, Open Banking, Hardening | 600+ | Maturité |
| **Hors scope v4.0** | — | — | — | Dérivés, hedge funds, blockchain |

---

## 12. Métriques de Succès v4.0

✓ **Couverture Temenos** : v4.0 MVP ~50% (350 endpoints), v4.1 ~70%, v4.2 85%+
✓ **Conformité BCT** : 100% P0 exigences
✓ **Coverage code** : 95%+ (domain + app layers)
✓ **BDD scenarios** : ≥400 gherkin (Cucumber)
✓ **Sécurité** : 0 vulnérabilités critiques (cargo audit)
✓ **Performance** : P99 < 5ms API interne
✓ **Audit trail** : 100% opérations immutables
✓ **ISO 27001:2022** : 93/93 contrôles mappés
✓ **PCI DSS v4.0.1** : 100% exigences obligatoires
✓ **Loi données 2025** : Conforme avant 11-07-2026
✓ **GAFI R.16** : Travel rule 100% effective
✓ **i18n** : AR (RTL) + FR + EN complets
✓ **Déploiements** : ≥2 banques tunisiennes live

---

---

## 13. Anti-Corruption Layer (ACL) — Systèmes externes (post-validation Phase F)

> **Warning résolu** : La validation Phase F identifiait l'absence d'ACL pour goAML, SWIFT, BVMT, Sanctions.

### Architecture ACL

```
Domain Layer (pure)          Application Layer            Infrastructure ACL
┌─────────────────┐    ┌──────────────────────┐    ┌──────────────────────────┐
│ SuspicionReport │←───│ SubmitDosUseCase     │←───│ GoAmlAdapter             │
│ (domain struct) │    │ (orchestration)      │    │ ├─ XML serialization     │
│                 │    │                      │    │ ├─ CTAF API client       │
│ PaymentOrder    │←───│ ExecuteTransferUC    │←───│ SwiftAdapter             │
│ (domain struct) │    │                      │    │ ├─ ISO 20022 parser      │
│                 │    │                      │    │ ├─ MT103/MT940 mapping   │
│ SanctionList    │←───│ ScreenSanctionsUC    │←───│ SanctionsListAdapter     │
│ (domain enum)   │    │                      │    │ ├─ OFAC CSV parser       │
│                 │    │                      │    │ ├─ EU XML parser         │
│ SecurityOrder   │←───│ ExecuteOrderUC       │←───│ BvmtAdapter              │
│ (domain struct) │    │                      │    │ ├─ Protocol TBD          │
└─────────────────┘    └──────────────────────┘    └──────────────────────────┘
```

### Ports ACL (traits)

```rust
// Port — le domaine définit ce qu'il attend
#[async_trait]
pub trait ExternalAmlSubmitter {
    async fn submit_sar(&self, report: &SuspicionReport) -> Result<SubmissionId, DomainError>;
    async fn get_status(&self, id: &SubmissionId) -> Result<SarStatus, DomainError>;
}

#[async_trait]
pub trait ExternalPaymentGateway {
    async fn send_swift(&self, order: &PaymentOrder) -> Result<SwiftReference, DomainError>;
    async fn parse_incoming(&self, raw: &[u8]) -> Result<PaymentOrder, DomainError>;
}

#[async_trait]
pub trait ExternalSanctionsProvider {
    async fn fetch_lists(&self) -> Result<Vec<SanctionEntry>, DomainError>;
    async fn last_update(&self) -> Result<DateTime<Utc>, DomainError>;
}

#[async_trait]
pub trait ExternalSecuritiesExchange {
    async fn submit_order(&self, order: &SecurityOrder) -> Result<OrderReference, DomainError>;
    async fn get_positions(&self, portfolio_id: &PortfolioId) -> Result<Vec<SecurityPosition>, DomainError>;
}
```

### Fichier cible
`crates/infrastructure/src/external/acl/mod.rs` avec sous-modules :
- `goaml_adapter.rs` — XML CTAF ↔ SuspicionReport
- `swift_adapter.rs` — ISO 20022 MT103/MT940 ↔ PaymentOrder
- `sanctions_adapter.rs` — CSV/XML OFAC/UE ↔ SanctionEntry
- `bvmt_adapter.rs` — Protocol BVMT ↔ SecurityOrder

---

## 14. Event Store — Audit Trail immutable (post-validation Phase F)

> **Warning résolu** : La validation Phase F identifiait l'absence de design Event Store.

### Décision : Event-sourced audit trail avec snapshots

```rust
/// Événement domaine générique — stocké immutablement
pub struct DomainEvent {
    pub id: EventId,
    pub aggregate_type: String,      // "Customer", "Account", "Loan"...
    pub aggregate_id: String,        // UUID de l'agrégat
    pub event_type: String,          // "CustomerCreated", "LoanDisbursed"...
    pub payload: serde_json::Value,  // Données événement sérialisées
    pub metadata: EventMetadata,
    pub created_at: DateTime<Utc>,
    pub hash: String,                // SHA256(previous_hash + payload)
    pub previous_hash: String,       // Chain linking
}

pub struct EventMetadata {
    pub user_id: UserId,
    pub ip_address: Option<String>,
    pub session_id: Option<SessionId>,
    pub correlation_id: Uuid,        // Pour tracer les chaînes de causalité
}
```

### Schema SQL Event Store

```sql
CREATE TABLE domain_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_type VARCHAR(64) NOT NULL,
    aggregate_id UUID NOT NULL,
    event_type VARCHAR(128) NOT NULL,
    payload JSONB NOT NULL,
    user_id UUID NOT NULL,
    ip_address INET,
    session_id UUID,
    correlation_id UUID NOT NULL,
    hash VARCHAR(64) NOT NULL,      -- SHA256 hex
    previous_hash VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Partitions mensuelles (performance + archivage)
CREATE TABLE domain_events_2026_04 PARTITION OF domain_events
    FOR VALUES FROM ('2026-04-01') TO ('2026-05-01');

-- Index pour requêtes audit
CREATE INDEX idx_events_aggregate ON domain_events (aggregate_type, aggregate_id);
CREATE INDEX idx_events_user ON domain_events (user_id);
CREATE INDEX idx_events_correlation ON domain_events (correlation_id);
CREATE INDEX idx_events_created ON domain_events (created_at);
```

### Politique rétention
- **Hot** (0-12 mois) : PostgreSQL primary, index complets
- **Warm** (12-36 mois) : PostgreSQL read-replica, index réduits
- **Cold** (36-120 mois) : Archive S3 chiffrée GPG, requêtes sur demande
- **Legal retention** : 10 ans minimum (Circ. 2025-17, Loi 2015-26)

### Port Event Store

```rust
#[async_trait]
pub trait EventStore {
    async fn append(&self, event: &DomainEvent) -> Result<(), DomainError>;
    async fn get_events(&self, aggregate_type: &str, aggregate_id: &Uuid) -> Result<Vec<DomainEvent>, DomainError>;
    async fn verify_chain(&self, aggregate_type: &str, aggregate_id: &Uuid) -> Result<bool, DomainError>;
}
```

---

## 15. HSM Interface — Signatures cryptographiques (post-validation Phase F)

> **Warning résolu** : La validation Phase F identifiait l'absence d'interface HSM.

### Port HSM

```rust
#[async_trait]
pub trait HsmSigner {
    /// Signer un message avec clé privée stockée HSM
    async fn sign(&self, key_id: &str, message: &[u8]) -> Result<Vec<u8>, HsmError>;
    /// Vérifier signature
    async fn verify(&self, key_id: &str, message: &[u8], signature: &[u8]) -> Result<bool, HsmError>;
    /// Générer nouvelle paire de clés dans HSM
    async fn generate_key_pair(&self, label: &str) -> Result<String, HsmError>;
}

pub enum HsmError {
    ConnectionFailed,
    KeyNotFound,
    SignatureFailed,
    Timeout,
}
```

### Implémentations

```rust
// Production : PKCS#11 (Thales Luna, SoftHSM2)
pub struct Pkcs11HsmSigner { /* ... */ }

// Développement : Mock HSM (signatures en mémoire, clés éphémères)
pub struct MockHsmSigner { /* ... */ }
```

### Fichier cible
`crates/infrastructure/src/security/hsm.rs`

---

## 16. Invariants compilés — Fichier centralisé (post-validation Phase F)

> **Warning résolu** : Invariants soft (SLA, timing) non compilés.

### Fichier cible : `crates/domain/src/invariants.rs`

```rust
use rust_decimal::Decimal;
use std::time::Duration;

/// INV-02/06 : Solvabilité minimum (FPN/RWA ≥ 10%)
pub const SOLVENCY_RATIO_MIN: Decimal = Decimal::from_parts(10, 0, 0, false, 0); // 10%
pub const SOLVENCY_ALERT_THRESHOLD: Decimal = Decimal::from_parts(12, 0, 0, false, 0); // 12%

/// INV-03/07 : Tier 1 minimum (CET1+AT1 ≥ 7%)
pub const TIER1_RATIO_MIN: Decimal = Decimal::from_parts(7, 0, 0, false, 0);
pub const TIER1_ALERT_THRESHOLD: Decimal = Decimal::from_parts(8, 0, 0, false, 0);

/// INV-04/08 : C/D plafond (≤ 120%)
pub const CD_RATIO_MAX: Decimal = Decimal::from_parts(120, 0, 0, false, 0);
pub const CD_ALERT_THRESHOLD: Decimal = Decimal::from_parts(110, 0, 0, false, 0);

/// INV-05 : Concentration (≤ 25% FPN)
pub const CONCENTRATION_MAX_PCT: Decimal = Decimal::from_parts(25, 0, 0, false, 0);

/// INV-09 : Provisions obligatoires par classe créance
pub fn minimum_provision_rate(asset_class: u8) -> Decimal {
    match asset_class {
        0 => Decimal::ZERO,
        1 => Decimal::from_parts(20, 0, 0, false, 0),
        2 => Decimal::from_parts(50, 0, 0, false, 0),
        3 => Decimal::from_parts(75, 0, 0, false, 0),
        4 => Decimal::from_parts(100, 0, 0, false, 0),
        _ => Decimal::from_parts(100, 0, 0, false, 0),
    }
}

/// INV-08 : AML seuil espèces
pub const AML_CASH_THRESHOLD_TND: Decimal = Decimal::from_parts(5000, 0, 0, false, 0);

/// INV-10 : Rétention données KYC post-clôture
pub const KYC_RETENTION_YEARS: u32 = 10;

/// INV-15/17 : DOS CTAF SLA
pub const DOS_SUBMISSION_SLA: Duration = Duration::from_secs(24 * 3600); // 24h

/// INV-17 : Travel rule seuil
pub const TRAVEL_RULE_THRESHOLD_TND: Decimal = Decimal::from_parts(250_000, 0, 0, false, 0);

/// INV-24 : Breach notification INPDP SLA
pub const BREACH_NOTIFICATION_SLA: Duration = Duration::from_secs(72 * 3600); // 72h

/// INV-25 : Portabilité données SLA
pub const DATA_PORTABILITY_SLA_DAYS: u32 = 30;
```

---

**Architecture v4.0.1 — 7 avril 2026 — BANKO Core Banking System (itéré post-validation Phase F)**
