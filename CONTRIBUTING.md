# Contribuer à BANKO

Bienvenue ! Cette courte synthèse explique comment préparer vos contributions et suivre le workflow Git attendu par l'équipe.

---

## ⚙️ Pré-requis

1. **Cloner** le dépôt et initialiser l'environnement:
   ```bash
   git clone git@github.com:banko/banko.git
   cd banko
   make setup
   ```
2. Vérifier les hooks Git (`make install-hooks`) si vous n'avez pas exécuté `make setup`.

Pour plus de contexte (architecture hexagonale, DDD, bounded contexts), voyez `README.md`, `CLAUDE.md` et les guides dans `.claude/guides/`.

---

## 🌿 Workflow Git

1. **Synchroniser** `main` :
   ```bash
   git checkout main
   git pull origin main
   ```

2. **Créer une branche** à partir de `main`, selon la table ci-dessous :

   | Type de travail | Préfixe | Exemple |
   |-----------------|---------|---------|
   | Nouvelle fonctionnalité | `feature/` | `feature/open-account` |
   | Correction de bug | `fix/` | `fix/payment-validation` |
   | Refactoring | `refactor/` | `refactor/account-module` |
   | Documentation | `docs/` | `docs/aml-guide` |
   | Tâches de maintenance | `chore/` | `chore/update-deps` |

   ```bash
   git checkout -b <prefix>/<description-kebab-case>
   ```

3. **Commits avec DCO** : Tous les commits doivent être signés avec le Developer Certificate of Origin :
   ```bash
   git commit -s -m "feat: add customer onboarding"
   ```

   Le flag `-s` ajoute automatiquement `Signed-off-by: Votre Nom <email>` au commit.

   **Pourquoi DCO ?** En signant, vous certifiez avoir le droit de soumettre ce code et acceptez qu'il soit publié sous licence AGPL-3.0. Voir [GOVERNANCE.md](GOVERNANCE.md) pour détails.

4. **Commits descriptifs** : petits, cohérents et en anglais (`feat:`, `fix:`, `docs:`…).

5. **Hooks** : laissez tourner le `pre-commit` (format, lint) et `pre-push` (tests). Les commandes sont décrites dans `docs/GIT_HOOKS.md`.

6. **Tests** (`make test`, `cargo test`, `npm run test`) avant le push final.

7. **Pull Request** : référencez l'issue correspondante (ex. `Closes #57`) et décrivez les impacts.

---

## 📚 Ressources utiles

- `.claude/guides/feature-workflow.md` : déroulé complet « analyse → branche → TDD → PR ».
- `docs/README.md` : plan de la documentation et guides associés.
- `CLAUDE.md` : Vue d'ensemble de l'architecture et bounded contexts.
- `SECURITY.md` : Bonnes pratiques de sécurité pour le développement.

---

## 📋 Checklist Avant de Soumettre une PR

- [ ] Tests unitaires ajoutés/mis à jour
- [ ] Tests BDD ajoutés si applicable
- [ ] Code formaté (`make format`)
- [ ] Lint passé (`make lint`)
- [ ] Tous les tests passent localement (`make test`)
- [ ] Pas de credentials/secrets hardcodés
- [ ] Docstrings/commentaires ajoutés pour logique complexe
- [ ] Documentation mise à jour (si nécessaire)
- [ ] Commits signés avec DCO (`git commit -s`)

---

## 🔒 Sécurité et Conformité

BANKO est un système bancaire sous régulation. Lors du développement:

- **Validation des entrées**: Valider au Domain layer (constructeurs d'entités)
- **Pas de données sensibles**: Jamais de numéros de compte/IBAN/autres identifiants bancaires dans les logs
- **Audit logging**: Implémenter pour les opérations critiques (virements, changements KYC, etc.)
- **GDPR**: Respecter les directives de protection des données
- **AML/Sanctions**: Consulter les services AML et Sanctions avant changes

Voir `SECURITY.md` pour la politique complète.

---

Merci de contribuer à BANKO et de rester fiable et bien documenté! 🏦
