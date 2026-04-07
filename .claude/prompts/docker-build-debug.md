# Prompt Claude Code : Build & Debug BANKO dans Docker

## Contexte

BANKO est une plateforme bancaire Rust (Actix-web) avec architecture hexagonale.
Le code a été implémenté en session Cowork sans compilateur Rust disponible.
Il faut maintenant compiler, corriger les erreurs, et valider que tout fonctionne.

## Étape 1 : Lancer l'environnement Docker

```bash
# Depuis la racine du projet BANKO
docker compose up -d db    # PostgreSQL d'abord
docker compose up -d --build backend  # Build le container Rust
```

Si le Dockerfile.dev pose problème, utiliser directement :
```bash
docker run --rm -it \
  -v $(pwd):/app -w /app \
  -e DATABASE_URL="postgres://banko:banko@host.docker.internal:5432/banko" \
  -e SQLX_OFFLINE=true \
  rust:1.82-bookworm \
  bash
```

## Étape 2 : Première compilation

```bash
# Dans le container
cargo build 2>&1 | head -100
```

### Erreurs attendues et stratégie de résolution

**Catégorie 1 : Imports manquants**
- `use crate::shared::errors::DomainError;` manquant dans les nouveaux fichiers
- `use chrono::{DateTime, Utc};` manquant
- `use uuid::Uuid;` manquant
- **Fix** : Ajouter les imports nécessaires en haut de chaque fichier fautif

**Catégorie 2 : Traits non implémentés**
- Les ports (traits async) dans `application/*/ports.rs` peuvent avoir des signatures
  qui ne matchent pas exactement les implémentations dans `infrastructure/*/repository.rs`
- **Fix** : Aligner les signatures (types de retour, paramètres)

**Catégorie 3 : Types non trouvés**
- Les `mod.rs` peuvent manquer des `pub use` pour les nouveaux types
- **Fix** : Ajouter les re-exports dans chaque `mod.rs`

**Catégorie 4 : Erreurs de construction d'entités**
- Le constructeur `Customer::new()` a été modifié pour inclure `segment` et `documents`
- Tous les appels existants doivent être mis à jour
- **Fix** : Chercher `Customer::new(` dans tout le code et ajouter les nouveaux paramètres

**Catégorie 5 : sqlx compile-time check**
- Avec `SQLX_OFFLINE=true`, les requêtes SQL ne sont pas vérifiées au build
- Si `SQLX_OFFLINE` n'est pas set, les requêtes seront vérifiées contre la DB
- **Fix** : Soit `SQLX_OFFLINE=true`, soit lancer la DB et `sqlx prepare`

## Étape 3 : Approche itérative de debug

```bash
# Boucle de correction
cargo build 2>&1 | grep "^error" | head -20

# Pour chaque erreur :
# 1. Identifier le fichier et la ligne
# 2. Comprendre la cause (import, type, signature)
# 3. Corriger
# 4. Rebuild

# Quand ça compile :
cargo test 2>&1 | tail -30

# Vérifier la qualité :
cargo clippy -- -D warnings 2>&1 | head -50
```

## Étape 4 : Migrations et seed

```bash
# Une fois la DB up
DATABASE_URL="postgres://banko:banko@db:5432/banko" sqlx migrate run
```

## Étape 5 : Lancer le serveur

```bash
cargo run --bin server
# Tester : curl http://localhost:8080/api/v1/health
```

## Étape 6 : Tests

```bash
# Tests unitaires (domaine pur, pas besoin de DB)
cargo test --lib -p banko-domain

# Tests application (avec mocks)
cargo test --lib -p banko-application

# Tests BDD (besoin de DB)
cargo test --test bdd

# Couverture
cargo tarpaulin --out Html
```

## Points d'attention

1. **Les fichiers `advanced.rs`** dans chaque BC sont les nouveaux modules.
   S'ils ont des erreurs de compilation, vérifier que leur `mod.rs` parent
   les déclare bien avec `pub mod advanced;`

2. **Les fichiers `*_service.rs`** dans application dépendent des ports.
   Vérifier que les traits dans `ports.rs` matchent les implémentations.

3. **Le `server.rs`** importe 9 nouvelles fonctions de configuration de routes.
   Si certaines n'existent pas encore dans `routes.rs`, commenter les imports.

4. **L'ordre des migrations** est important. Les migrations 20260407* doivent
   s'exécuter après les 20260406* existantes.

## Métriques cibles

- `cargo build` : 0 erreurs
- `cargo test` : 400+ tests passent
- `cargo clippy` : 0 warnings
- `sqlx migrate run` : 38 migrations appliquées
- `curl /api/v1/health` : 200 OK
