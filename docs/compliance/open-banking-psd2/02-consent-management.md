# Gestion du Consentement -- Open Banking BANKO

| Metadata | Valeur |
|---|---|
| **Version** | 1.0.0 |
| **Date** | 6 avril 2026 |
| **Statut** | En vigueur |
| **Classification** | Interne -- Diffusion restreinte |
| **Auteur** | Equipe Architecture BANKO |
| **Approbation** | Comite de conformite / DPO |

---

## Table des matieres

1. [Fondements juridiques du consentement](#1-fondements-juridiques-du-consentement)
2. [Architecture du consentement BANKO](#2-architecture-du-consentement-banko)
3. [Granularite des permissions](#3-granularite-des-permissions)
4. [Dashboard client](#4-dashboard-client)
5. [Stockage et audit](#5-stockage-et-audit)
6. [API de consentement](#6-api-de-consentement)
7. [Conformite RGPD / Loi 2025 mapping](#7-conformite-rgpd--loi-2025-mapping)

---

## 1. Fondements juridiques du consentement

Le consentement constitue la pierre angulaire de tout dispositif d'Open Banking. Dans le contexte tunisien, trois cadres juridiques convergent pour definir les exigences applicables.

### 1.1 Loi tunisienne sur la protection des donnees personnelles (2025)

La loi organique adoptee en juin 2025, dont les sanctions entrent en vigueur en juillet 2026, etablit un regime de consentement aux caracteristiques suivantes :

| Exigence | Description | Article | Impact BANKO |
|---|---|---|---|
| **Consentement explicite** | Le consentement doit etre donne par un acte positif clair | Art. 7 | Pas de cases pre-cochees, opt-in obligatoire |
| **Consentement granulaire** | Possibilite de consentir a des finalites distinctes separement | Art. 7-2 | Permissions individuelles par type de donnee |
| **Consentement revocable** | Retrait du consentement aussi simple que son octroi | Art. 7-3 | Bouton de revocation immediat |
| **Consentement eclaire** | Information complete sur les finalites et destinataires | Art. 8 | Ecran d'information avant consentement |
| **Preuve du consentement** | Le responsable de traitement doit pouvoir demontrer le consentement | Art. 7-4 | Journal immutable des consentements |
| **Designation DPO** | Obligation de designer un delegue a la protection des donnees | Art. 35 | Supervision du service de consentement |
| **DPIA obligatoire** | Etude d'impact pour les traitements a risque eleve | Art. 39 | DPIA prealable au lancement Open Banking |
| **Notification 72h** | Notification de violation de donnees sous 72 heures | Art. 43 | Procedure d'incident incluant les donnees partagees |

### 1.2 PSD3 -- Dispositions relatives au consentement

Le paquet PSD3/PSR (accord provisoire du 27 novembre 2025) renforce significativement la gestion du consentement :

| Disposition PSD3/PSR | Description | Reference BANKO |
|---|---|---|
| **Customer dashboards** | Obligation de fournir un tableau de bord de gestion des consentements | Section 4 de ce document |
| **Anti-obstruction** | Interdiction de rendre le consentement plus difficile que l'acces direct | UX design du flux de consentement |
| **Duree limitee** | Consentements a duree definie, renouvellement explicite | Champ `expires_at` du ConsentRecord |
| **Revocation instantanee** | Effet immediat de la revocation, notification au TPP | Webhook de revocation |
| **Consentement specifique** | Par TPP, par type de donnee, par compte | Modele de permissions granulaires |

### 1.3 FIDA -- Cadre de consentement elargi

Le reglement FIDA (Financial Data Access), dont l'adoption est attendue au premier semestre 2026, etend le perimetre du consentement au-dela des comptes de paiement :

| Perimetre FIDA | Types de donnees | Acteurs concernes | Phase BANKO |
|---|---|---|---|
| Comptes de paiement | Soldes, transactions, beneficiaires | AISP/PISP | Phase 2-3 |
| Credit | Encours, echeanciers, scoring | FISP | Phase 4 |
| Assurance | Contrats, garanties, sinistres | FISP | Post-Phase 4 |
| Investissement | Portefeuille, performances, frais | FISP | Post-Phase 4 |
| Retraite | Droits acquis, projections | FISP | Post-Phase 4 |

---

## 2. Architecture du consentement BANKO

### 2.1 Consent Service -- Bounded context transversal

Le service de gestion du consentement est concu comme un composant transversal interagissant avec plusieurs bounded contexts de BANKO. Conformement a l'architecture hexagonale du systeme, il est structure en trois couches.

**Couche Domain** :
- Entite `ConsentRecord` avec regles de validation metier
- Service de domaine `ConsentLifecycleService` gerant les transitions d'etat
- Invariants metier : un consentement ne peut etre accorde que par le titulaire du compte

**Couche Application** :
- Port `ConsentRepository` (trait) pour la persistance
- Use cases : `GrantConsentUseCase`, `RevokeConsentUseCase`, `ListConsentsUseCase`, `RenewConsentUseCase`
- DTOs pour les contrats API

**Couche Infrastructure** :
- Implementation PostgreSQL du `ConsentRepository`
- Handlers HTTP Actix-web pour les endpoints de consentement
- Integration avec le systeme de notification (webhooks TPP)

### 2.2 Modele de donnees -- ConsentRecord

```
ConsentRecord {
    id:              UUID,           -- Identifiant unique du consentement
    customer_id:     UUID,           -- Reference vers le client (Customer BC)
    tpp_id:          UUID,           -- Reference vers le TPP enregistre
    tpp_name:        String,         -- Nom commercial du TPP (pour affichage)
    permissions:     Vec<Permission>,-- Liste des permissions accordees
    scope:           ConsentScope,   -- Perimetre (comptes concernes)
    accounts:        Vec<UUID>,      -- Liste des comptes concernes
    purpose:         String,         -- Finalite declaree par le TPP
    granted_at:      Option<DateTime>,-- Date d'octroi
    expires_at:      DateTime,       -- Date d'expiration
    revoked_at:      Option<DateTime>,-- Date de revocation (si applicable)
    status:          ConsentStatus,  -- Statut courant
    created_at:      DateTime,       -- Date de creation de la demande
    updated_at:      DateTime,       -- Derniere modification
    ip_address:      String,         -- IP du client lors du consentement
    user_agent:      String,         -- User-Agent du client
    version:         u32,            -- Version pour controle de concurrence
}
```

### 2.3 Statuts du consentement

| Statut | Description | Transitions possibles |
|---|---|---|
| `REQUESTED` | Demande initiee par le TPP, en attente d'action client | `GRANTED`, `REJECTED`, `EXPIRED` |
| `GRANTED` | Consentement accorde par le client | `ACTIVE`, `REVOKED`, `EXPIRED` |
| `ACTIVE` | Consentement en cours d'utilisation (premier acces effectue) | `REVOKED`, `EXPIRED`, `SUSPENDED` |
| `SUSPENDED` | Temporairement suspendu (ex: suspicion fraude) | `ACTIVE`, `REVOKED` |
| `REVOKED` | Revoque par le client ou par la banque | Terminal |
| `REJECTED` | Refuse par le client | Terminal |
| `EXPIRED` | Duree de validite depassee | Terminal |

### 2.4 Cycle de vie du consentement

```
[TPP]                    [BANKO]                   [Client]
  |                         |                         |
  |-- POST /consents ------>|                         |
  |                         |-- Notification --------->|
  |                         |                         |
  |                         |<-- Review + Approve -----|
  |                         |                         |
  |<-- 201 + consent_id ----|                         |
  |                         |                         |
  |-- GET /accounts ------->|                         |
  |   (avec consent_id)     |-- Verification -------->|
  |                         |   consentement          |
  |<-- 200 + donnees -------|                         |
  |                         |                         |
  |                         |<-- Revocation ----------|
  |<-- Webhook revocation --|                         |
  |                         |                         |
```

**Etapes detaillees** :

| Etape | Acteur | Action | Validation |
|---|---|---|---|
| 1. Demande | TPP | `POST /api/v1/consents` avec permissions souhaitees | Verification du TPP dans le registre |
| 2. Notification | BANKO | Notification au client (push, SMS, email) | -- |
| 3. Revue | Client | Consultation des permissions demandees | Affichage clair et comprehensible |
| 4. Decision | Client | Accord ou refus (acte positif) | SCA obligatoire pour l'accord |
| 5. Activation | BANKO | Passage en statut `GRANTED` | Journalisation audit |
| 6. Utilisation | TPP | Acces aux APIs avec reference au consentement | Verification a chaque appel |
| 7. Revocation | Client | Revocation via dashboard ou API | Effet immediat, notification TPP |

---

## 3. Granularite des permissions

### 3.1 Catalogue des permissions

| Permission | Description | Scope API | Donnees accessibles | Duree max |
|---|---|---|---|---|
| `account_balance` | Consultation des soldes | `GET /accounts/{id}/balances` | Solde courant, solde disponible, devise | 180 jours |
| `account_details` | Informations du compte | `GET /accounts/{id}` | IBAN, type, devise, statut, titulaire | 180 jours |
| `transaction_history` | Historique des operations | `GET /accounts/{id}/transactions` | Transactions des 24 derniers mois max | 90 jours |
| `payment_initiation` | Initiation de virement | `POST /payments/sepa-credit-transfers` | Execution d'un paiement pour le compte du client | Unique (par operation) |
| `payment_status` | Suivi d'un paiement initie | `GET /payments/{id}/status` | Statut, date execution, motif rejet | 30 jours |
| `identity_verification` | Verification d'identite | `GET /identity/verify` | Nom, prenom, date de naissance | Unique |
| `credit_info` | Informations de credit | `GET /credits/{id}` | Encours, echeancier, taux | 90 jours |
| `fx_rates` | Taux de change | `GET /fx/rates` | Taux de change en vigueur | 24 heures |
| `beneficiary_list` | Liste des beneficiaires | `GET /accounts/{id}/beneficiaries` | Beneficiaires enregistres | 90 jours |
| `standing_orders` | Ordres permanents | `GET /accounts/{id}/standing-orders` | Virements recurrents parametres | 90 jours |

### 3.2 Regroupements de permissions

Pour simplifier l'experience utilisateur tout en maintenant la granularite, des groupements predifinis sont proposes :

| Groupe | Permissions incluses | Cas d'usage typique |
|---|---|---|
| **Agregation basique** | `account_balance`, `account_details` | Application de suivi budgetaire |
| **Agregation complete** | `account_balance`, `account_details`, `transaction_history`, `beneficiary_list`, `standing_orders` | Agregateur bancaire complet |
| **Initiation paiement** | `payment_initiation`, `payment_status`, `account_balance` | Application de paiement |
| **Scoring credit** | `account_balance`, `transaction_history`, `credit_info` | Organisme de credit alternatif |
| **Verification identite** | `identity_verification` | Service KYC tiers |

### 3.3 Restrictions par type de TPP

| Type de TPP | Permissions autorisees | Permissions interdites |
|---|---|---|
| **AISP** (Information) | `account_balance`, `account_details`, `transaction_history`, `beneficiary_list`, `standing_orders` | `payment_initiation`, `credit_info` |
| **PISP** (Paiement) | `payment_initiation`, `payment_status`, `account_balance` | `transaction_history`, `credit_info` |
| **CISP** (Carte) | `account_balance`, `identity_verification` | `transaction_history`, `payment_initiation` |
| **FISP** (Information financiere) | Toutes les permissions dans leur perimetre de licence | Selon licence specifique |

---

## 4. Dashboard client

### 4.1 Exigences fonctionnelles

Le dashboard client est une composante obligatoire du dispositif Open Banking, conformement aux exigences PSD3 et a la loi tunisienne de 2025. Il est implemente dans le frontend BANKO avec Astro et les composants interactifs Svelte.

| Fonctionnalite | Description | Priorite |
|---|---|---|
| **Liste des TPP** | Affichage de tous les TPP ayant un acces actif ou historique | Critique |
| **Detail par TPP** | Permissions accordees, date d'octroi, date d'expiration | Critique |
| **Historique des acces** | Journal des acces aux donnees par chaque TPP | Critique |
| **Revocation en un clic** | Bouton de revocation par TPP, avec confirmation | Critique |
| **Revocation selective** | Possibilite de revoquer des permissions individuelles | Haute |
| **Historique des consentements** | Consentements passes (revoques, expires, refuses) | Haute |
| **Notifications** | Alertes lors de nouveaux acces ou demandes de consentement | Moyenne |
| **Export donnees** | Export de l'historique des consentements (JSON, PDF) | Moyenne |
| **Filtres et recherche** | Filtrer par TPP, par statut, par periode | Moyenne |

### 4.2 Interface utilisateur -- Specifications

**Vue liste des consentements actifs** :

| Colonne | Contenu | Actions |
|---|---|---|
| TPP | Logo + Nom commercial du TPP | Clic pour detail |
| Permissions | Icones representant les permissions accordees | -- |
| Comptes | Nombre de comptes concernes | -- |
| Accorde le | Date d'octroi du consentement | -- |
| Expire le | Date d'expiration prevue | -- |
| Dernier acces | Date et heure du dernier acces effectif | -- |
| Actions | Bouton "Revoquer" (rouge) + "Detail" | Revocation, consultation |

**Vue detail d'un consentement** :

| Section | Contenu |
|---|---|
| Informations TPP | Nom, numero d'agrement, site web, contact DPO du TPP |
| Permissions detaillees | Liste des permissions avec description en langage clair |
| Comptes concernes | Liste des comptes avec IBAN masque partiellement |
| Historique d'acces | Tableau chronologique des acces (date, heure, endpoint, volume) |
| Actions | Revoquer tout, Revoquer par permission, Contacter le TPP |

### 4.3 Exigences d'accessibilite

| Critere | Norme | Exigence |
|---|---|---|
| Contraste | WCAG 2.1 AA | Ratio minimum 4.5:1 pour le texte |
| Navigation clavier | WCAG 2.1 AA | Toutes les actions accessibles au clavier |
| Lecteur d'ecran | WCAG 2.1 AA | Labels ARIA sur tous les elements interactifs |
| Langue | Localisation | Francais et arabe (RTL) |
| Mobile | Responsive | Fonctionnel sur ecrans >= 320px |

---

## 5. Stockage et audit

### 5.1 Journal immutable des consentements (Append-Only Log)

Conformement a la loi tunisienne de 2025 (preuve du consentement) et aux exigences LBC/FT (tracabilite), le journal des consentements est concu comme un registre en ajout seul (append-only).

| Propriete | Specification |
|---|---|
| **Type de stockage** | Table PostgreSQL avec politique `INSERT ONLY` (pas de `UPDATE` ni `DELETE`) |
| **Integrite** | Hash SHA-256 chainant chaque entree a la precedente |
| **Horodatage** | Timestamp UTC avec precision microseconde |
| **Non-repudiation** | Signature numerique de chaque entree par le serveur |
| **Redondance** | Replication synchrone sur le replica PostgreSQL |

### 5.2 Structure du journal d'audit

```
ConsentAuditEntry {
    id:              BIGSERIAL,      -- Identifiant sequentiel
    consent_id:      UUID,           -- Reference au ConsentRecord
    event_type:      AuditEventType, -- Type d'evenement
    actor_type:      ActorType,      -- Client, TPP, Systeme, Administrateur
    actor_id:        UUID,           -- Identifiant de l'acteur
    timestamp:       DateTime,       -- Horodatage UTC
    ip_address:      String,         -- Adresse IP source
    details:         JSONB,          -- Donnees contextuelles
    previous_hash:   String,         -- Hash de l'entree precedente
    entry_hash:      String,         -- Hash de cette entree
}
```

### 5.3 Types d'evenements audites

| Evenement | Description | Acteur typique | Donnees enregistrees |
|---|---|---|---|
| `CONSENT_REQUESTED` | Demande de consentement recue | TPP | Permissions demandees, comptes |
| `CONSENT_VIEWED` | Client a consulte la demande | Client | -- |
| `CONSENT_GRANTED` | Consentement accorde | Client | Permissions accordees, methode SCA |
| `CONSENT_REJECTED` | Consentement refuse | Client | Motif de refus (optionnel) |
| `CONSENT_REVOKED` | Consentement revoque | Client / Banque | Motif de revocation |
| `CONSENT_EXPIRED` | Consentement expire automatiquement | Systeme | -- |
| `CONSENT_SUSPENDED` | Consentement suspendu | Banque | Motif de suspension |
| `CONSENT_REACTIVATED` | Consentement reactive apres suspension | Banque | -- |
| `DATA_ACCESSED` | Donnees accedees par le TPP | TPP | Endpoint, volume de donnees |
| `PERMISSION_MODIFIED` | Permission individuelle modifiee | Client | Permission ajoutee/retiree |
| `CONSENT_RENEWED` | Consentement renouvele avant expiration | Client | Nouvelle date d'expiration |

### 5.4 Durees de conservation

| Type de donnee | Duree de conservation | Base legale | Action a expiration |
|---|---|---|---|
| ConsentRecord (actif) | Duree du consentement + 5 ans | Loi 2025, Art. 15 | Archivage |
| ConsentRecord (revoque/expire) | 10 ans apres revocation/expiration | LBC/FT (Circ. BCT 2025-17) | Suppression securisee |
| ConsentAuditEntry | 10 ans | LBC/FT + Loi 2025 | Archivage froid |
| Donnees d'acces detaillees | 5 ans | Loi 2025, Art. 15 | Anonymisation |
| Logs techniques (IP, User-Agent) | 1 an | Loi 2025, Art. 12 | Suppression |

---

## 6. API de consentement

### 6.1 Endpoints REST

| Endpoint | Methode | Description | Auth requise | SCA requise |
|---|---|---|---|---|
| `/api/v1/consents` | `POST` | Creer une demande de consentement | OAuth 2.0 (TPP) | Non (demande seulement) |
| `/api/v1/consents/{id}` | `GET` | Consulter un consentement | OAuth 2.0 (TPP ou Client) | Non |
| `/api/v1/consents/{id}` | `PUT` | Accorder ou modifier un consentement | OAuth 2.0 (Client) | Oui |
| `/api/v1/consents/{id}` | `DELETE` | Revoquer un consentement | OAuth 2.0 (TPP ou Client) | Non (Client) / Oui (TPP) |
| `/api/v1/consents` | `GET` | Lister les consentements du client | OAuth 2.0 (Client) | Non |
| `/api/v1/consents/{id}/audit` | `GET` | Historique d'audit d'un consentement | OAuth 2.0 (Client) | Non |
| `/api/v1/consents/{id}/renew` | `POST` | Renouveler un consentement | OAuth 2.0 (Client) | Oui |
| `/api/v1/consents/{id}/suspend` | `POST` | Suspendre un consentement | OAuth 2.0 (Banque) | Oui (Admin SCA) |

### 6.2 Requete de creation de consentement -- POST /api/v1/consents

**Corps de la requete (JSON)** :

```json
{
  "tpp_id": "uuid-du-tpp",
  "permissions": ["account_balance", "transaction_history"],
  "accounts": ["uuid-compte-1", "uuid-compte-2"],
  "purpose": "Agregation de comptes pour suivi budgetaire",
  "valid_until": "2026-10-06T23:59:59Z",
  "frequency_per_day": 4,
  "recurring_indicator": true
}
```

**Reponse (201 Created)** :

```json
{
  "consent_id": "uuid-du-consentement",
  "status": "REQUESTED",
  "permissions": ["account_balance", "transaction_history"],
  "accounts": ["uuid-compte-1", "uuid-compte-2"],
  "valid_until": "2026-10-06T23:59:59Z",
  "authorization_url": "https://banko.example/consent/authorize?id=uuid",
  "_links": {
    "self": "/api/v1/consents/uuid-du-consentement",
    "authorize": "https://banko.example/consent/authorize?id=uuid"
  }
}
```

### 6.3 Reponse d'erreur (RFC 7807)

```json
{
  "type": "https://banko.example/errors/consent-invalid-permissions",
  "title": "Permissions non autorisees",
  "status": 403,
  "detail": "Le TPP de type AISP ne peut pas demander la permission payment_initiation",
  "instance": "/api/v1/consents/uuid",
  "invalid_permissions": ["payment_initiation"]
}
```

### 6.4 Webhooks de notification

| Evenement webhook | Destinataire | Payload |
|---|---|---|
| `consent.granted` | TPP | `consent_id`, `permissions`, `expires_at` |
| `consent.revoked` | TPP | `consent_id`, `revoked_at`, `reason` |
| `consent.expired` | TPP | `consent_id`, `expired_at` |
| `consent.suspended` | TPP | `consent_id`, `suspended_at`, `reason` |

---

## 7. Conformite RGPD / Loi 2025 mapping

### 7.1 Correspondance des principes

| Principe RGPD | Article Loi 2025 | Implementation BANKO |
|---|---|---|
| Licite, loyaute, transparence | Art. 5, Art. 8 | Ecran d'information clair avant consentement, langage comprehensible |
| Limitation des finalites | Art. 6 | Champ `purpose` obligatoire dans ConsentRecord |
| Minimisation des donnees | Art. 9 | Permissions granulaires, pas d'acces au-dela du necessaire |
| Exactitude | Art. 10 | Donnees a jour via APIs temps reel |
| Limitation de conservation | Art. 12, Art. 15 | Durees de conservation definies (voir section 5.4) |
| Integrite et confidentialite | Art. 14 | Chiffrement TLS 1.3, acces controle, audit |
| Responsabilite | Art. 4, Art. 35 | DPO designe, DPIA realisee, registre des traitements |

### 7.2 Droits des personnes concernees

| Droit | Article Loi 2025 | Implementation dans le consentement |
|---|---|---|
| Droit d'acces | Art. 16 | Dashboard client : consultation de tous les consentements |
| Droit de rectification | Art. 17 | Modification des comptes concernes par un consentement |
| Droit a l'effacement | Art. 18 | Revocation + suppression des donnees partagees (demande au TPP) |
| Droit a la portabilite | Art. 20 | Export des consentements et historique (JSON, PDF) |
| Droit d'opposition | Art. 19 | Revocation en un clic via le dashboard |
| Droit a la limitation | Art. 18-2 | Suspension d'un consentement sans revocation |
| Notification violation | Art. 43 | Notification 72h si violation affectant des donnees partagees |

### 7.3 Mesures techniques et organisationnelles

| Mesure | Description | Responsable |
|---|---|---|
| Chiffrement au repos | Donnees de consentement chiffrees dans PostgreSQL (AES-256) | DBA |
| Chiffrement en transit | TLS 1.3 obligatoire pour toutes les communications | Infrastructure |
| Controle d'acces | RBAC strict : seul le client peut accorder/revoquer | Identity BC |
| Pseudonymisation | Les TPP n'accedent qu'a des identifiants opaques | Consent Service |
| Journalisation | Append-only audit log (section 5) | Consent Service |
| Tests de penetration | Tests ANCS obligatoires (Circ. BCT 2025-06) | Securite |
| Formation | Formation DPO et equipes sur la gestion du consentement | RH |
| DPIA | Etude d'impact avant mise en production du service de consentement | DPO |

### 7.4 Procedures en cas de violation

| Etape | Delai | Action | Responsable |
|---|---|---|---|
| Detection | T+0 | Alerte automatique via monitoring | Equipe SRE |
| Evaluation | T+4h max | Evaluation de la gravite et du perimetre | RSSI + DPO |
| Notification autorite | T+72h max | Notification a l'INPDP (autorite tunisienne) | DPO |
| Notification clients | Sans delai injustifie | Notification des clients si risque eleve | DPO + Communication |
| Notification TPP | T+24h max | Notification aux TPP concernes | Equipe Open Banking |
| Remediation | Variable | Correction de la vulnerabilite, revocation des acces compromis | Equipe technique |
| Rapport post-incident | T+30j max | Analyse des causes et mesures correctives | RSSI + DPO |

---

## Annexe A -- Diagramme d'etats du consentement

```
                    +-------------+
                    |  REQUESTED  |
                    +------+------+
                           |
              +------------+------------+
              |            |            |
              v            v            v
        +---------+  +---------+  +---------+
        | GRANTED |  | REJECTED|  | EXPIRED |
        +----+----+  +---------+  +---------+
             |
             v
        +---------+
        |  ACTIVE |<-----------+
        +----+----+            |
             |                 |
      +------+------+    +----+--------+
      |             |    | REACTIVATED |
      v             v    +-------------+
+---------+  +-----------+      ^
| REVOKED |  | SUSPENDED |------+
+---------+  +-----------+
```

---

## Annexe B -- References

| Reference | Source |
|---|---|
| Loi organique 2025-xx sur la protection des donnees | JORT, juin 2025 |
| PSD3/PSR accord provisoire | Conseil de l'UE, 27 novembre 2025 |
| FIDA proposition de reglement | Commission europeenne, COM(2023) 360 |
| Circulaire BCT 2025-17 (LBC/FT) | BCT, 2025 |
| NextGenPSD2 Consent Model | Berlin Group, v1.3.12 |
| OAuth 2.0 (RFC 6749) | IETF |
| RFC 7807 Problem Details for HTTP APIs | IETF |
| WCAG 2.1 | W3C |

---

*Document precedent : [01 -- Feuille de route Open Banking](./01-readiness-roadmap.md)*
*Document suivant : [03 -- Authentification forte (SCA)](./03-sca-strong-customer-authentication.md)*
