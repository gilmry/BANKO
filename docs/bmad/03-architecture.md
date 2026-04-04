# Architecture Technique — BANKO

## Méthode Maury — Phase TOGAF C-D (SI + Technique)

**Disciplines** : SOLID + DDD + Hexagonal + BDD + TDD
**Version** : 1.0.0 — 4 avril 2026
**Stack** : Rust + Actix-web 4.9 + PostgreSQL 16 + Astro 6 + Svelte 5

---

## 1. Vue d'ensemble (diagramme ASCII)

```
┌─────────────────────────────────────────────────────────────────────┐
│                      BANKO — Core Banking System                     │
└─────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────┐         ┌─────────────────────────┐
│       Frontend (Astro + Svelte)   │         │    Mobile (Svelte Native)│
│  ┌─ Pages / Composants AR/FR/EN  │         │     (future)            │
│  ├─ Stores (Svelte)              │         └─────────────────────────┘
│  ├─ API Client (fetch + svelteKit)
│  └─ i18n Router (RTL support)    │
└──────────────────────────────────┘
           ↑ HTTP/REST
           │
    ┌──────┴───────────────────────────────────────────────────┐
    │                  API Gateway (Traefik)                   │
    │            Rate limiting, CORS, TLS termination         │
    └──────┬───────────────────────────────────────────────────┘
           │
┌──────────┴──────────────────────────────────────────────────────────┐
│              BACKEND — Rust + Actix-web (Hexagonal)               │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  HTTP Handlers / Routes (Adapter) — 12 BC modules         │   │
│  ├─────────────────────────────────────────────────────────────┤   │
│  │  ┌──────────┬──────────┬──────────┬──────────┬──────────┐  │   │
│  │  │Customer  │ Account  │ Credit   │   AML    │Sanctions │  │   │
│  │  ├──────────┼──────────┼──────────┼──────────┼──────────┤  │   │
│  │  │Prudential│Accounting│ Reporting│ Payment  │ForeignEx │  │   │
│  │  ├──────────┼──────────┼──────────┼──────────┼──────────┤  │   │
│  │  │Governance│ Identity │  Audit   │ [...]    │  [...]   │  │   │
│  │  └──────────┴──────────┴──────────┴──────────┴──────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              ↓                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │        Application Layer (DTOs, UseCases, Ports)            │   │
│  │  ┌─────────────────────────────────────────────────────────┐│   │
│  │  │ DTOs (Request/Response) — Serde serialization          ││   │
│  │  │ Ports (traits) → Database, Email, SMS, HSM, Storage    ││   │
│  │  │ Use Cases (interactors) → Business logic orchestration ││   │
│  │  └─────────────────────────────────────────────────────────┘│   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              ↓                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │              Domain Layer (Pure Rust — no_std ready)         │   │
│  │  ┌─────────────────────────────────────────────────────────┐│   │
│  │  │ Entities (Customer, Account, Loan, Transaction...)     ││   │
│  │  │ Value Objects (Money, IBAN, KycProfile, ClassCode...) ││   │
│  │  │ Aggregates (AccountAggregate, CreditAggregate...)      ││   │
│  │  │ Domain Errors (DomainError enum)                       ││   │
│  │  │ Invariants → Compile-time enforcement                  ││   │
│  │  └─────────────────────────────────────────────────────────┘│   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              ↓                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Infrastructure (Database, Cache, External Services)        │   │
│  │  ┌─────────────────────────────────────────────────────────┐│   │
│  │  │ PostgreSQL Repository (SQLx async)                      ││   │
│  │  │ Redis Cache (distributed sessions, OTP)                 ││   │
│  │  │ HSM Interface (PKCS#11 for crypto signing)              ││   │
│  │  │ Email/SMS Sender (notification service)                 ││   │
│  │  │ Audit Trail Logger (immutable event store)              ││   │
│  │  │ File Storage (S3-compatible, local dev)                 ││   │
│  │  └─────────────────────────────────────────────────────────┘│   │
│  └─────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────┐
│                    PostgreSQL 16 (Primary DB)                        │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │ Schemas: customer, account, credit, aml, sanctions,           │  │
│  │          prudential, accounting, reporting, payment,          │  │
│  │          fx, governance, audit, identity                      │  │
│  ├────────────────────────────────────────────────────────────────┤  │
│  │ Features: ACID, partitioning by date/customer, FDW,           │  │
│  │           LUKS encryption, logical replication, backups       │  │
│  └────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────┐
│                    External Integrations                             │
│  ├─ SWIFT / ISO 20022 (Payment networks)                           │
│  ├─ Sanctions lists (ONU, UE, OFAC, national)                     │
│  ├─ Email / SMS providers (notification)                          │
│  ├─ HSM (Hardware Security Module for signatures)                 │
│  ├─ Monitoring (Prometheus, Loki, Alertmanager)                  │
│  └─ IaC (Terraform, Ansible, Helm)                               │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 2. Bounded Contexts → Modules code

| # | Bounded Context | Module code | Responsabilité | Agrégats | Value Objects |
|---|---|---|---|---|---|
| **BC1** | **Customer** | `src/customer/` | Clients, KYC, bénéficiaires effectifs, PEP, scoring risque | `CustomerAggregate` | `CustomerId`, `Iban`, `KycProfile`, `RiskScore`, `BeneficiaryInfo` |
| **BC2** | **Account** | `src/account/` | Comptes (courant, épargne, DAT), soldes, mouvements | `AccountAggregate` | `AccountId`, `Balance`, `AccountType`, `CurrencyCode` |
| **BC3** | **Credit** | `src/credit/` | Crédits, classification créances, provisionnement, remboursement | `LoanAggregate` | `LoanId`, `AssetClass`, `Provision`, `LoanSchedule` |
| **BC4** | **AML** | `src/aml/` | Surveillance transactionnelle, alertes, investigations, DOS, gel | `AlertAggregate` | `TransactionId`, `AlertType`, `SuspicionReport`, `InvestigationStatus` |
| **BC5** | **Sanctions** | `src/sanctions/` | Filtrage listes sanctions (ONU, UE, OFAC, nationales) | `ScreeningAggregate` | `SanctionListId`, `ScreeningResult`, `EntityName`, `MatchScore` |
| **BC6** | **Prudential** | `src/prudential/` | Ratios réglementaires (solvabilité, Tier 1, C/D, concentration) | `PrudentialRatioAggregate` | `RatioType`, `RiskWeightedAsset`, `RegulatoryCapital` |
| **BC7** | **Accounting** | `src/accounting/` | Comptabilité NCT, écritures, journal, grand livre, balance | `JournalAggregate` | `AccountingCode`, `JournalEntry`, `LedgerAccount`, `Period` |
| **BC8** | **Reporting** | `src/reporting/` | États réglementaires BCT, rapports prudentiels, reporting AML | `ReportAggregate` | `ReportId`, `ReportTemplate`, `SubmissionStatus` |
| **BC9** | **Payment** | `src/payment/` | Virements, compensation, SWIFT, ISO 20022 | `PaymentOrderAggregate` | `PaymentId`, `SwiftMessage`, `ClearingReference` |
| **BC10** | **ForeignExchange** | `src/fx/` | Opérations change, position change, conformité Loi 76-18 | `FxOperationAggregate` | `FxOperationId`, `ExchangeRate`, `FxPosition` |
| **BC11** | **Governance** | `src/governance/` | Contrôle interne, 3 lignes, comités, piste d'audit | `AuditTrailAggregate` | `AuditEventId`, `AuditTrail`, `ComplianceReport` |
| **BC12** | **Identity** | `src/identity/` | Authentification, RBAC, sessions, 2FA, JWT | `UserAggregate` | `UserId`, `Role`, `Permission`, `SessionToken` |

---

## 3. Architecture hexagonale SOLID

### 3.1 Couche Domain

```
src/domain/
├── customer/
│   ├── mod.rs
│   ├── entities.rs          # Customer, KycProfile, Beneficiary
│   ├── value_objects.rs     # CustomerId, Iban, RiskScore, PepStatus
│   ├── aggregates.rs        # CustomerAggregate
│   ├── errors.rs            # DomainError::Customer::*
│   └── rules.rs             # KycValidationRules, PepCheckRules
├── account/
│   ├── entities.rs          # Account, Balance, Movement
│   ├── value_objects.rs     # AccountId, Balance, AccountType
│   ├── aggregates.rs        # AccountAggregate
│   ├── errors.rs            # DomainError::Account::*
│   └── rules.rs             # AccountOpeningRules, BalanceRules
├── credit/
│   ├── entities.rs          # Loan, LoanSchedule, Provision
│   ├── value_objects.rs     # LoanId, AssetClass, Provision%
│   ├── aggregates.rs        # LoanAggregate
│   ├── errors.rs            # DomainError::Credit::*
│   └── rules.rs             # ClassificationRules, ProvisioningRules
├── aml/
│   ├── entities.rs          # Transaction, Alert, Investigation
│   ├── value_objects.rs     # TransactionId, AlertType, SuspicionReport
│   ├── aggregates.rs        # AlertAggregate
│   ├── errors.rs            # DomainError::Aml::*
│   └── rules.rs             # AmlScenarios, FreezeRules
├── sanctions/
│   ├── entities.rs          # SanctionList, SanctionEntry, ScreeningResult
│   ├── value_objects.rs     # SanctionListId, MatchScore, EntityName
│   ├── aggregates.rs        # ScreeningAggregate
│   ├── errors.rs            # DomainError::Sanctions::*
│   └── rules.rs             # ScreeningRules, MatchingAlgorithms
├── prudential/
│   ├── entities.rs          # PrudentialRatio, RiskWeightedAsset
│   ├── value_objects.rs     # RatioType, RegulatoryCapital
│   ├── aggregates.rs        # PrudentialRatioAggregate
│   ├── errors.rs            # DomainError::Prudential::*
│   └── rules.rs             # SolvencyRules, TierRules, CDRatioRules
├── accounting/
│   ├── entities.rs          # JournalEntry, Ledger, ChartOfAccounts
│   ├── value_objects.rs     # AccountingCode, Period, EntryAmount
│   ├── aggregates.rs        # JournalAggregate
│   ├── errors.rs            # DomainError::Accounting::*
│   └── rules.rs             # DoubleEntryRules, BalanceRules
├── reporting/
│   ├── entities.rs          # RegulatoryReport, ReportTemplate
│   ├── value_objects.rs     # ReportId, SubmissionStatus
│   ├── aggregates.rs        # ReportAggregate
│   ├── errors.rs            # DomainError::Reporting::*
│   └── rules.rs             # ReportingRules, BctRequirements
├── payment/
│   ├── entities.rs          # PaymentOrder, Transfer, SwiftMessage
│   ├── value_objects.rs     # PaymentId, ClearingReference
│   ├── aggregates.rs        # PaymentOrderAggregate
│   ├── errors.rs            # DomainError::Payment::*
│   └── rules.rs             # PaymentValidationRules, SwiftRules
├── fx/
│   ├── entities.rs          # FxOperation, ExchangeRate
│   ├── value_objects.rs     # FxOperationId, FxPosition
│   ├── aggregates.rs        # FxOperationAggregate
│   ├── errors.rs            # DomainError::Fx::*
│   └── rules.rs             # FxComplianceRules, PositionLimits
├── governance/
│   ├── entities.rs          # AuditTrail, Committee, ControlCheck
│   ├── value_objects.rs     # AuditEventId, AuditTrailEntry
│   ├── aggregates.rs        # AuditTrailAggregate
│   ├── errors.rs            # DomainError::Governance::*
│   └── rules.rs             # AuditRules, ControlRules
└── identity/
    ├── entities.rs          # User, Role, Permission
    ├── value_objects.rs     # UserId, SessionToken, PasswordHash
    ├── aggregates.rs        # UserAggregate
    ├── errors.rs            # DomainError::Identity::*
    └── rules.rs             # AuthenticationRules, RbacRules
```

#### Key Domain Principles (SOLID)

- **S** (Single Responsibility) : Chaque entité = une seule raison de changer
- **O** (Open/Closed) : Domain extensible via traits, fermé à la modification
- **L** (Liskov) : Substitutabilité des value objects
- **I** (Interface Segregation) : Ports spécialisés par BC
- **D** (Dependency Inversion) : Domain n'import aucune dépendance externe

### 3.2 Couche Application

```
src/application/
├── customer/
│   ├── dto.rs               # CreateCustomerRequest, CustomerResponse
│   ├── use_cases.rs         # CreateCustomerUseCase, UpdateKycUseCase
│   ├── ports.rs             # CustomerRepository (trait), KycValidator (trait)
│   └── errors.rs            # ApplicationError::Customer::*
├── account/
│   ├── dto.rs               # OpenAccountRequest, AccountResponse
│   ├── use_cases.rs         # OpenAccountUseCase, TransferUseCase
│   ├── ports.rs             # AccountRepository, BalanceCalculator
│   └── errors.rs            # ApplicationError::Account::*
├── credit/
│   ├── dto.rs               # GrantLoanRequest, LoanResponse
│   ├── use_cases.rs         # GrantLoanUseCase, ClassifyLoanUseCase
│   ├── ports.rs             # LoanRepository, AssetClassifier
│   └── errors.rs            # ApplicationError::Credit::*
├── aml/
│   ├── dto.rs               # CreateAlertRequest, AlertResponse
│   ├── use_cases.rs         # DetectAnomalyUseCase, InvestigateAlertUseCase
│   ├── ports.rs             # AmlScenarioEngine, AlertRepository
│   └── errors.rs            # ApplicationError::Aml::*
├── sanctions/
│   ├── dto.rs               # ScreenEntityRequest, ScreeningResponse
│   ├── use_cases.rs         # ScreenEntityUseCase, UpdateSanctionsListUseCase
│   ├── ports.rs             # SanctionListRepository, MatchingEngine
│   └── errors.rs            # ApplicationError::Sanctions::*
├── prudential/
│   ├── dto.rs               # CalculateRatioRequest, PrudentialResponse
│   ├── use_cases.rs         # CalculateRatiosUseCase, CheckSolvencyUseCase
│   ├── ports.rs             # PrudentialRepository, RwaCalculator
│   └── errors.rs            # ApplicationError::Prudential::*
├── accounting/
│   ├── dto.rs               # PostEntryRequest, TrialBalanceResponse
│   ├── use_cases.rs         # PostJournalEntryUseCase, GenerateBalanceUseCase
│   ├── ports.rs             # JournalRepository, LedgerCalculator
│   └── errors.rs            # ApplicationError::Accounting::*
├── reporting/
│   ├── dto.rs               # GenerateReportRequest, ReportResponse
│   ├── use_cases.rs         # GenerateBctReportUseCase, SubmitReportUseCase
│   ├── ports.rs             # ReportRepository, BctSubmissionClient
│   └── errors.rs            # ApplicationError::Reporting::*
├── payment/
│   ├── dto.rs               # InitiatePaymentRequest, PaymentResponse
│   ├── use_cases.rs         # InitiatePaymentUseCase, ExecutePaymentUseCase
│   ├── ports.rs             # PaymentRepository, SwiftClient
│   └── errors.rs            # ApplicationError::Payment::*
├── fx/
│   ├── dto.rs               # ExecuteFxRequest, FxResponse
│   ├── use_cases.rs         # ExecuteFxUseCase, CalculateFxPositionUseCase
│   ├── ports.rs             # FxRepository, RateProvider
│   └── errors.rs            # ApplicationError::Fx::*
├── governance/
│   ├── dto.rs               # LogAuditEventRequest, AuditResponse
│   ├── use_cases.rs         # LogAuditEventUseCase, GenerateAuditReportUseCase
│   ├── ports.rs             # AuditRepository, AuditLogger
│   └── errors.rs            # ApplicationError::Governance::*
└── identity/
    ├── dto.rs               # RegisterUserRequest, LoginRequest, UserResponse
    ├── use_cases.rs         # RegisterUserUseCase, AuthenticateUseCase
    ├── ports.rs             # UserRepository, PasswordHasher, TokenGenerator
    └── errors.rs            # ApplicationError::Identity::*
```

### 3.3 Couche Infrastructure

```
src/infrastructure/
├── persistence/
│   ├── postgres/
│   │   ├── customer_repository.rs    # impl CustomerRepository
│   │   ├── account_repository.rs     # impl AccountRepository
│   │   ├── credit_repository.rs      # impl LoanRepository
│   │   ├── aml_repository.rs         # impl AlertRepository
│   │   ├── sanctions_repository.rs   # impl SanctionListRepository
│   │   ├── prudential_repository.rs  # impl PrudentialRepository
│   │   ├── accounting_repository.rs  # impl JournalRepository
│   │   ├── reporting_repository.rs   # impl ReportRepository
│   │   ├── payment_repository.rs     # impl PaymentRepository
│   │   ├── fx_repository.rs          # impl FxRepository
│   │   ├── governance_repository.rs  # impl AuditRepository
│   │   ├── identity_repository.rs    # impl UserRepository
│   │   ├── connection.rs             # PgPool, migrations
│   │   └── queries.rs                # SQL compiled queries (sqlx macros)
│   ├── redis/
│   │   ├── session_cache.rs          # Redis session store
│   │   ├── otp_cache.rs              # OTP temporary storage
│   │   └── rate_limiter.rs           # DDoS protection
│   └── migrations/
│       ├── 0001_initial_schema.sql   # All 12 BC tables
│       ├── 0002_audit_trail.sql
│       ├── 0003_indexes.sql
│       └── [...]
├── external/
│   ├── hsm/
│   │   ├── pkcs11_client.rs          # HSM interface (cryptographic signing)
│   │   └── key_management.rs         # Key rotation, storage
│   ├── notifications/
│   │   ├── email_sender.rs           # SMTP integration
│   │   └── sms_sender.rs             # SMS provider
│   ├── sanctions/
│   │   ├── un_list_provider.rs       # UN Consolidated List API
│   │   ├── eu_list_provider.rs       # EU Sanctions List
│   │   ├── ofac_list_provider.rs     # OFAC SDN List
│   │   └── update_scheduler.rs       # Periodic updates
│   ├── swift/
│   │   ├── swift_client.rs           # SWIFT network interface
│   │   └── iso20022_parser.rs        # ISO 20022 message parsing
│   ├── storage/
│   │   ├── s3_client.rs              # S3-compatible (backup, KYC docs)
│   │   └── local_storage.rs          # Local file storage (dev)
│   └── monitoring/
│       ├── prometheus_exporter.rs    # Metrics export
│       ├── loki_logger.rs            # Structured logging
│       └── jaeger_tracer.rs          # Distributed tracing
├── http/
│   ├── routes.rs                     # Actix-web route registration
│   ├── handlers/
│   │   ├── customer_handler.rs       # HTTP handlers for Customer BC
│   │   ├── account_handler.rs        # HTTP handlers for Account BC
│   │   ├── credit_handler.rs         # HTTP handlers for Credit BC
│   │   ├── aml_handler.rs            # HTTP handlers for AML BC
│   │   ├── sanctions_handler.rs      # HTTP handlers for Sanctions BC
│   │   ├── prudential_handler.rs     # HTTP handlers for Prudential BC
│   │   ├── accounting_handler.rs     # HTTP handlers for Accounting BC
│   │   ├── reporting_handler.rs      # HTTP handlers for Reporting BC
│   │   ├── payment_handler.rs        # HTTP handlers for Payment BC
│   │   ├── fx_handler.rs             # HTTP handlers for FX BC
│   │   ├── governance_handler.rs     # HTTP handlers for Governance BC
│   │   └── identity_handler.rs       # HTTP handlers for Identity BC
│   ├── middleware/
│   │   ├── auth_middleware.rs        # JWT verification
│   │   ├── audit_middleware.rs       # Request/response logging
│   │   ├── rate_limit_middleware.rs  # Rate limiting
│   │   └── error_handler.rs          # Global error handling
│   └── error_responses.rs            # HTTP error serialization
├── config.rs                         # Configuration management (env vars, YAML)
└── lib.rs                            # Infrastructure module root
```

---

## 4. Glossaire DDD → Mapping Code

| Terme métier | Définition | Type code | Nom exact code | Bounded Context |
|---|---|---|---|---|
| **Compte** | Instrument de dépôt/crédit identifié par RIB | `struct` | `Account` | BC2 |
| **Client** | Personne physique ou morale titulaire d'un compte | `struct` | `Customer` | BC1 |
| **Fiche KYC** | Document structuré de connaissance du client | `struct` | `KycProfile` | BC1 |
| **Bénéficiaire effectif** | Personne physique possédant/contrôlant ≥25% | `struct` | `BeneficiaryInfo` | BC1 |
| **PEP** | Personne Politiquement Exposée | `enum` | `PepStatus` | BC1 |
| **Créance** | Engagement de crédit de la banque | `struct` | `Loan` | BC3 |
| **Classe de créance** | Classification 0-4 (courant → compromis) | `enum` | `AssetClass` | BC3 |
| **Provision** | Montant pour couvrir risque de perte | `struct` | `Provision` | BC3 |
| **ECL** | Expected Credit Loss (IFRS 9) | `struct` | `ExpectedCreditLoss` | BC3 |
| **Ratio de solvabilité** | FP réglementaires / RWA (min 10%) | `struct` | `SolvencyRatio` | BC6 |
| **Tier 1** | Fonds propres de base (min 7%) | `struct` | `Tier1Ratio` | BC6 |
| **Ratio C/D** | Crédits / Dépôts (max 120%) | `struct` | `CreditToDepositRatio` | BC6 |
| **Ratio de concentration** | Risque même bénéficiaire / FPN (max 25%) | `struct` | `ConcentrationRatio` | BC6 |
| **Déclaration de soupçon** | Signalement à CTAF d'opération suspecte | `struct` | `SuspicionReport` | BC4 |
| **Gel des avoirs** | Blocage fonds personne/entité sanctions | `struct` | `AssetFreeze` | BC4 |
| **Piste d'audit** | Enregistrement chronologique immutable | `struct` | `AuditTrail` | BC11 |
| **Écriture comptable** | Enregistrement journal selon plan NCT | `struct` | `JournalEntry` | BC7 |
| **Virement SWIFT** | Transfert international ISO 20022 | `struct` | `SwiftMessage` | BC9 |
| **RIB** | Relevé d'Identité Bancaire — identifiant compte | `struct` | `Iban` | BC2 |
| **DAT** | Dépôt À Terme — placement durée fixe | `enum` | `AccountType::TermDeposit` | BC2 |
| **FPN** | Fonds Propres Nets — base ratios | `struct` | `RegulatoryCapital` | BC6 |
| **RWA** | Risk-Weighted Assets — actifs pondérés | `struct` | `RiskWeightedAsset` | BC6 |
| **PNB** | Produit Net Bancaire — marge | `struct` | `ProfitAndLoss` | BC7 |
| **NCT** | Norme Comptable Tunisienne | `struct` | `AccountingCode` | BC7 |
| **Alerte AML** | Détection opération suspecte | `struct` | `Alert` | BC4 |
| **Investigation** | Processus d'examen alerte | `struct` | `Investigation` | BC4 |
| **Liste de sanctions** | UN, UE, OFAC, nationales | `struct` | `SanctionList` | BC5 |
| **Score de correspondance** | Confiance match entité/sanctions | `struct` | `MatchScore` | BC5 |
| **Opération change** | Achat/vente devises | `struct` | `FxOperation` | BC10 |
| **Position change** | Exposition nette devise | `struct` | `FxPosition` | BC10 |
| **Ordre de paiement** | Instruction virement/compensation | `struct` | `PaymentOrder` | BC9 |
| **Clearing** | Compensation multilatérale | `struct` | `ClearingReference` | BC9 |
| **Rapport réglementaire** | État prudentiel/AML/financier BCT | `struct` | `RegulatoryReport` | BC8 |
| **Utilisateur** | Agent bancaire, administrateur, auditeur | `struct` | `User` | BC12 |
| **Rôle** | RBAC (Role-Based Access Control) | `enum` | `Role` | BC12 |
| **Permission** | Action autorisée par rôle | `enum` | `Permission` | BC12 |
| **Session** | JWT token avec contexte utilisateur | `struct` | `SessionToken` | BC12 |

---

## 5. Stratégie de tests (TDD + BDD + Documentation Vivante)

### Pyramide de tests

```
                        ▲
                       ╱ ╲
                      ╱ E2E╲        (10% du code)
                     ╱ API ╲        Playwright, API client
                    ╱-------╲
                   ╱ Integration╲   (30% du code)
                  ╱ (DB + Cache)╲   SQLx + test DB
                 ╱───────────────╲
                ╱   Unit (BDD)    ╲  (60% du code)
               ╱   Domain + App   ╲  Cucumber + Gherkin
              ╱───────────────────╲
             ╱   Compile-time      ╲ Rust type system
```

### Couverture cible par couche

| Couche | Couverture | Stratégie |
|---|---|---|
| **Domain** | 100% | TDD strict — write test before entity |
| **Application (UseCases)** | 100% | BDD + Gherkin — spécifications vivantes |
| **Infrastructure (Repositories)** | 95% | Integration tests avec test DB |
| **HTTP (Handlers)** | 85% | Integration + E2E tests |
| **Middleware** | 90% | Unit tests |
| **Global** | ≥ 80% | Tarpaulin coverage check in CI |

### Exemple structure tests

```
src/
├── customer/
│   ├── tests.rs                  # Tests unitaires domain
│   │   ├── test_kyc_validation.rs
│   │   ├── test_pep_detection.rs
│   │   └── test_customer_aggregate.rs
│   └── [...]
tests/
├── bdd/
│   ├── features/
│   │   ├── customer.feature      # Gherkin scenarios
│   │   ├── account.feature
│   │   ├── credit.feature
│   │   ├── aml.feature
│   │   ├── sanctions.feature
│   │   ├── prudential.feature
│   │   ├── accounting.feature
│   │   ├── reporting.feature
│   │   ├── payment.feature
│   │   ├── fx.feature
│   │   ├── governance.feature
│   │   └── identity.feature
│   └── steps/
│       ├── customer_steps.rs     # Cucumber step definitions
│       ├── account_steps.rs
│       ├── [...]
│       └── common_steps.rs       # Shared context
├── integration/
│   ├── customer_repo_test.rs     # Repository tests
│   ├── account_repo_test.rs
│   └── [...]
└── e2e/
    ├── customer_workflow_test.rs # Multi-step scenarios
    ├── account_workflow_test.rs
    └── [...]
```

### Test-Driven Emergence (BDD ↔ E2E ↔ Vidéo)

1. **Spécification BDD en Gherkin** → Acceptation métier
2. **Steps Cucumber** → Implémentation détaillée
3. **Unit tests (TDD)** → Code domain protection
4. **E2E tests (Playwright/API)** → Flux complets
5. **Vidéo documentation** → Démonstration en temps réel

---

## 6. Modèle de données (DDD → PostgreSQL)

### Schema: customer

```sql
-- Clients (PP/PM)
CREATE TABLE customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_type VARCHAR(2) NOT NULL CHECK (customer_type IN ('PP', 'PM')), -- Personne Physique ou Morale
    name VARCHAR(255) NOT NULL,
    legal_form VARCHAR(50),  -- SARL, EIRL, SA, etc.
    registration_number VARCHAR(50) UNIQUE,  -- CIN, RCCM
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP
);

-- Fiches KYC (Connaissance du Client)
CREATE TABLE kyc_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    identity_document_type VARCHAR(20) NOT NULL, -- CIN, PASSPORT, RES_PERMIT
    identity_document_number VARCHAR(50) NOT NULL,
    identity_verified_at TIMESTAMP,
    profession VARCHAR(100),
    revenue_annual DECIMAL(15, 2),
    revenue_currency VARCHAR(3) DEFAULT 'TND',
    pep_status VARCHAR(20) DEFAULT 'NOT_PEP', -- NOT_PEP, DOMESTIC, FOREIGN, CLOSE_RELATIVE
    pep_verified_at TIMESTAMP,
    edd_required BOOLEAN DEFAULT FALSE,  -- Enhanced Due Diligence
    edd_completed_at TIMESTAMP,
    risk_score SMALLINT, -- 1-5 scale
    approved_at TIMESTAMP,
    approved_by UUID,  -- User who approved
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Bénéficiaires effectifs (PM seulement)
CREATE TABLE beneficial_owners (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kyc_profile_id UUID NOT NULL REFERENCES kyc_profiles(id),
    full_name VARCHAR(255) NOT NULL,
    ownership_stake DECIMAL(5, 2) NOT NULL CHECK (ownership_stake >= 0 AND ownership_stake <= 100),
    control_of_fact BOOLEAN DEFAULT FALSE,
    relationship VARCHAR(100),  -- Actionnaire, Administrateur, etc.
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_owner_per_kyc UNIQUE (kyc_profile_id, full_name)
);

-- Consentements données (INPDP Loi 2004-63)
CREATE TABLE data_consents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    consent_type VARCHAR(50) NOT NULL, -- PERSONAL_DATA, MARKETING, CREDIT_BUREAU
    granted BOOLEAN NOT NULL,
    granted_at TIMESTAMP,
    revoked_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: account

```sql
-- Comptes
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    iban VARCHAR(34) NOT NULL UNIQUE,
    account_type VARCHAR(20) NOT NULL, -- CURRENT, SAVINGS, TERM_DEPOSIT
    currency VARCHAR(3) DEFAULT 'TND',
    balance_debit DECIMAL(18, 3) DEFAULT 0,  -- Solde débiteur (négatif)
    balance_credit DECIMAL(18, 3) DEFAULT 0,  -- Solde créditeur (positif)
    interest_rate DECIMAL(5, 3),  -- Pour comptes épargne/DAT
    opening_date DATE NOT NULL,
    closing_date DATE,
    status VARCHAR(20) DEFAULT 'ACTIVE', -- ACTIVE, DORMANT, CLOSED
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT kyc_required CHECK (
        (SELECT approved_at FROM kyc_profiles WHERE customer_id = accounts.customer_id) IS NOT NULL
    )
);

-- Mouvements (transactions)
CREATE TABLE movements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    transaction_type VARCHAR(20) NOT NULL, -- DEBIT, CREDIT
    amount DECIMAL(18, 3) NOT NULL CHECK (amount > 0),
    currency VARCHAR(3),
    description VARCHAR(255),
    reference_number VARCHAR(50),
    execution_date DATE NOT NULL,
    value_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Dépôts à terme (DAT)
CREATE TABLE term_deposits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    principal DECIMAL(18, 3) NOT NULL,
    interest_rate DECIMAL(5, 3) NOT NULL,
    start_date DATE NOT NULL,
    maturity_date DATE NOT NULL,
    status VARCHAR(20) DEFAULT 'ACTIVE', -- ACTIVE, MATURED, CLOSED
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: credit

```sql
-- Crédits (Prêts)
CREATE TABLE loans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    account_id UUID NOT NULL REFERENCES accounts(id),
    principal_amount DECIMAL(18, 3) NOT NULL,
    currency VARCHAR(3) DEFAULT 'TND',
    interest_rate DECIMAL(5, 3) NOT NULL,
    disbursement_date DATE NOT NULL,
    maturity_date DATE NOT NULL,
    outstanding_balance DECIMAL(18, 3) NOT NULL,
    asset_class VARCHAR(1) NOT NULL CHECK (asset_class IN ('0', '1', '2', '3', '4')),
    asset_class_updated_at TIMESTAMP,
    status VARCHAR(20) DEFAULT 'DISBURSED', -- PENDING, DISBURSED, MATURED, DEFAULTED
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Calendrier remboursement
CREATE TABLE loan_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL REFERENCES loans(id),
    due_date DATE NOT NULL,
    principal_payment DECIMAL(18, 3) NOT NULL,
    interest_payment DECIMAL(18, 3) NOT NULL,
    paid_date DATE,
    status VARCHAR(20) DEFAULT 'PENDING', -- PENDING, PAID, OVERDUE, WAIVED
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Provisions (Réserves pour dépréciation)
CREATE TABLE loan_provisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL REFERENCES loans(id),
    asset_class VARCHAR(1) NOT NULL,
    provision_rate DECIMAL(5, 2) NOT NULL, -- %, selon classe
    provision_amount DECIMAL(18, 3) NOT NULL,
    minimum_required DECIMAL(18, 3) NOT NULL,
    compliant BOOLEAN DEFAULT FALSE,
    effective_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ECL (Expected Credit Loss — IFRS 9)
CREATE TABLE ecl_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL REFERENCES loans(id),
    stage SMALLINT NOT NULL CHECK (stage IN (1, 2, 3)), -- Stage 1/2/3 IFRS 9
    probability_of_default DECIMAL(5, 4) NOT NULL,
    loss_given_default DECIMAL(5, 4) NOT NULL,
    exposure_at_default DECIMAL(18, 3) NOT NULL,
    ecl_amount DECIMAL(18, 3) NOT NULL,
    calculation_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: aml

```sql
-- Transactions (AML surveillance)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    counterparty_name VARCHAR(255),
    counterparty_iban VARCHAR(34),
    amount DECIMAL(18, 3) NOT NULL,
    currency VARCHAR(3) DEFAULT 'TND',
    transaction_type VARCHAR(30) NOT NULL, -- TRANSFER, CASH_WITHDRAWAL, DEPOSIT, CHEQUE
    risk_level VARCHAR(20) DEFAULT 'LOW', -- LOW, MEDIUM, HIGH, CRITICAL
    suspicious BOOLEAN DEFAULT FALSE,
    execution_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Alertes AML
CREATE TABLE aml_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID REFERENCES transactions(id),
    alert_type VARCHAR(50) NOT NULL, -- STRUCTURING, ROUND_AMOUNT, CASH_HEAVY, PEP_TRANSACTION, SANCTIONS_HIT
    alert_reason TEXT,
    alert_level VARCHAR(20) NOT NULL, -- LOW, MEDIUM, HIGH, CRITICAL
    status VARCHAR(20) DEFAULT 'OPEN', -- OPEN, UNDER_INVESTIGATION, CLOSED, DOS_FILED
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Investigations
CREATE TABLE investigations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alert_id UUID NOT NULL REFERENCES aml_alerts(id),
    assigned_to UUID,  -- Investigator user ID
    findings TEXT,
    conclusion VARCHAR(20), -- SUSPICIOUS, LEGITIMATE, INCONCLUSIVE
    closed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Déclarations de soupçon (DOS)
CREATE TABLE suspicion_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    investigation_id UUID NOT NULL REFERENCES investigations(id),
    description TEXT NOT NULL,
    submitted_to_ctaf BOOLEAN DEFAULT FALSE,
    submission_date TIMESTAMP,
    ctaf_reference VARCHAR(50),  -- Numéro CTAF
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Gels d'avoirs
CREATE TABLE asset_freezes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    account_id UUID NOT NULL REFERENCES accounts(id),
    freeze_reason VARCHAR(50) NOT NULL, -- SANCTIONS_MATCH, JUDICIAL_ORDER
    ctaf_notified BOOLEAN DEFAULT FALSE,
    unfrozen_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: sanctions

```sql
-- Listes de sanctions
CREATE TABLE sanction_lists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    list_type VARCHAR(50) NOT NULL UNIQUE, -- UN, EU, OFAC, TUNISIA
    source_url VARCHAR(500),
    last_updated TIMESTAMP NOT NULL,
    entry_count INT DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Entrées sanctions
CREATE TABLE sanction_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sanction_list_id UUID NOT NULL REFERENCES sanction_lists(id),
    entity_name VARCHAR(500) NOT NULL,
    entity_type VARCHAR(20), -- PERSON, ENTITY
    designation_details TEXT,
    external_reference VARCHAR(100),
    entry_date DATE,
    effective_date DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Résultats filtrage sanctions
CREATE TABLE screening_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    sanction_entry_id UUID NOT NULL REFERENCES sanction_entries(id),
    match_score DECIMAL(3, 2) NOT NULL CHECK (match_score >= 0 AND match_score <= 1), -- 0-1
    screening_type VARCHAR(30) NOT NULL, -- ONBOARDING, TRANSACTION, PERIODIC
    match_reason TEXT,
    action_taken VARCHAR(20) NOT NULL DEFAULT 'FREEZE', -- FREEZE, FLAG_FOR_REVIEW
    screening_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: prudential

```sql
-- Ratios prudentiels
CREATE TABLE prudential_ratios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calculation_date DATE NOT NULL UNIQUE,
    regulatory_capital DECIMAL(18, 3) NOT NULL,
    risk_weighted_assets DECIMAL(18, 3) NOT NULL,
    solvency_ratio DECIMAL(5, 2) NOT NULL CHECK (solvency_ratio >= 0), -- %, min 10%
    tier1_ratio DECIMAL(5, 2) NOT NULL, -- %, min 7%
    credit_to_deposit_ratio DECIMAL(5, 2) NOT NULL, -- %, max 120%
    concentration_ratio DECIMAL(5, 2) NOT NULL CHECK (concentration_ratio <= 25), -- %, max 25%
    compliant BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Calcul RWA par portefeuille
CREATE TABLE risk_weighted_assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    prudential_ratio_id UUID NOT NULL REFERENCES prudential_ratios(id),
    asset_type VARCHAR(50) NOT NULL, -- LOANS, SOVEREIGNS, COUNTERPARTY_CREDIT
    gross_amount DECIMAL(18, 3) NOT NULL,
    risk_weight DECIMAL(3, 1) NOT NULL CHECK (risk_weight >= 0 AND risk_weight <= 150), -- %
    rwa_amount DECIMAL(18, 3) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: accounting

```sql
-- Plan comptable bancaire (NCT)
CREATE TABLE chart_of_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_code VARCHAR(10) NOT NULL UNIQUE,  -- Ex: 1011, 2011
    account_name VARCHAR(255) NOT NULL,
    account_type VARCHAR(20) NOT NULL, -- ASSET, LIABILITY, EQUITY, INCOME, EXPENSE
    nct_reference VARCHAR(50),  -- Lien NCT
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Périodes comptables
CREATE TABLE accounting_periods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_name VARCHAR(20) NOT NULL UNIQUE, -- 202604, etc.
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    status VARCHAR(20) DEFAULT 'OPEN', -- OPEN, CLOSED
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Écritures comptables (double entrée)
CREATE TABLE journal_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    accounting_period_id UUID NOT NULL REFERENCES accounting_periods(id),
    journal_name VARCHAR(50) NOT NULL, -- GEN, ACA, OPE, etc.
    entry_date DATE NOT NULL,
    reference_number VARCHAR(50),  -- Numéro pièce justificative
    description VARCHAR(500),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Lignes écritures (débits/crédits)
CREATE TABLE journal_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journal_entry_id UUID NOT NULL REFERENCES journal_entries(id),
    account_code VARCHAR(10) NOT NULL,
    debit DECIMAL(18, 3),
    credit DECIMAL(18, 3),
    CHECK ((debit IS NOT NULL AND credit IS NULL) OR (debit IS NULL AND credit IS NOT NULL))
);

-- Grand livre (comptes de bilan/résultat)
CREATE TABLE ledger_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    accounting_period_id UUID NOT NULL REFERENCES accounting_periods(id),
    account_code VARCHAR(10) NOT NULL,
    opening_balance DECIMAL(18, 3),
    total_debit DECIMAL(18, 3) DEFAULT 0,
    total_credit DECIMAL(18, 3) DEFAULT 0,
    closing_balance DECIMAL(18, 3),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_period_account UNIQUE (accounting_period_id, account_code)
);
```

### Schema: reporting

```sql
-- Rapports réglementaires BCT
CREATE TABLE regulatory_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_type VARCHAR(50) NOT NULL, -- PRUDENTIAL_RATIO, AML_REPORT, FINANCIAL_STATEMENTS
    report_period DATE NOT NULL,
    report_version INT DEFAULT 1,
    report_content JSONB NOT NULL,  -- Données structurées
    submission_status VARCHAR(20) DEFAULT 'DRAFT', -- DRAFT, SUBMITTED, ACCEPTED, REJECTED
    submitted_at TIMESTAMP,
    bct_reference VARCHAR(50),  -- Numéro de transmission BCT
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_report UNIQUE (report_type, report_period)
);

-- Templates rapports
CREATE TABLE report_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_type VARCHAR(50) NOT NULL UNIQUE,
    bct_format_version VARCHAR(20),  -- Circulaire BCT référence
    template_json JSONB NOT NULL,  -- Structure attendue
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: payment

```sql
-- Ordres de paiement
CREATE TABLE payment_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    originating_account_id UUID NOT NULL REFERENCES accounts(id),
    beneficiary_iban VARCHAR(34) NOT NULL,
    beneficiary_name VARCHAR(255) NOT NULL,
    amount DECIMAL(18, 3) NOT NULL,
    currency VARCHAR(3) DEFAULT 'TND',
    payment_method VARCHAR(20) NOT NULL, -- TRANSFER, SWIFT, CLEARING
    execution_date DATE,
    status VARCHAR(20) DEFAULT 'PENDING', -- PENDING, SCREENED, EXECUTED, FAILED
    screening_status VARCHAR(20), -- CLEAR, BLOCKED, UNDER_REVIEW
    swift_message_id VARCHAR(50),  -- Lien SWIFT
    clearing_reference VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Messages SWIFT
CREATE TABLE swift_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payment_order_id UUID NOT NULL REFERENCES payment_orders(id),
    message_type VARCHAR(10) NOT NULL, -- MT103, MX204, etc.
    message_content TEXT NOT NULL,  -- XML ou texte SWIFT
    transmission_date TIMESTAMP,
    acknowledgement_date TIMESTAMP,
    swift_reference VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Clearing (compensation)
CREATE TABLE clearing_batches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    clearing_date DATE NOT NULL,
    batch_number VARCHAR(50) NOT NULL UNIQUE,
    total_amount DECIMAL(18, 3) NOT NULL,
    entry_count INT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'PENDING', -- PENDING, TRANSMITTED, SETTLED
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: fx

```sql
-- Opérations change
CREATE TABLE fx_operations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    operation_type VARCHAR(20) NOT NULL, -- BUY, SELL
    sold_currency VARCHAR(3) NOT NULL,
    sold_amount DECIMAL(18, 3) NOT NULL,
    bought_currency VARCHAR(3) NOT NULL,
    bought_amount DECIMAL(18, 3) NOT NULL,
    exchange_rate DECIMAL(10, 6) NOT NULL,
    execution_date DATE NOT NULL,
    settlement_date DATE,
    status VARCHAR(20) DEFAULT 'EXECUTED', -- PENDING, EXECUTED, SETTLED
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Positions change
CREATE TABLE fx_positions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calculation_date DATE NOT NULL,
    currency VARCHAR(3) NOT NULL UNIQUE,
    long_position DECIMAL(18, 3) DEFAULT 0,
    short_position DECIMAL(18, 3) DEFAULT 0,
    net_position DECIMAL(18, 3),  -- Long - Short
    spot_rate DECIMAL(10, 6),
    position_in_tnd DECIMAL(18, 3),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_position UNIQUE (calculation_date, currency)
);
```

### Schema: governance

```sql
-- Piste d'audit (Immutable Event Log)
CREATE TABLE audit_trail (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL, -- customer.created, account.opened, loan.classified
    entity_id UUID NOT NULL,  -- Customer/Account/Loan ID
    entity_type VARCHAR(50) NOT NULL,
    actor_id UUID NOT NULL REFERENCES users(id),  -- User who caused event
    action VARCHAR(50) NOT NULL, -- CREATE, UPDATE, DELETE, APPROVE, REJECT
    old_values JSONB,  -- Previous state
    new_values JSONB,  -- New state
    ip_address INET,
    user_agent VARCHAR(500),
    request_id VARCHAR(50),  -- Correlation ID
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Immutable constraint
    CHECK (timestamp <= CURRENT_TIMESTAMP)
);

-- Comités de gouvernance
CREATE TABLE committees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    committee_name VARCHAR(100) NOT NULL UNIQUE,  -- Comité crédit, Comité audit, etc.
    description VARCHAR(500),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Contrôles internes
CREATE TABLE control_checks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    control_name VARCHAR(255) NOT NULL,
    control_objective VARCHAR(500),
    owner_user_id UUID NOT NULL REFERENCES users(id),
    frequency VARCHAR(20) NOT NULL, -- DAILY, WEEKLY, MONTHLY, QUARTERLY
    last_check_date DATE,
    last_check_result VARCHAR(20), -- PASS, FAIL, NA
    line_of_defense SMALLINT NOT NULL CHECK (line_of_defense IN (1, 2, 3)), -- 1ère, 2e, 3e ligne
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Schema: identity

```sql
-- Utilisateurs
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,  -- Bcrypt hash
    full_name VARCHAR(255) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    last_login TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Rôles (RBAC)
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_name VARCHAR(50) NOT NULL UNIQUE, -- ADMIN, RISK_OFFICER, COMPLIANCE, TELLER, AUDITOR
    description VARCHAR(500),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Permissions
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    permission_name VARCHAR(100) NOT NULL UNIQUE, -- customer.create, loan.classify, report.submit
    description VARCHAR(500),
    resource VARCHAR(50) NOT NULL,  -- customer, account, loan, etc.
    action VARCHAR(20) NOT NULL,  -- create, read, update, delete, approve
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Attribution rôles aux utilisateurs
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    granted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id)
);

-- Attribution permissions aux rôles
CREATE TABLE role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    granted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role_id, permission_id)
);

-- Sessions
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    jwt_token VARCHAR(1000) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    is_valid BOOLEAN DEFAULT TRUE,
    ip_address INET,
    user_agent VARCHAR(500),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2FA (Two-Factor Authentication)
CREATE TABLE two_factor_auth (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    secret_key VARCHAR(100) NOT NULL,  -- TOTP secret
    is_enabled BOOLEAN DEFAULT FALSE,
    backup_codes TEXT[],  -- Comma-separated codes
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_2fa_per_user UNIQUE (user_id)
);
```

---

## 7. API REST (100+ endpoints par BC)

### BC1: Customer

```
POST   /api/v1/customers              Create customer + KYC
GET    /api/v1/customers/{id}         Get customer profile
PUT    /api/v1/customers/{id}         Update customer
GET    /api/v1/customers              List customers (paginated)
POST   /api/v1/customers/{id}/kyc     Submit KYC profile
GET    /api/v1/customers/{id}/kyc     Get KYC status
PUT    /api/v1/customers/{id}/kyc     Update KYC
POST   /api/v1/customers/{id}/kyc/validate  Validate KYC (compliance)
GET    /api/v1/customers/{id}/beneficial-owners  List beneficial owners
POST   /api/v1/customers/{id}/beneficial-owners  Add beneficial owner
PUT    /api/v1/customers/{id}/beneficial-owners/{owner-id}  Update beneficial owner
DELETE /api/v1/customers/{id}/beneficial-owners/{owner-id}  Remove beneficial owner
POST   /api/v1/customers/{id}/pep-check  Check PEP status
GET    /api/v1/customers/{id}/risk-score  Get risk score
PUT    /api/v1/customers/{id}/risk-score  Update risk score (analyst)
POST   /api/v1/customers/{id}/documents  Upload KYC documents
GET    /api/v1/customers/{id}/documents  List documents
DELETE /api/v1/customers/{id}/documents/{doc-id}  Delete document
POST   /api/v1/customers/{id}/consents  Grant data consent
GET    /api/v1/customers/{id}/consents  List consents
PUT    /api/v1/customers/{id}/consents/{consent-id}  Revoke consent
```

### BC2: Account

```
POST   /api/v1/accounts               Open account
GET    /api/v1/accounts/{id}          Get account details
PUT    /api/v1/accounts/{id}          Update account
GET    /api/v1/accounts               List accounts (with filters)
GET    /api/v1/accounts/{id}/balance  Get balance
POST   /api/v1/accounts/{id}/close    Close account
POST   /api/v1/accounts/{id}/freeze   Freeze account (AML)
POST   /api/v1/accounts/{id}/unfreeze Unfreeze account
GET    /api/v1/accounts/{id}/movements  List movements/transactions
POST   /api/v1/accounts/{id}/movements  Record movement (internal)
GET    /api/v1/accounts/{id}/statements  Generate statement PDF
POST   /api/v1/accounts/{id}/term-deposits  Open term deposit (DAT)
GET    /api/v1/accounts/{id}/term-deposits  List term deposits
PUT    /api/v1/accounts/{id}/term-deposits/{dat-id}  Update DAT
GET    /api/v1/accounts/{id}/interest  Calculate interest
```

### BC3: Credit

```
POST   /api/v1/loans                  Create loan application
GET    /api/v1/loans/{id}             Get loan details
PUT    /api/v1/loans/{id}             Update loan
GET    /api/v1/loans                  List loans
POST   /api/v1/loans/{id}/disburse    Disburse loan
POST   /api/v1/loans/{id}/classify    Classify loan (asset class)
GET    /api/v1/loans/{id}/classification  Get asset class
PUT    /api/v1/loans/{id}/classification  Update asset class (risk committee)
GET    /api/v1/loans/{id}/schedule    Get repayment schedule
POST   /api/v1/loans/{id}/schedule    Generate schedule
PUT    /api/v1/loans/{id}/schedule/{payment-id}  Record payment
POST   /api/v1/loans/{id}/provisions  Calculate provisions
GET    /api/v1/loans/{id}/provisions  Get provisions
PUT    /api/v1/loans/{id}/provisions  Update provisions
POST   /api/v1/loans/{id}/ecl         Calculate ECL (IFRS 9)
GET    /api/v1/loans/{id}/ecl         Get ECL values
POST   /api/v1/loans/{id}/restructure Restructure loan
GET    /api/v1/loans/{id}/history     Get change history
```

### BC4: AML

```
POST   /api/v1/aml/transactions       Record transaction (for monitoring)
GET    /api/v1/aml/transactions/{id}  Get transaction
GET    /api/v1/aml/transactions       List transactions
POST   /api/v1/aml/alerts             Create alert (manual)
GET    /api/v1/aml/alerts/{id}        Get alert details
GET    /api/v1/aml/alerts             List alerts (with filters)
PUT    /api/v1/aml/alerts/{id}/status Update alert status
POST   /api/v1/aml/investigations     Create investigation
GET    /api/v1/aml/investigations/{id}  Get investigation
PUT    /api/v1/aml/investigations/{id}  Update investigation findings
POST   /api/v1/aml/investigations/{id}/close  Close investigation
GET    /api/v1/aml/investigations     List investigations
POST   /api/v1/aml/suspicion-reports  File suspicion report (DOS)
GET    /api/v1/aml/suspicion-reports/{id}  Get DOS
PUT    /api/v1/aml/suspicion-reports/{id}/submit  Submit to CTAF
GET    /api/v1/aml/suspicion-reports  List DOS
POST   /api/v1/aml/asset-freezes      Create asset freeze
GET    /api/v1/aml/asset-freezes/{id}  Get freeze
PUT    /api/v1/aml/asset-freezes/{id}/unfreeze  Unfreeze
GET    /api/v1/aml/asset-freezes      List freezes
POST   /api/v1/aml/scenarios          List AML scenarios (read-only)
POST   /api/v1/aml/bulk-screening     Bulk transaction screening
```

### BC5: Sanctions

```
POST   /api/v1/sanctions/lists        Import/update sanctions list
GET    /api/v1/sanctions/lists        List loaded sanctions
GET    /api/v1/sanctions/lists/{list-id}  Get list metadata
POST   /api/v1/sanctions/entries      Add entry (manual)
GET    /api/v1/sanctions/entries      List entries (paginated)
POST   /api/v1/sanctions/screen       Screen entity/name
GET    /api/v1/sanctions/screening-results  List screening results
GET    /api/v1/sanctions/screening-results/{result-id}  Get result
POST   /api/v1/sanctions/rescreen     Rescreen customer (periodic)
GET    /api/v1/sanctions/pending-reviews  List pending reviews
PUT    /api/v1/sanctions/pending-reviews/{result-id}  Confirm/dismiss
POST   /api/v1/sanctions/whitelist    Whitelist entity (false positive)
GET    /api/v1/sanctions/whitelist    List whitelist
POST   /api/v1/sanctions/statistics   Get screening statistics (dashboard)
```

### BC6: Prudential

```
POST   /api/v1/prudential/calculate   Calculate all ratios (EOD job)
GET    /api/v1/prudential/ratios      Get latest ratios
GET    /api/v1/prudential/ratios/{date}  Get ratios for date
POST   /api/v1/prudential/ratios      Get historical trends
POST   /api/v1/prudential/solvency    Calculate solvency ratio
GET    /api/v1/prudential/solvency    Get solvency
POST   /api/v1/prudential/tier1       Calculate Tier 1
GET    /api/v1/prudential/tier1       Get Tier 1
POST   /api/v1/prudential/cd-ratio    Calculate C/D
GET    /api/v1/prudential/cd-ratio    Get C/D
POST   /api/v1/prudential/concentration  Calculate concentration
GET    /api/v1/prudential/concentration  Get concentration by customer
POST   /api/v1/prudential/rwa         Calculate RWA
GET    /api/v1/prudential/rwa         Get RWA breakdown
POST   /api/v1/prudential/validate    Validate compliance (all ratios)
GET    /api/v1/prudential/compliance  Get compliance dashboard
POST   /api/v1/prudential/alerts      Get breaches/alerts
```

### BC7: Accounting

```
POST   /api/v1/accounting/chart-of-accounts  Get chart (read-only)
POST   /api/v1/accounting/periods     Create accounting period
GET    /api/v1/accounting/periods/{period}  Get period status
PUT    /api/v1/accounting/periods/{period}/close  Close period
POST   /api/v1/accounting/journal-entries  Post journal entry
GET    /api/v1/accounting/journal-entries  List entries
GET    /api/v1/accounting/journal-entries/{id}  Get entry
DELETE /api/v1/accounting/journal-entries/{id}  Delete entry (if period open)
POST   /api/v1/accounting/journal-lines  Post line (typically within entry)
GET    /api/v1/accounting/ledger      Get ledger account balances
GET    /api/v1/accounting/ledger/{account-code}  Get account balance
POST   /api/v1/accounting/trial-balance  Generate trial balance
POST   /api/v1/accounting/balance-sheet  Generate balance sheet
POST   /api/v1/accounting/income-statement  Generate P&L
POST   /api/v1/accounting/reconciliation  Reconcile accounts
GET    /api/v1/accounting/cash-flow   Get cash flow statement
POST   /api/v1/accounting/validate    Validate period (all entries balanced)
```

### BC8: Reporting

```
POST   /api/v1/reporting/reports      Create report request
GET    /api/v1/reporting/reports/{id}  Get report
GET    /api/v1/reporting/reports      List reports
POST   /api/v1/reporting/reports/{id}/generate  Generate report content
POST   /api/v1/reporting/reports/{id}/validate  Validate before submission
POST   /api/v1/reporting/reports/{id}/submit    Submit to BCT
GET    /api/v1/reporting/templates    List report templates
GET    /api/v1/reporting/templates/{type}  Get template
POST   /api/v1/reporting/prudential-report  Generate prudential report
POST   /api/v1/reporting/aml-report   Generate AML report
POST   /api/v1/reporting/financial-report  Generate financial statements
GET    /api/v1/reporting/submission-status  Get BCT submission status
POST   /api/v1/reporting/export/{format}  Export (CSV, PDF, XML)
```

### BC9: Payment

```
POST   /api/v1/payments               Initiate payment
GET    /api/v1/payments/{id}          Get payment details
GET    /api/v1/payments               List payments
PUT    /api/v1/payments/{id}/cancel   Cancel payment (if pending)
POST   /api/v1/payments/{id}/execute  Execute payment (after screening)
POST   /api/v1/payments/{id}/swift    Generate SWIFT message
GET    /api/v1/payments/{id}/swift    Get SWIFT message
POST   /api/v1/payments/swift-send    Send SWIFT (external)
GET    /api/v1/payments/swift-receive  Receive incoming SWIFT
POST   /api/v1/payments/clearing-batch  Create clearing batch
GET    /api/v1/payments/clearing-batch/{batch-id}  Get batch
POST   /api/v1/payments/clearing-submit  Submit batch to clearing
GET    /api/v1/payments/clearing-status  Get clearing settlement status
POST   /api/v1/payments/reconcile     Reconcile payments
GET    /api/v1/payments/statistics    Get payment statistics
```

### BC10: ForeignExchange

```
POST   /api/v1/fx/operations          Execute FX operation
GET    /api/v1/fx/operations/{id}     Get operation
GET    /api/v1/fx/operations          List operations
POST   /api/v1/fx/rates               Get exchange rates
GET    /api/v1/fx/positions           Get FX positions
POST   /api/v1/fx/positions/calculate Calculate positions (EOD)
POST   /api/v1/fx/positions/validate  Validate exposure limits
GET    /api/v1/fx/limits              Get position limits
PUT    /api/v1/fx/limits              Update limits (compliance)
POST   /api/v1/fx/pnl                 Calculate daily P&L
GET    /api/v1/fx/statements          Get FX statement
```

### BC11: Governance

```
POST   /api/v1/audit-trail            Log event (system-generated)
GET    /api/v1/audit-trail/{id}       Get audit event
GET    /api/v1/audit-trail            List events (filterable)
POST   /api/v1/audit-trail/export     Export audit trail (compliance)
POST   /api/v1/audit-trail/search     Full-text search audit log
POST   /api/v1/committees             Create committee
GET    /api/v1/committees/{id}        Get committee details
GET    /api/v1/committees             List committees
POST   /api/v1/committees/{id}/members  Add member
DELETE /api/v1/committees/{id}/members/{member-id}  Remove member
POST   /api/v1/control-checks         Create control check
GET    /api/v1/control-checks/{id}    Get control
PUT    /api/v1/control-checks/{id}    Update control
POST   /api/v1/control-checks/{id}/execute  Execute control
GET    /api/v1/control-checks/{id}/history  Get execution history
GET    /api/v1/compliance-dashboard   Get compliance dashboard
```

### BC12: Identity

```
POST   /api/v1/auth/register          Register user
POST   /api/v1/auth/login             Login + JWT
POST   /api/v1/auth/refresh-token     Refresh JWT
POST   /api/v1/auth/logout            Logout (invalidate session)
POST   /api/v1/auth/password-reset    Request password reset
PUT    /api/v1/auth/password-reset/{token}  Reset password
POST   /api/v1/users                  Create user (admin)
GET    /api/v1/users/{id}             Get user
GET    /api/v1/users                  List users
PUT    /api/v1/users/{id}             Update user
DELETE /api/v1/users/{id}             Deactivate user
POST   /api/v1/users/{id}/roles       Assign role
DELETE /api/v1/users/{id}/roles/{role-id}  Revoke role
GET    /api/v1/roles                  List roles
POST   /api/v1/roles                  Create role
GET    /api/v1/permissions            List permissions
POST   /api/v1/2fa/enable             Enable 2FA for user
POST   /api/v1/2fa/verify             Verify 2FA code
GET    /api/v1/2fa/backup-codes       Get backup codes
POST   /api/v1/sessions               List active sessions
DELETE /api/v1/sessions/{session-id}  Logout session
```

---

## 8. Frontend (Architecture Hexagonale Light)

```
frontend/
├── src/
│   ├── lib/
│   │   ├── api/
│   │   │   ├── client.ts            # Fetch wrapper + interceptors
│   │   │   ├── customer.api.ts      # Typed API calls
│   │   │   ├── account.api.ts
│   │   │   ├── credit.api.ts
│   │   │   ├── aml.api.ts
│   │   │   ├── sanctions.api.ts
│   │   │   ├── prudential.api.ts
│   │   │   ├── accounting.api.ts
│   │   │   ├── reporting.api.ts
│   │   │   ├── payment.api.ts
│   │   │   ├── fx.api.ts
│   │   │   ├── governance.api.ts
│   │   │   └── identity.api.ts
│   │   ├── stores/
│   │   │   ├── auth.store.ts        # Svelte writable (user session)
│   │   │   ├── customer.store.ts    # Customer data cache
│   │   │   ├── account.store.ts     # Account data cache
│   │   │   ├── [...]
│   │   │   ├── toast.store.ts       # Toast notifications (Interface Segregation)
│   │   │   ├── modal.store.ts       # Modal state management (Interface Segregation)
│   │   │   └── loading.store.ts     # Loading indicators (Interface Segregation)
│   │   ├── utils/
│   │   │   ├── validators.ts        # Form validators
│   │   │   ├── formatters.ts        # Number, date, currency (TND/USD/EUR)
│   │   │   ├── errors.ts            # Error handler
│   │   │   └── constants.ts         # Enums, configs
│   │   └── i18n/
│   │       ├── ar.json              # Arabic (RTL)
│   │       ├── fr.json              # French
│   │       ├── en.json              # English
│   │       └── i18n.ts              # Setup + middleware
│   ├── components/
│   │   ├── shared/
│   │   │   ├── Header.svelte        # Nav + language selector
│   │   │   ├── Sidebar.svelte       # Menu (role-based)
│   │   │   ├── Button.svelte        # Reusable button
│   │   │   ├── Modal.svelte         # Confirmation, alerts
│   │   │   ├── Toast.svelte         # Notifications
│   │   │   ├── Form.svelte          # Base form
│   │   │   └── Table.svelte         # Pagination, sorting
│   │   ├── customer/
│   │   │   ├── CustomerForm.svelte
│   │   │   ├── KycForm.svelte
│   │   │   ├── BeneficiaryForm.svelte
│   │   │   ├── PepCheckCard.svelte
│   │   │   └── RiskScoreDisplay.svelte
│   │   ├── account/
│   │   │   ├── AccountForm.svelte
│   │   │   ├── BalanceCard.svelte
│   │   │   ├── MovementsList.svelte
│   │   │   └── TermDepositForm.svelte
│   │   ├── credit/
│   │   │   ├── LoanForm.svelte
│   │   │   ├── ClassificationForm.svelte
│   │   │   ├── ScheduleTable.svelte
│   │   │   └── ProvisionCard.svelte
│   │   ├── aml/
│   │   │   ├── AlertsList.svelte
│   │   │   ├── InvestigationForm.svelte
│   │   │   └── SuspicionReportForm.svelte
│   │   ├── sanctions/
│   │   │   ├── ScreeningForm.svelte
│   │   │   └── ResultsTable.svelte
│   │   ├── prudential/
│   │   │   ├── RatioDashboard.svelte
│   │   │   ├── RatioChart.svelte
│   │   │   └── ComplianceIndicator.svelte
│   │   ├── accounting/
│   │   │   ├── JournalEntryForm.svelte
│   │   │   ├── LedgerView.svelte
│   │   │   └── TrialBalanceReport.svelte
│   │   ├── reporting/
│   │   │   ├── ReportForm.svelte
│   │   │   └── ReportPreview.svelte
│   │   ├── payment/
│   │   │   ├── PaymentForm.svelte
│   │   │   ├── SwiftPreview.svelte
│   │   │   └── ClearingBatchList.svelte
│   │   ├── fx/
│   │   │   ├── FxOperationForm.svelte
│   │   │   ├── PositionDashboard.svelte
│   │   │   └── RateDisplay.svelte
│   │   ├── governance/
│   │   │   ├── AuditTrailViewer.svelte
│   │   │   └── ControlCheckForm.svelte
│   │   └── identity/
│   │       ├── LoginForm.svelte
│   │       ├── UserManagement.svelte
│   │       ├── RoleForm.svelte
│   │       └── 2FASetup.svelte
│   ├── routes/
│   │   ├── +layout.svelte           # Root layout
│   │   ├── +page.svelte             # Dashboard
│   │   ├── login/+page.svelte
│   │   ├── customers/
│   │   │   ├── +page.svelte         # List
│   │   │   ├── [id]/+page.svelte    # Detail
│   │   │   └── [id]/edit/+page.svelte
│   │   ├── accounts/
│   │   ├── loans/
│   │   ├── aml/
│   │   ├── sanctions/
│   │   ├── prudential/
│   │   ├── accounting/
│   │   ├── reporting/
│   │   ├── payments/
│   │   ├── fx/
│   │   ├── governance/
│   │   └── admin/
│   │       ├── users/+page.svelte
│   │       ├── roles/+page.svelte
│   │       └── audit/+page.svelte
│   └── app.svelte                   # Root component
├── static/
│   ├── logo.svg
│   └── flags/                       # AR, FR, EN flags
├── tailwind.config.js               # Tailwind CSS config
├── svelte.config.js
└── astro.config.mjs                 # Astro SSG config
```

#### i18n Strategy (AR RTL + FR + EN)

- **RTL Support** : Arabic UI flipped, CSS `direction: rtl` auto-applied
- **Language Selector** : Header dropdown, persists in localStorage
- **Translation Keys** : `i18n.ar.json`, `i18n.fr.json`, `i18n.en.json`
- **Date/Currency Formatting** : `Intl.DateTimeFormat()`, currency symbol based on language
- **Form Validation** : Locale-aware error messages

#### WCAG 2.1 AA Accessibility Requirements

Tous les composants frontend doivent respecter WCAG 2.1 Level AA :

**ARIA & Labels** :
- ✅ Tous les éléments interactifs (input, button, select) ont des labels associés (`<label for="">` ou `aria-label`)
- ✅ Composants custom (modale, alerte, menu) ont rôles ARIA appropriés (`role="dialog"`, `role="alert"`, `role="navigation"`)
- ✅ Listes et menus ont structure sémantique (`<ul>`, `<li>`, ou ARIA listbox)
- ✅ Icônes sans texte ont `aria-label` ou texte caché (`.sr-only`)

**Navigation Clavier** :
- ✅ Tab order logique et séquentiel (LTR pour EN/FR, RTL pour AR)
- ✅ Focus visible sur tous éléments interactifs (outline ou custom style)
- ✅ Touches spéciales supportées : Enter/Space pour activation, Escape pour fermer modales, Arrow keys pour menus/listes
- ✅ Focus trap dans modales (clavier reste confiné à la modale)
- ✅ Skip navigation link en haut de chaque page (pour passer la navigation)

**Contraste & Couleurs** :
- ✅ Texte normal (body) : contraste minimum 4.5:1 (noir #000000 sur blanc #FFFFFF = 21:1)
- ✅ Grands textes (≥18pt ou ≥14pt bold) : contraste minimum 3:1
- ✅ Les couleurs seules NE sont PAS utilisées pour transmettre l'information (ex: utiliser "⚠️ Erreur" en plus du rouge)
- ✅ Outils de vérification : WebAIM Contrast Checker, WAVE

**Images & Icônes** :
- ✅ Toutes images/icônes ont texte alternatif (`alt` pour `<img>`, `aria-label` pour SVG)
- ✅ Logos/images décoratives : `alt=""` (vide)
- ✅ Graphiques/charts : `aria-label` avec description des données ou tableau de données alternatif

**Rôles ARIA Personnalisés** :
```html
<!-- Modal/Dialog -->
<div role="dialog" aria-modal="true" aria-labelledby="modalTitle">
  <h1 id="modalTitle">Confirmation</h1>
  <!-- Focus trap ici -->
</div>

<!-- Alert Banner -->
<div role="alert" aria-live="polite" aria-atomic="true">
  Erreur: Veuillez corriger les champs marqués en rouge.
</div>

<!-- Custom Navigation -->
<nav role="navigation" aria-label="Main navigation">
  <!-- Menu items -->
</nav>

<!-- Table Headers -->
<table>
  <thead>
    <tr>
      <th scope="col">Nom</th>
      <th scope="col">Montant</th>
    </tr>
  </thead>
</table>
```

**Langues & RTL** :
- ✅ `lang="ar"` sur `<html>` pour arabe (screenreader prononce correctement)
- ✅ `lang="fr"` pour français, `lang="en"` pour anglais
- ✅ `dir="rtl"` sur éléments RTL si requis
- ✅ Clavier approprié pour chaque langue (virtuel AR pour mobile)

**Testing Automatisé** :
- ✅ **axe-core intégré** dans tests Playwright E2E (npm install @axe-core/playwright)
  ```typescript
  import { injectAxe, checkA11y } from 'axe-playwright';

  test('page complies with WCAG AA', async ({ page }) => {
    await page.goto('/accounts');
    await injectAxe(page);
    await checkA11y(page);
  });
  ```
- ✅ **CI/CD** : Tests axe-core exécutés à chaque commit (bloquant si violations)
- ✅ **Audit manuel** : Review WCAG tous les trimestres avec outil WAVE ou Lighthouse

**Accessibility** : `lang` attribute on `<html>`, full WCAG 2.1 AA compliance

---

## 9. ADR (Architecture Decision Records)

### ADR-001: SOLID + Hexagonal

**Status** : Accepted
**Context** : BANKO is a regulated banking system with high audit/compliance needs.
**Decision** : Adopt hexagonal architecture with SOLID principles to protect domain logic from external changes.
**Rationale** :
- Domain layer isolated = easier to test, audit, refactor
- Ports & Adapters = swap PostgreSQL for MySQL/Oracle without touching domain
- SOLID enforced = high cohesion, low coupling
**Consequences** : More layers initially, but long-term flexibility + compliance + testability.

### ADR-002: TDD + BDD + Documentation Vivante

**Status** : Accepted
**Context** : Banking invariants must be ironclad; stakeholders (BCT, auditors) need proof of compliance.
**Decision** : Write Gherkin scenarios FIRST, then unit tests, then code. BDD = executable spec = compliance proof.
**Rationale** :
- Gherkin = shared language with business/legal experts
- Scenarios are living documentation of regulatory requirements
- Can demo to BCT inspector: "Here's the test, it passes, system compliant"
**Consequences** : 20-30% slower initial velocity, but 90% fewer post-release bugs + regulatory confidence.

### ADR-003: DDD Ubiquitous Language Mapping

**Status** : Accepted
**Context** : Tunisian banking domain has specific vocabulary (classes 0-4, FPN, Tier 1, Circ. 91-24, etc.).
**Decision** : Map ALL domain terms to exact code struct/enum names; glossaire in docs maps bidirectionally.
**Rationale** :
- Developers speak same language as auditors/domain experts
- No "what does _balance_ mean here?" ambiguity
- Code audit = reading droit bancaire directly
**Consequences** : Longer identifiers sometimes, but zero semantic confusion.

### ADR-004: Rust + Actix-web (Justification)

**Status** : Accepted
**Context** : Banking system = performance-critical, memory-safe, audit-friendly.
**Decision** : Rust + Actix-web 4.9 for async, type-safe HTTP, impossible to have use-after-free/buffer overflow bugs.
**Rationale** :
- Memory safety = fewer security patches than C/C++/Java
- Type system = compile-time invariant checks (Tier 1 ≥ 7%)
- Async = handle thousands of concurrent customers on single core
- Ownership = clear resource lifetimes, no GC pauses
**Consequences** : Steep learning curve, slower compilation, but 10x fewer runtime surprises.

### ADR-005: PostgreSQL (Justification Bancaire)

**Status** : Accepted
**Context** : Banking data = ACID, audit trail, replication, partitioning, compliance.
**Decision** : PostgreSQL 16 with LUKS encryption, logical replication, partitioning by date/customer.
**Rationale** :
- ACID guarantees = no split-brain transactions
- JSON/JSONB = reporting flexibility
- FDW (Foreign Data Wrapper) = federated audit trail
- Open source = auditability
- Partitioning = manage 10+ years historical data efficiently
**Consequences** : No cloud vendor lock-in, but ops team must manage replication/backups.

### ADR-006: HSM for Cryptographic Signatures

**Status** : Accepted
**Context** : Banking regulations require non-repudiation (can't deny you signed transaction). Private keys = critical.
**Decision** : Integrate PKCS#11 HSM (Hardware Security Module) for all cryptographic signing (audit trail, SWIFT, declarations).
**Rationale** :
- Private keys NEVER leave HSM = can't be stolen even if server compromised
- Hardware-backed = FIPS 140-2 certified
- Audit trail = signed with HSM key = auditor can verify "this action was really approved"
**Consequences** : Adds hardware cost, latency ~5-10ms per signature, but legally defensible.

### ADR-007: Audit Trail Immutability Strategy

**Status** : Accepted
**Context** : Circ. 2006-19 requires piste d'audit that can't be erased, even by admin.
**Decision** : PostgreSQL event log (insert-only table) + cryptographic hash chain (each row includes hash of previous row).
**Rationale** :
- INSERT-only = can't UPDATE/DELETE once written
- Hash chain = tampering detected immediately (hash chain breaks)
- Replication to secure off-site = even if production DB destroyed, audit trail survives
**Consequences** : Audit table grows indefinitely, but partitioning by date keeps queries fast.

---

## 10. Sécurité & INPDP (Loi 2004-63)

### Principes

1. **Privacy-by-design** : Minimal data collection, purpose limitation
2. **Encryption at rest** : LUKS AES-XTS-512 on all disks
3. **Encryption in transit** : TLS 1.3 mandatory on all channels
4. **Consent management** : INPDP consent stored, revocable
5. **Right to be forgotten** : Anonymization routines (soft-delete + GDPR-like)
6. **Access controls** : RBAC, 2FA, session timeouts
7. **Audit trail** : All personal data access logged

### Mesures techniques

| Mesure | Implémentation | Scope |
|---|---|---|
| **Chiffrement de repos** | LUKS AES-512 | PostgreSQL, backups, S3 |
| **Chiffrement transit** | TLS 1.3 | HTTP, replication, SWIFT |
| **Hachage mots de passe** | Bcrypt (cost 12) | Credentials utilisateurs |
| **Tokens JWT** | RS256 (RSA 4096), expires 1h | Sessions API |
| **Rotation clés HSM** | Annuelle | Signatures bancaires |
| **Rate limiting** | 1000 req/min par IP | DDoS protection |
| **WAF (Web Application Firewall)** | CrowdSec | Injection SQL, XSS, CSRF |
| **IDS (Intrusion Detection)** | Suricata | Network-level anomalies |
| **Fail2ban** | Ban après 5 failed logins | Brute force protection |

---

## 11. Infrastructure de déploiement

### Docker Compose (Développement)

```yaml
version: '3.9'
services:
  postgres:
    image: postgres:16-alpine
    env_file: .env.dev
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

  backend:
    build:
      context: .
      dockerfile: Dockerfile.backend
    env_file: .env.dev
    ports:
      - "8000:8000"
    depends_on:
      - postgres
      - redis
    volumes:
      - ./src:/app/src

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.frontend
    ports:
      - "3000:3000"
    depends_on:
      - backend

  mailhog:
    image: mailhog/mailhog:latest
    ports:
      - "1025:1025"
      - "8025:8025"

  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    depends_on:
      - prometheus
```

### Production (Kubernetes)

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: banko-backend
  labels:
    app: banko
spec:
  replicas: 3
  selector:
    matchLabels:
      app: banko-backend
  template:
    metadata:
      labels:
        app: banko-backend
    spec:
      serviceAccountName: banko
      securityContext:
        runAsNonRoot: true
        fsReadOnlyRootFilesystem: true
      containers:
      - name: backend
        image: banko-backend:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 8000
          name: http
        livenessProbe:
          httpGet:
            path: /health
            port: 8000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8000
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: banko-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: banko-secrets
              key: redis-url
        volumeMounts:
        - name: tmp
          mountPath: /tmp
        - name: var-tmp
          mountPath: /var/tmp
      volumes:
      - name: tmp
        emptyDir: {}
      - name: var-tmp
        emptyDir: {}
---
apiVersion: v1
kind: Service
metadata:
  name: banko-backend
spec:
  selector:
    app: banko-backend
  type: ClusterIP
  ports:
  - port: 8000
    targetPort: 8000
    protocol: TCP
```

---

## 12. IaC — Infrastructure as Code

```
infrastructure/
├── terraform/
│   ├── main.tf                  # Provider config (OVH Cloud / sovereign)
│   ├── vpc.tf                   # VPC, subnets, security groups
│   ├── database.tf              # PostgreSQL RDS + backups
│   ├── cache.tf                 # Redis
│   ├── storage.tf               # S3-compatible (KYC docs, backups)
│   ├── kubernetes.tf            # K8s cluster (production)
│   ├── monitoring.tf            # Prometheus, Loki, Grafana
│   ├── firewall.tf              # WAF, DDoS protection
│   ├── hsm.tf                   # Hardware Security Module
│   ├── variables.tf
│   ├── outputs.tf
│   └── terraform.tfvars.example
└── ansible/
    ├── playbooks/
    │   ├── provision-k8s.yml    # Initialize K8s nodes
    │   ├── deploy-banko.yml     # Deploy backend + frontend
    │   ├── configure-monitoring.yml
    │   └── backup-restore.yml
    ├── roles/
    │   ├── kubernetes-node/
    │   ├── postgres-backup/
    │   ├── hsm-setup/
    │   └── firewall/
    └── inventory/
        ├── dev.yml
        ├── staging.yml
        └── production.yml
```

---

## 13. CI/CD — Pipeline

```yaml
# .github/workflows/main.yml
name: CI/CD

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  lint-test-backend:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16-alpine
        env:
          POSTGRES_DB: banko_test
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7-alpine
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with:
        components: clippy, rustfmt
    - uses: Swatinem/rust-cache@v2

    - name: Lint (rustfmt)
      run: cargo fmt --check

    - name: Lint (clippy)
      run: cargo clippy --all -- -D warnings

    - name: Security audit
      run: cargo audit

    - name: Unit tests
      run: cargo test --lib
      env:
        DATABASE_URL: postgres://test:test@localhost/banko_test
        REDIS_URL: redis://localhost

    - name: BDD tests
      run: cargo test --test '*' --features bdd
      env:
        DATABASE_URL: postgres://test:test@localhost/banko_test

    - name: Coverage
      run: |
        cargo install tarpaulin
        cargo tarpaulin --out Xml --exclude-files tests/

    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3

  build-docker:
    needs: lint-test-backend
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: docker/setup-buildx-action@v3

    - name: Build backend image
      uses: docker/build-push-action@v5
      with:
        context: .
        file: ./Dockerfile.backend
        push: false
        tags: banko-backend:${{ github.sha }}

    - name: Build frontend image
      uses: docker/build-push-action@v5
      with:
        context: ./frontend
        file: ./frontend/Dockerfile.frontend
        push: false
        tags: banko-frontend:${{ github.sha }}

  e2e-tests:
    needs: build-docker
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: actions/setup-node@v4
      with:
        node-version: '20'

    - name: Start services (docker-compose)
      run: docker-compose -f docker-compose.test.yml up -d

    - name: Wait for API ready
      run: |
        timeout 60s bash -c 'until curl -f http://localhost:8000/health; do sleep 2; done'

    - name: E2E tests (Playwright)
      run: npm run test:e2e
      working-directory: ./frontend

    - name: Upload Playwright report
      if: always()
      uses: actions/upload-artifact@v3
      with:
        name: playwright-report
        path: ./frontend/playwright-report/

  deploy-staging:
    if: github.ref == 'refs/heads/develop'
    needs: [ lint-test-backend, e2e-tests ]
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Deploy to staging (Kubernetes)
      run: |
        kubectl set image deployment/banko-backend \
          banko-backend=banko-backend:${{ github.sha }} \
          --record=true
      env:
        KUBE_CONFIG: ${{ secrets.KUBE_CONFIG_STAGING }}

  deploy-production:
    if: github.ref == 'refs/heads/main'
    needs: [ lint-test-backend, e2e-tests ]
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Deploy to production (Kubernetes)
      run: |
        kubectl set image deployment/banko-backend \
          banko-backend=banko-backend:${{ github.sha }} \
          --record=true
      env:
        KUBE_CONFIG: ${{ secrets.KUBE_CONFIG_PRODUCTION }}
    - name: Verify deployment health
      run: |
        kubectl rollout status deployment/banko-backend --timeout=5m
```

---

## 14. Observabilité

### Prometheus Metrics

```rust
// src/infrastructure/monitoring/metrics.rs
lazy_static::lazy_static! {
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec =
        IntCounterVec::new(
            Opts::new("http_requests_total", "Total HTTP requests"),
            &["method", "path", "status"]
        ).unwrap();

    pub static ref HTTP_REQUEST_DURATION_SECS: HistogramVec =
        HistogramVec::new(
            HistogramOpts::new("http_request_duration_secs", "HTTP request duration"),
            &["method", "path"]
        ).unwrap();

    pub static ref DATABASE_QUERY_DURATION_SECS: HistogramVec =
        HistogramVec::new(
            HistogramOpts::new("db_query_duration_secs", "Database query duration"),
            &["query_type"]
        ).unwrap();

    pub static ref AUDIT_TRAIL_ENTRIES_TOTAL: IntCounter =
        IntCounter::new("audit_trail_entries_total", "Total audit trail entries").unwrap();

    pub static ref AML_ALERTS_ACTIVE: IntGauge =
        IntGauge::new("aml_alerts_active", "Active AML alerts").unwrap();

    pub static ref PRUDENTIAL_RATIO_SOLVENCY: Gauge =
        Gauge::new("prudential_solvency_ratio", "Solvency ratio (%)").unwrap();

    pub static ref PRUDENTIAL_RATIO_TIER1: Gauge =
        Gauge::new("prudential_tier1_ratio", "Tier 1 ratio (%)").unwrap();

    pub static ref PRUDENTIAL_RATIO_CD: Gauge =
        Gauge::new("prudential_cd_ratio", "Credit-to-Deposit ratio (%)").unwrap();
}
```

### Grafana Dashboards

- **System Dashboard** : CPU, memory, disk, network
- **Business Dashboard** : Customers, accounts, loans, AUM
- **Prudential Dashboard** : Ratios, RWA, concentration
- **AML Dashboard** : Alerts, investigations, DOS count
- **Audit Dashboard** : Trail entries per hour, top actors
- **SLA Dashboard** : API latency, uptime, error rates

### Loki Structured Logging

```rust
// src/infrastructure/monitoring/logging.rs
use tracing::{info, warn, error};
use tracing_subscriber::fmt;

#[tracing::instrument(skip(request))]
pub async fn handle_create_customer(
    request: CreateCustomerRequest,
) -> Result<CustomerResponse> {
    info!(
        customer_name = %request.name,
        customer_type = %request.customer_type,
        "Creating customer"
    );
    // ...
    info!(customer_id = %customer.id, "Customer created");
    Ok(response)
}
```

All logs ship to Loki, queryable by:
- `{job="banko-backend", level="error"}`
- `{bounded_context="customer", action="create"}`
- Full-text search: "invalid KYC"

---

## 15. Scalabilité organisationnelle

### Teams & Responsibilities

| Rôle | Responsabilité | Bounded Contexts |
|---|---|---|
| **CRO (Chief Risk Officer)** | Credit classification, provisioning, concentration ratios | Credit, Prudential |
| **CMLCO (AML Compliance)** | AML surveillance, KYC, DOS filing, sanctions | Customer, AML, Sanctions |
| **CFO (Finance)** | Accounting, reporting, P&L, IFRS 9 | Accounting, Reporting |
| **Operations** | Payments, settlement, cash management | Payment, FX, Clearing |
| **Audit** | Audit trail, governance, control checks | Governance, Audit |
| **IT Security** | HSM, backups, disaster recovery, access control | Infrastructure, Identity |
| **Developers** | Implement use cases, maintain code quality | All BCs |

### Scaling Path

1. **Phase 1 (MVP)** : Solo dev (or 2) — Core domain (Customer, Account, Credit, AML, Accounting)
2. **Phase 2** : Add Risk Officer + Analyst — Prudential, Governance
3. **Phase 3** : Add Operations — Payment, FX, Reporting
4. **Phase 4** : Specialized squads per BC (Product, Dev, QA each)

---

## Conclusion

BANKO implements **by-design** regulatory compliance through:
- **DDD** : Domain protects Tunisian banking law
- **TDD/BDD** : Specs = proof of compliance
- **Hexagonal** : Easy to audit, test, modify
- **Audit trail** : Non-repudiation, regulatory confidence
- **HSM** : Cryptographic protection of critical signing
- **SOLID** : Maintainability for 10+ year lifetime

**Reference Commits** [REF-01] through [REF-70] in `docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md` map each invariant to Tunisian law and international standards (Bâle III, GAFI, IFRS 9, ISO 20022/27001).

Architecture is designed for autonomous operation by solo-dev or small team with clear module boundaries, comprehensive testing, and regulatory traceability.

---

**Document Version** : 1.0.0
**Last Updated** : 4 avril 2026
**Maintainer** : GILMRY / BANKO Project
**License** : AGPL-3.0
