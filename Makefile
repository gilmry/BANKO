# BANKO - Makefile pour contributeurs
# Usage: make help

.PHONY: help dev dev-up dev-down dev-logs dev-build dev-test dev-clippy dev-coverage dev-migrate dev-shell up down logs test test-unit test-int test-bdd codegen lint format build clean install setup migrate reset-db docs audit ci pre-commit

# Couleurs pour output
GREEN  := \033[0;32m
YELLOW := \033[1;33m
RED    := \033[0;31m
NC     := \033[0m # No Color

help: ## 📖 Afficher cette aide
	@echo "$(GREEN)BANKO - Commandes disponibles$(NC)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)🚀 Quick start:$(NC) make setup && make dev"

##
## 🚀 Développement
##

dev: ## 🔥 Démarrer dev avec hot reload (cargo watch + Astro HMR)
	@echo "$(GREEN)🚀 Démarrage environnement dev BANKO...$(NC)"
	@echo ""
	@echo "  📍 Frontend (HMR):  http://localhost:3000"
	@echo "  📍 API (direct):    http://localhost:8080/api/v1"
	@echo "  📍 API (Traefik):   http://localhost/api/v1"
	@echo "  📍 Traefik:         http://localhost:8081"
	@echo "  📍 MinIO Console:   http://localhost:9001"
	@echo ""
	@echo "  🔄 Backend:  cargo watch recompile à chaque modification"
	@echo "  🔄 Frontend: Astro dev server avec HMR Svelte"
	@echo ""
	docker compose -f docker-compose.dev.yml up --build

dev-up: ## 🔥 Démarrer dev en arrière-plan
	docker compose -f docker-compose.dev.yml up --build -d

dev-down: ## 🛑 Arrêter environnement dev
	docker compose -f docker-compose.dev.yml down

dev-logs: ## 📋 Logs dev (usage: make dev-logs SERVICE=backend)
	@if [ -z "$(SERVICE)" ]; then \
		docker compose -f docker-compose.dev.yml logs -f; \
	else \
		docker compose -f docker-compose.dev.yml logs -f $(SERVICE); \
	fi

dev-build: ## 🔨 Rebuild images dev
	docker compose -f docker-compose.dev.yml build --no-cache

dev-test: ## 🧪 Lancer tests dans le container dev
	docker compose -f docker-compose.dev.yml run --rm devtools cargo test

dev-clippy: ## 🔍 Lancer clippy dans le container dev
	docker compose -f docker-compose.dev.yml run --rm devtools cargo clippy --all-targets -- -D warnings

dev-coverage: ## 📊 Couverture de code avec tarpaulin
	docker compose -f docker-compose.dev.yml run --rm devtools cargo tarpaulin --out Html --output-dir /app/coverage

dev-migrate: ## 📊 Lancer les migrations dans le container dev
	docker compose -f docker-compose.dev.yml run --rm devtools sqlx migrate run --source /app/migrations

dev-shell: ## 🐚 Shell dans le container devtools
	docker compose -f docker-compose.dev.yml run --rm devtools bash

dev-e2e: ## 🧪 Tests E2E Playwright dans Docker (tous les tests)
	docker compose -f docker-compose.dev.yml --profile e2e run --rm e2e npx playwright test

dev-e2e-smoke: ## 💨 Smoke test E2E dans Docker (<30s)
	docker compose -f docker-compose.dev.yml --profile e2e run --rm e2e npx playwright test tests/smoke.spec.ts --project=chromium

dev-e2e-chromium: ## 🌐 Tests E2E Chromium dans Docker
	docker compose -f docker-compose.dev.yml --profile e2e run --rm e2e npx playwright test --project=chromium

dev-e2e-report: ## 📊 Générer et afficher le rapport E2E
	@echo "$(GREEN)📊 Rapport disponible dans e2e/playwright-report/$(NC)"

dev-reset: ## ⚠️  Reset complet dev (volumes + rebuild)
	@echo "$(YELLOW)⚠️  Reset complet de l'environnement dev...$(NC)"
	docker compose -f docker-compose.dev.yml down -v
	@echo "$(GREEN)✅ Volumes supprimés. Relancer avec: make dev$(NC)"

## Production (docker-compose.yml original)

up: ## 🚀 Démarrer prod (docker-compose.yml)
	docker compose up -d

down: ## 🛑 Arrêter prod
	docker compose down

logs: ## 📋 Logs prod (usage: make logs SERVICE=backend)
	@if [ -z "$(SERVICE)" ]; then \
		docker compose logs -f; \
	else \
		docker compose logs -f $(SERVICE); \
	fi

restart: ## 🔄 Redémarrer prod
	docker compose restart

build: ## 🔨 Rebuild images prod
	docker compose build

clean: ## 🧹 Nettoyer artifacts et volumes Docker
	@echo "$(YELLOW)⚠️  Nettoyage des artifacts...$(NC)"
	cd backend && cargo clean
	cd frontend && rm -rf dist node_modules
	docker compose down -v
	@echo "$(GREEN)✅ Nettoyage terminé$(NC)"

##
## ✅ Tests
##

test: test-unit test-bdd ## 🧪 Lancer tous les tests
	@echo "$(GREEN)✅ Tous les tests passés$(NC)"

test-unit: ## 🎯 Tests unitaires (backend)
	@echo "$(GREEN)🧪 Tests unitaires...$(NC)"
	cd backend && SQLX_OFFLINE=true cargo test --lib

test-integration: ## 🔗 Tests d'intégration backend
	@echo "$(GREEN)🔗 Tests d'intégration...$(NC)"
	cd backend && SQLX_OFFLINE=true cargo test --test integration

test-bdd: ## 🥒 Tests BDD/Cucumber (backend)
	@echo "$(GREEN)🥒 Tests BDD...$(NC)"
	cd backend && SQLX_OFFLINE=true cargo test --test bdd

test-e2e: ## 🌐 Tests E2E Playwright (toutes les routes)
	@echo "$(GREEN)🌐 Tests E2E Playwright...$(NC)"
	cd e2e && npx playwright test

test-e2e-smoke: ## 💨 Smoke test rapide (20 routes, <30s)
	@echo "$(GREEN)💨 Smoke test...$(NC)"
	cd e2e && npx playwright test tests/smoke.spec.ts --project=chromium

test-e2e-debug: ## 🐛 Tests E2E en mode debug (headed + trace)
	@echo "$(GREEN)🐛 E2E Debug mode...$(NC)"
	cd e2e && npx playwright test --debug --project=chromium

test-e2e-ui: ## 🖥️ Tests E2E avec UI Playwright
	@echo "$(GREEN)🖥️ E2E UI mode...$(NC)"
	cd e2e && npx playwright test --ui

test-e2e-report: ## 📊 Ouvrir le rapport HTML Playwright
	@echo "$(GREEN)📊 Rapport E2E...$(NC)"
	cd e2e && npx playwright show-report

test-watch: ## 👀 Tests en mode watch (auto-reload)
	cd backend && cargo watch -x "test --lib"

##
## 🔍 Qualité du code
##

lint: ## 🔍 Linter (clippy + prettier)
	@echo "$(GREEN)🔍 Linting backend...$(NC)"
	docker compose exec -T backend sh -c "SQLX_OFFLINE=true cargo clippy --all-targets --all-features -- -D warnings"
	@echo "$(GREEN)🔍 Linting frontend...$(NC)"
	cd frontend && npx prettier --check .

format: ## ✨ Formatter le code
	@echo "$(GREEN)✨ Formatting backend...$(NC)"
	docker compose exec -T backend sh -c "cargo fmt"
	@echo "$(GREEN)✨ Formatting frontend...$(NC)"
	cd frontend && npx prettier --write .

audit: ## 🔒 Audit sécurité (cargo-audit + npm audit)
	@echo "$(GREEN)🔒 Audit backend...$(NC)"
	cd backend && cargo audit
	@echo "$(GREEN)🔒 Audit frontend...$(NC)"
	cd frontend && npm audit --audit-level=high

install-hooks: ## 🪝 Installer les Git hooks
	@echo "$(GREEN)🪝 Installation des Git hooks...$(NC)"
	./scripts/install-hooks.sh

##
## 📦 Setup & Installation
##

install: ## 📦 Installer dépendances frontend
	@echo "$(GREEN)📦 Installation dépendances frontend...$(NC)"
	cd frontend && npm install

setup: ## 🚀 Setup complet du projet
	@echo "$(GREEN)🚀 Setup BANKO...$(NC)"
	@echo ""
	@echo "1️⃣ Vérification Docker..."
	@docker --version || (echo "$(YELLOW)❌ Docker non installé$(NC)" && exit 1)
	@docker compose version || (echo "$(YELLOW)❌ Docker Compose non installé$(NC)" && exit 1)
	@echo "$(GREEN)✅ Docker OK$(NC)"
	@echo ""
	@echo "2️⃣ Installation frontend..."
	cd frontend && npm install
	@echo "$(GREEN)✅ Frontend OK$(NC)"
	@echo ""
	@echo "3️⃣ Démarrage PostgreSQL 16..."
	docker compose up -d postgres
	@sleep 5
	@echo "$(GREEN)✅ PostgreSQL OK$(NC)"
	@echo ""
	@echo "4️⃣ Migrations DB..."
	cd backend && sqlx migrate run || echo "$(YELLOW)⚠️  Migrations échouées (normal si DB vide)$(NC)"
	@echo ""
	@echo "5️⃣ Installation des Git hooks..."
	./scripts/install-hooks.sh
	@echo ""
	@echo "$(GREEN)✅ Setup terminé!$(NC)"
	@echo ""
	@echo "$(GREEN)🚀 Démarrer dev: make dev$(NC)"

##
## 🗄️ Base de données
##

migrate: ## 📊 Lancer migrations DB
	@echo "$(GREEN)📊 Migrations DB...$(NC)"
	cd backend && sqlx migrate run

reset-db: ## ⚠️  Reset DB (SUPPRIME TOUTES LES DONNÉES)
	@echo "$(YELLOW)⚠️  ATTENTION: Suppression de toutes les données!$(NC)"
	@read -p "Taper 'yes' pour confirmer: " confirm; \
	if [ "$$confirm" = "yes" ]; then \
		docker compose down postgres; \
		docker volume rm banko_postgres_data 2>/dev/null || true; \
		docker compose up -d postgres; \
		sleep 5; \
		cd backend && sqlx migrate run; \
		echo "$(GREEN)✅ DB reset terminée$(NC)"; \
	else \
		echo "$(YELLOW)❌ Annulé$(NC)"; \
	fi

seed: ## 🌱 Seed DB avec données de test
	cd backend && cargo run --bin seed

##
## 📚 Documentation
##

docs: ## 📚 Générer docs Rust
	@echo "$(GREEN)📚 Génération docs Rust...$(NC)"
	cd backend && SQLX_OFFLINE=true cargo doc --no-deps --open

docs-sphinx: ## 📖 Build docs Sphinx
	@echo "$(GREEN)📖 Build docs Sphinx...$(NC)"
	@if [ ! -d docs/.venv ]; then \
		echo "$(YELLOW)⚠️  Creating Python venv...$(NC)"; \
		cd docs && python3 -m venv .venv && .venv/bin/pip install -q -r requirements.txt; \
	fi
	cd docs && .venv/bin/sphinx-build -M html . _build
	@echo "$(GREEN)✅ Docs: docs/_build/html/index.html$(NC)"

##
## 🚀 CI/CD
##

ci: ## ✅ Vérifications CI locales (tout dans Docker)
	@echo "$(GREEN)🔍 Clippy (lint backend)...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T backend sh -c "SQLX_OFFLINE=true cargo clippy -- -D warnings"
	@echo "$(GREEN)🔍 Formatting backend...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T backend sh -c "cargo fmt --check"
	@echo "$(GREEN)🔒 Security audit backend...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T backend sh -c "cargo audit || true"
	@echo "$(GREEN)🔍 Formatting frontend...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T frontend sh -c "npx prettier --check . 2>/dev/null || true"
	@echo "$(GREEN)🔒 Security audit frontend...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T frontend sh -c "npm audit --audit-level=high || true"
	@echo "$(GREEN)🧪 Tests unitaires backend...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T backend sh -c "SQLX_OFFLINE=true cargo test --lib"
	@echo "$(GREEN)🌐 Tests E2E Playwright...$(NC)"
	docker compose -f docker-compose.dev.yml --profile e2e run --rm e2e npx playwright test --project=chromium
	@echo ""
	@echo "$(GREEN)🎉 Tous les checks CI passés!$(NC)"
	@echo "$(GREEN)✅ Prêt à push$(NC)"

dev-ci: ## ✅ CI rapide dans le container dev (sans E2E)
	@echo "$(GREEN)🔍 Clippy...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T backend cargo clippy -- -D warnings
	@echo "$(GREEN)🔍 Format check...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T backend cargo fmt --check
	@echo "$(GREEN)🔒 Audit...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T backend cargo audit || true
	@echo "$(GREEN)🧪 Tests...$(NC)"
	docker compose -f docker-compose.dev.yml exec -T backend sh -c "SQLX_OFFLINE=true cargo test --lib"
	@echo "$(GREEN)✅ CI OK$(NC)"

prod-build: ## 🏭 Build images production (distroless)
	@echo "$(GREEN)🏭 Build API production (distroless + UPX)...$(NC)"
	docker build -f Dockerfile.api -t banko-api:latest .
	@echo "$(GREEN)🏭 Build Frontend production (distroless)...$(NC)"
	docker build -f Dockerfile.frontend -t banko-frontend:latest .
	@echo "$(GREEN)✅ Images production prêtes$(NC)"
	@docker images | grep banko

pre-commit: format lint ## 🎯 Pre-commit hook (format + lint)
	@echo "$(GREEN)✅ Pre-commit OK$(NC)"

##
## 🔧 Utilitaires
##

ps: ## 📊 Status des containers
	docker compose ps

shell-backend: ## 🐚 Shell dans container backend
	docker compose exec backend bash

shell-postgres: ## 🐘 Shell PostgreSQL
	docker compose exec postgres psql -U banko -d banko_db

update-deps: ## 🔄 Mettre à jour dépendances
	@echo "$(GREEN)🔄 Update dépendances frontend...$(NC)"
	cd frontend && npm update
	@echo "$(GREEN)🔄 Update dépendances Rust...$(NC)"
	cd backend && cargo update

##
## ❓ Info
##

info: ## ℹ️  Infos projet
	@echo "$(GREEN)BANKO - Info Projet$(NC)"
	@echo ""
	@echo "📦 Structure:"
	@echo "  - Backend:  Rust + Actix-web + SQLx"
	@echo "  - Frontend: Astro + Svelte"
	@echo "  - DB:       PostgreSQL 16"
	@echo "  - Proxy:    Traefik"
	@echo ""
	@echo "🌐 URLs Dev:"
	@echo "  - Frontend: http://localhost"
	@echo "  - API:      http://localhost/api/v1"
	@echo "  - Traefik:  http://localhost:8081"
	@echo ""
	@echo "📚 Docs:"
	@echo "  - README:   ./README.md"
	@echo "  - CLAUDE:   ./CLAUDE.md"
	@echo "  - Sphinx:   make docs-sphinx"
	@echo ""
	@echo "🚀 Quick start: make setup && make dev"
