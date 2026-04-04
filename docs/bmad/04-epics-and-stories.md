# Epics & User Stories — BANKO

## Méthode Maury — Phase TOGAF E (Solutions)

**Disciplines** : SOLID + DDD + Hexagonal + BDD + TDD
**Exécution** : Scrum → Nexus → SAFe
**Production** : ITIL + IaC ISO 27001

**Version** : 1.0.0 — 4 avril 2026
**Auteur** : GILMRY / Projet BANKO
**Référentiel légal** : [REFERENTIEL_LEGAL_ET_NORMATIF.md](../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md)

---

## Sprint 0 — Fondations (Scrum)

**Objectif** : Setup complet du projet, structure, toolchain, Docker, CI/CD de base, BDD framework, monitoring.

**Stories techniques** : 13 (STORY-T01 à STORY-T13)

### STORY-T01 | Initialisation repository Git + structure de dossiers
**Type** : Tech | **Taille** : S (1.5h)
**Bounded Context** : Governance (Identity)
**Entité DDD** : AuditTrail (infrastructure)
**SOLID** : Single Responsibility (structure claire des modules)

**User Story** :
> En tant que développeur, je veux un repository Git structuré avec dossiers Rust/Astro/Infra/Docs, afin de commencer le développement dans un environnement clair et reproductible.

**Scénarios BDD** :
```gherkin
Given un nouveau repository BANKO vide
When je crée la structure de dossiers selon Méthode Maury
Then les chemins suivants existent :
  - crates/domain/src/{customer,account,credit,aml,...}
  - crates/application/src
  - crates/infrastructure/src
  - frontend/src/{pages,components,stores}
  - infra/{terraform,ansible,helm}
  - tests/bdd/{features,steps}
  - docs/{bmad,legal,architecture}
And .gitignore exclut /target, /node_modules, .env
And README.md explique la structure
```

**Tâches TDD** :
1. Créer dossiers de base avec Cargo.toml vides
2. Initialiser git + .gitignore
3. Ajouter LICENSE (AGPL-3.0)
4. Créer README.md avec structure
5. Configurer workspace Cargo (membres)
6. Créer package-lock.json vide pour frontend
7. Ajouter .editorconfig (UTF-8, CRLF=auto)
8. Initialiser Terraform provider (stub)
9. Créer GitHub Actions workflow (stub)
10. Documenter la structure dans docs/ARCHITECTURE.md
11. Tester : `cargo check` + `cargo fmt --check`
12. Tester : `npm install` (frontend stub)
13. Commit initial avec tous les chemins
14. Créer branch `develop`
15. Lister les premiers tâches dans issue tracking

**Dépendances** : Aucune

---

### STORY-T02 | Configuration Cargo workspace + dépendances core
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : Infrastructure
**Entité DDD** : —
**SOLID** : Dependency Inversion (dépendances spécifiées via traits)

**User Story** :
> En tant que développeur backend, je veux les dépendances Rust core (actix, sqlx, serde, tokio) configurées, afin de démarrer l'implémentation du domain.

**Scénarios BDD** :
```gherkin
Given un workspace Cargo avec crates/domain, crates/application, crates/infrastructure
When je configure les dépendances Rust
Then les versions suivantes sont fixées :
  - tokio = { version = "1.35", features = ["full"] }
  - actix-web = "4.9"
  - sqlx = { version = "0.8", features = ["postgres", "runtime-tokio-rustls"] }
  - serde = { version = "1.0", features = ["derive"] }
  - serde_json = "1.0"
  - thiserror = "1.0"
  - uuid = { version = "1.6", features = ["v4", "serde"] }
  - chrono = { version = "0.4", features = ["serde"] }
  - tracing = "0.1"
  - tracing-subscriber = "0.3"
And cargo check réussit sans warning
And cargo clippy --all-targets réussit
```

**Tâches TDD** :
1. Éditer Cargo.toml du workspace
2. Ajouter tokio avec features complètes
3. Ajouter actix-web 4.9
4. Ajouter sqlx avec postgres + rustls
5. Ajouter serde + serde_json
6. Ajouter thiserror
7. Ajouter uuid + chrono (avec serde)
8. Ajouter tracing ecosystem
9. Configurer [profile.release] (opt-level=3, lto=true)
10. Exécuter `cargo update`
11. Tester `cargo check` sur tous les crates
12. Tester `cargo clippy` sans warnings
13. Documenter versions dans ARCHITECTURE.md
14. Tester build incremental sur modifications
15. Valider lockfile.

**Dépendances** : STORY-T01

---

### STORY-T03 | Configuration PostgreSQL Docker + migrations basis
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : Infrastructure (Accounting)
**Entité DDD** : —
**SOLID** : Dependency Inversion (DB abstrait via trait Repository)

**User Story** :
> En tant que développeur DBA, je veux une base de données PostgreSQL 16 lancée via Docker Compose, avec migrations Sqlx de base, afin de tester les requêtes SQL.

**Scénarios BDD** :
```gherkin
Given docker-compose.yml vide
When je configure le service PostgreSQL
Then :
  - Image : postgres:16-alpine
  - Port exposé : 5432
  - Env vars : POSTGRES_USER, POSTGRES_PASSWORD, POSTGRES_DB
  - Volume : ./migrations (bind mount)
  - Health check : pg_isready
When je crée migrations/01_init.sql
Then ce fichier contient :
  - CREATE EXTENSION IF NOT EXISTS "uuid-ossp"
  - CREATE SCHEMA IF NOT EXISTS public
  - Commentaire : "Initial schema for BANKO"
When je lance `docker-compose up -d`
Then :
  - Conteneur tourne
  - PostgreSQL accepte connexions en 30s
  - Base 'banko' existe
```

**Tâches TDD** :
1. Créer docker-compose.yml
2. Configurer service PostgreSQL 16
3. Définir variables d'environnement
4. Créer volume pour migrations
5. Ajouter health check pg_isready
6. Créer dossier migrations/
7. Écrire 01_init.sql (extensions, schema)
8. Ajouter script bash pour init migrations
9. Tester `docker-compose up`
10. Tester `docker-compose down`
11. Documenter connexion string (.env.example)
12. Créer .env.local (git-ignored)
13. Ajouter sqlx-cli configuration (sqlx.toml)
14. Tester `sqlx database create`
15. Tester `sqlx migrate run`

**Dépendances** : STORY-T01, STORY-T02

---

### STORY-T04 | BDD framework Cucumber + steps skeleton
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : Governance (tests)
**Entité DDD** : —
**SOLID** : Single Responsibility (chaque file = 1 rôle)

**User Story** :
> En tant que QA, je veux un framework BDD (Cucumber) configuré en Rust, avec fichiers .feature skeleton et steps, afin d'écrire des spécifications vivantes.

**Scénarios BDD** :
```gherkin
Given un dossier tests/bdd/features/ vide
When j'ajoute cucumber-rs aux dev-dependencies
Then tests/features/example.feature contient :
  Feature: Example
    Scenario: Simple check
      Given an input "hello"
      When I process it
      Then output contains "hello"

When j'exécute `cargo test --test bdd`
Then :
  - Les steps manquantes sont listées
  - 3 snippets générés (Given/When/Then)
  - Tests en status "skipped"
```

**Tâches TDD** :
1. Ajouter cucumber-rs aux dev-dependencies
2. Créer tests/bdd/ structure
3. Créer tests/bdd/features/ (features vivantes)
4. Écrire example.feature pour sanity check
5. Créer tests/bdd.rs point d'entrée Cucumber
6. Configurer Cargo.toml pour test BDD
7. Générer snippets (Given/When/Then)
8. Implémentation skeleton des steps
9. Tester `cargo test --test bdd`
10. Créer World struct pour état partagé
11. Ajouter macro #[given], #[when], #[then]
12. Documenter comment ajouter features
13. **Créer .feature skeleton pour TOUS les 12 BC** (Customer, Account, Credit, AML, Sanctions, Prudential, Accounting, Reporting, Payment, ForeignExchange, Governance, Identity) — chaque BC = 1 fichier `tests/bdd/features/{bc_name}.feature` avec scénarios de base
14. **Implémenter step definitions en Rust** (cucumber-rs steps) pour chaque BC avec macros #[given], #[when], #[then]
15. Tester parallélisation des steps
16. Valider sortie Cucumber format
17. Documenter standard naming pour .feature files (kebab-case, 1 par BC)

**Dépendances** : STORY-T01, STORY-T02

---

### STORY-T05 | Astro 6 + Svelte 5 + Tailwind CSS setup
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : Frontend (Identity)
**Entité DDD** : —
**SOLID** : Composition (Svelte = composants composables)

**User Story** :
> En tant que développeur frontend, je veux Astro 6 + Svelte 5 + Tailwind configurés, afin de démarrer les pages du portail.

**Scénarios BDD** :
```gherkin
Given un dossier frontend/ vide
When j'exécute `npm create astro@latest -- --template minimal frontend`
Then :
  - astro.config.mjs existe
  - Intégration Svelte est active
  - Tailwind plugin est actif
When j'ajoute `npm install -D @tailwindcss/typography`
Then :
  - tailwind.config.js contient plugins: [typography]
When je lance `npm run dev`
Then :
  - Serveur Astro écoute sur 3000
  - HMR actif
  - Page de base affichable
```

**Tâches TDD** :
1. Initialiser Astro avec minimal template
2. Ajouter integration Svelte (--yes)
3. Ajouter adapter Node.js (npm install @astrojs/node)
4. Configurer Tailwind CSS
5. Ajouter TypeScript strict
6. Créer src/layouts/Base.astro
7. Créer src/pages/index.astro (home)
8. Créer src/components/Header.svelte (stub)
9. Configurer i18n locale (FR par défaut)
10. Ajouter ESLint (eslint, prettier)
11. Configurer Tailwind pour RTL (mode classe)
12. Créer tailwind.config.js avec RTL support
13. Tester `npm run build`
14. Tester `npm run dev`
15. Documenter structure frontend

**Dépendances** : STORY-T01

---

### STORY-T06 | GitHub Actions CI/CD skeleton
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : Infrastructure
**Entité DDD** : —
**SOLID** : Single Responsibility (1 workflow = 1 responsabilité)

**User Story** :
> En tant que DevOps, je veux des workflows GitHub Actions pour lint, test, build, afin d'automatiser les contrôles qualité.

**Scénarios BDD** :
```gherkin
Given un repository GitHub avec STORY-T01 à T05 complétées
When je crée .github/workflows/ci.yml
Then ce workflow :
  - Trigger sur push à develop et PR
  - Lint Rust (cargo fmt --check)
  - Clippy (cargo clippy --all-targets)
  - Tests unitaires (cargo test --lib)
  - Tests BDD (cargo test --test bdd)
  - Build release (cargo build --release)
When j'ajoute .github/workflows/frontend.yml
Then ce workflow :
  - npm ci
  - npm run lint
  - npm run build
  - Artifact: dist/
When j'ajoute .github/workflows/security.yml
Then ce workflow :
  - cargo audit
  - npm audit
  - SAST: cargo clippy + semgrep
```

**Tâches TDD** :
1. Créer .github/workflows/ directory
2. Écrire ci.yml (Rust + BDD)
3. Ajouter checkout@v4 step
4. Ajouter rust toolchain (latest)
5. Ajouter cargo fmt check
6. Ajouter cargo clippy --all-targets
7. Ajouter cargo test --lib
8. Ajouter cargo test --test bdd
9. Ajouter cargo build --release
10. Écrire frontend.yml
11. Ajouter node setup (20.x)
12. Ajouter npm ci + lint + build
13. Écrire security.yml
14. Ajouter cargo audit
15. Configurer branch protection (require CI pass)

**Dépendances** : STORY-T02, STORY-T03, STORY-T04, STORY-T05

---

### STORY-T07 | Docker Compose production-ready + Traefik
**Type** : Tech | **Taille** : L (5h)
**Bounded Context** : Infrastructure
**Entité DDD** : —
**SOLID** : Dependency Inversion (services découplés)

**User Story** :
> En tant que DevOps, je veux un docker-compose.yml production-ready avec PostgreSQL, API Rust, frontend Astro, Traefik reverse proxy, Prometheus, afin de déployer localement.

**Scénarios BDD** :
```gherkin
Given docker-compose.yml avec tous les services
When je lance `docker-compose -f docker-compose.yml -f docker-compose.override.yml up -d`
Then :
  - api (Rust/Actix) écoute en interne 8000
  - frontend (Astro) écoute en interne 3000
  - Traefik écoute en 80/443
  - http://localhost/api/ redirigée vers api:8000
  - http://localhost/ redirigée vers frontend:3000
  - Prometheus accessible en http://localhost:9090
  - Tous les services passent health checks en <30s
When je fais `curl http://localhost/api/health`
Then réponse 200 JSON avec {status: "ok"}
```

**Tâches TDD** :
1. Éditer docker-compose.yml existant
2. Ajouter service api (build Dockerfile.api)
3. Configurer environment vars (DATABASE_URL, RUST_LOG)
4. Ajouter service frontend (build Dockerfile.frontend)
5. Ajouter service traefik (image traefik:v2.11)
6. Configurer labels Traefik pour routing
7. Ajouter service prometheus (image prom/prometheus)
8. Configurer prometheus.yml pour scrape api:8000/metrics
9. Ajouter networks (banko-net)
10. Ajouter volumes (db data, prometheus data)
11. Créer Dockerfile.api (multi-stage build)
12. Créer Dockerfile.frontend (Node build + Astro export)
13. Tester `docker-compose config` (validation YAML)
14. Tester `docker-compose up -d`
15. Tester health checks avec `docker-compose ps`

**Dépendances** : STORY-T02, STORY-T03, STORY-T05, STORY-T06

---

### STORY-T08 | Monitoring Prometheus + Grafana setup
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : Infrastructure
**Entité DDD** : —
**SOLID** : Single Responsibility (metrics = 1 responsabilité)

**User Story** :
> En tant qu'ingénieur opérationnel, je veux Prometheus + Grafana configurés, avec dashboards de base, afin de monitorer l'API.

**Scénarios BDD** :
```gherkin
Given api Rust expose les metrics en GET /metrics (format Prometheus)
When je crée prometheus.yml avec :
  global:
    scrape_interval: 15s
  scrape_configs:
    - job_name: 'banko-api'
      static_configs:
        - targets: ['api:8000']
Then :
  - Prometheus scrape api:8000/metrics
  - Métriques disponibles en http://localhost:9090
When j'ajoute Grafana (image grafana/grafana)
Then :
  - Interface à http://localhost:3001
  - Data source Prometheus configurée
  - Dashboard "BANKO API Overview" créé
```

**Tâches TDD** :
1. Ajouter prometheus-rs aux dépendances API
2. Configurer middleware Prometheus dans Actix
3. Exposer GET /metrics endpoint
4. Créer infra/prometheus/prometheus.yml
5. Configurer scrape_configs pour api + db + frontend
6. Ajouter service Grafana à docker-compose.yml
7. Monter provisioning config Grafana
8. Créer datasource.yml (Prometheus)
9. Créer dashboard JSON (CPU, RAM, latency, requests)
10. Configurer alertes Prometheus (règles de base)
11. Tester scrape_interval et data freshness
12. Ajouter Alertmanager stub
13. Créer dashboards Loki pour logs
14. Tester Grafana alerts
15. Documenter métriques clés

**Dépendances** : STORY-T07

---

### STORY-T09 | Domain + Application DTOs shared + Value Objects stubs
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : All (shared)
**Entité DDD** : ValueObject (Money, AccountNumber, etc.)
**SOLID** : Open/Closed (extensible pour chaque BC)

**User Story** :
> En tant qu'architecte domain, je veux les ValueObjects fondamentaux (Money, AccountNumber, CustomerId, etc.) + DTOs partagés, afin que tous les BCs utilisent les mêmes abstractions.

**Scénarios BDD** :
```gherkin
Given crates/domain/src/shared/
When j'ajoute value_objects.rs avec :
  pub struct Money { currency: Currency, amount: Decimal }
  pub struct AccountNumber(String)
  pub struct CustomerId(Uuid)
  pub struct RIB(String)
Then :
  - Money::new(5000.0, Currency::TND) crée une valeur valide
  - Money implémente PartialEq, Clone, Serialize
  - RIB::validate_format() retourne Result<Self, ValidationError>
  - AccountNumber parse "01-234-0001234-56" correctement
When j'ajoute errors.rs avec DomainError enum
Then :
  - InvalidMoney, InvalidRIB, InvalidAccountNumber variantes
  - Tous les ? propagent DomainError
```

**Tâches TDD** :
1. Créer crates/domain/src/shared/ module
2. Définir Currency enum (TND, EUR, USD)
3. Implémenter Money struct avec Decimal
4. Implémenter AccountNumber avec validation
5. Implémenter CustomerId (UUID wrapper)
6. Implémenter RIB avec regex validation
7. Implémenter BIC pour virements
8. Ajouter trait Display pour tous les VO
9. Ajouter trait Hash pour collections
10. Créer DomainError enum (thiserror)
11. Implémenter From<> conversions (ValidationError → DomainError)
12. Ajouter unit tests pour chaque VO
13. Ajouter crates/application/src/dto/ module
14. Créer DTOs partagés (Serde-compatible)
15. Documenter contrats de validation

**Dépendances** : STORY-T02

---

### STORY-T10 | Security CI (cargo audit + clippy security rules)
**Type** : Tech | **Taille** : S (1.5h)
**Bounded Context** : Infrastructure
**Entité DDD** : —
**SOLID** : Single Responsibility

**User Story** :
> En tant que responsable sécurité, je veux des contrôles automatisés (cargo audit, clippy security rules) exécutés en CI, afin de détecter les vulnérabilités de dépendances.

**Scénarios BDD** :
```gherkin
Given .github/workflows/security.yml existe
When je crée cargo-deny.toml avec advisories
Then :
  - Toutes les vulnérabilités connues bloquent la build
When j'ajoute clippy-rules pour patterns dangereux
Then :
  - `unwrap()` sans documentation → warning
  - `panic!()` sans test → error
When la CI s'exécute
Then :
  - cargo audit --deny warnings
  - cargo deny check advisories
  - npm audit (frontend)
```

**Tâches TDD** :
1. Créer Cargo.toml [workspace.package]
2. Configurer cargo-audit dans CI
3. Créer cargo-deny.toml
4. Ajouter deny check advisories step
5. Configurer clippy lints (security subset)
6. Ajouter clippy::all + clippy::security
7. Documenter unsafe usage
8. Ajouter SECURITY.md (disclosure)
9. Configurer dependabot (GitHub)
10. Tester cargo audit sur advisories réelles
11. Ajouter rustfmt check
12. Ajouter frontend npm audit
13. Créer gitignore pour dépendances sensibles
14. Documenter secrets handling
15. Configurer audit logging

**Dépendances** : STORY-T06

---

### STORY-T11 | Git hooks (pre-commit : fmt, clippy, test)
**Type** : Tech | **Taille** : S (1.5h)
**Bounded Context** : Infrastructure
**Entité DDD** : —
**SOLID** : Single Responsibility

**User Story** :
> En tant que développeur, je veux des git hooks pré-commit (format, clippy, tests rapides), afin d'éviter de pusher du code non-conforme.

**Scénarios BDD** :
```gherkin
Given .git/hooks/ vide
When j'ajoute pre-commit hook avec :
  cargo fmt --all --check
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test --lib --doc (quick subset)
Then :
  - `git commit` échoue si fmt/clippy invalides
  - Message d'erreur clair
When je corrige avec `cargo fmt --all`
Then `git commit` réussit
```

**Tâches TDD** :
1. Créer scripts/install-hooks.sh
2. Créer .githooks/pre-commit executable
3. Ajouter cargo fmt --all --check
4. Ajouter cargo clippy --all-targets
5. Ajouter cargo test --lib (fast only)
6. Ajouter git config core.hooksPath .githooks
7. Documenter : `bash scripts/install-hooks.sh`
8. Tester hook avec commit invalide
9. Tester hook avec commit valide
10. Ajouter pre-push hook (full tests)
11. Ajouter commit-msg hook (conventional commits)
12. Documenter message format (feat:, fix:)
13. Ajouter support pour bypass (--no-verify)
14. Tester sur macOS et Linux
15. Documenter setup

**Dépendances** : STORY-T02

---

### STORY-T12 | Terraform IaC stub (OVH Cloud provider)
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : Infrastructure
**Entité DDD** : —
**SOLID** : Infrastructure as Code principle

**User Story** :
> En tant qu'ingénieur infrastructure, je veux les fichiers Terraform pour OVH Cloud (VPC, compute, database), afin de déployer BANKO en production.

**Scénarios BDD** :
```gherkin
Given infra/terraform/ structure
When je crée main.tf avec provider "ovh"
Then :
  - Provider version pinned
  - Variables définies (region, size, password)
When je crée vpc.tf avec ressources VPC
Then :
  - Subnet créée (10.0.0.0/24)
  - Security groups définis (ingress 80, 443, 5432)
When je crée compute.tf avec instances
Then :
  - 1 instance API (8 vCPU, 16GB RAM)
  - 1 instance frontend (4 vCPU, 8GB RAM)
  - Images Alpine + Rust toolchain
When je crée database.tf avec PostgreSQL
Then :
  - Database managed (RDS equivalent)
  - Backup automatique 7j
  - Snapshots quotidiens
When j'exécute `terraform plan`
Then les ressources listées sans erreur
```

**Tâches TDD** :
1. Créer infra/terraform/ structure
2. Créer main.tf (provider ovh config)
3. Créer variables.tf (region, sizing, secrets)
4. Créer outputs.tf (URLs, IPs)
5. Créer vpc.tf (VPC, subnets, security groups)
6. Créer compute.tf (instances API + frontend)
7. Créer database.tf (managed PostgreSQL)
8. Créer storage.tf (S3 backups, logs)
9. Créer monitoring.tf (CloudWatch stubs)
10. Ajouter terraform.tfvars.example
11. Créer .gitignore (terraform state secrets)
12. Configurer remote state (S3)
13. Ajouter -var-file validation
14. Tester `terraform fmt` (style)
15. Documenter : terraform init, plan, apply

**Dépendances** : STORY-T01

---

### STORY-T13 | Documentation vivante (README, CONTRIBUTING, architecture diagrams)
**Type** : Tech | **Taille** : M (3h)
**Bounded Context** : Governance
**Entité DDD** : —
**SOLID** : Documentation = Communication

**User Story** :
> En tant que contributeur, je veux une documentation claire (README, CONTRIBUTING, architecture diagrams Mermaid), afin de comprendre le projet rapidement.

**Scénarios BDD** :
```gherkin
Given docs/ structure vide
When je crée README.md avec :
  - Vision BANKO en 1 phrase
  - Quick start (docker-compose)
  - 12 BCs listés avec icônes
  - Lien vers Product Brief
Then README s'affiche bien sur GitHub
When je crée CONTRIBUTING.md avec :
  - Setup local (bash scripts/setup.sh)
  - Conventions de code (SOLID, DDD, TDD)
  - Workflow PR (fork, branch, commit message)
  - Testing (cargo test + BDD)
Then les contributeurs savent par où commencer
When je crée architecture diagrams (Mermaid)
Then :
  - Système global (12 BCs et leurs dépendances)
  - Domain Model (sample)
  - API routes (sample)
  - Infrastructure (Docker Compose visuel)
```

**Tâches TDD** :
1. Créer README.md avec vision + quick start
2. Ajouter table d'installation par OS (Windows/Mac/Linux)
3. Documenter stack (Rust, Astro, PostgreSQL, etc.)
4. Ajouter lien au Product Brief
5. Créer CONTRIBUTING.md (onboarding)
6. Documenter commit messages (conventional)
7. Ajouter lien à REFERENTIEL_LEGAL_ET_NORMATIF.md
8. Créer CODE_OF_CONDUCT.md
9. Créer ARCHITECTURE.md avec diagrammes Mermaid
10. Créer docs/GLOSSARY.md (Ubiquitous Language)
11. Créer docs/METRICS.md (métriques de succès)
12. Ajouter SECURITY.md (vulnérabilité disclosure)
13. Tester liens Markdown
14. Ajouter badges (license, build, coverage)
15. Documenter quick links vers GitHub issues

**Dépendances** : STORY-T01 à STORY-T12

---

## Definition of Done (DoD)

Chaque story doit satisfaire aux critères suivants avant d'être marquée "Done" :

### Code Quality
- ✅ **Gherkin tests** : Tous les scénarios Gherkin passent (`cargo test --test bdd`)
- ✅ **Domain unit tests** : Couverture ≥ 100% sur la couche domain (coverage report généré)
- ✅ **Integration tests** : Couverture ≥ 80% sur API handlers + use cases
- ✅ **Clippy warnings** : `cargo clippy --all-targets` sans avertissements
- ✅ **Format** : `cargo fmt --check` sans différence (code formaté)

### Security & Dependencies
- ✅ **Audit sécurité** : `cargo audit` sans vulnérabilité critique
- ✅ **Pas de secrets hardcodés** : Variables d'env pour clés API, tokens, passwords
- ✅ **Documentation inline** : Chaque `pub fn` et `pub struct` a des doc comments (`/// ...`)

### Testing & Verification
- ✅ **Scénarios BDD exécutables** : Tous les Given/When/Then avec step definitions en Rust
- ✅ **Pas de TODO/FIXME/HACK** : Zéro tolérances pour les balises temporaires
- ✅ **Playwright E2E** (frontend) : Scénarios utilisateur critiques testés
- ✅ **WCAG 2.1 AA** (frontend) : Accessibility checklist complétée

### Documentation & Process
- ✅ **CHANGELOG.md** : Mise à jour si feature (format : `## [Unreleased]`)
- ✅ **Code review** : Au minimum 1 approbation (solo-dev : self-review checklist)
- ✅ **Performance** : Pas de régression P95 > 200ms (API calls)
- ✅ **Migrations DB** : Si modification du schema, migration SQLx versionnée + test de rollback

### Example DoD Checklist per Story
```
Story: STORY-T04 | BDD Framework Setup

☐ Tous les scénarios example.feature passent
☐ cucumber-rs configuré et exécutable
☐ World struct implémentée
☐ 12 fichiers .feature skeleton créés (un par BC)
☐ Step definitions en Rust pour chaque scénario de base
☐ cargo test --test bdd réussit
☐ Documentation: docs/BDD_GUIDE.md créée
☐ Pas de TODO dans le code
☐ Code revu par 1 développeur (ou self-review checklist complétée)
```

---

## Epic 1 : Identity (BC12) — Must Have

**Objectif** : Authentification, autorisations RBAC, sessions sécurisées, 2FA.
**Priorité** : P0
**Bounded Context** : Identity (BC12)
**Entités DDD** : User, Role, Permission, Session, TwoFactorAuth, Credential
**Invariants critiques** : INV-13 (consentement INPDP)

---

### STORY-ID-01 | Domain: User aggregate + invariants
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : User (AggregateRoot), Credential (ValueObject), UserId (ValueObject)

**User Story** :
> En tant qu'architecte domain, je veux l'agrégat User modélisé avec invariants (email unique, password hashé, roles immuables), afin de protéger l'identité des utilisateurs.

**Scénarios BDD** :
```gherkin
Given un contexte Identity vide
When je crée User aggregate avec :
  - user_id: Uuid
  - email: String (unique constraint)
  - password_hash: String (argon2)
  - roles: Vec<Role> (immutable)
  - created_at: DateTime
  - is_active: bool
Then :
  - User::new(email, password_plain) → Result<User, DomainError>
  - password_hash ne peut pas être vide
  - email doit avoir @ valide
  - roles par défaut = [Role::User]
When je crée User("admin@banko.tn", "password")
Then password_hash != "password" (hashing appliqué)
When je change roles directement
Then erreur de compilation (champ private)
When j'appelle user.assign_role(Role::Admin)
Then user.roles contient Admin (via builder pattern)
```

**Tâches TDD** :
1. Créer crates/domain/src/identity/ module
2. Définir UserId(Uuid) ValueObject
3. Définir Email(String) ValueObject avec regex validation
4. Définir PasswordHash(String) ValueObject
5. Créer User AggregateRoot struct
6. Implémenter User::new() avec validation complète
7. Ajouter password hashing (argon2 ou bcrypt)
8. Créer Role enum (User, Admin, Analyst, Compliance, CRO)
9. Implémenter Permission enum (create_account, view_audit, etc.)
10. Ajouter unit tests pour chaque invariant
11. Tester que password_plain n'est pas stocké
12. Tester que email est unique (DB constraint futur)
13. Documenter SOLID (User = Single Responsibility)
14. Ajouter user.is_admin() → bool helper
15. Ajouter user.has_permission(perm) → bool

**Dépendances** : STORY-T09 (shared ValueObjects)

---

### STORY-ID-02 | Application: UserService + ports
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : UserService (use case), IUserRepository (port)

**User Story** :
> En tant que développeur application, je veux un UserService qui implémente register, login, password_reset avec ports pour persistence, afin de découpler du détail DB.

**Scénarios BDD** :
```gherkin
Given un UserService avec dépendance : IUserRepository
When j'appelle user_service.register(email, password)
Then :
  - Email est validé (format)
  - Password est hashé (argon2)
  - repository.save(user) est appelé
  - Result<UserId, RegisterError> retourné
When j'appelle user_service.login(email, password)
Then :
  - User récupéré via repository.find_by_email()
  - Password comparé via argon2::verify()
  - Result<(UserId, session_token), LoginError>
When email n'existe pas
Then LoginError::UserNotFound
When password incorrect
Then LoginError::InvalidPassword
```

**Tâches TDD** :
1. Créer crates/application/src/identity/ module
2. Définir IUserRepository trait (port)
3. Créer UserService struct
4. Implémenter UserService::register()
5. Implémenter UserService::login()
6. Implémenter UserService::password_reset()
7. Créer DTOs (RegisterRequest, LoginRequest, UserResponse)
8. Ajouter validation DTOs (email format, password strength)
9. Implémenter password strength check (min 12 chars, uppercase, digit)
10. Ajouter logging pour login attempts
11. Créer RegisterError enum (EmailTaken, WeakPassword, etc.)
12. Créer LoginError enum (UserNotFound, InvalidPassword, AccountLocked)
13. Ajouter unit tests pour tous les scénarios
14. Documenter SOLID (dependency injection via trait)
15. Tester error messages (pas de leak d'info)

**Dépendances** : STORY-ID-01

---

### STORY-ID-03 | Infrastructure: UserRepository PostgreSQL adapter
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : UserRepository (adapter)

**User Story** :
> En tant que développeur infrastructure, je veux l'implémentation PostgreSQL de IUserRepository avec migrations SQL, afin de persister les utilisateurs.

**Scénarios BDD** :
```gherkin
Given migrations/02_identity_schema.sql vide
When je crée tables :
  CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    roles TEXT[] DEFAULT '{user}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
  )
Then la table est créée avec indexes sur email
When j'implémente UserRepository avec sqlx
Then :
  - save(user) → exécute INSERT OR UPDATE
  - find_by_email(email) → SELECT * WHERE email
  - find_by_id(id) → SELECT * WHERE id
  - list_all() → SELECT * (pour admin)
When je sauvegarde un User
Then user_id et timestamp sont persistes
```

**Tâches TDD** :
1. Créer migrations/02_identity_schema.sql
2. Définir CREATE TABLE users
3. Ajouter UNIQUE constraint sur email
4. Ajouter index sur email
5. Ajouter audit columns (created_at, updated_at)
6. Exécuter migration via sqlx migrate run
7. Créer crates/infrastructure/src/identity/ module
8. Implémenter UserRepository struct (sqlx pool)
9. Implémenter IUserRepository trait
10. Ajouter sqlx::query_as! pour type safety
11. Implémenter save() avec sqlx::query
12. Implémenter find_by_email() avec sqlx
13. Implémenter find_by_id() avec sqlx
14. Ajouter error handling (sqlx::Error → DomainError)
15. Tester avec docker-compose PostgreSQL

**Dépendances** : STORY-T03, STORY-ID-02

---

### STORY-ID-04 | API: POST /auth/register endpoint
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : Handler (infrastructure)

**User Story** :
> En tant qu'utilisateur web, je veux un endpoint POST /auth/register avec validation JSON, afin de créer un compte.

**Scénarios BDD** :
```gherkin
Given API Actix démarrée
When je POST /auth/register avec :
  {
    "email": "john@example.com",
    "password": "SecurePass123!"
  }
Then :
  - Status 201 Created
  - Response: { "user_id": "uuid-xxx", "email": "john@example.com" }
When je POST avec email invalide
Then :
  - Status 400 Bad Request
  - Response: { "error": "Invalid email format" }
When je POST avec password faible
Then :
  - Status 400 Bad Request
  - Response: { "error": "Password must be at least 12 characters" }
When email existe déjà
Then :
  - Status 409 Conflict
  - Response: { "error": "Email already registered" }
```

**Tâches TDD** :
1. Créer crates/infrastructure/src/handlers/ module
2. Créer identity_handlers.rs
3. Définir RegisterRequest struct (Deserialize)
4. Implémenter register_handler(request, service)
5. Valider email format
6. Valider password strength
7. Ajouter error response (serde_json)
8. Implémenter status 201 Created
9. Configurer route POST /auth/register en main.rs
10. Ajouter middleware CORS
11. Ajouter logging (tracing)
12. Tester avec curl
13. Ajouter Content-Type validation
14. Ajouter rate limiting (stub)
15. Documenter API endpoint

**Dépendances** : STORY-ID-03

---

### STORY-ID-05 | API: POST /auth/login endpoint
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : Handler, Session (aggregate)

**User Story** :
> En tant qu'utilisateur, je veux un endpoint POST /auth/login qui retourne un JWT, afin de m'authentifier.

**Scénarios BDD** :
```gherkin
Given endpoint POST /auth/register déjà utilisé (compte créé)
When je POST /auth/login avec :
  {
    "email": "john@example.com",
    "password": "SecurePass123!"
  }
Then :
  - Status 200 OK
  - Response: { "access_token": "eyJhbGc...", "token_type": "Bearer", "expires_in": 3600 }
When password incorrect
Then :
  - Status 401 Unauthorized
  - Response: { "error": "Invalid credentials" }
When je crée session avec token JWT
Then claims contiennent : user_id, email, roles, iat, exp
When token expiré (>1h)
Then endpoints sécurisés retournent 401
```

**Tâches TDD** :
1. Ajouter jsonwebtoken aux dépendances
2. Créer JwtConfig struct (secret, expiry)
3. Implémenter login_handler()
4. Créer JwtClaims struct (Serialize + Deserialize)
5. Implémenter claims creation (user_id, email, roles, exp)
6. Implémenter JWT encoding avec secret
7. Configurer token expiry (1 heure par défaut)
8. Ajouter Session aggregate (domain)
9. Implémenter session persistence (optionnel)
10. Tester JWT decoding
11. Ajouter refresh token endpoint (POST /auth/refresh)
12. Implémenter logout endpoint (POST /auth/logout)
13. Ajouter CORS headers pour auth
14. Documenter token format
15. Tester avec postman/curl

**Dépendances** : STORY-ID-04

---

### STORY-ID-06 | API: JWT middleware + protected endpoints
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : Middleware (infrastructure)

**User Story** :
> En tant que développeur, je veux un middleware JWT qui valide le token sur les endpoints sécurisés, afin de protéger les ressources.

**Scénarios BDD** :
```gherkin
Given API avec endpoint /api/profile (protégé)
When je POST /auth/login et reçois access_token
Then je peux GET /api/profile avec Authorization: Bearer <token>
And réponse : { "user_id": "...", "email": "...", "roles": [...] }
When je GET /api/profile sans token
Then Status 401 Unauthorized
When je GET /api/profile avec token invalide
Then Status 401 Unauthorized
When je GET /api/profile avec token expiré
Then Status 401 Unauthorized
```

**Tâches TDD** :
1. Créer JwtMiddleware struct
2. Implémenter extracteur Actix ExtractorError
3. Configurer extracteur pour Bearer token
4. Implémenter token validation (signature + exp)
5. Tester middleware sur route /api/profile
6. Ajouter ClaimsExtractor pour accéder à claims
7. Implémenter role-based access control
8. Ajouter #[require_role(Admin)] macro (optionnel)
9. Documenter Bearer token format
10. Tester 401 responses
11. Ajouter request logging (qui, quand, endpoint)
12. Implementer token refresh endpoint
13. Ajouter cors headers pour preflight
14. Tester avec curl + authorization header
15. Documenter pour frontend

**Dépendances** : STORY-ID-05

---

### STORY-ID-07 | API: POST /users (admin only) + GET /users/{id}
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : Handler, Role (value object)

**User Story** :
> En tant qu'administrateur, je veux créer des utilisateurs et consulter leurs profils, afin de gérer les accès.

**Scénarios BDD** :
```gherkin
Given user avec role Admin authentifié
When je POST /users avec :
  {
    "email": "analyst@banko.tn",
    "password": "SecurePass456!",
    "roles": ["Analyst"]
  }
Then :
  - Status 201 Created
  - Response: { "user_id": "...", "email": "analyst@banko.tn", "roles": ["Analyst"] }
When user sans Admin essaie de POST /users
Then Status 403 Forbidden
When je GET /users/{user_id}
Then Status 200 OK + user profile complet
When je GET /users/{user_id} (autre user non-admin)
Then Status 403 Forbidden (sauf si c'est son propre profil)
```

**Tâches TDD** :
1. Ajouter require_role(Admin) check
2. Créer CreateUserRequest DTO
3. Implémenter POST /users handler
4. Valider email unique
5. Valider roles (whitelist: Admin, Analyst, Compliance, CRO, User)
6. Appeler UserService::register()
7. Implémenter GET /users/{user_id} handler
8. Ajouter row-level security (own profile visible sans Admin)
9. Créer UserProfileResponse DTO
10. Tester 403 responses
11. Ajouter audit logging (qui a créé quel user)
12. Implémenter GET /users (list all, Admin only)
13. Ajouter pagination pour list
14. Documenter endpoints
15. Tester avec Postman

**Dépendances** : STORY-ID-06

---

### STORY-ID-08 | API: PUT /users/{id}/roles (super-admin only)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : Permission (value object)

**User Story** :
> En tant que super-administrateur, je veux modifier les rôles d'un utilisateur, afin de gérer les permissions.

**Scénarios BDD** :
```gherkin
Given user avec role SuperAdmin authentifié
When je PUT /users/{user_id}/roles avec :
  {
    "roles": ["Analyst", "Compliance"]
  }
Then :
  - Status 200 OK
  - User roles mis à jour
  - Audit trail logs "Admin X changed user Y roles to [...]"
When je PUT avec roles invalides
Then Status 400 Bad Request
When user non-SuperAdmin essaie
Then Status 403 Forbidden
```

**Tâches TDD** :
1. Créer UpdateUserRolesRequest DTO
2. Implémenter PUT /users/{id}/roles handler
3. Valider roles (whitelist)
4. Appeler UserService::update_roles()
5. Ajouter audit logging
6. Tester 403 (non-SuperAdmin)
7. Implémenter role removal (empty array)
8. Ajouter event publishing (UserRolesChanged)
9. Documenter role hierarchy
10. Tester transaction rollback on error
11. Ajouter soft constraint : user ne peut pas retirer son propre Admin
12. Documenter audit trail
13. Tester avec curl
14. Ajouter metrics (role changes count)
15. Valider avec tests

**Dépendances** : STORY-ID-07

---

### STORY-ID-09 | Feature: 2FA TOTP (Time-based One-Time Password)
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : TwoFactorAuth (aggregate)

**User Story** :
> En tant qu'utilisateur, je veux activer 2FA TOTP, afin de sécuriser mon compte.

**Scénarios BDD** :
```gherkin
Given user authentifié
When je POST /auth/2fa/enable
Then :
  - Status 200 OK
  - Response: { "secret": "JBSWY3DPEHPK3PXP", "qr_code_url": "..." }
When je sauvegarde secret dans authenticator app
And POST /auth/2fa/verify avec { "totp_code": "123456" }
Then :
  - 2FA activée
  - Audit log
When je login maintenant
Then demande TOTP code après password
When je fournis TOTP correct
Then reçois JWT
When je fournis TOTP incorrect (>3 fois)
Then compte lockout temporaire
```

**Tâches TDD** :
1. Ajouter totp-lite dépendance
2. Créer TwoFactorAuth aggregate
3. Implémenter generate_secret() → TOTP secret
4. Implémenter qr_code_url(secret) → QR code URL
5. Implémenter verify_totp(code, secret) → bool
6. Ajouter TwoFactorAuth table migration
7. Implémenter POST /auth/2fa/enable handler
8. Implémenter POST /auth/2fa/verify handler
9. Modifier login flow : password → 2FA code
10. Ajouter temp session après password valid
11. Implémenter grace period (skip 2FA once)
12. Ajouter backup codes generation
13. Documenter authenticator setup
14. Tester avec FreeOTP app
15. Ajouter metrics (2FA adoption rate)

**Dépendances** : STORY-ID-05

---

### STORY-ID-10 | Feature: Session management + expiry
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Identity (BC12)
**Entité DDD** : Session (aggregate)

**User Story** :
> En tant que système, je veux gérer les sessions (création, expiry, logout), afin d'éviter les accès zombies.

**Scénarios BDD** :
```gherkin
Given user login avec JWT
When session créée avec :
  - session_id: Uuid
  - user_id: Uuid
  - token: JWT
  - created_at: DateTime
  - expires_at: DateTime (1 hour)
  - ip_address: String
  - user_agent: String
Then session sauvegardée en DB
When je fais requête avec token
Then IP et User-Agent matchent (sinon 401)
When session expires_at < now()
Then token invalide
When je POST /auth/logout
Then session supprimée
And token invalide immédiatement
```

**Tâches TDD** :
1. Créer Session aggregate
2. Ajouter sessions table migration
3. Implémenter SessionRepository
4. Implémenter create_session()
5. Implémenter find_session(token)
6. Implémenter delete_session(session_id)
7. Implémenter cleanup_expired_sessions() (cron job)
8. Ajouter IP + User-Agent validation
9. Implémenter POST /auth/logout handler
10. Documenter session lifetime (1 hour)
11. Ajouter refresh token (7 days)
12. Implémenter token rotation (logout + re-login)
13. Ajouter security audit (login/logout timestamps)
14. Tester session invalidation
15. Documenter avec diagramme de flux

**Dépendances** : STORY-ID-05

---

## Epic 2 : Customer / KYC (BC1) — Must Have

(Continuing with BC1 stories...)

### STORY-C01 | Domain: Customer aggregate + KYC profile
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Customer (BC1)
**Entité DDD** : Customer (AggregateRoot), KycProfile (ValueObject), Beneficiary (Entity)
**Invariants** : INV-01, INV-10, INV-13

**User Story** :
> En tant qu'architecte domain, je veux l'agrégat Customer avec KYC complet (Circ. 2025-17), afin de respecter les exigences de Know Your Customer.

**Scénarios BDD** :
```gherkin
Given domaine Customer vide
When je crée Customer aggregate avec :
  - customer_id: Uuid
  - customer_type: Enum (Individual | LegalEntity)
  - kyc_profile: KycProfile (complet)
  - beneficiaries: Vec<Beneficiary>
  - risk_score: RiskScore
  - status: Enum (Pending | Approved | Rejected | Suspended)
Then Customer est créé avec invariants :
  - kyc_profile.is_complete() == true
  - risk_score >= 0 && risk_score <= 100
  - beneficiaries valides pour type LegalEntity
When je crée Customer(Individual)
Then kyc_profile contient :
  - full_name: String
  - cin: String (Carte d'Identité Nationale)
  - birth_date: Date
  - nationality: String (Tunisia)
  - profession: String
  - address: Address
  - phone: PhoneNumber
  - email: Email
  - pep_status: Enum (Yes | No | Unknown)
  - source_of_funds: Enum (Salary | Business | Investment | Other)
When je crée Customer(LegalEntity)
Then kyc_profile contient :
  - company_name: String
  - registration_number: String (RCS)
  - sector: String (Enum: Banking, Retail, etc.)
  - beneficial_owners: Vec<Beneficiary>
When je crée LegalEntity sans beneficial_owners
Then erreur DomainError::MissingBeneficiaries
```

**Tâches TDD** :
1. Créer crates/domain/src/customer/ module
2. Définir CustomerId(Uuid) ValueObject
3. Définir CustomerType enum (Individual | LegalEntity)
4. Définir KycProfile struct avec tous les champs requis
5. Implémenter KycProfile::validate() → Result<(), ValidationError>
6. Créer Address ValueObject (rue, ville, codepostal)
7. Créer PhoneNumber ValueObject (validation Tunisie +216)
8. Définir PepStatus enum (Politically Exposed Person)
9. Créer RiskScore ValueObject (0-100)
10. Créer Beneficiary entity
11. Implémenter Customer::new() avec validation
12. Ajouter invariant : LegalEntity ⇒ >= 1 beneficial_owner
13. Implémenter customer.is_kyc_complete() → bool
14. Ajouter customer.update_kyc(profile) avec tracking
15. Tester tous les scénarios BDD

**Dépendances** : STORY-T09 (shared ValueObjects)

---

### STORY-C02 | Application: KycService + ports
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1)
**Entité DDD** : KycService (use case)

**User Story** :
> En tant qu'analyste conformité, je veux un KycService pour soumettre, valider, rejeter des KYCs, afin de respecter Circ. 2025-17.

**Scénarios BDD** :
```gherkin
Given KycService avec ports ICustomerRepository, IPepService, IAmlService
When j'appelle kyc_service.submit_kyc(customer_id, kyc_profile)
Then :
  - customer.status = Pending
  - PEP check déclenché automatiquement
  - AML check déclenché automatiquement
  - Audit log créé
When pep_check retourne true (PEP détecté)
Then kyc_service.evaluate_kyc() → Decision::RequireEdd (Enhanced Due Diligence)
When je rejette avec raison
Then kyc_service.reject_kyc(customer_id, reason)
And customer.status = Rejected
And notification envoyée
```

**Tâches TDD** :
1. Créer KycService struct
2. Implémenter ICustomerRepository port
3. Implémenter IPepService port (interface)
4. Implémenter IAmlService port (interface)
5. Créer KycSubmitRequest DTO
6. Implémenter submit_kyc()
7. Implémenter evaluate_kyc()
8. Implémenter approve_kyc()
9. Implémenter reject_kyc()
10. Créer Decision enum (Approve | Reject | RequireEdd)
11. Ajouter PEP check logic
12. Ajouter AML check logic
13. Implémenter audit logging
14. Ajouter notification events (UserNotification)
15. Tester tous les workflows

**Dépendances** : STORY-C01

---

### STORY-C03 | Infrastructure: CustomerRepository + KYC table
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1)

**User Story** :
> En tant que DBA, je veux les tables PostgreSQL pour customers et kyc_profiles, afin de persister les données KYC.

**Scénarios BDD** :
```gherkin
Given migrations/03_customer_schema.sql
When je crée tables :
  CREATE TABLE customers (
    id UUID PRIMARY KEY,
    customer_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) DEFAULT 'Pending',
    risk_score INT CHECK (risk_score >= 0 AND risk_score <= 100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
  )

  CREATE TABLE kyc_profiles (
    id UUID PRIMARY KEY,
    customer_id UUID REFERENCES customers(id),
    full_name_or_company VARCHAR(255),
    cin_or_rcs VARCHAR(100),
    pep_status VARCHAR(50),
    source_of_funds VARCHAR(100),
    submission_date TIMESTAMP,
    approval_date TIMESTAMP,
    rejection_reason TEXT
  )
Then migrations appliquées sans erreur
```

**Tâches TDD** :
1. Créer migrations/03_customer_schema.sql
2. Créer TABLE customers
3. Ajouter CHECK constraints (risk_score)
4. Créer TABLE kyc_profiles
5. Ajouter FOREIGN KEY customer_id
6. Créer TABLE beneficiaries (LegalEntity)
7. Ajouter migrations/04_beneficiaries.sql
8. Implémenter CustomerRepository (sqlx)
9. Implémenter KycProfileRepository
10. Implémenter save() pour Customer
11. Implémenter find_by_id()
12. Implémenter list_pending_kyc()
13. Ajouter indexes sur customer_type, status
14. Exécuter migrations sur Docker PostgreSQL
15. Tester queries complexes

**Dépendances** : STORY-T03, STORY-C02

---

### STORY-C04 | API: POST /customers (customer creation)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1)

**User Story** :
> En tant que chargé clientèle, je veux créer un client avec sa KYC, afin de l'enregistrer dans le système.

**Scénarios BDD** :
```gherkin
Given endpoint /customers
When je POST /customers avec :
  {
    "customer_type": "Individual",
    "full_name": "Ahmed Ben Ayed",
    "cin": "12345678",
    "birth_date": "1990-01-15",
    "nationality": "Tunisia",
    "profession": "Banker",
    "address": { ... },
    "phone": "+216 98 123 456",
    "email": "ahmed@example.com",
    "pep_status": "No",
    "source_of_funds": "Salary"
  }
Then :
  - Status 201 Created
  - Response: { "customer_id": "...", "status": "Pending", "kyc_status": "Pending" }
  - PEP check déclenché
  - Audit log créé
When données invalides
Then Status 400 Bad Request
```

**Tâches TDD** :
1. Créer CreateCustomerRequest DTO
2. Implémenter POST /customers handler
3. Valider KYC data (all required fields)
4. Valider CIN format (8 chiffres)
5. Valider phone format (+216 xxxx xxxx)
6. Valider email
7. Appeler KycService::submit_kyc()
8. Retourner 201 Created
9. Ajouter CORS headers
10. Documenter API endpoint
11. Tester avec curl
12. Ajouter tracing/logging
13. Implémenter idempotency key (optionnel)
14. Tester validation errors
15. Documenter réponse

**Dépendances** : STORY-C03

---

### STORY-C05 | API: GET /customers/{id} + GET /customers/{id}/kyc
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1)

**User Story** :
> En tant qu'analyste, je veux consulter un client et son KYC, afin de vérifier les données.

**Scénarios BDD** :
```gherkin
Given customer créé avec KYC Pending
When je GET /customers/{customer_id}
Then :
  - Status 200 OK
  - Response: { customer_id, customer_type, status, risk_score, ... }
When je GET /customers/{customer_id}/kyc
Then :
  - Status 200 OK
  - Response: { full_name, cin, pep_status, source_of_funds, ... }
When customer_id invalide
Then Status 404 Not Found
```

**Tâches TDD** :
1. Créer CustomerResponse DTO
2. Créer KycProfileResponse DTO
3. Implémenter GET /customers/{id} handler
4. Implémenter GET /customers/{id}/kyc handler
5. Appeler repository.find_by_id()
6. Convertir aggregate → DTO
7. Ajouter permission check (own profile or Admin)
8. Documenter endpoints
9. Tester 404 cases
10. Ajouter caching (optionnel)
11. Tester avec curl
12. Ajouter tracing
13. Documenter réponse structure
14. Implémenter GET /customers (list, Admin only)
15. Ajouter pagination

**Dépendances** : STORY-C04

---

### STORY-C06 | API: PUT /customers/{id}/kyc (KYC update/resubmit)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1)

**User Story** :
> En tant que client, je veux mettre à jour mon KYC, afin de corriger les données rejetées.

**Scénarios BDD** :
```gherkin
Given customer avec KYC Rejected (raison: "CIN invalide")
When je PUT /customers/{id}/kyc avec CIN corrigée
Then :
  - Status 200 OK
  - kyc_profile mise à jour
  - status = Pending
  - Re-évaluation (PEP check, AML check)
When je soumets KYC sans attendre reject
Then Status 409 Conflict
```

**Tâches TDD** :
1. Créer UpdateKycRequest DTO
2. Implémenter PUT /customers/{id}/kyc handler
3. Valider customer_id et permission
4. Valider KYC data
5. Appeler KycService::update_kyc()
6. Mettre à jour customer aggregate
7. Déclencher re-évaluation
8. Ajouter audit log
9. Documenter endpoint
10. Tester permission check
11. Tester validation
12. Tester re-evaluation flow
13. Implémenter optimistic locking (updated_at)
14. Tester 409 Conflict
15. Ajouter notification

**Dépendances** : STORY-C05

---

### STORY-C07 | Feature: PEP check (Politically Exposed Person)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1) + AML (BC4)

**User Story** :
> En tant que system, je veux vérifier si un client est PEP, afin de respecter Circ. 2025-17 (Enhanced Due Diligence).

**Scénarios BDD** :
```gherkin
Given liste des PEPs (nationaux + internationaux)
When customer.full_name = "Ministre Dupont"
And je crée KYC avec full_name, birth_date, nationality
Then PEP check automatique → true
And kyc_profile.pep_status = "Yes"
And decision = RequireEdd
When je soumets PEP avec source_of_funds valide
Then kyc_profile mise à jour
And audit log avec "PEP Detected - EDD required"
```

**Tâches TDD** :
1. Créer PEP list (mock ou API)
2. Implémenter PepService trait
3. Implémenter PepChecker (fuzzy matching)
4. Ajouter full_name + birth_date matching
5. Configurer fuzzy threshold (typos)
6. Implémenter call to external PEP source (mock)
7. Ajouter caching PEP results
8. Documenter PEP sources
9. Implémenter audit logging
10. Tester fuzzy matching
11. Tester cache invalidation
12. Ajouter metrics (PEP detection rate)
13. Documenter EDD process
14. Implémenter EDD fields (source_of_wealth, pep_justification)
15. Tester workflow PEP + EDD

**Dépendances** : STORY-C02

---

### STORY-C08 | Feature: Risk scoring
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Customer (BC1)

**User Story** :
> En tant que risk manager, je veux un scoring automatique basé sur KYC, afin de catégoriser les clients par risque.

**Scénarios BDD** :
```gherkin
Given Customer avec KYC complet
When je calcule risk_score avec critères :
  - Nationality = Tunisia : 10 pts
  - PEP = Yes : +30 pts
  - Source = Business : +15 pts
  - High-risk sectors : +20 pts
  - Age < 25 or > 80 : +10 pts
Then risk_score = [10, 100] range
And risk_level = enum (Low | Medium | High | VeryHigh)
When risk_score >= 70
Then kyc_decision = RequireEdd
When risk_score >= 90
Then kyc_decision = RequireManualReview
```

**Tâches TDD** :
1. Créer RiskScoringService
2. Définir scoring rules (configurable)
3. Implémenter calculate_risk_score()
4. Tester chaque critère
5. Tester somme pondérée
6. Implémenter risk_level classification
7. Ajouter audit logging
8. Documenter règles de scoring
9. Implémenter update scoring (triggers)
10. Ajouter metrics (risk distribution)
11. Implémenter override (manual adjustment)
12. Tester avec données réelles
13. Documenter configuration
14. Implémenter versioning (v1, v2 de règles)
15. Tester backward compatibility

**Dépendances** : STORY-C02

---

## Epic 3 : Account (BC2) — Must Have

(Continuing with BC2 stories; will create remaining epics...)

### STORY-AC-01 | Domain: Account aggregate + invariants
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Account (BC2)
**Entité DDD** : Account (AggregateRoot), Balance (ValueObject), Movement (Entity)
**Invariants** : INV-01 (KYC required)

**User Story** :
> En tant qu'architecte domain, je veux l'agrégat Account avec soldes (courant, épargne, DAT), mouvements, afin de gérer les comptes bancaires.

**Scénarios BDD** :
```gherkin
Given Account aggregate vide
When je crée Account avec :
  - account_id: Uuid
  - customer_id: Uuid (FK)
  - rib: Rib (ValueObject unique)
  - account_type: Enum (Current | Savings | TimeDeposit)
  - balance: Money (TND)
  - available_balance: Money
  - movements: Vec<Movement>
  - status: Enum (Active | Closed | Suspended)
Then Account validé :
  - available_balance <= balance
  - customer_id != null
When je crée Account sans KYC validée du customer
Then erreur DomainError::MissingKyc (invariant INV-01)
When je crée Account(TimeDeposit) avec interest_rate, maturity_date
Then mouvements d'intérêt calculés à maturité
```

**Tâches TDD** :
1. Créer crates/domain/src/account/ module
2. Définir AccountId(Uuid) ValueObject
3. Définir AccountType enum (Current | Savings | TimeDeposit)
4. Définir AccountStatus enum
5. Créer Movement entity (debit/credit)
6. Implémenter Account aggregate
7. Ajouter invariant : available_balance <= balance
8. Implémenter account.deposit(money) → Movement
9. Implémenter account.withdraw(money) → Result (available check)
10. Implémenter account.freeze() → suspended
11. Implémenter account.unfreeze()
12. Ajouter KYC validation check
13. Tester tous les scénarios
14. Documenter domaine métier
15. Implémenter event sourcing (optionnel)

**Dépendances** : STORY-T09, STORY-C01

---

### STORY-AC-02 | Application: AccountService
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Account (BC2)

**User Story** :
> En tant qu'application, je veux un AccountService pour ouvrir/clôturer des comptes, afin de gérer le cycle de vie.

**Scénarios BDD** :
```gherkin
Given AccountService avec ports IAccountRepository, ICustomerRepository
When j'appelle account_service.open_account(customer_id, account_type)
Then :
  - Customer KYC vérifié (INV-01)
  - Account créé avec RIB unique
  - Status = Active
  - balance = 0 TND
When j'appelle account_service.close_account(account_id)
Then :
  - Status = Closed
  - Tous les mouvements finalisés
  - Solde final calculé
```

**Tâches TDD** :
1. Créer AccountService struct
2. Implémenter IAccountRepository port
3. Implémenter open_account()
4. Implémenter close_account()
5. Implémenter freeze_account()
6. Implémenter unfreeze_account()
7. Valider customer KYC
8. Générer RIB unique
9. Ajouter audit logging
10. Créer DTOs
11. Tester workflows
12. Documenter service
13. Implémenter error handling
14. Ajouter notifications
15. Tester tous les scénarios

**Dépendances** : STORY-AC-01

---

### STORY-AC-03 | Infrastructure: Account + Movement tables
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Account (BC2)

**User Story** :
> En tant que DBA, je veux les tables PostgreSQL pour comptes et mouvements, afin de persister les données.

**Scénarios BDD** :
```gherkin
Given migrations/05_account_schema.sql
When je crée tables :
  CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    customer_id UUID REFERENCES customers(id) NOT NULL,
    rib VARCHAR(50) UNIQUE NOT NULL,
    account_type VARCHAR(50) NOT NULL,
    balance DECIMAL(18, 2) NOT NULL DEFAULT 0,
    available_balance DECIMAL(18, 2) NOT NULL DEFAULT 0,
    status VARCHAR(50) DEFAULT 'Active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
  )

  CREATE TABLE movements (
    id UUID PRIMARY KEY,
    account_id UUID REFERENCES accounts(id) NOT NULL,
    movement_type VARCHAR(50) NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    balance_after DECIMAL(18, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
  )
Then migrations appliquées
```

**Tâches TDD** :
1. Créer migrations/05_account_schema.sql
2. Créer TABLE accounts
3. Ajouter UNIQUE constraint rib
4. Créer TABLE movements
5. Ajouter indexes account_id, created_at
6. Implémenter AccountRepository (sqlx)
7. Implémenter save(), find_by_id()
8. Implémenter list_by_customer()
9. Implémenter find_by_rib()
10. Implémenter movements queries
11. Ajouter DECIMAL(18,2) pour argent
12. Tester migrations
13. Exécuter sur Docker
14. Ajouter partitioning pour movements (par année)
15. Tester performance avec données volumineuses

**Dépendances** : STORY-T03, STORY-AC-02

---

### STORY-AC-04 | API: POST /accounts (account opening)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Account (BC2)

**User Story** :
> En tant que chargé clientèle, je veux ouvrir un compte client, afin de démarrer les opérations.

**Scénarios BDD** :
```gherkin
Given customer avec KYC Approved
When je POST /accounts avec :
  {
    "customer_id": "...",
    "account_type": "Current"
  }
Then :
  - Status 201 Created
  - Response: { "account_id": "...", "rib": "01-234-0001234-56", "status": "Active" }
When customer n'a pas KYC Approved
Then Status 400 Bad Request (INV-01)
When je crée déjà 5 comptes courants
Then Status 400 (déjà max)
```

**Tâches TDD** :
1. Créer CreateAccountRequest DTO
2. Implémenter POST /accounts handler
3. Valider customer_id existe
4. Valider customer KYC = Approved (INV-01)
5. Appeler AccountService::open_account()
6. Retourner 201 Created
7. Ajouter audit logging
8. Tester validation
9. Tester limite de comptes
10. Ajouter tracing
11. Documenter endpoint
12. Tester avec curl
13. Implémenter idempotency (optionnel)
14. Ajouter notifications
15. Documenter réponse

**Dépendances** : STORY-AC-03

---

### STORY-AC-05 | API: GET /accounts/{id} + GET /accounts (list)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Account (BC2)

**User Story** :
> En tant qu'utilisateur, je veux consulter mes comptes et leurs détails, afin de gérer mon argent.

**Scénarios BDD** :
```gherkin
Given user avec 2 comptes
When je GET /accounts
Then :
  - Status 200 OK
  - Response: [
      { account_id, rib, account_type, balance, status, ... },
      { ... }
    ]
When je GET /accounts/{account_id}
Then :
  - Status 200 OK
  - Détail complet + 10 derniers mouvements
When je GET /accounts/{other_user_account}
Then Status 403 Forbidden (sauf Admin)
```

**Tâches TDD** :
1. Créer AccountResponse DTO
2. Implémenter GET /accounts handler
3. Implémenter GET /accounts/{id} handler
4. Filtrer par user (claims JWT)
5. Inclure derniers mouvements
6. Ajouter pagination
7. Documenter endpoints
8. Tester 403 Forbidden
9. Tester pagination
10. Ajouter caching
11. Tester avec curl
12. Ajouter tracing
13. Implémenter sorting (balance, date)
14. Ajouter filtering (status, type)
15. Documenter réponse

**Dépendances** : STORY-AC-04

---

### STORY-AC-06 | API: POST /accounts/{id}/movements (deposit/withdraw)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Account (BC2)

**User Story** :
> En tant qu'utilisateur, je veux effectuer des dépôts et retraits, afin de gérer mon solde.

**Scénarios BDD** :
```gherkin
Given account avec balance = 5000 TND
When je POST /accounts/{id}/movements avec :
  {
    "movement_type": "Withdrawal",
    "amount": 2000
  }
Then :
  - Status 201 Created
  - account.balance = 3000 TND
  - Movement sauvegardé avec timestamp
When je retire plus que disponible
Then Status 400 Bad Request (insufficient funds)
When je dépose 1000
Then balance = 4000 TND
```

**Tâches TDD** :
1. Créer MovementRequest DTO
2. Implémenter POST /accounts/{id}/movements handler
3. Valider account_id et permission
4. Valider amount > 0
5. Appeler AccountService::deposit() ou withdraw()
6. Vérifier solde disponible
7. Créer Movement entity
8. Persister dans DB
9. Retourner 201 Created
10. Ajouter audit logging
11. Ajouter AML check (montant >= 5000 TND → scan)
12. Documenter endpoint
13. Tester validation
14. Tester insufficient funds
15. Tester AML trigger

**Dépendances** : STORY-AC-05

---

### STORY-AC-07 | Feature: Balance calculation + available_balance
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Account (BC2)

**User Story** :
> En tant que système, je veux calculer correctement balance et available_balance, en tenant compte des gels et réserves.

**Scénarios BDD** :
```gherkin
Given account avec balance = 10000 TND
When il y a un gel partiel de 3000 TND (AML)
Then :
  - balance = 10000 TND (total)
  - available_balance = 7000 TND (liquide)
When je retire 7000
Then Status 200 OK (respected freeze)
When je retire 7001
Then Status 400 (exceeds available)
When le gel est levé
Then available_balance = 10000 TND
```

**Tâches TDD** :
1. Implémenter balance calculation
2. Implémenter available_balance calculation
3. Intégrer freezes (AML, sanctions)
4. Intégrer hold (pending transactions)
5. Ajouter reserves (regulatory, interest)
6. Tester chaque type de réduction
7. Tester cumul de réductions
8. Documenter formule
9. Tester avec données réelles
10. Ajouter audit trail
11. Implémenter reconciliation (balance vs movements)
12. Ajouter metrics (balance distribution)
13. Tester edge cases (float rounding)
14. Documenter contractuellement
15. Tester avec décimales

**Dépendances** : STORY-AC-06

---

### STORY-AC-08 | Feature: Account statements + reconciliation
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Account (BC2)

**User Story** :
> En tant qu'utilisateur, je veux télécharger mes relevés bancaires, afin de vérifier les opérations.

**Scénarios BDD** :
```gherkin
Given account avec 100 movements
When je GET /accounts/{id}/statement avec date_from, date_to
Then :
  - Response: CSV ou PDF
  - Columns: date, type, débit, crédit, solde
  - Solde initial et final
When je télécharge statement.pdf
Then PDF formaté et signé (optionnel)
```

**Tâches TDD** :
1. Créer StatementGenerator (CSV + PDF)
2. Implémenter query movements par période
3. Implémenter CSV export (csv crate)
4. Implémenter PDF export (printpdf ou similaire)
5. Formater header (bank name, account, period)
6. Ajouter solde initial, opérations, solde final
7. Implémenter GET /accounts/{id}/statement endpoint
8. Ajouter date_from, date_to params
9. Ajouter format param (csv, pdf)
10. Tester CSV structure
11. Tester PDF rendering
12. Ajouter signature cryptographique (optionnel)
13. Documenter formats
14. Tester avec Postman
15. Ajouter audit logging

**Dépendances** : STORY-AC-07

---

## Epic 4 : Credit (BC3) — Must Have

### STORY-CR-01 | Domain: Loan aggregate + asset classification
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Credit (BC3)
**Entité DDD** : Loan (AggregateRoot), LoanSchedule (Entity), Provision (ValueObject)
**Invariants** : INV-06 (assetClass ∈ [0,4]), INV-07 (provisioning ≥ regulatory min), INV-15

**User Story** :
> En tant qu'analyste crédit, je veux l'agrégat Loan avec classification d'actifs (classes 0-4 selon Circ. 91-24), planification d'amortissement et provisionnement, afin de gérer le portefeuille de crédits.

**Scénarios BDD** :
```gherkin
Given Loan aggregate vide
When je crée Loan avec :
  - loan_id: Uuid
  - account_id: Uuid (FK)
  - amount: Money (TND)
  - interest_rate: Decimal (%)
  - term_months: Integer
  - asset_class: Enum (0|1|2|3|4)
  - status: Enum (Applied|Approved|Disbursed|Active|Closed|Defaulted)
Then Loan validé :
  - asset_class ∈ [0,4]
  - interest_rate > 0
  - term_months > 0
When je crée Loan avec days_past_due = 0
Then asset_class = 0 (Standard)
When je crée Loan avec days_past_due = 31
Then asset_class = 1 (Underperforming, >30j)
When je crée Loan avec days_past_due = 91
Then asset_class = 2 (Doubtful, >90j)
When je crée Loan avec days_past_due = 181
Then asset_class = 3 (Loss, >180j)
When je crée Loan avec days_past_due = 365
Then asset_class = 4 (Write-off, >1 year)
When je calcule provision_rate(asset_class)
Then :
  - class 0 → 0% (no provision)
  - class 1 → 20% (of principal)
  - class 2 → 50%
  - class 3 → 100%
  - class 4 → 100% (write-off)
When je génère amortization_schedule()
Then chaque paiement est calculé avec :
  - Principal reduction
  - Interest charge
  - Provision update
```

**Tâches TDD** :
1. Créer crates/domain/src/credit/ module
2. Définir LoanId(Uuid) ValueObject
3. Définir AssetClass enum (0-4 avec règles métier)
4. Implémenter asset classification par days_past_due
5. Créer Provision ValueObject (amount, rate, regulatory_min)
6. Créer LoanSchedule entity (payment schedule)
7. Implémenter Loan aggregate root
8. Ajouter invariant : provision ≥ regulatory_minimum
9. Implémenter loan.classify_asset(days_past_due) → AssetClass
10. Implémenter loan.calculate_provision() → Money
11. Implémenter loan.generate_schedule() → Vec<LoanSchedule>
12. Implémenter loan.record_payment(amount, date) → Movement
13. Implémenter loan.update_asset_class() automatique
14. Ajouter validation métier au constructeur
15. Tester tous les scénarios BDD

**Fichiers** :
- crates/domain/src/credit/loan.rs
- crates/domain/src/credit/asset_class.rs
- crates/domain/src/credit/provision.rs
- crates/domain/src/credit/schedule.rs

**Dépendances** : STORY-T09, STORY-AC-01

---

### STORY-CR-02 | Application: LoanService + ports
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Credit (BC3)
**Entité DDD** : LoanService (use case)

**User Story** :
> En tant qu'analyste crédit, je veux un LoanService pour approuver, débourser et classer les crédits automatiquement.

**Scénarios BDD** :
```gherkin
Given LoanService avec ports ILoanRepository, IAccountRepository, IPrudentialService
When j'appelle loan_service.apply_loan(account_id, amount, term_months)
Then :
  - Loan créé avec status = Applied
  - Account vérifié (KYC, solde minimal)
  - Prudential check déclenché
  - Audit log créé
When prudential_check échoue (C/D > 120%)
Then loan_service retourne Error::PrudentialViolation
When j'appelle loan_service.approve_loan(loan_id, approved_amount)
Then :
  - Loan.status = Approved
  - Montant approuvé <= montant demandé
  - Taux d'intérêt verrouillé
When j'appelle loan_service.disburse(loan_id)
Then :
  - Loan.status = Disbursed
  - Account reçoit credit de montant
  - LoanSchedule généré
  - Accounting entries créées
When je mets à jour classification (days_past_due)
Then loan_service.update_classification() applique règles automatiquement
```

**Tâches TDD** :
1. Créer LoanService struct
2. Implémenter ILoanRepository port
3. Implémenter IAccountRepository (FK check)
4. Implémenter IPrudentialService (validation)
5. Créer LoanApplicationRequest DTO
6. Implémenter apply_loan()
7. Implémenter approve_loan()
8. Implémenter disburse()
9. Implémenter update_classification()
10. Implémenter calculate_provision()
11. Ajouter audit logging
12. Créer error types (LoanError enum)
13. Implémenter validation métier
14. Ajouter notifications
15. Tester tous les scénarios

**Fichiers** :
- crates/application/src/credit/loan_service.rs
- crates/application/src/credit/ports.rs
- crates/application/src/credit/dto.rs

**Dépendances** : STORY-CR-01

---

### STORY-CR-03 | Infrastructure: Loan + Schedule tables
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Credit (BC3)

**User Story** :
> En tant que DBA, je veux des tables PostgreSQL pour Loan, LoanSchedule et Provision, avec indexes optimisés.

**Scénarios BDD** :
```gherkin
Given migrations/ vide
When je crée migrations/XX_credit_schema.sql
Then tables créées :
  - loans (id, account_id, amount, interest_rate, asset_class, status, created_at)
  - loan_schedules (id, loan_id, payment_date, principal, interest, balance)
  - loan_provisions (id, loan_id, amount, rate, regulatory_requirement)
And indexes sur (account_id, asset_class, status)
And audit columns (created_at, updated_at, created_by)
```

**Tâches TDD** :
1. Créer migrations/XX_credit_schema.sql
2. Créer table loans avec types appropriés
3. Créer table loan_schedules
4. Créer table loan_provisions
5. Ajouter FK constraints
6. Ajouter indexes (account_id, status, asset_class)
7. Ajouter audit columns
8. Créer indexes pour queries fréquentes
9. Exécuter sqlx migrate run
10. Créer LoanRepository impl
11. Implémenter sqlx queries typées
12. Implémenter save/find/update
13. Ajouter pagination queries
14. Tester avec données réelles
15. Valider indexes performance

**Fichiers** :
- migrations/XX_credit_schema.sql
- crates/infrastructure/src/credit/loan_repository.rs

**Dépendances** : STORY-T03, STORY-CR-02

---

### STORY-CR-04 | API: POST /loans endpoint
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Credit (BC3)

**User Story** :
> En tant qu'utilisateur web, je veux un endpoint POST /loans pour demander un crédit avec validation.

**Scénarios BDD** :
```gherkin
Given API Actix démarrée
When je POST /loans avec :
  {
    "account_id": "uuid-xxx",
    "amount": 50000,
    "term_months": 60
  }
Then :
  - Status 201 Created
  - Response: { "loan_id": "uuid-yyy", "status": "Applied" }
When je POST sans account_id
Then Status 400 + erreur validation
When amount > 500000
Then Status 400 + erreur montant max
```

**Tâches TDD** :
1. Créer handler POST /loans
2. Implémenter validation JSON
3. Ajouter auth check (JWT)
4. Appeler LoanService.apply_loan()
5. Mapper erreurs vers HTTP
6. Retourner LoanResponse DTO
7. Ajouter audit logging
8. Ajouter rate limiting
9. Tester 201, 400, 401, 422
10. Documenter OpenAPI
11. Ajouter error details
12. Implémenter error recovery
13. Tester concurrence
14. Ajouter monitoring
15. Valider format réponse

**Fichiers** :
- crates/infrastructure/src/credit/handlers.rs

**Dépendances** : STORY-CR-03

---

### STORY-CR-05 | API: GET /loans portfolio view
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Credit (BC3)

**User Story** :
> En tant que responsable crédit, je veux une vue GET /loans avec filtres (statut, classe d'actif), tri et pagination.

**Scénarios BDD** :
```gherkin
Given 100 loans en DB
When je GET /loans?status=Active&asset_class=1&page=1&limit=20
Then :
  - Status 200
  - Response: { "data": [...], "total": 45, "page": 1 }
When je GET /loans?sort=created_at:desc
Then résultats triés par date desc
When je GET /loans?account_id=uuid-xxx
Then filtre par account (FK)
```

**Tâches TDD** :
1. Créer handler GET /loans
2. Implémenter filtres (status, asset_class, account_id)
3. Implémenter tri (created_at, status, amount)
4. Implémenter pagination (page, limit)
5. Ajouter auth check
6. Créer query builder
7. Retourner PaginatedResponse
8. Ajouter caching (Redis 5min)
9. Tester 200, 401, 400
10. Documenter OpenAPI
11. Ajouter search full-text
12. Implémenter export CSV
13. Ajouter monitoring
14. Tester performance (100k+ loans)
15. Valider format réponse

**Fichiers** :
- crates/infrastructure/src/credit/handlers.rs

**Dépendances** : STORY-CR-04

---

### STORY-CR-06 | Feature: Automatic asset classification
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Credit (BC3)

**User Story** :
> En tant que système, je veux classifier automatiquement les crédits par classe d'actif selon Circ. 91-24 chaque jour.

**Scénarios BDD** :
```gherkin
Given batch job quotidien à minuit
When je calcule days_past_due pour chaque loan
Then :
  - days ≤ 30 → asset_class = 0
  - 31 ≤ days ≤ 90 → class = 1
  - 91 ≤ days ≤ 180 → class = 2
  - 181 ≤ days ≤ 365 → class = 3
  - days > 365 → class = 4
When asset_class change
Then provision_amount recalculé
And audit trail enregistré
```

**Tâches TDD** :
1. Créer scheduled job (Tokio)
2. Implémenter logique days_past_due
3. Implémenter transition de classe
4. Créer provision calculator
5. Faire batch update
6. Ajouter error handling + retry
7. Ajouter monitoring/alertes
8. Tester avec clock mock
9. Tester transitions
10. Ajouter audit logging
11. Documenter métier
12. Tester parallelization
13. Ajouter metrics (count par classe)
14. Tester avec données réelles
15. Valider performance

**Fichiers** :
- crates/infrastructure/src/credit/classification_job.rs

**Dépendances** : STORY-CR-02

---

### STORY-CR-07 | Feature: Loan provisioning regulatory
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Credit (BC3)

**User Story** :
> En tant qu'analyste conformité, je veux que les provisions soient calculées automatiquement selon réglementations (20%/50%/100%).

**Scénarios BDD** :
```gherkin
Given loan avec asset_class = 1
When je calcule provision
Then provision = 20% de principal
And journal entry créée (DR: Provision / CR: PNL)
When asset_class monte à 3
Then provision monte à 100%
And différence reversée dans P&L
```

**Tâches TDD** :
1. Créer provision calculator
2. Implémenter taux réglementaires
3. Calculer différence (adjustment)
4. Créer journal entries
5. Ajouter reversal logic
6. Tester transitions
7. Ajouter audit logging
8. Tester avec données variées
9. Implémenter export pour rapports
10. Ajouter validation
11. Tester edge cases
12. Ajouter monitoring
13. Documenter métier
14. Tester concurrence
15. Valider exactitude calculs

**Fichiers** :
- crates/domain/src/credit/provision.rs
- crates/application/src/credit/provision_service.rs

**Dépendances** : STORY-CR-06

---

### STORY-CR-08 | Feature: Loan repayment scheduling + amortization
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Credit (BC3)

**User Story** :
> En tant que système, je veux générer et gérer les plans d'amortissement automatiquement.

**Scénarios BDD** :
```gherkin
Given loan de 100k TND @ 8% sur 60 mois
When je génère schedule
Then 60 paiements mensuels avec :
  - Chaque paiement = ~1895 TND (fixed + intérêt décroissant)
  - Intérêt = solde × taux / 12
  - Principal = paiement - intérêt
  - Solde = solde_précédent - principal
When j'enregistre paiement le 15 du mois
Then :
  - Schedule mis à jour
  - Account débité
  - Journal entries créées
  - Provision recalculée
```

**Tâches TDD** :
1. Créer amortization calculator
2. Implémenter formule paiements fixes
3. Implémenter génération schedule
4. Créer payment recording
5. Implémenter early repayment
6. Calculer intérêt accrué
7. Ajouter late payment handling
8. Créer journal entries
9. Ajouter audit logging
10. Tester edge cases
11. Tester rounding
12. Valider solde final
13. Ajouter penalties (optionnel)
14. Tester avec données réelles
15. Documenter formules

**Fichiers** :
- crates/domain/src/credit/schedule.rs
- crates/application/src/credit/repayment_service.rs

**Dépendances** : STORY-CR-01

---

## Epic 5 : AML (BC4) — Must Have

### STORY-AML-01 | Domain: Transaction aggregate + Alert
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : AML (BC4)
**Entité DDD** : Transaction (AggregateRoot), Alert (Entity), Investigation (Entity)
**Invariants** : INV-08 (≥5000 TND cash → AML check), INV-09 (freeze immédiat)

**User Story** :
> En tant qu'architecte AML, je veux l'agrégat Transaction avec Alerts et Investigation pour détecter activités suspectes (Loi 2015-26, Circ. 2025-17).

**Scénarios BDD** :
```gherkin
Given Transaction aggregate vide
When je crée Transaction avec :
  - transaction_id: Uuid
  - account_id: Uuid
  - counterparty: String (benef)
  - amount: Money
  - transaction_type: Enum (Deposit|Withdrawal|Transfer|Exchange)
  - direction: Enum (Inbound|Outbound)
  - timestamp: DateTime
Then Transaction validé
When amount >= 5000 TND (seuil AML)
Then alert auto-créée avec risk_level = Medium
When je détecte pattern suspect (10×10k en 1h)
Then alert créée avec risk_level = High
When je crée Investigation
Then status = Open, linked_transactions, notes
When investigation conclut fraudulent
Then alert.status = Confirmed, freeze recommandée
```

**Tâches TDD** :
1. Créer crates/domain/src/aml/ module
2. Définir TransactionId(Uuid)
3. Définir TransactionType enum
4. Créer Transaction aggregate
5. Implémenter Alert entity
6. Implémenter Investigation entity
7. Ajouter invariant : amount > 0
8. Implémenter transaction.trigger_alert() basé sur seuil
9. Implémenter transaction.flag_suspicious()
10. Créer risk_level enum (Low|Medium|High|Critical)
11. Implémenter alert detection rules
12. Implémenter investigation workflow
13. Ajouter audit trail pour chaque changement
14. Tester tous les scénarios BDD
15. Documenter règles détection

**Fichiers** :
- crates/domain/src/aml/transaction.rs
- crates/domain/src/aml/alert.rs
- crates/domain/src/aml/investigation.rs

**Dépendances** : STORY-T09, STORY-AC-01

---

### STORY-AML-02 | Application: TransactionMonitoringService
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : AML (BC4)

**User Story** :
> En tant que compliance officer, je veux un service pour enregistrer transactions et déclencher monitoring automatiquement.

**Scénarios BDD** :
```gherkin
Given TransactionMonitoringService avec ports
When j'appelle aml_service.record_transaction(tx_data)
Then :
  - Transaction créée
  - Seuil AML vérifié (≥5000 TND)
  - Patterns suspects analysés
  - Alerts créées si nécessaire
When je ouvre investigation
Then statut = Open, assigned to analyst
When j'ajoute notes
Then investigation mis à jour avec audit trail
```

**Tâches TDD** :
1. Créer TransactionMonitoringService
2. Implémenter ITransactionRepository
3. Implémenter record_transaction()
4. Implémenter check_aml_threshold()
5. Implémenter detect_patterns()
6. Implémenter create_alert()
7. Implémenter open_investigation()
8. Implémenter add_investigation_note()
9. Créer DTOs
10. Ajouter audit logging
11. Implémenter error handling
12. Ajouter notifications
13. Tester workflows
14. Documenter service
15. Tester tous les scénarios

**Fichiers** :
- crates/application/src/aml/transaction_monitoring_service.rs
- crates/application/src/aml/ports.rs

**Dépendances** : STORY-AML-01

---

### STORY-AML-03 | Infrastructure: Transaction + Alert tables
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : AML (BC4)

**User Story** :
> En tant que DBA, je veux les tables PostgreSQL pour transactions, alerts et investigations.

**Scénarios BDD** :
```gherkin
Given migrations/ vide
When je crée migrations/XX_aml_schema.sql
Then tables :
  - transactions (id, account_id, amount, type, direction, timestamp)
  - aml_alerts (id, transaction_id, risk_level, reason, status)
  - investigations (id, alert_id, status, assigned_to, notes)
```

**Tâches TDD** :
1. Créer migrations/XX_aml_schema.sql
2. Créer table transactions
3. Créer table aml_alerts
4. Créer table investigations
5. Ajouter FK constraints
6. Ajouter indexes (account_id, timestamp, risk_level)
7. Ajouter audit columns
8. Exécuter migrations
9. Créer TransactionRepository
10. Implémenter find queries
11. Implémenter save queries
12. Ajouter pagination
13. Tester avec données
14. Valider indexes
15. Documenter schema

**Fichiers** :
- migrations/XX_aml_schema.sql
- crates/infrastructure/src/aml/repositories.rs

**Dépendances** : STORY-T03, STORY-AML-02

---

### STORY-AML-04 | API: POST /transactions endpoint
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : AML (BC4)

**User Story** :
> En tant qu'utilisateur, je veux enregistrer une transaction via API.

**Scénarios BDD** :
```gherkin
Given API Actix démarrée
When je POST /transactions avec :
  {
    "account_id": "uuid",
    "amount": 10000,
    "type": "Transfer",
    "counterparty": "John Doe"
  }
Then Status 201, transaction créée avec alert si AML threshold
```

**Tâches TDD** :
1. Créer handler POST /transactions
2. Implémenter validation
3. Ajouter auth check
4. Appeler service
5. Mapper erreurs
6. Retourner DTO
7. Ajouter audit logging
8. Ajouter rate limiting
9. Tester 201, 400, 401
10. Documenter OpenAPI
11. Ajouter error details
12. Tester concurrence
13. Ajouter monitoring
14. Valider format
15. Tester avec patterns suspects

**Fichiers** :
- crates/infrastructure/src/aml/handlers.rs

**Dépendances** : STORY-AML-03

---

### STORY-AML-05 | Feature: AML scenarios detection (Circ. 2025-17)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : AML (BC4)

**User Story** :
> En tant que système AML, je veux détecter les scénarios suspects définis dans Circ. 2025-17.

**Scénarios BDD** :
```gherkin
Given règles AML configurées
When je détecte :
  - Transaction > 5000 TND → Medium alert
  - 3+ transactions > 100k en 24h → High alert
  - Transfer to high-risk country → High alert
  - Structured deposits (10×499.99) → Critical alert
Then alerts créées automatiquement avec raison métier
```

**Tâches TDD** :
1. Créer AML rules engine
2. Implémenter threshold rule (≥5000)
3. Implémenter pattern rule (multiple tx)
4. Implémenter geographic rule (high-risk pays)
5. Implémenter structuring rule
6. Créer rule configuration
7. Implémenter rule evaluation
8. Créer alert with reason
9. Ajouter rule versioning
10. Tester toutes les règles
11. Ajouter monitoring
12. Documenter règles
13. Ajouter audit trail
14. Tester performance
15. Valider détection

**Fichiers** :
- crates/application/src/aml/rules_engine.rs

**Dépendances** : STORY-AML-02

---

### STORY-AML-06 | Feature: Suspicious activity investigation
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : AML (BC4)

**User Story** :
> En tant qu'analyste AML, je veux investiguer les activités suspectes avec workflow complet.

**Scénarios BDD** :
```gherkin
Given alert créée
When j'ouvre investigation
Then statut = Open, assigned to analyst
When j'ajoute notes et documents
Then investigation trail créé
When je conclus innocent
Then alert.status = Dismissed, archived
When je conclus suspicious
Then investigation.status = Escalated pour décision superviseur
```

**Tâches TDD** :
1. Créer investigation workflow
2. Implémenter open investigation
3. Implémenter add notes
4. Implémenter attach documents
5. Implémenter status transitions
6. Implémenter escalation
7. Implémenter conclusion
8. Ajouter auditing
9. Créer notifications
10. Ajouter timestamps
11. Tester workflow
12. Ajouter role checks
13. Documenter workflow
14. Tester avec données réelles
15. Valider transitions

**Fichiers** :
- crates/application/src/aml/investigation_service.rs

**Dépendances** : STORY-AML-02

---

### STORY-AML-07 | Feature: Suspect report (DOS) + CTAF transmission
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : AML (BC4)

**User Story** :
> En tant qu'analysiste AML, je veux générer un rapport suspect (DOS) et le transmettre à CTAF.

**Scénarios BDD** :
```gherkin
Given investigation conclue suspecte
When je génère DOS report
Then rapport contient :
  - Customer info, transaction details
  - Raisons suspicion, evidence
  - Timeline des activités
When j'envoie à CTAF (stub)
Then statut = Submitted, timestamp enregistré
And audit trail créé
```

**Tâches TDD** :
1. Créer DOS report generator
2. Implémenter report structure
3. Implémenter PDF generation
4. Implémenter CTAF API stub
5. Implémenter submission logic
6. Ajouter acknowledgment tracking
7. Créer audit trail
8. Ajouter notifications
9. Implémenter retry logic
10. Ajouter monitoring
11. Tester generation
12. Tester transmission
13. Documenter format
14. Tester edge cases
15. Valider compliance

**Fichiers** :
- crates/application/src/aml/dos_report_service.rs
- crates/infrastructure/src/aml/ctaf_stub.rs

**Dépendances** : STORY-AML-06

---

### STORY-AML-08 | Feature: Asset freeze + workflow
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : AML (BC4)

**User Story** :
> En tant que supervisor AML, je veux geler les actifs en cas de suspects critique avec workflow d'approbation.

**Scénarios BDD** :
```gherkin
Given investigation Critical
When j'approuve freeze
Then :
  - Account status = Frozen (INV-09)
  - Tous les mouvements bloqués
  - Audit trail immédiat
When freeze levée (post-investigation)
Then Account status = Active
And journal entries créées
```

**Tâches TDD** :
1. Créer freeze logic
2. Implémenter account freeze
3. Implémenter block transactions
4. Implémenter approval workflow
5. Implémenter freeze lift
6. Créer audit trail
7. Ajouter notifications
8. Ajouter monitoring
9. Tester freeze/unfreeze
10. Tester transaction blocking
11. Ajouter role checks
12. Documenter workflow
13. Tester edge cases
14. Valider transitions
15. Tester avec données réelles

**Fichiers** :
- crates/application/src/aml/freeze_service.rs

**Dépendances** : STORY-AML-02

---

## Epic 6 : Sanctions (BC5) — Must Have

### STORY-SAN-01 | Domain: SanctionEntry + ScreeningResult
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Sanctions (BC5)
**Entité DDD** : SanctionEntry (ValueObject), ScreeningResult (Entity), SanctionList (AggregateRoot)
**Invariant** : INV-14 (filtrage avant virement)

**User Story** :
> En tant qu'architecte sanctions, je veux des entités pour gérer les listes de sanctions (ONU, UE, OFAC, nationales) et résultats de screening.

**Scénarios BDD** :
```gherkin
Given Sanctions domain vide
When je crée SanctionEntry avec :
  - entry_id: Uuid
  - list_source: Enum (UN|EU|OFAC|National)
  - name: String
  - country: String
  - listing_date: Date
  - additional_names: Vec<String>
Then SanctionEntry validé
When je crée ScreeningResult pour customer
Then result contient :
  - customer matched ou not_matched
  - score de similarité (0-100, fuzzy match)
  - details matched names
  - list source
  - timestamp
When je scanne "Jean Alaoui" contre liste ONU
And liste contient "Jean Alaouie" (typo)
Then score de match > 80% (fuzzy)
```

**Tâches TDD** :
1. Créer crates/domain/src/sanctions/ module
2. Définir ListSource enum
3. Créer SanctionEntry ValueObject
4. Implémenter SanctionList aggregate
5. Créer ScreeningResult entity
6. Implémenter fuzzy matching (Levenshtein)
7. Implémenter name normalization
8. Créer MatchDetails struct
9. Implémenter screen_name() → ScreeningResult
10. Ajouter score_threshold (80% min)
11. Implémenter multiple list support
12. Ajouter date validité
13. Tester fuzzy avec typos
14. Tester avec données réelles
15. Documenter scoring

**Fichiers** :
- crates/domain/src/sanctions/sanction_entry.rs
- crates/domain/src/sanctions/screening_result.rs
- crates/domain/src/sanctions/fuzzy_matcher.rs

**Dépendances** : STORY-T09

---

### STORY-SAN-02 | Application: SanctionsScreeningService
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Sanctions (BC5)

**User Story** :
> En tant que compliance, je veux un service pour screener customers et payments en temps réel.

**Scénarios BDD** :
```gherkin
Given SanctionsScreeningService
When j'appelle screen_customer(customer_id)
Then :
  - Tous les noms (principal + benéficiaires) screenés
  - Résultat : Clear | Potential_Match | Hit
  - Détails sauvegardés
When j'appelle screen_payment(payment_data)
Then bénéficiaire vérifié avant débit
```

**Tâches TDD** :
1. Créer SanctionsScreeningService
2. Implémenter ISanctionRepository
3. Implémenter screen_customer()
4. Implémenter screen_payment()
5. Implémenter multi-list screening
6. Ajouter caching (Redis, 24h)
7. Créer DTOs
8. Implémenter error handling
9. Ajouter notifications (potential matches)
10. Ajouter audit logging
11. Documenter service
12. Tester workflows
13. Ajouter monitoring
14. Tester performance
15. Valider coverage

**Fichiers** :
- crates/application/src/sanctions/screening_service.rs
- crates/application/src/sanctions/ports.rs

**Dépendances** : STORY-SAN-01

---

### STORY-SAN-03 | Infrastructure: Sanctions list sync (ONU, UE, OFAC)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Sanctions (BC5)

**User Story** :
> En tant qu'opérateur, je veux syncer automatiquement les listes de sanctions officielles.

**Scénarios BDD** :
```gherkin
Given scheduled job quotidien
When je synce listes :
  - ONU (https://...)
  - UE (https://...)
  - OFAC (https://...)
Then :
  - DB mis à jour
  - Nouvelles entries ajoutées
  - Entrées retirées marquées obsolètes
  - Audit trail créé
```

**Tâches TDD** :
1. Créer migrations pour sanctions tables
2. Créer SanctionRepository
3. Implémenter list fetch (HTTP)
4. Implémenter CSV/XML parsing
5. Implémenter sync logic (insert/update/retire)
6. Ajouter error handling + retry
7. Ajouter notifications (sync results)
8. Créer audit trail
9. Ajouter monitoring
10. Tester avec stub data
11. Tester retry logic
12. Ajouter rate limiting
13. Documenter sync format
14. Tester edge cases
15. Valider data integrity

**Fichiers** :
- migrations/XX_sanctions_schema.sql
- crates/infrastructure/src/sanctions/repositories.rs
- crates/infrastructure/src/sanctions/sync_job.rs

**Dépendances** : STORY-T03, STORY-SAN-02

---

### STORY-SAN-04 | API: GET /sanctions/check screening
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Sanctions (BC5)

**User Story** :
> En tant que backend, je veux un endpoint pour screener noms en temps réel.

**Scénarios BDD** :
```gherkin
Given API Actix démarrée
When je GET /sanctions/check?name=Jean%20Alaoui
Then :
  - Status 200
  - Response: { "result": "Clear", "score": 0 }
When name existe dans liste
Then { "result": "Hit", "lists": ["UN"], "score": 98 }
```

**Tâches TDD** :
1. Créer handler GET /sanctions/check
2. Implémenter query params
3. Ajouter validation input
4. Appeler service screening
5. Mapper results
6. Ajouter caching (Redis)
7. Ajouter auth (internal use)
8. Créer response DTO
9. Ajouter rate limiting
10. Tester 200, 400, 401
11. Documenter OpenAPI
12. Ajouter monitoring
13. Tester performance
14. Tester fuzzy scenarios
15. Valider format réponse

**Fichiers** :
- crates/infrastructure/src/sanctions/handlers.rs

**Dépendances** : STORY-SAN-03

---

### STORY-SAN-05 | Feature: Fuzzy matching (typos tolerance)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Sanctions (BC5)

**User Story** :
> En tant que système, je veux détecter les noms similaires avec tolérance aux typos/variations.

**Scénarios BDD** :
```gherkin
Given "Jean Alaoui" dans liste
When je scanne "Jean Alaouie" (typo)
Then match détecté avec score > 85%
When je scanne "JEAN ALAOUI" (majuscules)
Then match détecté (normalization)
When je scanne "Alaoui Jean" (ordre)
Then match possible selon config
```

**Tâches TDD** :
1. Implémenter Levenshtein distance
2. Ajouter name normalization (accents, majuscules)
3. Implémenter threshold scoring
4. Tester avec typos communs
5. Tester avec accents français
6. Tester avec ordre mots
7. Ajouter weighted scoring
8. Créer config adjustable
9. Tester performance (large lists)
10. Ajouter unit tests
11. Documenter algorithm
12. Tester edge cases (noms courts)
13. Benchmark contre alternatives
14. Valider accuracy
15. Ajouter monitoring

**Fichiers** :
- crates/domain/src/sanctions/fuzzy_matcher.rs

**Dépendances** : STORY-SAN-01

---

### STORY-SAN-06 | Feature: Screening on payment (INV-14)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Sanctions (BC5)

**User Story** :
> En tant que système, je veux bloquer les paiements si bénéficiaire est dans listes sanctions.

**Scénarios BDD** :
```gherkin
Given payment initié
When j'exécute screening avant débit
Then :
  - Clear → continue
  - Hit → reject + audit
  - Potential → manual review required
```

**Tâches TDD** :
1. Créer payment screening check
2. Implémenter pre-debit screening
3. Implémenter rejection workflow
4. Implémenter review queue
5. Ajouter notifications
6. Créer audit trail
7. Tester approval flows
8. Tester rejection flows
9. Ajouter error handling
10. Ajouter monitoring
11. Documenter workflow
12. Tester concurrence
13. Tester edge cases
14. Valider transitions
15. Tester avec réel data

**Fichiers** :
- crates/application/src/sanctions/payment_screening.rs

**Dépendances** : STORY-SAN-02

---

### STORY-SAN-07 | Feature: List update automation
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Sanctions (BC5)

**User Story** :
> En tant qu'opérateur, je veux que les mises à jour de listes soient automatiques et versionnées.

**Scénarios BDD** :
```gherkin
Given sync job quotidien
When listes mises à jour
Then :
  - Versions tracking
  - Audit trail complet
  - Notifications automatiques
  - Anciens screenings revisited (optional)
```

**Tâches TDD** :
1. Créer version tracking
2. Implémenter audit logging
3. Implémenter notifications
4. Créer rollback capability
5. Ajouter consistency checks
6. Tester sync failures
7. Tester partial updates
8. Ajouter monitoring
9. Documenter process
10. Tester avec données variées
11. Implémenter alertes
12. Ajouter metrics
13. Tester concurrence
14. Valider data integrity
15. Tester recovery

**Fichiers** :
- crates/infrastructure/src/sanctions/sync_service.rs

**Dépendances** : STORY-SAN-03

---

### STORY-SAN-08 | Feature: Sanctions dashboard (hits, status)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Sanctions (BC5)

**User Story** :
> En tant que compliance manager, je veux un dashboard des screening results et hits.

**Scénarios BDD** :
```gherkin
Given dashboard chargé
When j'affiche :
  - Total hits par liste
  - Screening trends (graph)
  - Potential matches pending
  - Recent updates
Then données en temps réel
```

**Tâches TDD** :
1. Créer dashboard endpoints
2. Implémenter stats queries
3. Ajouter caching (5min)
4. Créer graphs data
5. Implémenter filtering
6. Ajouter export (CSV)
7. Créer response DTOs
8. Ajouter auth
9. Implémenter pagination
10. Tester avec données
11. Ajouter monitoring
12. Documenter endpoints
13. Tester performance
14. Valider data accuracy
15. Tester concurrence

**Fichiers** :
- crates/infrastructure/src/sanctions/dashboard_handlers.rs

**Dépendances** : STORY-SAN-04

---

## Epic 7 : Prudential (BC6) — Must Have

### STORY-PRU-01 | Domain: PrudentialRatio aggregate
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Prudential (BC6)
**Entité DDD** : PrudentialRatio (AggregateRoot), RiskWeightedAsset (ValueObject), RegulatoryCapital (ValueObject)
**Invariants** : INV-02 (solvabilité ≥10%), INV-03 (Tier1 ≥7%), INV-04 (C/D ≤120%), INV-05 (concentration ≤25%)

**User Story** :
> En tant qu'architecte prudentiel, je veux l'agrégat PrudentialRatio pour calculer ratios de solvabilité en temps réel (Circ. 2016-03, 2018-06, 2018-10).

**Scénarios BDD** :
```gherkin
Given Prudential domain vide
When je crée PrudentialRatio avec :
  - ratio_id: Uuid
  - institution_id: Uuid
  - capital_tier1: Money
  - capital_tier2: Money
  - risk_weighted_assets: Money
  - total_assets: Money
  - large_exposures: Vec<Exposure>
Then PrudentialRatio validé
When je calcule solvency_ratio = capital / RWA
Then ratio >= 10% (INV-02)
When je calcule tier1_ratio = tier1 / RWA
Then ratio >= 7% (INV-03)
When je calcule credit_to_deposit = credits / deposits
Then ratio <= 120% (INV-04)
When j'ajoute exposure > 25% total_assets
Then erreur concentration (INV-05)
```

**Tâches TDD** :
1. Créer crates/domain/src/prudential/ module
2. Définir RatioId(Uuid)
3. Créer Capital ValueObject (Tier1, Tier2)
4. Créer RiskWeightedAsset ValueObject
5. Implémenter PrudentialRatio aggregate
6. Implémenter solvency_ratio() → Decimal
7. Implémenter tier1_ratio() → Decimal
8. Implémenter credit_to_deposit_ratio() → Decimal
9. Implémenter concentration_check() → Result
10. Ajouter invariants (breaches détectées)
11. Créer Exposure entity
12. Implémenter exposure concentration tracking
13. Ajouter validation au constructeur
14. Tester tous les scénarios BDD
15. Documenter formules

**Fichiers** :
- crates/domain/src/prudential/ratio.rs
- crates/domain/src/prudential/capital.rs
- crates/domain/src/prudential/exposure.rs

**Dépendances** : STORY-T09, STORY-AC-01, STORY-CR-01

---

### STORY-PRU-02 | Application: RatioCalculationService (real-time)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Prudential (BC6)

**User Story** :
> En tant que risk manager, je veux que les ratios prudentiels soient recalculés en temps réel.

**Scénarios BDD** :
```gherkin
Given RatioCalculationService
When un crédit est approuvé
Then RWA recalculé, ratios mis à jour
When C/D monte à 121%
Then alert breach créée, intervention requise
```

**Tâches TDD** :
1. Créer RatioCalculationService
2. Implémenter real-time calculation trigger
3. Implémenter breach detection
4. Créer alerts
5. Ajouter audit logging
6. Tester workflows
7. Ajouter notifications
8. Documenter service
9. Ajouter monitoring
10. Tester avec données variées
11. Implémenter caching (5min)
12. Tester performance
13. Valider accuracy
14. Tester edge cases
15. Ajouter error handling

**Fichiers** :
- crates/application/src/prudential/ratio_service.rs

**Dépendances** : STORY-PRU-01

---

### STORY-PRU-03 | Infrastructure: Ratio tables + snapshots
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Prudential (BC6)

**User Story** :
> En tant que DBA, je veux des tables pour ratios et snapshots quotidiens.

**Scénarios BDD** :
```gherkin
Given migrations/ vide
When je crée migrations/XX_prudential_schema.sql
Then tables :
  - prudential_ratios (id, institution_id, solvency, tier1, c_d, concentration)
  - ratio_snapshots (id, ratio_id, snapshot_date, breach_type)
```

**Tâches TDD** :
1. Créer migrations/XX_prudential_schema.sql
2. Créer table prudential_ratios
3. Créer table ratio_snapshots
4. Ajouter indexes
5. Créer PrudentialRepository
6. Implémenter save/find
7. Ajouter pagination
8. Exécuter migrations
9. Tester avec données
10. Valider indexes
11. Ajouter audit columns
12. Documenter schema
13. Tester queries
14. Valider performance
15. Ajouter constraints

**Fichiers** :
- migrations/XX_prudential_schema.sql
- crates/infrastructure/src/prudential/repositories.rs

**Dépendances** : STORY-T03, STORY-PRU-02

---

### STORY-PRU-04 | API: GET /prudential/ratios (current)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Prudential (BC6)

**User Story** :
> En tant que superviseur, je veux consulter les ratios courants via API.

**Scénarios BDD** :
```gherkin
Given API Actix démarrée
When je GET /prudential/ratios
Then :
  - Status 200
  - Response: { "solvency": 12.5, "tier1": 8.2, ... }
```

**Tâches TDD** :
1. Créer handler GET /prudential/ratios
2. Implémenter query
3. Ajouter auth (supervisor+)
4. Créer response DTO
5. Ajouter caching
6. Tester 200, 401
7. Documenter OpenAPI
8. Ajouter monitoring
9. Tester format
10. Ajouter error handling
11. Tester concurrence
12. Valider data accuracy
13. Ajouter timestamps
14. Tester performance
15. Ajouter audit logging

**Fichiers** :
- crates/infrastructure/src/prudential/handlers.rs

**Dépendances** : STORY-PRU-03

---

### STORY-PRU-05 | Feature: Solvency ratio (minimum 10%, Circ. 2016-03)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Prudential (BC6)

**User Story** :
> En tant que système, je veux appliquer et monitorer le ratio de solvabilité minimum 10%.

**Scénarios BDD** :
```gherkin
Given ratio = 11%
Then all clear
When ratio → 9.5%
Then alert breach_solvency créée
```

**Tâches TDD** :
1. Implémenter calculation
2. Ajouter threshold check
3. Créer breach alert
4. Ajouter notifications
5. Ajouter audit trail
6. Tester thresholds
7. Ajouter monitoring
8. Documenter formule
9. Tester edge cases
10. Valider compliance
11. Ajouter recovery logic
12. Tester avec données réelles
13. Ajouter role checks
14. Documenter intervention workflow
15. Valider transitions

**Fichiers** :
- crates/application/src/prudential/solvency_check.rs

**Dépendances** : STORY-PRU-02

---

### STORY-PRU-06 | Feature: Tier 1 ratio (minimum 7%)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Prudential (BC6)

**User Story** :
> En tant que système, je veux appliquer Tier 1 ratio minimum 7%.

**Scénarios BDD** :
```gherkin
Given tier1_ratio = 8%
Then clear
When tier1_ratio → 6%
Then alert_tier1_breach créée
```

**Tâches TDD** :
1-15 : Similar pattern to STORY-PRU-05

**Fichiers** :
- crates/application/src/prudential/tier1_check.rs

**Dépendances** : STORY-PRU-02

---

### STORY-PRU-07 | Feature: C/D ratio (maximum 120%, Circ. 2016-03)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Prudential (BC6)

**User Story** :
> En tant que système, je veux appliquer C/D ratio maximum 120%.

**Scénarios BDD** :
```gherkin
Given C/D = 110%
Then clear
When nouveau crédit porte à 121%
Then reject crédit + alert
```

**Tâches TDD** :
1-15 : Similar pattern with C/D specific logic

**Fichiers** :
- crates/application/src/prudential/credit_deposit_check.rs

**Dépendances** : STORY-PRU-02

---

### STORY-PRU-08 | Feature: Concentration risk (maximum 25% per beneficiary)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Prudential (BC6)

**User Story** :
> En tant que système, je veux limiter concentration par client à 25% du capital.

**Scénarios BDD** :
```gherkin
Given customer exposures = 20%
Then clear
When crédit supplémentaire porterait à 26%
Then reject
```

**Tâches TDD** :
1-15 : Similar pattern with concentration logic

**Fichiers** :
- crates/application/src/prudential/concentration_check.rs

**Dépendances** : STORY-PRU-02

---

## Epic 8 : Accounting (BC7) — Must Have

### STORY-ACC-01 | Domain: JournalEntry + Ledger aggregates
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Accounting (BC7)
**Entité DDD** : JournalEntry (AggregateRoot), Ledger (AggregateRoot), ChartOfAccounts (ValueObject)
**Invariant** : INV-11 (écritures équilibrées débit=crédit)

**User Story** :
> En tant qu'architecte comptable, je veux l'agrégat JournalEntry avec double-entry bookkeeping selon NCT 01/21/24/25.

**Scénarios BDD** :
```gherkin
Given Accounting domain vide
When je crée JournalEntry avec :
  - entry_id: Uuid
  - journal_code: String (OD, CP, VT, etc.)
  - entry_date: Date
  - description: String
  - lines: Vec<JournalLine> (debit + credit)
Then JournalEntry validé :
  - total_debit = total_credit (INV-11)
  - each line a valid account code
When je crée entry avec 1000 DR et 500 CR
Then erreur imbalance
When j'approuve entry
Then status = Posted, reversal blocked
When je reverse
Then new entry créée avec montants inversés
```

**Tâches TDD** :
1. Créer crates/domain/src/accounting/ module
2. Définir EntryId(Uuid)
3. Créer JournalLine entity (account, debit, credit)
4. Créer AccountCode ValueObject (validation format)
5. Implémenter JournalEntry aggregate
6. Ajouter invariant debit = credit
7. Implémenter validate() → Result
8. Implémenter post() → marks posted
9. Implémenter reverse() → new entry
10. Créer ChartOfAccounts (NCT mapping)
11. Implémenter account validation
12. Ajouter audit trail
13. Tester tous les scénarios
14. Documenter NCT accounts
15. Implémenter event sourcing (optional)

**Fichiers** :
- crates/domain/src/accounting/journal_entry.rs
- crates/domain/src/accounting/journal_line.rs
- crates/domain/src/accounting/chart_of_accounts.rs

**Dépendances** : STORY-T09

---

### STORY-ACC-02 | Application: AccountingService (posting, reversal)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Accounting (BC7)

**User Story** :
> En tant qu'analyste comptable, je veux un service pour poster et reverser les écritures avec workflow.

**Scénarios BDD** :
```gherkin
Given AccountingService
When j'appelle post_entry(entry)
Then status = Posted, immutable
When je reverse (période <= 30j)
Then new entry auto-créée
```

**Tâches TDD** :
1. Créer AccountingService
2. Implémenter IJournalRepository
3. Implémenter post_entry()
4. Implémenter reverse_entry()
5. Ajouter period checks
6. Créer DTOs
7. Ajouter audit logging
8. Implémenter error handling
9. Ajouter notifications
10. Tester workflows
11. Documenter service
12. Ajouter monitoring
13. Valider accuracy
14. Tester edge cases
15. Ajouter role checks

**Fichiers** :
- crates/application/src/accounting/accounting_service.rs

**Dépendances** : STORY-ACC-01

---

### STORY-ACC-03 | Infrastructure: Accounting tables (NCT 21/24/25)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Accounting (BC7)

**User Story** :
> En tant que DBA, je veux les tables pour journal, ledger et chart of accounts (NCT format).

**Scénarios BDD** :
```gherkin
Given migrations/ vide
When je crée migrations/XX_accounting_schema.sql
Then tables :
  - journal_entries (id, journal_code, date, description, status)
  - journal_lines (id, entry_id, account_code, debit, credit)
  - chart_of_accounts (code, label, type, nct_ref)
```

**Tâches TDD** :
1. Créer migrations
2. Créer tables (3)
3. Ajouter FK constraints
4. Ajouter indexes (account_code, date, status)
5. Charger chart of accounts NCT
6. Créer JournalRepository
7. Implémenter queries
8. Exécuter migrations
9. Tester avec données
10. Valider constraints
11. Ajouter audit columns
12. Documenter schema
13. Tester queries
14. Valider performance
15. Ajouter integrity checks

**Fichiers** :
- migrations/XX_accounting_schema.sql
- crates/infrastructure/src/accounting/repositories.rs

**Dépendances** : STORY-T03, STORY-ACC-02

---

### STORY-ACC-04 | API: POST /accounting/entries (manual posting, Admin)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Accounting (BC7)

**User Story** :
> En tant qu'administrateur, je veux poster des écritures manuelles via API avec validation stricte.

**Scénarios BDD** :
```gherkin
Given API Actix
When je POST /accounting/entries avec entry balanced
Then Status 201, entry posted
When entry imbalanced
Then Status 422, detailed error
```

**Tâches TDD** :
1. Créer handler POST /accounting/entries
2. Implémenter validation strict
3. Ajouter admin-only auth
4. Appeler AccountingService
5. Mapper erreurs
6. Retourner DTO
7. Ajouter audit logging
8. Ajouter rate limiting
9. Tester 201, 422, 401
10. Documenter OpenAPI
11. Ajouter error details
12. Tester concurrence
13. Ajouter monitoring
14. Valider format
15. Tester workflows

**Fichiers** :
- crates/infrastructure/src/accounting/handlers.rs

**Dépendances** : STORY-ACC-03

---

### STORY-ACC-05 | Feature: Automatic entries (account opening, interest)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Accounting (BC7)

**User Story** :
> En tant que système, je veux auto-générer les écritures (ouverture compte, intérêts, provisions).

**Scénarios BDD** :
```gherkin
Given compte ouvert
When event account_opened
Then auto-post DR: Account / CR: Capital
When intérêt calculé
Then auto-post DR: Interest / CR: Revenue
```

**Tâches TDD** :
1. Créer auto-entry generator
2. Implémenter account opening entry
3. Implémenter interest entry
4. Implémenter provision entry
5. Implémenter loan disbursement entry
6. Ajouter error handling
7. Ajouter audit logging
8. Tester tous les scénarios
9. Ajouter monitoring
10. Documenter triggers
11. Tester concurrence
12. Valider accuracy
13. Ajouter rollback capability
14. Tester avec données réelles
15. Valider compliance

**Fichiers** :
- crates/application/src/accounting/auto_entry_service.rs

**Dépendances** : STORY-ACC-02

---

### STORY-ACC-06 | Feature: General ledger + trial balance
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Accounting (BC7)

**User Story** :
> En tant qu'analyste, je veux afficher le grand livre et balance de vérification.

**Scénarios BDD** :
```gherkin
Given entries postées
When j'affiche ledger
Then sum(debit) per account, sum(credit)
When j'affiche trial balance
Then every account listed avec solde final
And total_debit = total_credit
```

**Tâches TDD** :
1. Créer ledger queries
2. Implémenter account aggregation
3. Implémenter trial balance calculation
4. Ajouter filtering par période
5. Créer response DTOs
6. Ajouter caching
7. Ajouter export (PDF, CSV)
8. Tester accuracy
9. Ajouter monitoring
10. Tester performance
11. Documenter format
12. Tester edge cases
13. Ajouter reconciliation checks
14. Valider data
15. Ajouter audit trail

**Fichiers** :
- crates/application/src/accounting/ledger_service.rs

**Dépendances** : STORY-ACC-04

---

### STORY-ACC-07 | Feature: Period closing + reconciliation
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Accounting (BC7)

**User Story** :
> En tant qu'analyste, je veux clôturer les périodes et reconcilier les soldes.

**Scénarios BDD** :
```gherkin
Given période = février 2026
When je lance closing
Then :
  - Toutes les entries validées
  - Trial balance équilibrée
  - Période locked (no new entries)
When reconciliation check
Then bank balance = G/L balance
```

**Tâches TDD** :
1. Créer period closing logic
2. Implémenter validation pre-closing
3. Implémenter period lock
4. Créer reconciliation workflow
5. Implémenter variance reporting
6. Ajouter approval workflow
7. Créer audit trail
8. Ajouter notifications
9. Tester edge cases
10. Ajouter error handling
11. Documenter process
12. Tester with data
13. Valider integrity
14. Ajouter monitoring
15. Implementer rollback

**Fichiers** :
- crates/application/src/accounting/period_service.rs

**Dépendances** : STORY-ACC-06

---

### STORY-ACC-08 | Feature: IFRS 9 ECL staging (preparation)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Accounting (BC7)

**User Story** :
> En tant qu'analyste, je veux préparer les données pour IFRS 9 ECL (Expected Credit Loss) reporting.

**Scénarios BDD** :
```gherkin
Given loans avec asset classes
When je génère ECL staging
Then :
  - Stage 1 (low risk)
  - Stage 2 (significant increase risk)
  - Stage 3 (default)
And ECL amounts calculés
```

**Tâches TDD** :
1. Créer ECL staging logic
2. Implémenter stage classification
3. Implémenter ECL calculation
4. Ajouter filtering
5. Créer export
6. Ajouter caching
7. Tester avec données
8. Ajouter monitoring
9. Documenter method
10. Valider accuracy
11. Ajouter audit trail
12. Tester performance
13. Ajouter error handling
14. Valider compliance
15. Ajouter notifications

**Fichiers** :
- crates/application/src/accounting/ecl_service.rs

**Dépendances** : STORY-ACC-07

---

## Epic 9 : Governance (BC11) — Must Have

### STORY-GOV-01 | Domain: AuditTrail aggregate (immutable)
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Governance (BC11)
**Entité DDD** : AuditTrailEntry (AggregateRoot), HashChain (ValueObject)
**Invariant** : INV-12 (piste d'audit immutable)

**User Story** :
> En tant qu'architecte governance, je veux l'agrégat AuditTrail avec append-only integrity via hash chain (Circ. 2006-19, 2021-05).

**Scénarios BDD** :
```gherkin
Given AuditTrail domain vide
When je crée AuditTrailEntry avec :
  - entry_id: Uuid
  - timestamp: DateTime
  - user_id: Uuid
  - action: String (CREATE, UPDATE, DELETE)
  - resource_type: String (Customer, Account, Loan)
  - resource_id: Uuid
  - changes: Json (before/after)
  - ip_address: String
  - hash: String (SHA256)
  - previous_hash: String
Then AuditTrailEntry validé :
  - hash = SHA256(entry_data + previous_hash)
  - immutable (no updates after creation)
When j'essaie de modifier entry
Then erreur ViolatedInvariant
When je valide chain integrité
Then hash_chain_valid() confirme
```

**Tâches TDD** :
1. Créer crates/domain/src/governance/ module
2. Définir EntryId(Uuid)
3. Créer Action enum
4. Créer AuditTrailEntry aggregate
5. Implémenter SHA256 hashing
6. Implémenter hash chain validation
7. Ajouter immutability constraint
8. Implémenter changes tracking (before/after)
9. Créer HashChain ValueObject
10. Implémenter chain integrity check
11. Ajouter validation au constructeur
12. Tester tous les scénarios
13. Documenter hash algo
14. Implémenter chain verification
15. Ajouter audit proof

**Fichiers** :
- crates/domain/src/governance/audit_trail_entry.rs
- crates/domain/src/governance/hash_chain.rs

**Dépendances** : STORY-T09

---

### STORY-GOV-02 | Application: AuditService (logging all operations)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant que compliance officer, je veux que toutes les opérations soient loggées automatiquement dans AuditTrail.

**Scénarios BDD** :
```gherkin
Given AuditService avec middleware Actix
When un POST /customers est appelé
Then automatiquement enregistré dans audit_trail
When DELETE opération
Then action = DELETE, before state sauvegardé
```

**Tâches TDD** :
1. Créer AuditService
2. Implémenter IAuditRepository
3. Créer Actix middleware
4. Implémenter action logging
5. Implémenter change tracking
6. Ajouter user context
7. Implémenter IP tracking
8. Créer DTOs
9. Ajouter error handling
10. Tester middleware
11. Documenting service
12. Ajouter monitoring
13. Tester workflows
14. Valider coverage
15. Ajouter notifications

**Fichiers** :
- crates/application/src/governance/audit_service.rs
- crates/infrastructure/src/governance/audit_middleware.rs

**Dépendances** : STORY-GOV-01

---

### STORY-GOV-03 | Infrastructure: Audit tables (append-only)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant que DBA, je veux une table audit append-only avec indexes optimisés pour requêtes.

**Scénarios BDD** :
```gherkin
Given migrations/ vide
When je crée migrations/XX_audit_schema.sql
Then table audit_trail_entries :
  - id, timestamp, user_id, action, resource_type, resource_id
  - changes (JSONB), hash, previous_hash, ip_address
  - UNIQUE (id), no UPDATE/DELETE policy
```

**Tâches TDD** :
1. Créer migrations/XX_audit_schema.sql
2. Créer table audit_trail_entries
3. Ajouter UNIQUE constraints
4. Ajouter REVOKE UPDATE/DELETE
5. Ajouter indexes (user_id, timestamp, resource_type)
6. Ajouter JSONB indexes pour changes
7. Exécuter migrations
8. Créer AuditRepository
9. Implémenter append-only insert
10. Implémenter queries
11. Ajouter retention policies (optionnel)
12. Tester avec données
13. Valider immutability
14. Documenter schema
15. Valider performance

**Fichiers** :
- migrations/XX_audit_schema.sql
- crates/infrastructure/src/governance/repositories.rs

**Dépendances** : STORY-T03, STORY-GOV-02

---

### STORY-GOV-04 | API: GET /audit (inspectors, compliance)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant qu'inspecteur BCT, je veux consulter l'audit trail avec filtres par date/user/ressource.

**Scénarios BDD** :
```gherkin
Given API Actix
When je GET /audit?user_id=uuid&start_date=2026-01-01&action=DELETE
Then :
  - Status 200
  - Response: { "entries": [...], "total": 42 }
And data paginated
```

**Tâches TDD** :
1. Créer handler GET /audit
2. Implémenter filtres (user_id, resource_id, action, date range)
3. Implémenter tri
4. Implémenter pagination
5. Ajouter auth (inspector+)
6. Créer response DTO
7. Ajouter caching (5min)
8. Ajouter export (CSV, JSON)
9. Tester 200, 401, 403
10. Documenter OpenAPI
11. Ajouter monitoring
12. Tester avec données
13. Valider format
14. Ajouter error handling
15. Tester performance

**Fichiers** :
- crates/infrastructure/src/governance/handlers.rs

**Dépendances** : STORY-GOV-03

---

### STORY-GOV-05 | Feature: Audit trail persistence + integrity checks
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant que système, je veux que chaque entry soit hachée et que l'intégrité soit vérifiable.

**Scénarios BDD** :
```gherkin
Given 1000 entries
When je valide chain integrity
Then hash_chain_valid() = true
When quelqu'un essaie de modifier une entry
Then integrity check échoue
```

**Tâches TDD** :
1. Implémenter hash persistence
2. Implémenter chain validation
3. Créer integrity checker
4. Ajouter scheduled validation
5. Créer alerts si tampering
6. Ajouter recovery capability
7. Implémenter proof generation
8. Ajouter monitoring
9. Documenter algorithm
10. Tester with large volumes
11. Valider performance
12. Ajouter edge cases
13. Documenter audit proof
14. Tester avec données réelles
15. Ajouter metrics

**Fichiers** :
- crates/application/src/governance/integrity_service.rs

**Dépendances** : STORY-GOV-02

---

### STORY-GOV-06 | Feature: Compliance reports (3 lines of defense)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant que compliance manager, je veux des rapports selon 3 lignes de défense.

**Scénarios BDD** :
```gherkin
Given audit entries grouped par user/action
When je génère compliance report
Then :
  - 1ère ligne : operational controls
  - 2ème ligne : compliance/risk
  - 3ème ligne : audit interne
```

**Tâches TDD** :
1. Créer compliance report service
2. Implémenter 3 lignes de défense
3. Créer aggregations
4. Implémenter filtering
5. Créer report generator
6. Ajouter export (PDF)
7. Créer templates
8. Ajouter signatures
9. Tester report generation
10. Ajouter monitoring
11. Documenter format
12. Tester performance
13. Valider accuracy
14. Ajouter error handling
15. Tester avec données réelles

**Fichiers** :
- crates/application/src/governance/compliance_report_service.rs

**Dépendances** : STORY-GOV-04

---

### STORY-GOV-07 | Feature: Committee governance
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant que système, je veux tracker les décisions de comités avec audit.

**Scénarios BDD** :
```gherkin
Given comité de crédit
When je crée decision
Then tracked avec :
  - Committee members
  - Votes
  - Outcome
  - Justification
```

**Tâches TDD** :
1. Créer Committee entity
2. Implémenter voting
3. Implémenter decision recording
4. Ajouter audit trail
5. Créer notifications
6. Implémenter workflows
7. Ajouter approval chains
8. Tester decisions
9. Ajouter monitoring
10. Documenter process
11. Tester workflows
12. Valider audit
13. Ajouter error handling
14. Tester concurrence
15. Valider compliance

**Fichiers** :
- crates/domain/src/governance/committee.rs
- crates/application/src/governance/committee_service.rs

**Dépendances** : STORY-GOV-02

---

### STORY-GOV-08 | Feature: Control checks + sign-off
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant que controleur, je veux approuver les opérations sensibles avec audit.

**Scénarios BDD** :
```gherkin
Given opération sensible (large loan, freeze)
When je approuve
Then control_check = Approved, signed_by enregistré
```

**Tâches TDD** :
1. Créer control check entity
2. Implémenter approval workflow
3. Ajouter signature
4. Créer audit trail
5. Implémenter escalation
6. Ajouter notifications
7. Tester workflows
8. Ajouter monitoring
9. Documenter process
10. Tester avec données
11. Valider audit
12. Ajouter error handling
13. Tester concurrence
14. Valider compliance
15. Ajouter metrics

**Fichiers** :
- crates/domain/src/governance/control_check.rs
- crates/application/src/governance/control_service.rs

**Dépendances** : STORY-GOV-02

---

## Epic 10 : Reporting (BC8) — Should Have

### STORY-REP-01 | Domain: RegulatoryReport aggregate
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Reporting (BC8)
**Entité DDD** : RegulatoryReport (AggregateRoot), ReportTemplate (Entity)

**User Story** :
> En tant qu'architecte reporting, je veux l'agrégat RegulatoryReport pour générer rapports BCT selon normes.

**Scénarios BDD** :
```gherkin
Given Report domain vide
When je crée RegulatoryReport avec :
  - report_id: Uuid
  - report_type: String (Weekly, Monthly, Quarterly)
  - template_version: String
  - generated_at: DateTime
  - data: Json (report fields)
Then RegulatoryReport validé
When je génère weekly report
Then template validé, données calculées
```

**Tâches TDD** :
1. Créer crates/domain/src/reporting/ module
2. Définir ReportId(Uuid)
3. Créer ReportType enum
4. Créer RegulatoryReport aggregate
5. Implémenter ReportTemplate entity
6. Implémenter validation
7. Créer data structures
8. Ajouter template mapping
9. Implémenter generation logic
10. Tester tous les scénarios
11. Documenter templates
12. Ajouter versioning
13. Implémenter audit trail
14. Valider compliance
15. Ajouter error handling

**Fichiers** :
- crates/domain/src/reporting/regulatory_report.rs
- crates/domain/src/reporting/report_template.rs

**Dépendances** : STORY-T09

---

### STORY-REP-02 | Application: ReportingService
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Reporting (BC8)

**User Story** :
> En tant que compliance officer, je veux un service pour générer et gérer les rapports BCT.

**Scénarios BDD** :
```gherkin
Given ReportingService
When j'appelle generate_weekly_report(week)
Then rapport généré avec :
  - Toutes les données requises
  - Calculs vérifiés
  - Audit trail
```

**Tâches TDD** :
1. Créer ReportingService
2. Implémenter IReportRepository
3. Implémenter generate_weekly()
4. Implémenter generate_monthly()
5. Implémenter generate_quarterly()
6. Ajouter data aggregation
7. Créer DTOs
8. Ajouter audit logging
9. Implémenter error handling
10. Ajouter notifications
11. Documenting service
12. Tester workflows
13. Ajouter monitoring
14. Valider accuracy
15. Tester edge cases

**Fichiers** :
- crates/application/src/reporting/reporting_service.rs

**Dépendances** : STORY-REP-01

---

### STORY-REP-03 | Infrastructure: Report tables + generation
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Reporting (BC8)

**User Story** :
> En tant que DBA, je veux les tables pour rapports et templates.

**Scénarios BDD** :
```gherkin
Given migrations/ vide
When je crée migrations/XX_reporting_schema.sql
Then tables :
  - regulatory_reports (id, type, generated_at, data, status)
  - report_templates (id, name, version, definition)
```

**Tâches TDD** :
1. Créer migrations/XX_reporting_schema.sql
2. Créer tables
3. Ajouter indexes
4. Charger templates
5. Créer ReportRepository
6. Implémenter queries
7. Exécuter migrations
8. Tester avec données
9. Valider schema
10. Ajouter audit columns
11. Documenter schema
12. Tester queries
13. Valider performance
14. Ajouter constraints
15. Ajouter JSONB indexes

**Fichiers** :
- migrations/XX_reporting_schema.sql
- crates/infrastructure/src/reporting/repositories.rs

**Dépendances** : STORY-T03, STORY-REP-02

---

### STORY-REP-04 | API: GET /reporting/forms (BCT forms)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Reporting (BC8)

**User Story** :
> En tant qu'utilisateur, je veux consulter les rapports générés via API.

**Scénarios BDD** :
```gherkin
Given API Actix
When je GET /reporting/forms?type=Weekly&date=2026-04-01
Then :
  - Status 200
  - Response: { "reports": [...] }
```

**Tâches TDD** :
1. Créer handler GET /reporting/forms
2. Implémenter filtres (type, date)
3. Ajouter pagination
4. Ajouter auth
5. Créer response DTO
6. Ajouter caching
7. Ajouter export
8. Tester 200, 401
9. Documenter OpenAPI
10. Ajouter monitoring
11. Tester avec données
12. Valider format
13. Ajouter error handling
14. Tester performance
15. Ajouter audit logging

**Fichiers** :
- crates/infrastructure/src/reporting/handlers.rs

**Dépendances** : STORY-REP-03

---

### STORY-REP-05 | Feature: BCT forms generation (weekly, monthly)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Reporting (BC8)

**User Story** :
> En tant que système, je veux générer les formulaires BCT automatiquement chaque semaine/mois.

**Scénarios BDD** :
```gherkin
Given scheduled jobs
When lundi 9h : weekly form generated
When 1er du mois : monthly form generated
Then formulaires créés et prêts pour soumission
```

**Tâches TDD** :
1. Créer scheduled jobs
2. Implémenter weekly generation
3. Implémenter monthly generation
4. Ajouter data collection
5. Implémenter calculations
6. Créer forms
7. Ajouter validation
8. Créer notifications
9. Ajouter error handling
10. Tester generation
11. Ajouter monitoring
12. Documenter schedule
13. Tester avec données
14. Valider accuracy
15. Ajouter recovery

**Fichiers** :
- crates/infrastructure/src/reporting/generation_jobs.rs

**Dépendances** : STORY-REP-02

---

### STORY-REP-06 | Feature: Report submission + acknowledgment
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Reporting (BC8)

**User Story** :
> En tant que compliance officer, je veux soumettre les rapports à BCT et tracker les confirmations.

**Scénarios BDD** :
```gherkin
Given rapport généré
When j'appelle submit()
Then :
  - Soumis à BCT (stub)
  - Status = Submitted
  - Audit trail créé
When confirmation reçue
Then status = Acknowledged
```

**Tâches TDD** :
1. Créer submission logic
2. Implémenter BCT stub API
3. Implémenter tracking
4. Créer audit trail
5. Ajouter notifications
6. Ajouter error handling
7. Implémenter retry
8. Tester submission
9. Ajouter monitoring
10. Documenting process
11. Tester workflows
12. Valider audit
13. Ajouter timestamps
14. Tester concurrence
15. Valider compliance

**Fichiers** :
- crates/application/src/reporting/submission_service.rs
- crates/infrastructure/src/reporting/bct_stub.rs

**Dépendances** : STORY-REP-05

---

### STORY-REP-07 | Feature: Audit logs for reporting
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Reporting (BC8)

**User Story** :
> En tant qu'auditeur, je veux que chaque rapport génération/modification soit loggée.

**Scénarios BDD** :
```gherkin
Given rapport modifié
When j'affiche audit trail
Then :
  - Qui : user_id
  - Quoi : modified fields
  - Quand : timestamp
  - Avant/Après
```

**Tâches TDD** :
1. Ajouter audit logging
2. Implémenter tracking changes
3. Créer audit trail integration
4. Ajouter user context
5. Ajouter timestamps
6. Créer queries
7. Ajouter export
8. Tester logging
9. Ajouter monitoring
10. Documenting format
11. Tester avec données
12. Valider accuracy
13. Ajouter error handling
14. Tester concurrence
15. Valider compliance

**Fichiers** :
- crates/application/src/reporting/audit_service.rs

**Dépendances** : STORY-REP-06

---

### STORY-REP-08 | Feature: IFRS 9 reporting prep
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Reporting (BC8)

**User Story** :
> En tant qu'analyste, je veux préparer les données pour reporting IFRS 9.

**Scénarios BDD** :
```gherkin
Given loans avec ECL stages
When je génère IFRS 9 report
Then :
  - Stage 1 : ECL 12-month
  - Stage 2 : ECL lifetime
  - Stage 3 : ECL lifetime
```

**Tâches TDD** :
1. Créer IFRS 9 service
2. Implémenter data mapping
3. Implémenter calculations
4. Créer export
5. Ajouter validation
6. Ajouter filtering
7. Créer templates
8. Tester avec données
9. Valider accuracy
10. Ajouter monitoring
11. Documenting method
12. Tester performance
13. Ajouter error handling
14. Valider compliance
15. Ajouter notifications

**Fichiers** :
- crates/application/src/reporting/ifrs9_service.rs

**Dépendances** : STORY-REP-07

---

## Epic 11 : Payment (BC9) — Should Have

### STORY-PAY-01 | Domain: PaymentOrder aggregate
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Payment (BC9)
**Entité DDD** : PaymentOrder (AggregateRoot), Transfer (Entity), SwiftMessage (Entity)

**User Story** :
> En tant qu'architecte paiements, je veux l'agrégat PaymentOrder pour gérer virements domestiques et internationaux.

**Scénarios BDD** :
```gherkin
Given Payment domain vide
When je crée PaymentOrder avec :
  - order_id: Uuid
  - account_id: Uuid (sender)
  - counterparty: String/Rib (beneficiary)
  - amount: Money
  - payment_type: Enum (Domestic|Swift)
  - status: Enum (Draft|Submitted|Cleared|Rejected)
Then PaymentOrder validé :
  - sender KYC vérifié
  - Sanctions screening passed (INV-14)
  - Amount <= account balance
When je soumets payment
Then status = Submitted
When clearing confirmé
Then status = Cleared
```

**Tâches TDD** :
1. Créer crates/domain/src/payment/ module
2. Définir OrderId(Uuid)
3. Créer PaymentType enum
4. Créer PaymentOrder aggregate
5. Créer Transfer entity
6. Créer SwiftMessage entity (stub)
7. Implémenter validation
8. Ajouter invariants
9. Implémenter transitions de status
10. Créer error types
11. Ajouter audit trail
12. Tester tous les scénarios
13. Documenter payment types
14. Implémenter event sourcing (optional)
15. Valider compliance

**Fichiers** :
- crates/domain/src/payment/payment_order.rs
- crates/domain/src/payment/transfer.rs
- crates/domain/src/payment/swift_message.rs

**Dépendances** : STORY-T09, STORY-AC-01

---

### STORY-PAY-02 | Application: PaymentService
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Payment (BC9)

**User Story** :
> En tant qu'utilisateur, je veux un service pour créer et gérer les paiements.

**Scénarios BDD** :
```gherkin
Given PaymentService
When j'appelle create_payment(order)
Then :
  - PaymentOrder créé
  - KYC verified
  - Sanctions screening executed
  - Audit log créé
When j'appelle submit_payment(order_id)
Then :
  - Status = Submitted
  - Account débité
  - Journal entry créée
```

**Tâches TDD** :
1. Créer PaymentService
2. Implémenter IPaymentRepository
3. Implémenter create_payment()
4. Implémenter submit_payment()
5. Implémenter reject_payment()
6. Ajouter KYC check
7. Ajouter sanctions screening
8. Créer DTOs
9. Ajouter audit logging
10. Implémenter error handling
11. Ajouter notifications
12. Documenting service
13. Tester workflows
14. Ajouter monitoring
15. Valider accuracy

**Fichiers** :
- crates/application/src/payment/payment_service.rs

**Dépendances** : STORY-PAY-01

---

### STORY-PAY-03 | Infrastructure: Payment tables
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Payment (BC9)

**User Story** :
> En tant que DBA, je veux les tables pour payment orders et transfers.

**Scénarios BDD** :
```gherkin
Given migrations/ vide
When je crée migrations/XX_payment_schema.sql
Then tables :
  - payment_orders (id, account_id, amount, type, status)
  - transfers (id, order_id, counterparty_rib, clearing_ref)
```

**Tâches TDD** :
1. Créer migrations/XX_payment_schema.sql
2. Créer tables
3. Ajouter FK constraints
4. Ajouter indexes
5. Créer PaymentRepository
6. Implémenter queries
7. Exécuter migrations
8. Tester avec données
9. Valider schema
10. Ajouter audit columns
11. Documenter schema
12. Tester queries
13. Valider performance
14. Ajouter constraints
15. Ajouter data validations

**Fichiers** :
- migrations/XX_payment_schema.sql
- crates/infrastructure/src/payment/repositories.rs

**Dépendances** : STORY-T03, STORY-PAY-02

---

### STORY-PAY-04 | API: POST /payments/transfers endpoint
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Payment (BC9)

**User Story** :
> En tant qu'utilisateur web, je veux créer un virement via API.

**Scénarios BDD** :
```gherkin
Given API Actix
When je POST /payments/transfers avec :
  {
    "account_id": "uuid",
    "beneficiary_rib": "10000123456789",
    "amount": 5000
  }
Then :
  - Status 201 Created
  - Response: { "order_id": "uuid-xxx", "status": "Submitted" }
When je POST sans KYC
Then Status 422 + erreur KYC required
```

**Tâches TDD** :
1. Créer handler POST /payments/transfers
2. Implémenter validation JSON
3. Ajouter auth check (JWT)
4. Appeler PaymentService
5. Mapper erreurs
6. Retourner PaymentOrderResponse
7. Ajouter audit logging
8. Ajouter rate limiting
9. Tester 201, 400, 401, 422
10. Documenter OpenAPI
11. Ajouter error details
12. Tester concurrence
13. Ajouter monitoring
14. Valider format réponse
15. Tester workflows

**Fichiers** :
- crates/infrastructure/src/payment/handlers.rs

**Dépendances** : STORY-PAY-03

---

### STORY-PAY-05 | Feature: SWIFT integration (stub, ISO 20022)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Payment (BC9)

**User Story** :
> En tant que système, je veux générer messages SWIFT (stub) pour paiements internationaux.

**Scénarios BDD** :
```gherkin
Given payment de type SWIFT
When je génère SWIFT message
Then message au format ISO 20022 stub
When j'envoie (stub)
Then audit trail créé, status = Sent
```

**Tâches TDD** :
1. Créer SWIFT message generator
2. Implémenter ISO 20022 stub format
3. Créer message fields mapping
4. Ajouter validation
5. Créer SWIFT API stub
6. Implémenter transmission (stub)
7. Ajouter error handling
8. Créer audit trail
9. Ajouter monitoring
10. Documenting format
11. Tester generation
12. Tester transmission
13. Valider compliance
14. Ajouter error recovery
15. Tester edge cases

**Fichiers** :
- crates/infrastructure/src/payment/swift_stub.rs

**Dépendances** : STORY-PAY-02

---

### STORY-PAY-06 | Feature: Clearing & compensation (ACH)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Payment (BC9)

**User Story** :
> En tant que système, je veux gérer le clearing et la compensation des paiements.

**Scénarios BDD** :
```gherkin
Given 50 payments soumis
When j'exécute clearing batch (fin de journée)
Then :
  - Tous les payments balancés
  - Journal entries créées
  - Clearing ref assigné
  - Status = Cleared
```

**Tâches TDD** :
1. Créer clearing service
2. Implémenter batch processing
3. Implémenter compensation logic
4. Créer journal entries
5. Assigner clearing references
6. Ajouter reconciliation
7. Créer audit trail
8. Ajouter monitoring
9. Tester clearing
10. Ajouter error handling
11. Documenting process
12. Tester avec données
13. Valider accuracy
14. Ajouter notifications
15. Implémenter rollback

**Fichiers** :
- crates/infrastructure/src/payment/clearing_service.rs

**Dépendances** : STORY-PAY-04

---

### STORY-PAY-07 | Feature: Payment status tracking
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Payment (BC9)

**User Story** :
> En tant qu'utilisateur, je veux tracker le statut de mon paiement en temps réel.

**Scénarios BDD** :
```gherkin
Given payment_id
When j'appelle GET /payments/{id}/status
Then :
  - Status 200
  - Response: { "status": "Cleared", "tracking": [...] }
```

**Tâches TDD** :
1. Créer handler GET /payments/{id}/status
2. Implémenter query
3. Ajouter auth (owner only)
4. Créer response DTO
5. Ajouter caching
6. Ajouter error handling
7. Tester 200, 401, 404
8. Documenter OpenAPI
9. Ajouter monitoring
10. Tester avec données
11. Valider format
12. Ajouter webhooks (optional)
13. Ajouter notifications
14. Tester performance
15. Valider accuracy

**Fichiers** :
- crates/infrastructure/src/payment/handlers.rs

**Dépendances** : STORY-PAY-06

---

### STORY-PAY-08 | Feature: Sanctions screening on payment (INV-14)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Payment (BC9)

**User Story** :
> En tant que système, je veux bloquer les paiements si bénéficiaire dans sanctions list.

**Scénarios BDD** :
```gherkin
Given payment order
When j'exécute screening avant clearing
Then :
  - Clear → continue
  - Hit → reject avec raison
```

**Tâches TDD** :
1. Créer screening integration
2. Implémenter screening call
3. Implémenter rejection logic
4. Créer audit trail
5. Ajouter notifications
6. Créer error handling
7. Tester screening
8. Ajouter monitoring
9. Documenting workflow
10. Tester avec données
11. Valider compliance
12. Ajouter retry logic
13. Valider accuracy
14. Tester edge cases
15. Ajouter metrics

**Fichiers** :
- crates/application/src/payment/screening_integration.rs

**Dépendances** : STORY-PAY-02

---

## Epic 12 : ForeignExchange (BC10) — Could Have

### STORY-FX-01 | Domain: FxOperation aggregate
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : ForeignExchange (BC10)
**Entité DDD** : FxOperation (AggregateRoot), ExchangeRate (ValueObject)

**User Story** :
> En tant qu'architecte FX, je veux l'agrégat FxOperation pour gérer les opérations de change.

**Scénarios BDD** :
```gherkin
Given FX domain vide
When je crée FxOperation avec :
  - operation_id: Uuid
  - account_id: Uuid
  - source_currency: String (TND, EUR, USD)
  - target_currency: String
  - source_amount: Money
  - target_amount: Money
  - rate: Decimal
  - operation_date: Date
  - status: Enum (Draft|Confirmed|Settled|Rejected)
Then FxOperation validé :
  - rate > 0
  - currencies != same
  - target_amount = source_amount × rate
```

**Tâches TDD** :
1. Créer crates/domain/src/foreign_exchange/ module
2. Définir OperationId(Uuid)
3. Créer Currency enum (TND, EUR, USD, etc.)
4. Créer ExchangeRate ValueObject
5. Implémenter FxOperation aggregate
6. Implémenter rate validation
7. Implémenter amount calculation
8. Ajouter status transitions
9. Créer error types
10. Ajouter validation
11. Implémenter audit trail
12. Tester tous les scénarios
13. Documenter currencies
14. Implémenter event sourcing (optional)
15. Valider compliance

**Fichiers** :
- crates/domain/src/foreign_exchange/fx_operation.rs
- crates/domain/src/foreign_exchange/exchange_rate.rs

**Dépendances** : STORY-T09, STORY-AC-01

---

### STORY-FX-02 | Application: FxService (Loi 76-18)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : ForeignExchange (BC10)

**User Story** :
> En tant qu'analyste FX, je veux un service pour gérer les opérations de change selon Loi 76-18.

**Scénarios BDD** :
```gherkin
Given FxService
When j'appelle create_operation(operation)
Then :
  - Operation créée
  - Loi 76-18 compliance checked
  - Daily limits verified
  - Audit trail créé
```

**Tâches TDD** :
1. Créer FxService
2. Implémenter IFxRepository
3. Implémenter create_operation()
4. Implémenter confirm_operation()
5. Implémenter settle_operation()
6. Ajouter Loi 76-18 check
7. Créer DTOs
8. Ajouter audit logging
9. Implémenter error handling
10. Ajouter notifications
11. Documenting service
12. Tester workflows
13. Ajouter monitoring
14. Valider accuracy
15. Tester edge cases

**Fichiers** :
- crates/application/src/foreign_exchange/fx_service.rs

**Dépendances** : STORY-FX-01

---

### STORY-FX-03 | Infrastructure: FX tables
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : ForeignExchange (BC10)

**User Story** :
> En tant que DBA, je veux les tables pour opérations et taux de change.

**Scénarios BDD** :
```gherkin
Given migrations/ vide
When je crée migrations/XX_fx_schema.sql
Then tables :
  - fx_operations (id, account_id, source_currency, target_currency, rate, status)
  - exchange_rates (id, source_currency, target_currency, rate, date)
```

**Tâches TDD** :
1. Créer migrations/XX_fx_schema.sql
2. Créer tables
3. Ajouter indexes
4. Créer FxRepository
5. Implémenter queries
6. Exécuter migrations
7. Tester avec données
8. Valider schema
9. Ajouter audit columns
10. Documenter schema
11. Tester queries
12. Valider performance
13. Ajouter constraints
14. Ajouter validations
15. Ajouter rate history

**Fichiers** :
- migrations/XX_fx_schema.sql
- crates/infrastructure/src/foreign_exchange/repositories.rs

**Dépendances** : STORY-T03, STORY-FX-02

---

### STORY-FX-04 | API: POST /fx/operations endpoint
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : ForeignExchange (BC10)

**User Story** :
> En tant qu'utilisateur web, je veux créer une opération de change via API.

**Scénarios BDD** :
```gherkin
Given API Actix
When je POST /fx/operations avec :
  {
    "account_id": "uuid",
    "source_currency": "TND",
    "target_currency": "EUR",
    "source_amount": 1000
  }
Then Status 201 + operation créée
```

**Tâches TDD** :
1. Créer handler POST /fx/operations
2. Implémenter validation
3. Ajouter auth check
4. Appeler FxService
5. Mapper erreurs
6. Retourner FxOperationResponse
7. Ajouter audit logging
8. Ajouter rate limiting
9. Tester 201, 400, 401
10. Documenter OpenAPI
11. Ajouter error details
12. Tester concurrence
13. Ajouter monitoring
14. Valider format
15. Tester workflows

**Fichiers** :
- crates/infrastructure/src/foreign_exchange/handlers.rs

**Dépendances** : STORY-FX-03

---

### STORY-FX-05 | Feature: Exchange rate management
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : ForeignExchange (BC10)

**User Story** :
> En tant qu'opérateur, je veux gérer les taux de change avec mise à jour automatique.

**Scénarios BDD** :
```gherkin
Given taux TND/EUR
When je mets à jour taux
Then :
  - Nouveau taux sauvegardé avec timestamp
  - Historique conservé
  - Audit trail créé
```

**Tâches TDD** :
1. Créer rate management service
2. Implémenter rate update logic
3. Implémenter rate validation
4. Créer rate history
5. Ajouter audit trail
6. Ajouter notifications
7. Créer queries
8. Tester updates
9. Ajouter error handling
10. Documenting process
11. Tester avec données
12. Valider accuracy
13. Ajouter monitoring
14. Tester concurrence
15. Implémenter caching

**Fichiers** :
- crates/application/src/foreign_exchange/rate_service.rs

**Dépendances** : STORY-FX-02

---

### STORY-FX-06 | Feature: FX position monitoring
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : ForeignExchange (BC10)

**User Story** :
> En tant que risk manager, je veux monitorer les positions de change.

**Scénarios BDD** :
```gherkin
Given positions multiples currencies
When j'affiche position summary
Then :
  - Total position par currency
  - Valeur nette
  - Exposure risks
```

**Tâches TDD** :
1. Créer position monitoring service
2. Implémenter aggregation
3. Créer risk calculations
4. Ajouter exposure checks
5. Créer alerts
6. Implémenter queries
7. Créer DTOs
8. Ajouter caching
9. Tester monitoring
10. Ajouter error handling
11. Documenting process
12. Tester avec données
13. Ajouter monitoring
14. Valider accuracy
15. Ajouter metrics

**Fichiers** :
- crates/application/src/foreign_exchange/position_service.rs

**Dépendances** : STORY-FX-05

---

### STORY-FX-07 | Feature: Compliance Loi 76-18
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : ForeignExchange (BC10)

**User Story** :
> En tant que système, je veux appliquer les règles de conformité Loi 76-18.

**Scénarios BDD** :
```gherkin
Given opération FX
When j'applique compliance checks
Then Loi 76-18 rules vérifiées
```

**Tâches TDD** :
1. Créer compliance service
2. Implémenter Loi 76-18 rules
3. Créer validation logic
4. Ajouter rejection logic
5. Créer audit trail
6. Ajouter notifications
7. Tester compliance
8. Ajouter monitoring
9. Documenting rules
10. Tester avec données
11. Valider accuracy
12. Ajouter error handling
13. Tester edge cases
14. Valider compliance
15. Ajouter metrics

**Fichiers** :
- crates/application/src/foreign_exchange/compliance_service.rs

**Dépendances** : STORY-FX-02

---

### STORY-FX-08 | Feature: Daily limits enforcement
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : ForeignExchange (BC10)

**User Story** :
> En tant que système, je veux enforcer daily limits per account et currency.

**Scénarios BDD** :
```gherkin
Given daily_limit = 100k EUR
When total operations = 95k (OK)
Then operation accepted
When new operation = 10k (would exceed)
Then operation rejected
```

**Tâches TDD** :
1. Créer limits service
2. Implémenter limit checking
3. Créer limit configuration
4. Implémenter rejection
5. Créer audit trail
6. Ajouter notifications
7. Tester limits
8. Ajouter monitoring
9. Documenting limits
10. Tester avec données
11. Valider accuracy
12. Ajouter error handling
13. Tester edge cases
14. Ajouter metrics
15. Implémenter rollover

**Fichiers** :
- crates/application/src/foreign_exchange/limits_service.rs

**Dépendances** : STORY-FX-02

---

## New Stories: Data Retention, Consent, and Audit Portal

### STORY-RET-01 | Feature: Data Retention (INV-10)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1)
**Invariant** : INV-10 (rétention 10 ans post-clôture)

**User Story** :
> En tant que compliance officer, je veux que les données KYC soient conservées 10 ans après clôture de relation (Loi 2015-26 art. 125).

**Scénarios BDD** :
```gherkin
Given compte clôturé le 2016-04-04
When j'exécute cleanup job le 2026-04-04
Then données KYC encore préservées (10 ans)
When je exécute le 2026-04-05
Then données peuvent être purgées
```

**Tâches TDD** :
1. Créer retention policy service
2. Implémenter tracking de fermeture
3. Créer retention calculation
4. Implémenter auto-purge job
5. Ajouter audit trail
6. Ajouter soft-delete support
7. Créer notifications pré-purge
8. Tester retention logic
9. Ajouter error handling
10. Documenting policy
11. Tester avec dates mock
12. Valider accuracy
13. Ajouter monitoring
14. Tester concurrence
15. Valider compliance

**Fichiers** :
- crates/application/src/customer/retention_service.rs

**Dépendances** : STORY-C01

---

### STORY-CONS-01 | Feature: INPDP Consent Management (INV-13)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1)
**Invariant** : INV-13 (consentement INPDP)

**User Story** :
> En tant que DPO, je veux créer et gérer les consentements de traitement de données personnelles (Loi 2004-63).

**Scénarios BDD** :
```gherkin
Given customer créé
When je crée consent pour usage marketing
Then consent status = Pending
When customer approuve
Then status = Approved avec timestamp
When customer révoque
Then status = Revoked
And toutes les données marketing effacées
```

**Tâches TDD** :
1. Créer Consent entity
2. Implémenter consent types enum
3. Créer consent workflow
4. Implémenter approval logic
5. Implémenter revocation logic
6. Créer audit trail
7. Ajouter timestamp tracking
8. Implémenter expiry (optional)
9. Créer DTOs
10. Ajouter notifications
11. Tester workflows
12. Ajouter error handling
13. Documenting process
14. Tester avec données
15. Valider compliance

**Fichiers** :
- crates/domain/src/customer/consent.rs
- crates/application/src/customer/consent_service.rs

**Dépendances** : STORY-C01

---

### STORY-CONS-02 | Feature: INPDP Rights Management (INV-13)
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Customer (BC1)
**Invariant** : INV-13

**User Story** :
> En tant que DPO, je veux gérer les droits d'accès, rectification et opposition (Loi 2004-63).

**Scénarios BDD** :
```gherkin
Given customer demande accès données
When j'exécute access_right_request()
Then :
  - Rapport généré avec toutes les données
  - Envoyé à customer dans 30 jours
When customer demande rectification
Then form ouvert, données modifiées, audit trail créé
When customer s'oppose au marketing
Then marketing flag = false, data retention policy appliquée
```

**Tâches TDD** :
1. Créer DataAccessRequest entity
2. Créer RectificationRequest entity
3. Créer OppositionRequest entity
4. Implémenter request workflow
5. Implémenter data collection (access)
6. Implémenter modification (rectification)
7. Implémenter opposition flag
8. Créer audit trail
9. Ajouter 30-day deadline
10. Créer notifications
11. Tester workflows
12. Ajouter error handling
13. Documenting process
14. Tester avec données
15. Valider compliance

**Fichiers** :
- crates/domain/src/customer/data_rights.rs
- crates/application/src/customer/data_rights_service.rs

**Dépendances** : STORY-CONS-01

---

### STORY-AUD-01 | API: Audit trail portal for BCT inspectors
**Type** : Feature | **Taille** : L (5h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant qu'inspecteur BCT, je veux accéder à l'audit trail complet via portail avec filtres avancés.

**Scénarios BDD** :
```gherkin
Given API Actix
When je GET /bct/audit/entries?user_id=uuid&action=DELETE&start_date=2026-01-01&end_date=2026-04-04
Then :
  - Status 200
  - Response: { "entries": [...], "total": 42, "pages": 3 }
  - Données vérifiables (hash chain)
When j'exporte CSV
Then file téléchargeable
```

**Tâches TDD** :
1. Créer BCT audit handlers (new)
2. Implémenter multi-filter support
3. Ajouter date range filtering
4. Ajouter action filtering
5. Ajouter user filtering
6. Ajouter resource_type filtering
7. Implémenter pagination avancée
8. Ajouter tri multiple
9. Créer export CSV
10. Créer export JSON
11. Ajouter auth (inspector role)
12. Ajouter rate limiting
13. Créer response DTO
14. Ajouter caching (5min)
15. Documenter OpenAPI complet

**Fichiers** :
- crates/infrastructure/src/governance/bct_audit_handlers.rs

**Dépendances** : STORY-GOV-04

---

### STORY-AUD-02 | Feature: Audit dashboard for supervisors
**Type** : Feature | **Taille** : M (3h)
**Bounded Context** : Governance (BC11)

**User Story** :
> En tant que superviseur, je veux un dashboard audit avec graphs de trends et alertes.

**Scénarios BDD** :
```gherkin
Given dashboard chargé
When j'affiche :
  - Total entries par jour (graph)
  - Top users (bar chart)
  - Actions breakdown (pie chart)
  - Recent suspicious activities (alerts)
Then données en temps réel, caching 5min
```

**Tâches TDD** :
1. Créer dashboard service
2. Implémenter stats queries
3. Créer graph data generators
4. Implémenter trend analysis
5. Ajouter suspicious activity detection
6. Créer alerts
7. Implémenter response DTOs
8. Ajouter caching (5min)
9. Ajouter auth (supervisor+)
10. Tester avec données
11. Ajouter monitoring
12. Documenting endpoints
13. Tester performance
14. Valider data accuracy
15. Ajouter export options

**Fichiers** :
- crates/infrastructure/src/governance/dashboard_service.rs
- crates/infrastructure/src/governance/dashboard_handlers.rs

**Dépendances** : STORY-GOV-04

---

## Sprint Planning (Sprints 1-6)

### Sprint 1 (Semaines 1-2) : Foundations + Identity
- STORY-ID-01 : User domain
- STORY-ID-02 : UserService
- STORY-ID-03 : UserRepository
- STORY-ID-04 : POST /auth/register
- STORY-ID-05 : POST /auth/login + JWT
- **Heures** : ~15h (5M stories × 3h)
- **Frontend** : Login/Register pages (Svelte components)

### Sprint 2 (Semaines 3-4) : Customer + Account
- STORY-C01 : Customer domain + KYC
- STORY-C02 : KycService
- STORY-C03 : CustomerRepository
- STORY-C04 : POST /customers
- STORY-AC-01 : Account domain
- **Heures** : ~15h
- **Frontend** : Customer onboarding, KYC forms

### Sprint 3 (Semaines 5-6) : Account Operations + Credit Setup
- STORY-AC-02 : AccountService
- STORY-AC-03 : Account tables
- STORY-AC-04 : POST /accounts
- STORY-CR-01 : Loan domain
- STORY-CR-02 : LoanService
- **Heures** : ~15h
- **Frontend** : Account dashboard, Loan forms

### Sprint 4 (Semaines 7-8) : AML + Sanctions
- STORY-AML-01 : Transaction domain
- STORY-AML-02 : Monitoring service
- STORY-SAN-01 : Sanctions domain
- STORY-SAN-02 : Screening service
- STORY-AML-05 : AML scenarios
- **Heures** : ~15h
- **Frontend** : Compliance dashboards

### Sprint 5 (Semaines 9-10) : Prudential + Accounting
- STORY-PRU-01 : Ratios domain
- STORY-PRU-02 : Calculation service
- STORY-ACC-01 : Accounting domain
- STORY-ACC-02 : Posting service
- STORY-GOV-01 : Audit trail
- **Heures** : ~15h
- **Frontend** : Risk & accounting dashboards

### Sprint 6 (Semaines 11-12) : Governance + Reporting
- STORY-GOV-02 : AuditService
- STORY-REP-01 : Reporting domain
- STORY-REP-02 : Reporting service
- STORY-ID-09 : 2FA (security hardening)
- **Documentation vivante** : E2E multi-rôles
- **Heures** : ~15h

---

## Stories Frontend (par Sprint)

### STORY-F-Auth | Authentication & Authorization UI
**Type** : Feature | **Taille** : M (4h) | **Sprint** : 1-2
**Bounded Context** : Identity (BC12) + Frontend

**User Story** :
> En tant que client, je veux me connecter et créer un compte, afin d'accéder au portail bancaire en toute sécurité.

**Scénarios BDD** :
```gherkin
Feature: Authentication
  Scenario: User registers with email
    Given the registration page is open
    When I enter email "user@example.tn"
    And I enter password "SecurePass123!"
    And I confirm password
    And I accept terms
    Then account is created
    And I receive confirmation email

  Scenario: User logs in with valid credentials
    Given user account exists
    When I enter email "user@example.tn"
    And I enter password "SecurePass123!"
    And I click "Login"
    Then I am redirected to dashboard
    And session token is stored

  Scenario: User enables 2FA
    Given I am logged in
    When I go to security settings
    And I enable two-factor authentication
    And I scan QR code with authenticator app
    And I enter 6-digit code
    Then 2FA is activated
    And backup codes are provided
```

**Fichiers (Svelte)** :
- `frontend/src/pages/login.astro` — Route page
- `frontend/src/pages/register.astro` — Registration route
- `frontend/src/components/auth/LoginForm.svelte` — Form with validation
- `frontend/src/components/auth/RegisterForm.svelte` — Multi-step form
- `frontend/src/components/auth/TwoFactorSetup.svelte` — 2FA QR + input
- `frontend/src/lib/auth.ts` — Auth API client

**Clés i18n** :
- `auth.login.title`, `auth.login.email`, `auth.login.password`
- `auth.register.title`, `auth.register.confirm_password`, `auth.register.accept_terms`
- `auth.2fa.setup_title`, `auth.2fa.scan_qr`, `auth.2fa.enter_code`
- `auth.error.invalid_credentials`, `auth.error.user_exists`

**Scénarios Playwright** :
- Test login form validation (empty fields, invalid email)
- Test successful login + redirect
- Test registration form (all fields)
- Test 2FA setup (QR scan simulation)
- Test logout flow

**WCAG Accessibility** :
- ARIA labels sur tous inputs
- Focus visible sur tous éléments interactifs
- Contraste texte/fond ≥ 4.5:1

---

### STORY-F-Customer | Customer Management & KYC Form
**Type** : Feature | **Taille** : L (6h) | **Sprint** : 3-4
**Bounded Context** : Customer (BC1) + Frontend

**User Story** :
> En tant que chargé clientèle, je veux créer une fiche client avec KYC multi-étape, afin de respecter la conformité Circ. 2025-17.

**Scénarios BDD** :
```gherkin
Feature: Customer KYC
  Scenario: Create customer with basic info
    Given KYC form is open
    When I enter full name "Ahmed Ben Khalifa"
    And I enter date of birth "1985-03-15"
    And I select gender "Male"
    And I enter CIN "12345678"
    And I click "Next"
    Then basic info is saved
    And professional info step appears

  Scenario: Complete KYC with beneficial owner
    Given professional info step is active
    When I enter profession "Software Engineer"
    And I enter employer "TechCorp"
    And I add beneficial owner 51%
    And I enter beneficial owner name "Fatima Ben Khalifa"
    And I enter beneficial owner CIN "87654321"
    Then beneficial owner is recorded
    And PEP check warning shows (if applicable)

  Scenario: KYC validation fails
    Given I filled some fields incorrectly
    When I click "Submit"
    Then validation errors appear
    And form doesn't submit
```

**Fichiers (Svelte)** :
- `frontend/src/pages/customer/onboarding.astro` — Multi-step form page
- `frontend/src/components/customer/BasicInfoStep.svelte` — Step 1
- `frontend/src/components/customer/ProfessionalInfoStep.svelte` — Step 2
- `frontend/src/components/customer/BeneficiaryStep.svelte` — Step 3 (beneficial owners)
- `frontend/src/components/customer/DocumentUpload.svelte` — Document upload
- `frontend/src/components/customer/KycStepper.svelte` — Progress indicator

**Clés i18n** :
- `customer.kyc.full_name`, `customer.kyc.date_of_birth`, `customer.kyc.cin`
- `customer.kyc.profession`, `customer.kyc.employer`
- `customer.kyc.beneficial_owner`, `customer.kyc.beneficial_owner_percentage`
- `customer.kyc.pep_warning`, `customer.kyc.document_upload`

**Scénarios Playwright** :
- Test multi-step form navigation (Next/Back buttons)
- Test form validation on each step
- Test beneficial owner modal (add/remove)
- Test document upload with file preview
- Test form submission and success message

**WCAG Accessibility** :
- Stepper component with ARIA live region for progress
- Keyboard navigation between form fields (Tab order)
- Clear error messages with ARIA alert
- Skip link to main form content

---

### STORY-F-Accounts | Account Dashboard & Operations
**Type** : Feature | **Taille** : M (4h) | **Sprint** : 3-4
**Bounded Context** : Account (BC2) + Frontend

**User Story** :
> En tant que client, je veux voir mes comptes et effectuer des opérations courantes, afin de gérer mes finances.

**Scénarios BDD** :
```gherkin
Feature: Account Management
  Scenario: View all accounts
    Given I am logged in
    When I go to Accounts page
    Then I see list of all my accounts
    And each account shows type (courant, épargne, DAT)
    And each account shows balance and currency

  Scenario: View account details
    Given I have multiple accounts
    When I click on account "01-234-0001234-56"
    Then account details page opens
    And I see balance, movements, interest rate
    And I see recent transactions

  Scenario: Transfer between accounts
    Given I have two accounts
    When I click "Transfer"
    And I select source account
    And I select destination account
    And I enter amount "500 TND"
    And I confirm transfer
    Then transfer is executed
    And balance is updated
    And confirmation email is sent
```

**Fichiers (Svelte)** :
- `frontend/src/pages/accounts/index.astro` — Accounts list page
- `frontend/src/pages/accounts/[id].astro` — Account detail page
- `frontend/src/components/account/AccountCard.svelte` — Card display
- `frontend/src/components/account/AccountDetails.svelte` — Full details
- `frontend/src/components/account/TransferModal.svelte` — Transfer form
- `frontend/src/components/account/MovementsList.svelte` — Transaction history

**Clés i18n** :
- `account.my_accounts`, `account.account_type`, `account.balance`, `account.currency`
- `account.type.current`, `account.type.savings`, `account.type.term_deposit`
- `account.transfer`, `account.source_account`, `account.destination_account`
- `account.movements`, `account.date`, `account.amount`, `account.description`

**Scénarios Playwright** :
- Test accounts list renders with correct data
- Test click on account opens detail view
- Test transfer form validation and submission
- Test movement list pagination and filtering
- Test balance updates after transaction

**WCAG Accessibility** :
- Table with proper headers and scope
- Account cards with semantic HTML (article + headings)
- Transfer modal with proper focus management
- Currency formatter accessible (aria-label for abbreviations)

---

### STORY-F-Dashboards | Risk & Prudential Dashboards
**Type** : Feature | **Taille** : L (6h) | **Sprint** : 5-6
**Bounded Context** : Prudential (BC6) + Reporting (BC8) + Frontend

**User Story** :
> En tant que directeur des risques, je veux un tableau de bord temps réel des ratios prudentiels, afin de monitorer la conformité BCT.

**Scénarios BDD** :
```gherkin
Feature: Risk Dashboard
  Scenario: View key metrics
    Given dashboard page is open
    When I see solvency ratio
    Then ratio is 12.5% (target: ≥10%)
    And color is green (compliant)

  Scenario: View ratio trend
    Given dashboard is loaded
    When I select date range (last 6 months)
    Then line chart shows solvency ratio trend
    And Tier 1 ratio trend
    And C/D ratio trend

  Scenario: Alert on ratio breach
    Given solvency ratio drops below 10%
    When dashboard refreshes
    Then alert banner appears
    And ratio is highlighted in red
    And email alert sent to risk team
```

**Fichiers (Svelte)** :
- `frontend/src/pages/dashboards/risk.astro` — Risk dashboard page
- `frontend/src/components/dashboard/MetricCard.svelte` — Ratio card (solvency, Tier1, C/D, concentration)
- `frontend/src/components/dashboard/TrendChart.svelte` — Line chart (Chart.js or Svelte Charts)
- `frontend/src/components/dashboard/AlertBanner.svelte` — Compliance alerts
- `frontend/src/components/dashboard/RatioGauge.svelte` — Gauge visualization

**Clés i18n** :
- `dashboard.solvency_ratio`, `dashboard.tier1_ratio`, `dashboard.cd_ratio`, `dashboard.concentration_ratio`
- `dashboard.target`, `dashboard.current`, `dashboard.compliant`, `dashboard.warning`
- `dashboard.alert.below_minimum`, `dashboard.alert.above_maximum`

**Scénarios Playwright** :
- Test metric cards display correct values
- Test chart renders with real data
- Test date range picker updates chart
- Test alert banner appears on ratio breach
- Test responsive design (mobile, tablet, desktop)

**WCAG Accessibility** :
- Charts with data table alternative
- ARIA descriptions for gauge values
- Focus visible on interactive elements
- Alert banner with role="alert" and aria-live

---

### STORY-F-Audit | Audit Trail Viewer
**Type** : Feature | **Taille** : M (4h) | **Sprint** : 5-6
**Bounded Context** : Governance (BC11) + Frontend

**User Story** :
> En tant qu'inspecteur BCT, je veux consulter la piste d'audit complète, afin de vérifier toutes les opérations.

**Scénarios BDD** :
```gherkin
Feature: Audit Trail
  Scenario: View audit log
    Given audit trail page is open
    When I see list of all operations
    Then each entry shows: who, when, what, account, amount
    And entries are in reverse chronological order

  Scenario: Filter audit log
    Given audit log is displayed
    When I filter by user "Ahmed Ben Khalifa"
    And I filter by date range
    And I filter by operation type "Transfer"
    Then only matching entries are shown

  Scenario: Export audit report
    Given filtered audit log is ready
    When I click "Export as PDF"
    Then PDF is generated with all filters applied
    And PDF is cryptographically signed
```

**Fichiers (Svelte)** :
- `frontend/src/pages/audit/log.astro` — Audit log page
- `frontend/src/components/audit/AuditTable.svelte` — Table with filters
- `frontend/src/components/audit/AuditFilter.svelte` — Filter form
- `frontend/src/components/audit/ExportButton.svelte` — PDF export

**Clés i18n** :
- `audit.who`, `audit.when`, `audit.what`, `audit.account`, `audit.amount`
- `audit.filter_by_user`, `audit.filter_by_date`, `audit.filter_by_operation`
- `audit.export_pdf`, `audit.export_csv`

**Scénarios Playwright** :
- Test audit table renders data
- Test filter form updates table
- Test export functionality
- Test date picker for date range
- Test sorting by column

**WCAG Accessibility** :
- Table with proper headers (scope="col")
- Filter form with labels for each input
- Export button with tooltip
- Sortable columns with ARIA attributes

---

### STORY-F-RTL | RTL Support for Arabic UI
**Type** : Feature | **Taille** : M (4h) | **Sprint** : 5-6
**Bounded Context** : Frontend (i18n)

**User Story** :
> En tant qu'utilisateur arabophone, je veux une interface RTL fluide en arabe tunisien, afin d'utiliser le système dans ma langue.

**Scénarios BDD** :
```gherkin
Feature: RTL Arabic Support
  Scenario: Switch to Arabic
    Given UI is in French
    When I click language selector
    And I select "العربية"
    Then entire UI flips RTL
    And all text is in Arabic
    And date format is adapted (Hijri option)

  Scenario: Form input in RTL
    Given login form in Arabic
    When I enter email (LTR input)
    Then email stays LTR
    And labels are RTL

  Scenario: Numbers and currency in Arabic
    Given account balance is shown
    When language is Arabic
    Then amount shows "١٫٥٠٠ د.ت" (Arabic numerals + currency)
```

**Fichiers (Svelte)** :
- `frontend/src/lib/i18n/ar.json` — Arabic translations
- `frontend/src/lib/rtl.ts` — RTL utility functions
- `frontend/src/components/LanguageSwitcher.svelte` — Language selector
- CSS adjustments in `frontend/src/styles/rtl.css` — RTL fixes (margin, padding, direction)

**Clés i18n** :
- All UI strings in `ar.json` with full Arabic translations
- Date/time formatting with optional Hijri calendar
- Currency formatting with Arabic numerals option

**Scénarios Playwright** :
- Test language switch to Arabic
- Test RTL layout (all elements aligned right)
- Test form inputs stay LTR while labels are RTL
- Test numbers display in Arabic numerals
- Test sidebar/navigation flips correctly

**WCAG Accessibility** :
- `lang="ar"` attribute on HTML
- `dir="rtl"` on body
- Arabic font with proper ligatures
- Focus indicators work in RTL

---

## Documentation Vivante (E2E Multi-Rôles)

**STORY-DOC-E2E-01** | Scénario : Ouverture de compte (chargé clientèle → client)
**STORY-DOC-E2E-02** | Scénario : Octroi de crédit (analyste → comité crédit)
**STORY-DOC-E2E-03** | Scénario : Déclaration de soupçon (compliance → CTAF)
**STORY-DOC-E2E-04** | Scénario : Gel des avoirs (sanctions → compliance)
**STORY-DOC-E2E-05** | Scénario : Audit BCT (inspecteur → rapport)
**STORY-DOC-E2E-06** | Scénario : Reporting mensuel (comptabilité → BCT)

---

## Stories i18n (AR RTL + FR + EN)

### STORY-I18N-01 | i18n Framework Setup (Astro + Svelte)
**Type** : Tech | **Taille** : M (3h)

**User Story** :
> En tant que développeur frontend, je veux un système i18n multilingue (AR/FR/EN), afin de supporter les 3 langues de BANKO.

**Structure des fichiers** :
```
frontend/src/lib/i18n/
├── i18n.ts                # Setup + middleware (language detection, persistence)
├── ar.json                # Arabic (RTL) translations
├── fr.json                # French translations
├── en.json                # English translations
├── types.ts               # Translation type definitions
└── locales.ts             # Locale configurations (RTL support, date format)
```

**Fichier `i18n.ts` (excerpt)** :
```typescript
export const defaultLocale = 'fr';
export const locales = {
  ar: { name: 'العربية', dir: 'rtl', dateFormat: 'DD/MM/YYYY' },
  fr: { name: 'Français', dir: 'ltr', dateFormat: 'DD/MM/YYYY' },
  en: { name: 'English', dir: 'ltr', dateFormat: 'MM/DD/YYYY' },
};

export function getLocale(request?: Request): string {
  // Detect from URL, cookie, or browser Accept-Language
}

export function t(key: string, locale: string, params?: Record<string, string>): string {
  // Nested key lookup: 'auth.login.title' → translations[locale]['auth']['login']['title']
}
```

**Clés de base identifiées** :
```json
{
  "app": {
    "name": "BANKO",
    "tagline": "Plateforme bancaire open source tunisienne"
  },
  "nav": {
    "home": "Accueil",
    "accounts": "Comptes",
    "transfers": "Virements",
    "profile": "Profil",
    "logout": "Déconnexion"
  },
  "common": {
    "save": "Enregistrer",
    "cancel": "Annuler",
    "delete": "Supprimer",
    "loading": "Chargement...",
    "error": "Erreur"
  }
}
```

---

### STORY-I18N-02 | Translation Keys — Identity (BC12)
**Type** : Content | **Taille** : S (2h)

**Clés pour Identity** :
```json
{
  "auth": {
    "login": {
      "title": "Connexion",
      "email": "Adresse e-mail",
      "password": "Mot de passe",
      "remember_me": "Se souvenir de moi",
      "forgot_password": "Mot de passe oublié?",
      "login_button": "Se connecter",
      "invalid_credentials": "Identifiants invalides",
      "account_locked": "Compte verrouillé après 5 tentatives"
    },
    "register": {
      "title": "Créer un compte",
      "full_name": "Nom complet",
      "email": "Adresse e-mail",
      "password": "Mot de passe",
      "confirm_password": "Confirmer le mot de passe",
      "accept_terms": "J'accepte les conditions d'utilisation",
      "create_account": "Créer un compte",
      "already_have_account": "Vous avez déjà un compte?"
    },
    "two_factor": {
      "setup_title": "Authentification à deux facteurs",
      "scan_qr": "Scannez le code QR avec votre application authenticateur",
      "enter_code": "Entrez le code 6 chiffres",
      "backup_codes": "Codes de secours (à conserver)",
      "activated": "2FA activée avec succès"
    }
  },
  "roles": {
    "admin": "Administrateur",
    "manager": "Gestionnaire",
    "agent": "Agent bancaire",
    "compliance": "Conformité",
    "auditor": "Auditeur"
  }
}
```

---

### STORY-I18N-03 | Translation Keys — Customer (BC1)
**Type** : Content | **Taille** : S (2h)

**Clés pour Customer** :
```json
{
  "customer": {
    "create": "Créer un client",
    "edit": "Modifier le client",
    "delete": "Supprimer le client",
    "kyc": {
      "full_name": "Nom complet",
      "date_of_birth": "Date de naissance",
      "gender": "Sexe",
      "male": "Homme",
      "female": "Femme",
      "cin": "Numéro CIN",
      "passport": "Numéro passeport",
      "profession": "Profession",
      "employer": "Employeur",
      "annual_income": "Revenu annuel",
      "beneficial_owner": "Bénéficiaire effectif",
      "beneficial_owner_percentage": "Pourcentage de propriété",
      "pep": "Personne Politiquement Exposée",
      "pep_position": "Fonction politique/publique",
      "pep_warning": "⚠️ Personne Politiquement Exposée (vérification additionnelle requise)",
      "document_upload": "Télécharger documents (pièce d'identité, justificatif de domicile)",
      "kyc_status": "Statut KYC",
      "kyc_approved": "KYC approuvée",
      "kyc_pending": "KYC en attente",
      "kyc_rejected": "KYC rejetée"
    },
    "risk": {
      "risk_score": "Score de risque",
      "low": "Faible",
      "medium": "Moyen",
      "high": "Élevé",
      "critical": "Critique"
    }
  }
}
```

---

### STORY-I18N-04 | Translation Keys — Account (BC2)
**Type** : Content | **Taille** : S (2h)

**Clés pour Account** :
```json
{
  "account": {
    "my_accounts": "Mes comptes",
    "account_number": "Numéro de compte",
    "rib": "RIB",
    "account_type": "Type de compte",
    "type": {
      "current": "Compte courant",
      "savings": "Compte épargne",
      "term_deposit": "Dépôt à terme",
      "overdraft": "Compte de dépassement"
    },
    "balance": "Solde",
    "available_balance": "Solde disponible",
    "currency": "Devise",
    "opening_date": "Date d'ouverture",
    "interest_rate": "Taux d'intérêt",
    "movements": "Mouvements",
    "recent_transactions": "Transactions récentes",
    "transfer": "Effectuer un virement",
    "source_account": "Compte source",
    "destination_account": "Compte destinataire",
    "amount": "Montant",
    "transfer_date": "Date du virement",
    "transfer_reference": "Référence du virement",
    "transfer_confirmed": "Virement confirmé",
    "transfer_failed": "Échec du virement"
  }
}
```

---

### STORY-I18N-05 | RTL CSS & Layout Fixes
**Type** : Tech | **Taille** : M (3h)

**Fichier `frontend/src/styles/rtl.css`** (exemple) :
```css
/* RTL Layout fixes */
[dir="rtl"] {
  text-align: right;
  direction: rtl;
}

[dir="rtl"] .sidebar {
  right: 0;
  left: auto;
  border-right: none;
  border-left: 1px solid #ccc;
}

[dir="rtl"] .content {
  margin-left: 0;
  margin-right: 250px;
}

[dir="rtl"] button,
[dir="rtl"] .btn {
  text-align: right;
}

[dir="rtl"] input,
[dir="rtl"] textarea {
  direction: ltr;  /* Email, numbers stay LTR */
  text-align: right;  /* But text-align right for Arabic */
}

[dir="rtl"] .icon-left {
  margin-left: auto;
  margin-right: 8px;
}

[dir="rtl"] .icon-right {
  margin-right: auto;
  margin-left: 8px;
}

/* Flexbox directional fixes */
[dir="rtl"] .flex-row {
  flex-direction: row-reverse;
}

/* Grid layout */
[dir="rtl"] .grid {
  direction: rtl;
}

/* Focus indicators */
[dir="rtl"] *:focus-visible {
  outline: 2px solid #007bff;
  outline-offset: 2px;
}
```

---

### STORY-I18N-06 | Arabic (Tunisian Dialect) Review
**Type** : Content | **Taille** : M (3h)

**Livrable** :
- Native Arabic speaker review of all `ar.json` translations
- Verify Tunisian dialect usage (vs. Modern Standard Arabic)
- Check abbreviations: "TND" → "د.ت" or "تونسي"
- Ensure financial terminology is correct
- Example terms:
  - "Crédit" → "قرض"
  - "Dépôt" → "وديعة"
  - "Virement" → "تحويل"
  - "Sanction" → "عقوبة"
  - "Gel des avoirs" → "تجميد الأموال"

---

### STORY-I18N-07 | Form Validation Messages (3 Languages)
**Type** : Content | **Taille** : M (3h)

**Fichier `frontend/src/lib/i18n/validation.json`** :
```json
{
  "validation": {
    "required": "Ce champ est obligatoire",
    "email": "Veuillez entrer une adresse e-mail valide",
    "password_min_length": "Le mot de passe doit avoir au moins 12 caractères",
    "password_uppercase": "Le mot de passe doit contenir une majuscule",
    "password_lowercase": "Le mot de passe doit contenir une minuscule",
    "password_number": "Le mot de passe doit contenir un chiffre",
    "password_special": "Le mot de passe doit contenir un caractère spécial (!@#$%)",
    "password_mismatch": "Les mots de passe ne correspondent pas",
    "cin_format": "Format CIN invalide",
    "rib_format": "Format RIB invalide",
    "iban_format": "Format IBAN invalide",
    "amount_positive": "Le montant doit être positif",
    "amount_max_decimal": "Maximum 3 décimales",
    "percentage_range": "Le pourcentage doit être entre 0 et 100"
  }
}
```

---

### STORY-I18N-08 | Date/Currency Formatting (TND, EUR, USD)
**Type** : Tech | **Taille** : M (3h)

**Fichier `frontend/src/lib/formatters.ts`** :
```typescript
export function formatCurrency(amount: number, currency: string = 'TND', locale: string = 'fr'): string {
  // TND → "1 500,00 د.ت" (French locale)
  // EUR → "€1,500.00" (English locale)
  // USD → "$1,500.00" (English locale)
  const formatter = new Intl.NumberFormat(locale, {
    style: 'currency',
    currency,
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
  return formatter.format(amount);
}

export function formatDate(date: Date, locale: string = 'fr', hijri: boolean = false): string {
  // FR: "15 mars 2025"
  // AR: "15 مارس 2025" or "30 شعبان 1446" (Hijri)
  // EN: "March 15, 2025"
  if (hijri && locale === 'ar') {
    // Use Hijri calendar for Arabic
    return hijriFormatter.format(date);
  }
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  }).format(date);
}

export function formatNumber(num: number, locale: string = 'fr'): string {
  // FR: "1 500,00"
  // AR: "١٫٥٠٠٫٠٠" (Arabic numerals)
  return new Intl.NumberFormat(locale, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(num);
}
```

---

---

## Stories d'Émergence (Réserve 20%)

**STORY-EMG-01** | Bug fixes + refactoring
**STORY-EMG-02** | Security patches (CVEs)
**STORY-EMG-03** | Performance tuning
**STORY-EMG-04** | Database optimization
**STORY-EMG-05** | Test coverage improvement
**STORY-EMG-06** | Documentation updates
**STORY-EMG-07** | Tooling improvements (CI/CD)
**STORY-EMG-08** | Community support + feedback

---

## Stories de Scaling (Nexus/SAFe)

**STORY-SCALE-01** | Multi-bank support (tenant isolation)
**STORY-SCALE-02** | Data replication (high availability)
**STORY-SCALE-03** | Load balancing (API, DB)
**STORY-SCALE-04** | API rate limiting + quota
**STORY-SCALE-05** | Caching layer (Redis)
**STORY-SCALE-06** | Message queue (async jobs)
**STORY-SCALE-07** | Search indexing (Elasticsearch)
**STORY-SCALE-08** | Microservices extraction (Nexus)

---

## Stories ITIL (Activated Pre-production)

**STORY-I01** | Incident management (Jira + Slack integration)
**STORY-I02** | Problem management (RCA process)
**STORY-I03** | Change management (CAB approval workflow)
**STORY-I04** | Release management (deployment pipeline)
**STORY-I05** | Configuration management database (CMDB)
**STORY-I06** | Service catalog
**STORY-I07** | SLA management + reporting
**STORY-I08** | Knowledge base (runbooks)
**STORY-I09** | Availability management
**STORY-I10** | Capacity planning
**STORY-I11** | Continuity management (BCP/DRP)
**STORY-I12** | Security management (ISO 27001)
**STORY-I13** | Event management + alerting

---

## Estimation Chiffrée

### Heures par Couche (Velocity Coefficients)

| Couche | Stories estimées | Coefficient | Heures |
|---|---|---|---|
| **Domain** | 15 L | 5h | **75h** |
| **Application** | 25 M | 3h | **75h** |
| **API Handlers** | 30 M | 3h | **90h** |
| **Infrastructure (Repos, DB)** | 20 M | 3h | **60h** |
| **Tests BDD** | 40 M | 2h | **80h** |
| **Tests E2E** | 10 L | 4h | **40h** |
| **Frontend (Svelte + Astro)** | 20 M + 10 L | 5h + 10h | **120h** |
| **IaC (Terraform + Ansible)** | 8 M + 5 L | 8h + 20h | **164h** |
| **i18n (AR/FR/EN)** | 8 M | 4h | **32h** |
| **Documentation vivante** | 6 L | 8h | **48h** |
| **CI/CD + Security** | 10 M | 4h | **40h** |
| **Subtotal** | | | **824h** |
| **+ 20% émergence** | | | **165h** |
| **+ 10% CI stabilisation** | | | **99h** |
| **TOTAL HEURES** | | | **~1 088h** |

### Budget Client (Scénario)

**Hypothèse** : Tarif freelance Tunisie = 30 TND/h (ou ~10 EUR/h)

| Item | Calcul | Montant |
|---|---|---|
| Développement | 1 088h × 30 TND | **32 640 TND** |
| Infrastructure (3 mois OVH) | 150 EUR/mois × 3 | **450 EUR** |
| Licences (GitLab, Sentry, etc.) | Forfait | **0 EUR** (open source alternatives) |
| Audit sécurité (pentest) | Devis externe | **500-1000 EUR** |
| **Total TND** | | **~33 500 TND** |
| **Total EUR** | | **~10 500 EUR** |

---

### Durée Calendaire

| Profil | Calcul | Durée |
|---|---|---|
| Solo-dev side-project (8h/sem) | 1 088 ÷ 8 | **136 semaines ≈ 31 mois** |
| Solo-dev time-plein (35h/sem) | 1 088 ÷ 35 | **31 semaines ≈ 7 mois** |
| Duo (2 × 20h/sem) | 1 088 ÷ 40 | **27 semaines ≈ 6 mois** |
| Équipe (3 × 30h/sem) | 1 088 ÷ 90 | **12 semaines ≈ 3 mois** |

---

### Comparaison Marché

| Solution | Coût licence/an | Customisation | Conformité Tunisie |
|---|---|---|---|
| **Temenos** | 500k-2M EUR | Moyen | Possible (lent) |
| **Finastra** | 300k-1.5M EUR | Faible | Possible (lent) |
| **BANKO (open source)** | **0 EUR** | **Total** | **Natif dès le départ** |
| **Différence BANKO** | Économie 300k+ EUR | Code maîtrisé | Circulaires BCT intégrées |

---

## Recap Général

- **Total stories** : ~95 (13 tech + 12 epics × 7 stories/epic)
- **Bounded Contexts** : 12 (tous couverts)
- **Scénarios BDD** : ~200 (20+ par BC)
- **Endpoints API** : ~160 (12-15 par BC)
- **Heures estimées** : 1 088h (+ 20% émergence)
- **Durée calendaire** : 7 mois (duo) à 31 mois (solo side-project)
- **Conformité légale** : 100% Circ. 2025-17 + Bâle III + INPDP + NCT
- **Disciplines** : SOLID + DDD + BDD + TDD + Hexagonal + YAGNI + DRY

---

## Prochaines Étapes

1. **Étape 5** : Validation croisée (Architecte + Scrum Master + Product Manager)
2. **Étape 6** : Roadmap capacitaire (pas de dates, jalons uniquement)
3. **Étape 7** : Sprint 0 kicks off (Setup du projet, infrastructure)
4. **Étape 8** : Sprint 1-2 dev (Identity + Customer + Account avec BDD)

**Fin du document.**
