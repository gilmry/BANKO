# Politique de Sécurité BANKO

## Versions Supportées

Les correctifs de sécurité sont fournis pour les versions suivantes :

| Version | Supportée |
|---------|-----------|
| 0.1.x   | ✅        |

## Rapports de Vulnérabilités

L'équipe BANKO prend les bugs de sécurité très au sérieux. Nous apprécions vos efforts de divulgation responsable.

### Comment Signaler une Vulnérabilité de Sécurité ?

**Ne signalez PAS les vulnérabilités de sécurité via les issues GitHub publiques.**

À la place, signalez les vulnérabilités par email à:

**security@banko.tn**

Vous devriez recevoir une réponse dans les 48 heures. Si vous ne recevez pas de réponse, veuillez relancer par email.

### Informations à Inclure dans votre Rapport

Veuillez inclure les informations suivantes :

- Type de vulnérabilité (ex: SQL injection, authentification bypass, etc.)
- Chemins complets des fichiers source affectés
- Localisation du code affecté (tag/branche/commit ou URL directe)
- Configuration spéciale requise pour reproduire
- Instructions pas-à-pas pour reproduire la vulnérabilité
- Proof-of-concept ou code d'exploitation (si possible)
- Impact de la vulnérabilité et comment un attaquant pourrait l'exploiter

Cette information nous aide à trier votre rapport plus rapidement.

## Politique de Divulgation

Quand l'équipe de sécurité reçoit un rapport de vulnérabilité, elle va:

1. Confirmer le problème et déterminer les versions affectées
2. Auditer le code pour trouver les problèmes similaires potentiels
3. Préparer les correctifs pour toutes les versions en maintenance
4. Publier de nouvelles versions de sécurité dès que possible

La divulgation s'effectue de manière coordonnée et responsable.

## Bonnes Pratiques de Sécurité pour les Contributeurs

### Authentification & Autorisation

- **Ne jamais committer de credentials** (clés API, mots de passe, tokens, etc.)
- Utiliser des variables d'environnement pour la configuration sensible
- Implémenter la validation JWT et la rotation des refresh tokens
- Suivre le principe du moindre privilège pour les rôles utilisateur

### Protection des Données

- **Conformité GDPR**: Toutes les données personnelles doivent être traitées selon GDPR
- Chiffrer les données sensibles au repos (numéros de compte, IBAN, etc.)
- Utiliser HTTPS pour toutes les communications externes
- Implémenter des politiques de retention et suppression des données
- Masquer les identifiants bancaires sensibles dans les logs/réponses API

### Validation des Entrées

- **Valider TOUTES les entrées** au Domain layer (constructeurs d'entités)
- Sanitiser les entrées utilisateur pour prévenir les injections
- Utiliser les prepared statements pour toutes les requêtes DB (sqlx)
- Implémenter la limitation de taux (rate limiting) sur les endpoints API

### Dépendances

- Exécuter régulièrement `make audit` pour vérifier les dépendances vulnérables
- Maintenir toutes les dépendances à jour
- Consulter les avis de sécurité pour les crates Rust et packages npm
- Utiliser `cargo audit` dans le pipeline CI/CD

### Revue de Code

- Tous les changements de code doivent passer par une Pull Request review
- Les changements sensibles à la sécurité requièrent une review des mainteneurs
- Exécuter `make ci` avant le push (lint + test + audit)
- Utiliser les outils d'analyse statique (clippy avec `-D warnings`)

### Sécurité de l'Infrastructure

- Garder les images Docker à jour
- Utiliser des images minimales (Alpine, distroless)
- Exécuter les containers en tant qu'utilisateur non-root
- Implémenter un logging et monitoring appropriés

## Flux de Travail Sécurisé pour le Développement

### 1. Avant le Développement
- Revoir les exigences de sécurité
- Vérifier les patterns de sécurité existants dans la codebase
- Consulter les services AML/Sanctions si changements affectent ces modules

### 2. Pendant le Développement
- Suivre les principes d'architecture hexagonale (isolation)
- Valider les entrées aux frontières du domaine
- Écrire les tests pour le code critique de sécurité
- Ne jamais désactiver les contrôles de sécurité

### 3. Avant le Commit
- Exécuter `make pre-commit` (format + lint)
- Vérifier qu'il n'y a pas de secrets hardcodés (`git diff`)
- Revoir les changements pour implications de sécurité

### 4. Avant le Push
- Exécuter `make ci` (lint + test + audit)
- S'assurer que tous les tests passent
- Revoir le rapport d'audit

### 5. Pull Request
- Décrire les implications de sécurité si applicable
- Tagger les PRs sensibles pour la sécurité
- Attendre la review des mainteneurs

## Vulnérabilités Communes à Éviter

### SQL Injection

❌ **Mauvais** (vulnérable):
```rust
let query = format!("SELECT * FROM accounts WHERE iban = '{}'", iban);
```

✅ **Bon** (sûr):
```rust
sqlx::query!("SELECT * FROM accounts WHERE iban = $1", iban)
```

### Authentification Bypass

❌ **Mauvais** (pas de validation):
```rust
pub fn approve_transaction(&mut self) {
    self.status = TransactionStatus::Approved;
}
```

✅ **Bon** (avec validation):
```rust
pub fn approve_transaction(&mut self, user: &User) -> Result<(), String> {
    if !user.has_approval_permission() {
        return Err("Unauthorized".to_string());
    }
    self.status = TransactionStatus::Approved;
    Ok(())
}
```

### Exposition de Données Sensibles

❌ **Mauvais** (expose l'IBAN):
```rust
#[derive(Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub iban: String,  // ❌ Exposé dans API!
}
```

✅ **Bon** (IBAN masqué):
```rust
#[derive(Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub iban_masked: String,  // "DE89** **** **** ****"
}
```

### AML/Sanctions Bypass

❌ **Mauvais** (pas de vérification):
```rust
pub fn create_transaction(amount: Decimal) -> Result<()> {
    Transaction::new(amount)?;
    Ok(())
}
```

✅ **Bon** (avec vérification AML):
```rust
pub fn create_transaction(
    customer_id: CustomerId,
    amount: Decimal,
    aml_service: &AmlService,
) -> Result<()> {
    aml_service.check_threshold(customer_id, amount)?;
    Transaction::new(amount)?;
    Ok(())
}
```

## Tests de Sécurité

### Tests Unitaires

Tester la logique critique:

```rust
#[test]
fn test_negative_amount_rejected() {
    let result = Transaction::new(Uuid::new_v4(), Decimal::from(-100), ...);
    assert!(result.is_err());
}

#[test]
fn test_unauthorized_user_cannot_approve() {
    let mut transaction = create_test_transaction();
    let unauthorized_user = create_test_user_without_permissions();

    let result = transaction.approve(&unauthorized_user);
    assert!(result.is_err());
}
```

### Tests d'Intégration

Tester authentification et autorisation:

```rust
#[tokio::test]
async fn test_cannot_access_other_customer_account() {
    let customer1 = create_test_customer().await;
    let customer2 = create_test_customer().await;

    let token = login_as_customer(&customer1).await;
    let response = get_account_with_token(&token, customer2.id).await;

    assert_eq!(response.status(), 403); // Forbidden
}
```

## Checklist de Sécurité

Utilisez cette checklist pour les features sensibles :

- [ ] Validation des entrées implémentée au Domain layer
- [ ] Vérifications d'autorisation à tous les niveaux
- [ ] Données sensibles exclues des réponses API
- [ ] Prévention SQL injection (prepared statements sqlx)
- [ ] Audit logging implémenté pour opérations critiques
- [ ] Les données sensibles sont masquées/anonymisées dans les logs
- [ ] Messages d'erreur ne leaking pas les infos système
- [ ] Dépendances auditées (`make audit` passe)
- [ ] Tests de sécurité écrits et passant
- [ ] Code revu par un mainteneur
- [ ] Conformité GDPR vérifiée (si traitement données personnelles)

## Infrastructure de Sécurité (Production)

### Chiffrement au Repos

- **LUKS Encryption**: Chiffrement complet du disque pour données PostgreSQL (AES-XTS-512)
- **Encrypted Backups**: Sauvegardes chiffrées quotidiennes avec GPG + S3 offsite

### Monitoring & Alerting

- **Prometheus + Grafana**: Métriques de santé (30j rétention)
- **Loki + Promtail**: Logs applicatifs (7j rétention)
- **Alertmanager**: Alertes sur seuils critiques

### Détection des Intrusions

- **Suricata IDS**: Détection des patterns d'attaque (SQL injection, XSS, path traversal)
- **CrowdSec WAF**: Blocage des IPs malveillantes
- **fail2ban**: Jails personnalisés pour SSH, API abuse, brute-force DB

### Durcissement Sécurité

- **SSH**: Clés uniquement, ciphers modernes
- **Kernel**: Configuration sysctl (SYN cookies, anti-spoofing, ASLR)
- **Headers HTTP**: HSTS (1 an), CSP, X-Frame-Options, etc.
- **Rate Limiting**: 5 tentatives login par 15min/IP

## Contacts de Sécurité

- **Email de Sécurité**: security@banko.tn
- **Mainteneur du Projet**: [À définir]
- **Temps de Réponse**: Dans les 48 heures

---

**Dernière mise à jour**: 2026-04-04

Pour questions sur cette politique, contactez security@banko.tn.
