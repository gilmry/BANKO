# Specifications de Securite API -- Open Banking BANKO

| Metadata | Valeur |
|---|---|
| **Version** | 1.0.0 |
| **Date** | 6 avril 2026 |
| **Statut** | En vigueur |
| **Classification** | Interne -- Diffusion restreinte |
| **Auteur** | Equipe Architecture BANKO |
| **Approbation** | Comite de conformite / RSSI |

---

## Table des matieres

1. [Principes de securite API bancaire](#1-principes-de-securite-api-bancaire)
2. [Standards de reference](#2-standards-de-reference)
3. [Architecture de securite API BANKO](#3-architecture-de-securite-api-banko)
4. [Specifications des endpoints API](#4-specifications-des-endpoints-api)
5. [Gestion des erreurs et codes de retour](#5-gestion-des-erreurs-et-codes-de-retour)
6. [Rate limiting et throttling](#6-rate-limiting-et-throttling)
7. [Versioning API](#7-versioning-api)
8. [Tests de securite](#8-tests-de-securite)
9. [Conformite ANCS](#9-conformite-ancs)

---

## 1. Principes de securite API bancaire

### 1.1 Principes directeurs

La securite des APIs Open Banking de BANKO repose sur les principes fondamentaux suivants :

| Principe | Description | Application BANKO |
|---|---|---|
| **Defense en profondeur** | Multiples couches de securite independantes | 5 couches de securite (section 3) |
| **Moindre privilege** | Acces limite au strict necessaire | Scopes granulaires lies au consentement |
| **Zero trust** | Ne jamais faire confiance implicitement | Verification a chaque requete |
| **Securite par conception** | Securite integree des la conception | Architecture hexagonale avec validation au domaine |
| **Transparence** | Mecanismes de securite documentees et auditables | Logs d'audit immutables |
| **Resilience** | Fonctionnement degrade securise en cas de panne | Fallback vers mode restrictif |
| **Conformite reglementaire** | Respect des exigences BCT et standards internationaux | Mapping reglementaire continu |

### 1.2 Classification des donnees

| Niveau | Exemples | Exigences API |
|---|---|---|
| **Public** | Taux de change publics, conditions generales | Pas d'authentification, rate limiting standard |
| **Interne** | Statistiques agregees anonymisees | Authentification API key |
| **Confidentiel** | Soldes, transactions, informations client | OAuth 2.0 + consentement + SCA |
| **Strictement confidentiel** | Donnees biometriques, credentials, cles | Jamais exposees via API, acces interne uniquement |

---

## 2. Standards de reference

### 2.1 Panorama des standards

| Standard | Organisme | Version | Usage BANKO | Priorite |
|---|---|---|---|---|
| **Berlin Group NextGenPSD2** | Berlin Group | 1.3.12 | Modele d'API principal | Critique |
| **UK Open Banking Standard** | OBIE / OBL | 3.1.11 | Reference complementaire | Haute |
| **Financial-grade API (FAPI) 2.0** | OpenID Foundation | Final | Profil de securite | Critique |
| **OAuth 2.0 Security BCP** | IETF | RFC 9700 | Bonnes pratiques OAuth | Critique |
| **OpenAPI Specification** | OAI | 3.1.0 | Documentation API | Haute |
| **JSON:API** | jsonapi.org | 1.1 | Convention de formatage | Moyenne |
| **RFC 7807 / RFC 9457** | IETF | -- | Format d'erreurs | Haute |
| **ISO 20022** | ISO | 2024 | Format de messages financiers | Haute |

### 2.2 Berlin Group NextGenPSD2

Le framework NextGenPSD2 de Berlin Group constitue le modele principal pour la conception des APIs BANKO :

| Composante | Description | Adoption BANKO |
|---|---|---|
| **Account Information Services (AIS)** | APIs de consultation de comptes | Phase 2 |
| **Payment Initiation Services (PIS)** | APIs d'initiation de paiement | Phase 3 |
| **Confirmation of Funds (CoF)** | API de verification de provision | Phase 3 |
| **Signing Baskets** | Signature groupee de paiements | Phase 4 |
| **Consent Model** | Gestion standardisee du consentement | Phase 2 (voir [02-consent-management](./02-consent-management.md)) |
| **SCA Approaches** | Redirect, Decoupled, Embedded | Phase 2-3 (voir [03-sca](./03-sca-strong-customer-authentication.md)) |

### 2.3 FAPI 2.0 -- Profil de securite

FAPI 2.0 (Financial-grade API) definit un profil de securite renforce pour les APIs financieres :

| Exigence FAPI 2.0 | Description | Implementation BANKO |
|---|---|---|
| **OAuth 2.0 + PKCE obligatoire** | Code challenge avec methode S256 | Middleware d'autorisation |
| **PAR (Pushed Authorization Requests)** | Les requetes d'autorisation sont poussees cote serveur | Endpoint `/par` |
| **DPoP ou mTLS** | Proof-of-possession pour les tokens | mTLS (Phase 2), DPoP (Phase 3) |
| **JARM (JWT Secured Authorization Response)** | Reponses d'autorisation signees en JWT | Middleware de reponse |
| **Sender-constrained tokens** | Tokens lies a l'emetteur | Binding certificat / DPoP |
| **Requetes signees** | JWS pour les requetes sensibles | Middleware de verification |

---

## 3. Architecture de securite API BANKO

### 3.1 Vue d'ensemble des 5 couches

L'architecture de securite est structuree en 5 couches independantes, chacune apportant un niveau de protection specifique :

```
+----------------------------------------------------------+
|  Layer 5 : Monitoring & Audit                            |
|  (Rate limiting, anomaly detection, audit logging)       |
+----------------------------------------------------------+
|  Layer 4 : Message Security                              |
|  (JWS signed requests/responses, JWE encryption)        |
+----------------------------------------------------------+
|  Layer 3 : Authorization                                 |
|  (Scope-based, consent-verified, RBAC)                  |
+----------------------------------------------------------+
|  Layer 2 : Authentication                                |
|  (OAuth 2.0 + PKCE, client certificates, API keys)     |
+----------------------------------------------------------+
|  Layer 1 : Transport                                     |
|  (TLS 1.3, mTLS for TPPs, certificate pinning)         |
+----------------------------------------------------------+
```

### 3.2 Layer 1 -- Transport Security

| Specification | Valeur | Justification |
|---|---|---|
| **Protocole TLS** | TLS 1.3 uniquement (TLS 1.2 tolere en transition) | Performances + securite |
| **Cipher suites** | TLS_AES_256_GCM_SHA384, TLS_CHACHA20_POLY1305_SHA256 | AEAD uniquement |
| **Certificate Authority** | CA reconnue (Let's Encrypt ou CA qualifiee) | Confiance publique |
| **Certificate pinning** | Obligatoire pour les applications mobiles | Prevention MITM |
| **mTLS pour TPP** | Certificat client obligatoire pour les TPP | Identification forte du TPP |
| **HSTS** | `Strict-Transport-Security: max-age=31536000; includeSubDomains` | Forcer HTTPS |
| **OCSP Stapling** | Active | Verification de revocation performante |

**Configuration mTLS pour les TPP** :

| Parametre | Valeur | Description |
|---|---|---|
| CA racine acceptee | eIDAS QWAC ou CA approuvee BCT | Certificats qualifies |
| Attributs requis | `organizationIdentifier` (TPP ID), `organizationName` | Identification du TPP |
| Verification CRL/OCSP | A chaque connexion | Detection de revocation |
| Duree de validite max | 2 ans | Renouvellement regulier |
| Algorithme cle | RSA 2048+ ou ECDSA P-256+ | Securite cryptographique |

### 3.3 Layer 2 -- Authentication

| Methode | Usage | Specification | Phase |
|---|---|---|---|
| **OAuth 2.0 + PKCE** | Autorisation des TPP pour acces aux donnees client | RFC 6749 + RFC 7636, grant `authorization_code` | Phase 2 |
| **Client certificate (mTLS)** | Identification du TPP au niveau transport | RFC 8705 (OAuth 2.0 mTLS) | Phase 2 |
| **API Key** | Acces aux APIs publiques (taux, produits) | Header `X-API-Key` | Phase 1 |
| **Bearer token (JWT)** | Acces aux ressources protegees | RFC 6750, format JWT signe RS256 | Phase 1 |
| **DPoP** | Proof-of-possession des tokens | RFC 9449 | Phase 3 |

### 3.4 Layer 3 -- Authorization

| Composante | Description | Implementation |
|---|---|---|
| **Scope-based access** | Les tokens contiennent des scopes lies au consentement | Middleware de verification des scopes |
| **Consent verification** | Chaque acces aux donnees verifie le consentement actif | Service de consentement (voir [02-consent-management](./02-consent-management.md)) |
| **RBAC** | Role-based access control pour les APIs internes | Governance BC |
| **Resource ownership** | Le client ne peut acceder qu'a ses propres ressources | Verification `customer_id` dans le token |
| **TPP permission check** | Le TPP ne peut acceder qu'aux ressources couvertes par le consentement | Join consent-permission-resource |
| **Time-bound access** | Verification de l'expiration du consentement | Check `expires_at` a chaque requete |

### 3.5 Layer 4 -- Message Security

| Mecanisme | Usage | Standard | Implementation |
|---|---|---|---|
| **JWS (JSON Web Signature)** | Signature des requetes de paiement | RFC 7515 | Header `x-jws-signature` |
| **JWE (JSON Web Encryption)** | Chiffrement des donnees sensibles en reponse | RFC 7516 | Optionnel, par accord TPP |
| **Request Object** | Requetes d'autorisation signees | RFC 9101 (JAR) | Middleware OAuth |
| **Idempotency Key** | Prevention de la double soumission | Header `Idempotency-Key` (UUID) | Middleware Actix-web |
| **Digest** | Integrite du corps de la requete | Header `Digest: SHA-256=...` | Middleware de verification |
| **Date** | Protection contre le rejeu | Header `Date` (tolerance 5 min) | Middleware de verification |

### 3.6 Layer 5 -- Monitoring & Audit

| Composante | Description | Technologie |
|---|---|---|
| **Rate limiting** | Controle de debit par TPP, par client, par endpoint | Middleware Actix-web + Redis |
| **Anomaly detection** | Detection de patterns inhabituels | Regles metier + ML (Phase 3) |
| **Audit logging** | Journalisation immutable de tous les acces | PostgreSQL append-only + Prometheus |
| **Real-time alerting** | Alertes en temps reel sur les anomalies | Alertmanager + PagerDuty |
| **API analytics** | Analyse d'usage des APIs | Prometheus + Grafana dashboards |
| **SLA monitoring** | Suivi de la disponibilite et des temps de reponse | Prometheus + uptime checks |

---

## 4. Specifications des endpoints API

### 4.1 Endpoints Open Banking

| Endpoint | Methode | Description | Auth requise | SCA requise | Scope | Rate limit |
|---|---|---|---|---|---|---|
| `/api/v1/accounts` | `GET` | Liste des comptes du client | OAuth 2.0 Bearer | Non (si SCA < 90j) | `account_details` | 10/min/TPP/client |
| `/api/v1/accounts/{id}` | `GET` | Detail d'un compte | OAuth 2.0 Bearer | Non (si SCA < 90j) | `account_details` | 10/min/TPP/client |
| `/api/v1/accounts/{id}/balances` | `GET` | Soldes d'un compte | OAuth 2.0 Bearer | Non (si SCA < 90j) | `account_balance` | 10/min/TPP/client |
| `/api/v1/accounts/{id}/transactions` | `GET` | Historique des transactions | OAuth 2.0 Bearer | Non (si SCA < 90j) | `transaction_history` | 10/min/TPP/client |
| `/api/v1/accounts/{id}/beneficiaries` | `GET` | Beneficiaires enregistres | OAuth 2.0 Bearer | Non (si SCA < 90j) | `beneficiary_list` | 10/min/TPP/client |
| `/api/v1/accounts/{id}/standing-orders` | `GET` | Ordres permanents | OAuth 2.0 Bearer | Non (si SCA < 90j) | `standing_orders` | 10/min/TPP/client |
| `/api/v1/payments/sepa-credit-transfers` | `POST` | Initiation virement SEPA | OAuth 2.0 Bearer + JWS | Oui (Dynamic linking) | `payment_initiation` | 5/min/TPP/client |
| `/api/v1/payments/{id}/status` | `GET` | Statut d'un paiement | OAuth 2.0 Bearer | Non | `payment_status` | 20/min/TPP |
| `/api/v1/payments/{id}/confirm` | `PUT` | Confirmation du paiement apres SCA | OAuth 2.0 Bearer | Oui (incluse) | `payment_initiation` | 5/min/TPP/client |
| `/api/v1/consents` | `POST` | Creer un consentement | OAuth 2.0 Client | Non | -- | 10/min/TPP |
| `/api/v1/consents/{id}` | `GET` | Consulter un consentement | OAuth 2.0 Bearer/Client | Non | -- | 20/min/TPP |
| `/api/v1/consents/{id}` | `DELETE` | Revoquer un consentement | OAuth 2.0 Bearer/Client | Non | -- | 10/min/TPP |
| `/api/v1/funds-confirmations` | `POST` | Verification de provision | OAuth 2.0 Bearer | Non | `funds_confirmation` | 20/min/TPP |
| `/api/v1/identity/verify` | `POST` | Verification d'identite (VoP) | OAuth 2.0 Bearer | Non | `identity_verification` | 10/min/TPP |

### 4.2 Endpoints d'infrastructure

| Endpoint | Methode | Description | Auth requise | Rate limit |
|---|---|---|---|---|
| `/api/v1/health` | `GET` | Etat de sante de l'API | Aucune | 60/min |
| `/api/v1/health/detailed` | `GET` | Etat detaille (DB, services) | API Key (admin) | 10/min |
| `/.well-known/openid-configuration` | `GET` | Configuration OpenID Connect | Aucune | 60/min |
| `/.well-known/jwks.json` | `GET` | Cles publiques JWK | Aucune | 60/min |
| `/oauth/authorize` | `GET` | Point d'autorisation OAuth | Session client | 20/min/client |
| `/oauth/token` | `POST` | Echange de tokens | Client certificate / secret | 20/min/TPP |
| `/oauth/par` | `POST` | Pushed Authorization Request | Client certificate | 20/min/TPP |
| `/oauth/revoke` | `POST` | Revocation de token | Client certificate | 10/min/TPP |
| `/metrics` | `GET` | Metriques Prometheus | Reseau interne uniquement | -- |

### 4.3 Headers de requete obligatoires

| Header | Valeur | Obligatoire | Description |
|---|---|---|---|
| `Authorization` | `Bearer {token}` | Pour les endpoints proteges | Token d'acces OAuth 2.0 |
| `Content-Type` | `application/json` | Pour POST/PUT | Format du corps |
| `Accept` | `application/json` | Recommande | Format de reponse souhaite |
| `X-Request-ID` | UUID v4 | Oui | Identifiant unique de requete pour tracabilite |
| `X-TPP-ID` | Identifiant TPP | Pour les TPP | Identifiant du TPP (peut etre extrait du certificat) |
| `Idempotency-Key` | UUID v4 | Pour POST de paiement | Prevention de double soumission |
| `PSU-IP-Address` | IP du client final | Pour les requetes PSU-initiated | Adresse IP du client (Payment Service User) |
| `PSU-User-Agent` | User-Agent du client | Recommande | Navigateur/application du client |
| `Date` | RFC 7231 | Recommande | Horodatage de la requete |
| `Digest` | `SHA-256={base64}` | Pour POST avec JWS | Hash du corps de requete |
| `x-jws-signature` | JWS detache | Pour les paiements | Signature de la requete |

### 4.4 Headers de reponse

| Header | Valeur | Description |
|---|---|---|
| `X-Request-ID` | UUID (echo de la requete) | Correlation requete-reponse |
| `X-RateLimit-Limit` | Nombre | Limite de debit applicable |
| `X-RateLimit-Remaining` | Nombre | Requetes restantes dans la fenetre |
| `X-RateLimit-Reset` | Timestamp | Moment de reinitialisation du compteur |
| `Retry-After` | Secondes | Delai avant de reessayer (si 429) |
| `Sunset` | Date RFC 7231 | Date de depreciation de l'endpoint (si applicable) |
| `Deprecation` | Date | Date de depreciation annoncee |
| `Cache-Control` | `no-store` | Pas de mise en cache de donnees financieres |
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | HSTS |
| `X-Content-Type-Options` | `nosniff` | Prevention du MIME sniffing |
| `X-Frame-Options` | `DENY` | Prevention du clickjacking |
| `Content-Security-Policy` | Politique restrictive | Prevention XSS |

---

## 5. Gestion des erreurs et codes de retour

### 5.1 Format d'erreur -- RFC 7807 / RFC 9457

Toutes les erreurs API suivent le format Problem Details for HTTP APIs :

```json
{
  "type": "https://api.banko.tn/errors/{error-type}",
  "title": "Titre lisible de l'erreur",
  "status": 400,
  "detail": "Description detaillee du probleme",
  "instance": "/api/v1/payments/uuid-requete",
  "traceId": "uuid-correlation",
  "timestamp": "2026-04-06T10:30:00Z",
  "errors": [
    {
      "field": "amount",
      "code": "INVALID_AMOUNT",
      "message": "Le montant doit etre positif"
    }
  ]
}
```

### 5.2 Catalogue des codes d'erreur

| Code HTTP | Type d'erreur | Description | Action client |
|---|---|---|---|
| `400` | `BAD_REQUEST` | Requete malformee ou parametres invalides | Corriger la requete |
| `401` | `UNAUTHORIZED` | Token absent, invalide ou expire | Renouveler le token |
| `403` | `FORBIDDEN` | Permissions insuffisantes ou consentement manquant | Verifier le consentement et les scopes |
| `404` | `NOT_FOUND` | Ressource inexistante | Verifier l'identifiant |
| `405` | `METHOD_NOT_ALLOWED` | Methode HTTP non supportee | Utiliser la methode correcte |
| `406` | `NOT_ACCEPTABLE` | Format de reponse non supporte | Verifier le header `Accept` |
| `409` | `CONFLICT` | Conflit d'etat (ex: consentement deja revoque) | Verifier l'etat de la ressource |
| `415` | `UNSUPPORTED_MEDIA_TYPE` | Content-Type non supporte | Utiliser `application/json` |
| `422` | `UNPROCESSABLE_ENTITY` | Requete valide syntaxiquement mais rejetee metier | Corriger les donnees metier |
| `429` | `TOO_MANY_REQUESTS` | Rate limit atteint | Respecter `Retry-After` |
| `500` | `INTERNAL_SERVER_ERROR` | Erreur interne du serveur | Reessayer (idempotent) ou contacter le support |
| `502` | `BAD_GATEWAY` | Service amont indisponible | Reessayer apres delai |
| `503` | `SERVICE_UNAVAILABLE` | Service temporairement indisponible | Respecter `Retry-After` |

### 5.3 Codes d'erreur metier specifiques

| Code | Type | Description | Endpoint concerne |
|---|---|---|---|
| `CONSENT_EXPIRED` | `403` | Le consentement a expire | Tous les endpoints AIS/PIS |
| `CONSENT_REVOKED` | `403` | Le consentement a ete revoque | Tous les endpoints AIS/PIS |
| `CONSENT_INVALID_STATUS` | `409` | Le consentement n'est pas dans un etat valide | `PUT /consents/{id}` |
| `SCA_REQUIRED` | `401` | Authentification forte requise | Endpoints avec SCA |
| `SCA_FAILED` | `403` | Echec de l'authentification forte | Endpoints avec SCA |
| `INSUFFICIENT_FUNDS` | `422` | Provision insuffisante | `POST /payments/*` |
| `INVALID_BENEFICIARY` | `422` | Beneficiaire invalide ou sanctionne | `POST /payments/*` |
| `PAYMENT_REJECTED` | `422` | Paiement rejete par les regles metier | `POST /payments/*` |
| `ACCOUNT_BLOCKED` | `403` | Compte bloque ou gele | Endpoints comptes |
| `TPP_NOT_REGISTERED` | `403` | TPP non enregistre dans le registre | Tous |
| `TPP_CERTIFICATE_INVALID` | `401` | Certificat TPP invalide ou revoque | Tous |

---

## 6. Rate limiting et throttling

### 6.1 Strategie de rate limiting

| Dimension | Description | Implementation |
|---|---|---|
| **Par TPP** | Limite globale par TPP enregistre | Identifie par certificat mTLS ou `client_id` |
| **Par client (PSU)** | Limite par utilisateur final | Identifie par `customer_id` dans le token |
| **Par endpoint** | Limite specifique a chaque endpoint | Configuration par route |
| **Burst vs Sustained** | Tolerance aux pics courts vs debit moyen | Token bucket algorithm |
| **Global** | Limite totale de la plateforme | Protection contre la surcharge |

### 6.2 Limites par type d'endpoint

| Categorie | Burst (par seconde) | Sustained (par minute) | Par TPP (par heure) | Par client (par heure) |
|---|---|---|---|---|
| **AIS -- Soldes** | 5 | 10 | 600 | 60 |
| **AIS -- Transactions** | 3 | 10 | 600 | 60 |
| **AIS -- Details compte** | 5 | 10 | 600 | 60 |
| **PIS -- Initiation** | 2 | 5 | 300 | 30 |
| **PIS -- Statut** | 5 | 20 | 1200 | 120 |
| **Consentement** | 3 | 10 | 600 | -- |
| **OAuth/Token** | 5 | 20 | 1200 | -- |
| **Health/Public** | 10 | 60 | -- | -- |

### 6.3 Algorithme Token Bucket

| Parametre | Description | Valeur par defaut |
|---|---|---|
| `capacity` | Nombre max de tokens (taille du bucket) | Variable par endpoint |
| `refill_rate` | Tokens ajoutes par seconde | Variable par endpoint |
| `refill_interval` | Intervalle de remplissage | 1 seconde |
| `initial_tokens` | Tokens initiaux | Egal a `capacity` |
| `storage` | Backend de stockage des compteurs | Redis (avec TTL) |

### 6.4 Reponse en cas de depassement

```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/problem+json
X-RateLimit-Limit: 10
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1712390460
Retry-After: 30

{
  "type": "https://api.banko.tn/errors/rate-limit-exceeded",
  "title": "Limite de debit depassee",
  "status": 429,
  "detail": "Vous avez depasse la limite de 10 requetes par minute pour cet endpoint. Veuillez reessayer dans 30 secondes.",
  "instance": "/api/v1/accounts/uuid/balances"
}
```

### 6.5 Exemptions et ajustements

| Cas | Ajustement | Condition |
|---|---|---|
| TPP Premium | Limites doublees | Accord commercial + SLA |
| Periode de migration | Limites assouplies temporairement | Premiere semaine d'integration |
| Incident en cours | Limites reduites | Protection de la plateforme |
| Sandbox | Limites generales | Pas de rate limiting strict |
| APIs publiques (taux FX) | Limites genereuses | Pas de donnees sensibles |

---

## 7. Versioning API

### 7.1 Strategie de versioning

| Aspect | Choix BANKO | Justification |
|---|---|---|
| **Methode** | URL path versioning (`/api/v1/`, `/api/v2/`) | Simplicite, clarte, convention industrie |
| **Schema** | Semantic versioning (MAJOR.MINOR.PATCH) | Versions mineures retrocompatibles |
| **Breaking changes** | Nouvelle version majeure uniquement | Stabilite pour les TPP |
| **Deprecation** | Annonce 12 mois avant suppression | Temps d'adaptation suffisant |
| **Sunset** | Header `Sunset` sur les endpoints deprecies | Conformite RFC 8594 |

### 7.2 Politique de compatibilite

| Type de changement | Retro-compatible | Version | Exemple |
|---|---|---|---|
| Ajout d'un champ optionnel en reponse | Oui | MINOR | Nouveau champ `middle_name` |
| Ajout d'un endpoint | Oui | MINOR | `GET /accounts/{id}/cards` |
| Ajout d'un parametre optionnel | Oui | MINOR | `?currency=TND` |
| Suppression d'un champ | Non | MAJOR | Retrait de `legacy_id` |
| Modification de type d'un champ | Non | MAJOR | `amount: string` vers `amount: number` |
| Modification de la semantique d'un endpoint | Non | MAJOR | Changement du comportement de filtrage |
| Correction de bug | Oui | PATCH | Fix d'un calcul de solde |

### 7.3 Cycle de vie des versions

| Phase | Duree | Headers | Description |
|---|---|---|---|
| **Active** | Indefinie | -- | Version courante, pleinement supportee |
| **Deprecated** | 12 mois minimum | `Deprecation: {date}` | Supportee mais migration recommandee |
| **Sunset** | 3 mois | `Sunset: {date}` | Derniers mois avant suppression |
| **Retired** | -- | `410 Gone` | Version supprimee, erreur retournee |

### 7.4 Communication des changements

| Canal | Contenu | Frequence |
|---|---|---|
| **Changelog** | Liste detaillee des changements | A chaque release |
| **Developer portal** | Documentation mise a jour | Continue |
| **Email TPP** | Annonces de depreciation et breaking changes | Selon besoin |
| **Webhook** | Notification automatique aux TPP | Selon configuration |
| **Header HTTP** | `Deprecation` et `Sunset` dans les reponses | Automatique |

---

## 8. Tests de securite

### 8.1 OWASP API Security Top 10 (2023)

| Risque OWASP | Description | Mitigation BANKO | Test |
|---|---|---|---|
| **API1: Broken Object Level Authorization (BOLA)** | Acces a des objets d'autres utilisateurs | Verification `customer_id` a chaque requete | Tests d'acces croise |
| **API2: Broken Authentication** | Failles dans l'authentification | OAuth 2.0 + mTLS + SCA | Tests de flux OAuth |
| **API3: Broken Object Property Level Authorization** | Acces a des proprietes non autorisees | Filtrage des champs par scope | Tests de filtrage |
| **API4: Unrestricted Resource Consumption** | Abus de ressources | Rate limiting multi-niveaux | Tests de charge |
| **API5: Broken Function Level Authorization** | Acces a des fonctions non autorisees | RBAC + verification des roles | Tests RBAC |
| **API6: Unrestricted Access to Sensitive Business Flows** | Automatisation de flux sensibles | CAPTCHA + rate limiting + TRA | Tests d'automatisation |
| **API7: Server Side Request Forgery (SSRF)** | Requetes forgees cote serveur | Validation stricte des URLs, pas de redirection ouverte | Tests SSRF |
| **API8: Security Misconfiguration** | Mauvaise configuration | Hardening automatise, headers securite | Scan de configuration |
| **API9: Improper Inventory Management** | APIs non documentees ou obsoletes | Registre centralise, depreciation rigoureuse | Audit d'inventaire |
| **API10: Unsafe Consumption of Third-Party APIs** | Confiance excessive envers les APIs externes | Validation des reponses, timeouts, circuit breaker | Tests d'integration |

### 8.2 Types de tests de securite

| Type de test | Description | Frequence | Outil | Responsable |
|---|---|---|---|---|
| **SAST** | Analyse statique du code source | A chaque commit (CI) | `cargo clippy` + `cargo audit` | DevSecOps |
| **DAST** | Analyse dynamique de l'application en execution | Hebdomadaire | OWASP ZAP | Equipe securite |
| **SCA** | Analyse des composants tiers (dependances) | A chaque build (CI) | `cargo audit` + Snyk | DevSecOps |
| **Fuzzing** | Injection de donnees aleatoires/malformees | Mensuel | `cargo-fuzz` + API fuzzer | Equipe securite |
| **Penetration testing** | Test d'intrusion par des experts | Semestriel (+ ANCS) | Manuel + outils | Prestataire externe + ANCS |
| **Regression securite** | Tests automatises des vulnerabilites corrigees | A chaque release | Tests d'integration | DevSecOps |
| **Red team** | Simulation d'attaque avancee | Annuel | Manuel | Equipe dediee |

### 8.3 Criteres d'acceptation securite

| Critere | Seuil | Blocage release |
|---|---|---|
| Vulnerabilites critiques (CVSS >= 9.0) | 0 | Oui |
| Vulnerabilites hautes (CVSS 7.0-8.9) | 0 | Oui |
| Vulnerabilites moyennes (CVSS 4.0-6.9) | < 3 (avec plan de remediation) | Non (avec justification) |
| Couverture SAST | 100% du code application | Oui |
| Tests OWASP Top 10 | 100% des risques couverts | Oui |
| Dependances vulnerables connues | 0 critique/haute | Oui |

---

## 9. Conformite ANCS

### 9.1 Exigences de la Circulaire BCT 2025-06

La Circulaire BCT 2025-06 impose des tests d'intrusion realises par l'Agence Nationale de la Cybersecurite (ANCS) pour les systemes bancaires, notamment ceux impliquant l'identification electronique et les services de paiement.

| Exigence | Description | Periodicite | Responsable |
|---|---|---|---|
| **Test d'intrusion externe** | Tentatives de penetration depuis Internet | Annuel minimum | ANCS ou prestataire agree |
| **Test d'intrusion interne** | Tests depuis le reseau interne | Annuel minimum | ANCS ou prestataire agree |
| **Test des APIs** | Tests specifiques des endpoints Open Banking | A chaque mise a jour majeure | ANCS ou prestataire agree |
| **Rapport de conformite** | Rapport detaille avec recommandations | Apres chaque test | ANCS |
| **Plan de remediation** | Plan d'action pour corriger les failles | 30 jours apres rapport | Equipe securite BANKO |
| **Re-test** | Verification de la correction des failles | 90 jours apres remediation | ANCS |

### 9.2 Perimeter de test ANCS pour les APIs Open Banking

| Composante | Perimetre | Tests specifiques |
|---|---|---|
| **API Gateway** | Configuration TLS, headers, rate limiting | Scan SSL, header analysis, stress test |
| **Endpoints AIS** | Autorisation, filtrage, injection | BOLA, SQL injection, XSS, IDOR |
| **Endpoints PIS** | SCA, dynamic linking, idempotence | Replay, MITM, bypass SCA |
| **OAuth/OIDC** | Flux d'autorisation, tokens | Token forgery, redirect URI manipulation |
| **Consent API** | Gestion du consentement | Elevation de privilege, manipulation d'etat |
| **Infrastructure** | Serveurs, reseau, base de donnees | Scan de vulnerabilites, escalade de privilege |

### 9.3 Preparation aux tests ANCS

| Action preparatoire | Description | Delai |
|---|---|---|
| Inventaire des composants | Liste exhaustive des composants et versions | T-60j |
| Documentation d'architecture | Schemas reseau, flux de donnees, composants | T-45j |
| Environnement de test | Environnement identique a la production (donnees anonymisees) | T-30j |
| Contacts techniques | Liste des referents disponibles pendant les tests | T-15j |
| Baseline securite | Resultats des scans internes recents | T-7j |
| Acces et credentials | Credentials de test pour les differents profils | T-3j |

### 9.4 Suivi post-test

| Severite de la faille | Delai de correction | Verification |
|---|---|---|
| **Critique** | 48 heures (mitigation immediate) + 30 jours (correction definitive) | Re-test ANCS obligatoire |
| **Haute** | 30 jours | Re-test ANCS obligatoire |
| **Moyenne** | 90 jours | Verification interne |
| **Basse** | 180 jours | Verification interne |
| **Informative** | Prochaine release | Auto-verification |

---

## Annexe A -- Checklist de securite API

| Categorie | Verification | Statut |
|---|---|---|
| **Transport** | TLS 1.3 uniquement | A verifier |
| **Transport** | mTLS pour les TPP | A implementer (Phase 2) |
| **Transport** | HSTS configure | A verifier |
| **Transport** | Certificate pinning (mobile) | A implementer |
| **Auth** | OAuth 2.0 + PKCE | A implementer (Phase 2) |
| **Auth** | Tokens de courte duree (5 min) | A configurer |
| **Auth** | Refresh token rotation | A implementer |
| **Autorisation** | Verification consentement a chaque requete | A implementer (Phase 2) |
| **Autorisation** | BOLA protection | En cours |
| **Message** | JWS pour les paiements | A implementer (Phase 3) |
| **Message** | Idempotency-Key | A implementer |
| **Monitoring** | Rate limiting actif | En cours |
| **Monitoring** | Audit log immutable | En cours |
| **Monitoring** | Alerting configure | A configurer |
| **Headers** | Tous les headers de securite | A verifier |
| **Erreurs** | Format RFC 7807 | A implementer |
| **Test** | OWASP Top 10 API couvert | A planifier |
| **Test** | Tests ANCS planifies | A planifier |

---

## Annexe B -- References

| Reference | Source |
|---|---|
| NextGenPSD2 Framework v1.3.12 | Berlin Group |
| UK Open Banking Standard v3.1.11 | Open Banking Limited |
| FAPI 2.0 Security Profile | OpenID Foundation |
| OAuth 2.0 (RFC 6749) | IETF |
| PKCE (RFC 7636) | IETF |
| OAuth 2.0 Security BCP (RFC 9700) | IETF |
| mTLS for OAuth (RFC 8705) | IETF |
| DPoP (RFC 9449) | IETF |
| Problem Details (RFC 7807 / 9457) | IETF |
| Sunset Header (RFC 8594) | IETF |
| OWASP API Security Top 10 (2023) | OWASP Foundation |
| Circulaire BCT 2025-06 | Banque Centrale de Tunisie |

---

*Document precedent : [03 -- Authentification forte (SCA)](./03-sca-strong-customer-authentication.md)*
*Document suivant : [05 -- Mapping Open Banking tunisien](./05-tunisian-open-banking-mapping.md)*
