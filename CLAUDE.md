# CLAUDE.md

Ce fichier fournit des conseils à Claude Code (claude.ai/code) pour travailler sur ce dépôt.

## 🏦 Vue d'ensemble de BANKO

BANKO est une plateforme bancaire open source conçue selon les principes d'architecture hexagonale et Domain-Driven Design (DDD). Le système emphase la performance, la sécurité, la conformité réglementaire et la transparence.

**Stack**: Rust + Actix-web (backend), Astro + Svelte (frontend), PostgreSQL 16 + Traefik (proxy) + MinIO (stockage)

**Architecture**: 12 bounded contexts (Customer, Account, Credit, AML, Sanctions, Prudential, Accounting, Reporting, Payment, ForeignExchange, Governance, Identity)

**API Base**: http://localhost:8080/api/v1

## Architecture: Hexagonale (Ports & Adaptateurs)

Le backend suit une architecture stricte avec 3 couches :

```
Domain (Cœur métier)
  ↑ définit les interfaces
Application (Use Cases + Ports)
  ↑ implémente les ports
Infrastructure (Adaptateurs: Web, Base de données)
```

### Règles des Couches (CRITIQUE)

1. **Domain Layer** (`backend/src/domain/`):
   - Logique métier pure, AUCUNE dépendance externe
   - Contient `entities/` (agrégats avec validation) et `services/` (services métier)
   - Les entités appliquent les règles métier dans les constructeurs
   - Tests du domaine dans les blocs `#[cfg(test)]` intégrés

2. **Application Layer** (`backend/src/application/`):
   - `ports/`: Définitions de traits (interfaces) comme `CustomerRepository`
   - `use_cases/`: Logique d'orchestration (ex: `OpenAccountUseCase`)
   - `dto/`: Data Transfer Objects pour les contrats API
   - Dépend UNIQUEMENT de la couche Domain

3. **Infrastructure Layer** (`backend/src/infrastructure/`):
   - `database/repositories/`: Implémentations PostgreSQL des ports
   - `web/handlers/`: Handlers HTTP Actix-web
   - `web/routes.rs`: Configuration des routes API
   - Dépend de la couche Application

## 12 Bounded Contexts

1. **Customer**: Gestion des clients (onboarding, KYC, profil)
2. **Account**: Gestion des comptes (ouverture, solde, type de compte)
3. **Credit**: Octroi de crédit et gestion des prêts
4. **AML**: Anti-blanchiment d'argent (alertes, seuils)
5. **Sanctions**: Contrôle des sanctions internationales
6. **Prudential**: Exigences prudentielles et capital réglementaire
7. **Accounting**: Comptabilité générale (journaux, écritures)
8. **Reporting**: Rapports réglementaires et statistiques
9. **Payment**: Virements et paiements (SEPA, SWIFT)
10. **ForeignExchange**: Changes et taux de change
11. **Governance**: Gouvernance (rôles, permissions, audit)
12. **Identity**: Gestion des identités (authentification, biométrie)

## Commandes Principales (Makefile)

### Développement

```bash
make setup          # Configuration initiale (setup complet)
make dev            # Démarrer l'environnement dev (Traefik + backend + frontend)
make down           # Arrêter tous les services
make logs           # Afficher les logs
make logs SERVICE=backend  # Logs du backend seulement
```

### Tests

```bash
make test           # Tous les tests (unit + BDD + E2E)
make test-unit      # Tests unitaires backend
make test-bdd       # Tests BDD/Cucumber backend
make test-e2e       # Tests E2E Playwright frontend
make coverage       # Rapport de couverture
```

### Qualité du Code

```bash
make lint           # Vérifier le code (clippy + prettier)
make format         # Formater le code (rustfmt + prettier)
make audit          # Audit sécurité (cargo audit + npm audit)
make ci             # Vérifications CI locales complètes
```

### Infrastructure

```bash
make migrate        # Exécuter les migrations DB
make reset-db       # Reset DB (SUPPRIME TOUTES LES DONNÉES)
make seed           # Seed avec données de test
```

### Documentation

```bash
make docs           # Générer docs Rust (cargo doc)
make docs-sphinx    # Build docs Sphinx
make docs-serve     # Servir docs Sphinx avec live reload
```

## Stack Technique

### Backend (Rust)

- **Web**: Actix-web 4.x pour l'API REST
- **Database**: PostgreSQL 16 + SQLx pour les requêtes typées
- **Async Runtime**: Tokio
- **Serialization**: serde + serde_json
- **Error Handling**: Custom error types avec contexts

### Frontend (Astro + Svelte)

- **Framework**: Astro 4.x (SSG + islands)
- **Components**: Svelte pour les composants interactifs
- **Styling**: Tailwind CSS
- **Type Safety**: TypeScript
- **API Client**: Fetch API + custom client

### Infrastructure

- **Database**: PostgreSQL 16 (Alpine)
- **Reverse Proxy**: Traefik (routing HTTP)
- **Object Storage**: MinIO (S3-compatible)
- **Container Orchestration**: Docker Compose (dev), Kubernetes (prod)

## Documentation Légale et Normative

Pour les exigences légales et réglementaires en matière bancaire:

- Voir: `docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md`
- Documentation BMAD: `docs/bmad/`

## Workflow de Contribution

### 1. Préparer votre environnement

```bash
git clone <repo>
cd BANKO
make setup          # Installe dépendances, crée DB, Git hooks
```

### 2. Créer une branche

```bash
git checkout main
git pull origin main
git checkout -b feature/ma-fonctionnalite  # ou fix/, docs/, etc.
```

### 3. Développer (TDD)

```bash
make dev            # Lancer services
# Écrire tests en premier (tests unitaires → tests BDD)
# Implémenter la fonctionnalité
make test           # Vérifier que tout passe
```

### 4. Commit avec DCO

```bash
git commit -s -m "feat: description"  # Flag -s pour DCO
```

### 5. Pre-commit et Push

```bash
make pre-commit     # Format + lint
git push origin feature/ma-fonctionnalite
```

### 6. Pull Request

Référencez l'issue associée: `Closes #123`

Consultez:
- `.claude/guides/feature-workflow.md` pour le déroulé complet
- `CONTRIBUTING.md` pour les détails

## Sécurité

### Meilleures Pratiques

- **Validation des entrées**: Appliquer au Domain layer (constructeurs d'entités)
- **Pas de secrets hardcodés**: Utiliser variables d'environnement
- **SQL Injection**: Utiliser sqlx avec requêtes typées (jamais de format strings)
- **Authentification**: JWT avec refresh tokens
- **Autorisation**: Vérifier permissions à tous les niveaux
- **GDPR**: Chiffrer données sensibles, appliquer retention policies

### Audit Sécurité

```bash
make audit          # Vérifier dépendances vulnérables
```

Voir `SECURITY.md` pour la politique complète.

## Performance

- **Target P99**: < 5ms latency pour API calls
- **Caching**: Utiliser Redis pour sessions/caches fréquents
- **Database**: Indexes appropriés sur colonnes filtrées/triées
- **Monitoring**: Prometheus metrics sur `/metrics`

## Structure des Répertoires

```
BANKO/
├── backend/                  # Code Rust
│   ├── src/
│   │   ├── domain/          # Logique métier (entities, services)
│   │   ├── application/     # Use cases, DTOs, ports
│   │   ├── infrastructure/  # Repositories, HTTP handlers, config
│   │   └── main.rs
│   ├── migrations/          # SQLx migrations
│   ├── tests/               # Tests BDD/E2E
│   └── Cargo.toml
├── frontend/                 # Code Astro + Svelte
│   ├── src/
│   │   ├── pages/           # Routes Astro
│   │   ├── components/      # Composants Svelte
│   │   ├── lib/             # Utils, clients API
│   │   └── layouts/         # Layouts Astro
│   └── package.json
├── docs/
│   ├── legal/               # Documentation légale/normative
│   ├── bmad/                # Documentation BMAD
│   ├── api/                 # Docs API (OpenAPI)
│   └── guides/              # Guides utilisateur
├── .github/
│   ├── workflows/           # CI/CD GitHub Actions
│   ├── ISSUE_TEMPLATE/      # Templates issues
│   └── pull_request_template.md
├── .claude/
│   ├── guides/              # Guides Claude Code
│   └── settings.json
├── docker-compose.yml       # Stack dev
├── Makefile                 # Commands
└── README.md
```

## URLs de Développement

| Service | URL |
|---------|-----|
| Frontend | http://localhost |
| API REST | http://localhost/api/v1 |
| Traefik Dashboard | http://localhost:8081 |
| PostgreSQL | localhost:5432 |
| MinIO S3 API | localhost:9000 |
| MinIO Console | localhost:9001 |

## Déboguer

```bash
# Logs du backend
make logs SERVICE=backend

# Shell PostgreSQL
make shell-postgres

# Shell dans container backend
make shell-backend

# Vérifier les migrations
cd backend && sqlx migrate list
```

## Ressources Utiles

- **Architecture**: Voir `CLAUDE.md` (ce fichier)
- **Contribution**: `CONTRIBUTING.md`
- **Sécurité**: `SECURITY.md`
- **Gouvernance**: `GOVERNANCE.md`
- **Code of Conduct**: `CODE_OF_CONDUCT.md`
- **Guides Claude**: `.claude/guides/`

---

**Bienvenue dans BANKO! 🏦**
