# BANKO - Guide d'architecture hexagonale

## Vue d'ensemble

BANKO utilise l'**architecture hexagonale** (Ports & Adaptateurs) avec **Domain-Driven Design (DDD)** pour garantir:

- 🔒 **Sécurité bancaire** critique
- 🧪 **Testabilité** maximale
- 📦 **Maintenabilité** long terme
- 🔄 **Flexibilité** aux changements

## Principes fondamentaux

### 1. Inversion de dépendances

```
Domain (cœur)
    ↑
Application (logique métier)
    ↑
Infrastructure (implémentations concrètes)
```

- **Domain** n'a AUCUNE dépendance externe
- **Application** dépend de Domain uniquement
- **Infrastructure** implémente les ports Application

### 2. Isolation des domaines (Bounded Contexts)

BANKO organise 12 bounded contexts indépendants:

```
1. Authentification & Autorisation
2. Comptes bancaires
3. Transactions & Historique
4. Paiements domestiques
5. Virements nationaux/internationaux
6. Conformité & KYC/AML
7. Audit & Immuabilité
8. Reporting réglementaire
9. Intégrations tierces
10. Notifications
11. Gestion des risques & Fraude
12. Administration système
```

Chaque contexte a ses propres:
- Entités Domain
- Ports (interfaces)
- Repositories
- Use Cases

### 3. Architecture des couches

#### Couche Domain (`backend/src/domain/`)

**Logique métier pure**, AUCUNE dépendance externe.

```
domain/
├── entities/
│   ├── account.rs         # Agrégat BankAccount
│   ├── transaction.rs     # Agrégat Transaction
│   └── ...
├── value_objects/
│   ├── iban.rs            # IBAN strict type
│   ├── money.rs           # Money avec devise
│   └── ...
├── services/
│   ├── account_service.rs # Logique métier complexe
│   └── ...
└── errors/
    └── domain_error.rs    # Erreurs métier
```

**Exemple: Entité Account**

```rust
// backend/src/domain/entities/account.rs

pub struct BankAccount {
    id: AccountId,
    iban: Iban,           // Type strict (pas String!)
    owner: Owner,
    balance: Money,       // Devise incluse
    status: AccountStatus,
}

impl BankAccount {
    // Constructeur: valide invariants métier
    pub fn new(
        owner: Owner,
        iban: Iban,
    ) -> Result<Self, DomainError> {
        // Validation métier
        Ok(Self {
            id: AccountId::new(),
            iban,
            owner,
            balance: Money::zero(Currency::Eur),
            status: AccountStatus::Active,
        })
    }

    // Méthode métier: effectue virement
    pub fn transfer_to(
        &mut self,
        amount: Money,
        recipient: &BankAccount,
    ) -> Result<(), DomainError> {
        // Règles métier appliquées
        if amount > self.balance {
            return Err(DomainError::InsufficientFunds);
        }

        self.balance = self.balance.subtract(amount)?;
        Ok(())
    }
}
```

#### Couche Application (`backend/src/application/`)

Orchestration, ports (interfaces), use cases, DTOs.

```
application/
├── ports/
│   ├── account_repository.rs      # Trait AccountRepository
│   ├── transaction_repository.rs
│   └── ...
├── use_cases/
│   ├── transfer_money.rs          # UC: Effectuer virement
│   ├── create_account.rs
│   └── ...
├── dto/
│   ├── account_dto.rs             # DTO (sérialisable)
│   └── ...
└── errors/
    └── application_error.rs
```

**Exemple: Port Repository**

```rust
// backend/src/application/ports/account_repository.rs

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(
        &self,
        account: &BankAccount,
    ) -> Result<BankAccount, DomainError>;

    async fn find_by_id(
        &self,
        id: &AccountId,
    ) -> Result<Option<BankAccount>, DomainError>;

    async fn update(
        &self,
        account: &BankAccount,
    ) -> Result<(), DomainError>;
}
```

**Exemple: Use Case**

```rust
// backend/src/application/use_cases/transfer_money.rs

pub struct TransferMoneyUseCase<R: AccountRepository> {
    repo: Arc<R>,
    audit: Arc<dyn AuditPort>,
}

impl<R> TransferMoneyUseCase<R> {
    pub async fn execute(
        &self,
        from_id: AccountId,
        to_id: AccountId,
        amount: Money,
        user: &User,
    ) -> Result<Transaction, ApplicationError> {
        // Authentification
        if !user.can_transfer() {
            return Err(ApplicationError::Unauthorized);
        }

        // Charger comptes
        let mut from = self.repo.find_by_id(&from_id)
            .await?
            .ok_or(ApplicationError::AccountNotFound)?;
        let to = self.repo.find_by_id(&to_id)
            .await?
            .ok_or(ApplicationError::AccountNotFound)?;

        // Logique métier (Domain)
        from.transfer_to(&amount, &to)?;

        // Persister
        self.repo.update(&from).await?;

        // Audit logging
        self.audit.log_transfer(&from_id, &to_id, &amount).await?;

        Ok(Transaction::from(&from, &to, &amount))
    }
}
```

#### Couche Infrastructure (`backend/src/infrastructure/`)

Implémentations concrètes: PostgreSQL, HTTP, HSM.

```
infrastructure/
├── database/
│   ├── repositories/
│   │   ├── account_repository_impl.rs  # Implémentation PostgreSQL
│   │   └── ...
│   └── migrations/
│       └── ...
├── web/
│   ├── handlers/
│   │   ├── account_handler.rs          # Endpoints HTTP
│   │   └── ...
│   └── routes.rs                       # Configuration routes
├── security/
│   ├── hsm/                            # Hardware Security Module
│   ├── encryption/                     # Chiffrement données
│   └── audit/                          # Audit trail immuable
└── persistence/
    └── ...
```

**Exemple: Repository PostgreSQL**

```rust
// backend/src/infrastructure/database/repositories/account_repository.rs

pub struct PostgresAccountRepository {
    pool: PgPool,
    hsm: Arc<HsmClient>,  // Sécurité bancaire
}

#[async_trait]
impl AccountRepository for PostgresAccountRepository {
    async fn create(
        &self,
        account: &BankAccount,
    ) -> Result<BankAccount, DomainError> {
        // Utiliser prepared statements (SQLx)
        let row = sqlx::query!(
            r#"
            INSERT INTO bank_accounts (id, iban, owner_id, balance, currency, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, iban, owner_id, balance, currency, status
            "#,
            account.id().as_str(),
            account.iban().as_str(),
            account.owner().id().as_str(),
            account.balance().amount(),
            account.balance().currency().as_str(),
            account.status().as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::PersistenceError(e.to_string()))?;

        Ok(BankAccount::from_database(row)?)
    }
}
```

**Exemple: HTTP Handler**

```rust
// backend/src/infrastructure/web/handlers/account_handler.rs

#[post("/accounts")]
async fn create_account(
    user: AuthenticatedUser,  // Middleware autorisation
    body: Json<CreateAccountRequest>,
    repo: web::Data<Arc<dyn AccountRepository>>,
    uc: web::Data<Arc<CreateAccountUseCase>>,
) -> Result<Json<AccountResponse>, ApiError> {
    // Use Case exécute logique métier
    let account = uc.execute(
        body.owner_id,
        body.iban,
        &user,
    )
    .await
    .map_err(|e| ApiError::from(e))?;

    // Retourner DTO (pas entité Domain)
    Ok(Json(AccountResponse::from(&account)))
}
```

## Workflow ajout fonctionnalité

### Étape 1: Définir entité Domain

```rust
// backend/src/domain/entities/transfer.rs
pub struct Transfer {
    id: TransferId,
    from: IBAN,
    to: IBAN,
    amount: Money,
    status: TransferStatus,
}

impl Transfer {
    pub fn new(...) -> Result<Self, DomainError> {
        // Valider invariants métier
    }

    pub fn execute(&mut self) -> Result<(), DomainError> {
        // Logique métier
    }
}
```

### Étape 2: Définir port (interface)

```rust
// backend/src/application/ports/transfer_repository.rs
#[async_trait]
pub trait TransferRepository: Send + Sync {
    async fn create(&self, transfer: &Transfer) -> Result<Transfer, DomainError>;
    async fn find_by_id(&self, id: &TransferId) -> Result<Option<Transfer>, DomainError>;
    async fn update(&self, transfer: &Transfer) -> Result<(), DomainError>;
}
```

### Étape 3: Créer Use Case

```rust
// backend/src/application/use_cases/execute_transfer.rs
pub struct ExecuteTransferUseCase<R: TransferRepository> {
    repo: Arc<R>,
}

impl<R> ExecuteTransferUseCase<R> {
    pub async fn execute(&self, transfer: &Transfer, user: &User) -> Result<(), ApplicationError> {
        // Logique orchestration
    }
}
```

### Étape 4: Implémenter adapter

```rust
// backend/src/infrastructure/database/repositories/transfer_repo.rs
impl TransferRepository for PostgresTransferRepository {
    async fn create(&self, transfer: &Transfer) -> Result<Transfer, DomainError> {
        // Implémentation PostgreSQL avec SQLx
    }
}
```

### Étape 5: Ajouter handler HTTP

```rust
// backend/src/infrastructure/web/handlers/transfer_handler.rs
#[post("/transfers")]
async fn create_transfer(
    user: AuthenticatedUser,
    body: Json<CreateTransferRequest>,
    uc: web::Data<ExecuteTransferUseCase<PostgresTransferRepository>>,
) -> Result<Json<TransferResponse>, ApiError> {
    let transfer = uc.execute(..., &user).await?;
    Ok(Json(TransferResponse::from(&transfer)))
}
```

### Étape 6: Écrire tests

```rust
// Dans backend/src/domain/entities/transfer.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_must_have_positive_amount() {
        let result = Transfer::new(
            IBAN::new("FR76...").unwrap(),
            IBAN::new("FR99...").unwrap(),
            Money::new(Decimal::from(-100), Currency::Eur), // ❌ Négatif!
        );
        assert!(result.is_err());
    }
}
```

## Principes de sécurité bancaire

### 1. Types stricts (pas String!)

```rust
// ✅ BON: IBAN est un type strict
pub struct IBAN(String);

impl IBAN {
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        if !is_valid_iban(value) {
            return Err(ValidationError::InvalidIBAN);
        }
        Ok(IBAN(value.to_string()))
    }
}

// ❌ MAUVAIS: Accepter String pour IBAN
pub fn transfer(from: String, to: String, amount: Decimal) {
    // Pas de validation!
}
```

### 2. Prepared statements (SQLx)

```rust
// ✅ BON: Prepared statement
let user = sqlx::query_as::<_, User>(
    "SELECT * FROM users WHERE id = $1"
)
.bind(user_id)
.fetch_one(&pool)
.await?;

// ❌ MAUVAIS: SQL injection!
let user = sqlx::query_as::<_, User>(
    &format!("SELECT * FROM users WHERE id = '{}'", user_id)
)
.fetch_one(&pool)
.await?;
```

### 3. Audit trail immuable

```rust
// Chaque opération bancaire importante enregistrée
self.audit.log_transfer(
    AuditEntry {
        operation: Operation::Transfer,
        from_account: from_id,
        to_account: to_id,
        amount,
        timestamp: Utc::now(),
        user: Some(user.id),
        status: TransferStatus::Completed,
        ip_address: Some(request.ip()),
    }
).await?;
```

### 4. Validation Domain layer

```rust
impl BankAccount {
    // Validation au niveau métier, JAMAIS dans handler HTTP
    pub fn new(owner: Owner, iban: Iban) -> Result<Self, DomainError> {
        if owner.is_suspended() {
            return Err(DomainError::AccountSuspended);
        }
        // ...
        Ok(Self { /* ... */ })
    }
}
```

## Ressources

- `CLAUDE.md` - Guide Claude Code
- `CONTRIBUTING.md` - Guide contribution
- `SECURITY.md` - Sécurité bancaire détaillée
- `docs/legal/` - Conformité réglementaire
- `docs/bmad/` - Standards bancaires

---

**Dernière mise à jour**: 2026-04-04

Consultez ce guide avant de contribuer au backend BANKO!
