## 📝 Description

<!-- Décrivez clairement les changements apportés par cette PR -->

## 🔗 Issue(s) Liée(s)

<!-- Référencez l'issue associée (ex: Closes #42, Fixes #123) -->

Closes #

## 🎯 Type de Changement

<!-- Cochez les cases appropriées avec [x] -->

- [ ] 🐛 Bug fix (changement non-breaking qui corrige un problème)
- [ ] ✨ New feature (changement non-breaking qui ajoute une fonctionnalité)
- [ ] 💥 Breaking change (correction ou fonctionnalité causant un changement incompatible)
- [ ] 📚 Documentation (mise à jour de la documentation uniquement)
- [ ] 🎨 Style (formatage, lint, sans changement de logique)
- [ ] ♻️ Refactoring (ni correction ni ajout de fonctionnalité)
- [ ] ⚡ Performance (amélioration des performances)
- [ ] ✅ Tests (ajout ou correction de tests)
- [ ] 🔒 Sécurité (amélioration sécurité ou correction vulnérabilité)

## 🏗️ Architecture Hexagonale

<!-- Indiquez les couches impactées -->

### Backend (Rust)

- [ ] **Domain Layer** (`backend/src/domain/`)
  - [ ] Entities modifiées/ajoutées
  - [ ] Services de domaine modifiés/ajoutés
  - [ ] Validation métier ajoutée

- [ ] **Application Layer** (`backend/src/application/`)
  - [ ] Use Cases modifiés/ajoutés
  - [ ] Ports (traits) modifiés/ajoutés
  - [ ] DTOs modifiés/ajoutés

- [ ] **Infrastructure Layer** (`backend/src/infrastructure/`)
  - [ ] Repositories modifiés/ajoutés
  - [ ] Handlers HTTP modifiés/ajoutés
  - [ ] Routes modifiées/ajoutées
  - [ ] Migrations base de données ajoutées
  - [ ] Sécurité (HSM, chiffrement) modifiée

### Frontend (Astro + Svelte)

- [ ] **Components** (`frontend/src/components/`)
- [ ] **Pages** (`frontend/src/pages/`)
- [ ] **Stores** (`frontend/src/stores/`)
- [ ] **Types** (`frontend/src/lib/types.ts`)
- [ ] **API Client** (`frontend/src/lib/api.ts`)

## ✅ Checklist Qualité

### Tests

- [ ] Tests unitaires ajoutés/mis à jour
- [ ] Tests d'intégration ajoutés/mis à jour
- [ ] Tests BDD ajoutés/mis à jour
- [ ] Tests de sécurité effectués (si applicable)
- [ ] Tous les tests passent localement (`make test`)

### Code Quality

- [ ] Code suit les conventions du projet (hexagonal architecture)
- [ ] Code formaté (`make format`)
- [ ] Lint réussi (`make lint`)
- [ ] Pas de code commenté superflu
- [ ] Noms descriptifs et clairs

### Documentation

- [ ] `CHANGELOG.md` mis à jour
- [ ] Documentation API mise à jour (si applicable)
- [ ] `README.md` mis à jour (si nécessaire)
- [ ] Docstrings ajoutées (Rust, TypeScript)

### Base de Données

- [ ] Migration SQLx créée (`backend/migrations/`)
- [ ] Migration testée (up + down)
- [ ] Backward compatible OU plan de migration fourni

### Sécurité

- [ ] Pas de credentials/secrets hardcodés
- [ ] Validation des entrées utilisateur
- [ ] Authorization checks en place
- [ ] Données sensibles protégées
- [ ] Audit logging ajouté (si applicable)

## 🧪 Comment Tester

<!-- Décrivez les étapes pour tester cette PR -->

1. Checkout cette branche: `git checkout <branch-name>`
2. Install dependencies: `make setup`
3. Run migrations: `make migrate`
4. Start services: `make dev`
5. Testez: ...

## 📸 Screenshots/Vidéos

<!-- Si changements UI, ajoutez des captures d'écran -->

## 🔄 Breaking Changes

<!-- Si breaking change, décrivez l'impact et le plan de migration -->

**Impact:**
- Quoi: ...
- Qui est affecté: ...

## 📋 Checklist Finale

- [ ] J'ai testé cette PR localement
- [ ] J'ai lu et suivi le [CONTRIBUTING.md](../CONTRIBUTING.md)
- [ ] Cette PR est prête pour review
- [ ] Commit(s) signés avec DCO (`git commit -s`)

---

**Merci pour votre contribution à BANKO! 🔒**
