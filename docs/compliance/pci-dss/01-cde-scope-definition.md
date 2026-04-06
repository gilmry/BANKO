# Définition du Périmètre CDE (Cardholder Data Environment)

| Propriété        | Valeur                                           |
|------------------|--------------------------------------------------|
| **Version**      | 1.0.0                                            |
| **Date**         | 6 avril 2026                                     |
| **Référentiel**  | PCI DSS v4.0.1 (juin 2024)                       |
| **Classification**| Confidentiel -- Usage interne et auditeurs       |
| **Auteur**       | Équipe Sécurité BANKO                            |
| **Approbateur**  | RSSI / Comité Sécurité                           |
| **Prochaine revue** | 6 avril 2027                                  |

---

## Table des matières

1. [Objectif et contexte PCI DSS v4.0.1](#1-objectif-et-contexte-pci-dss-v401)
2. [Définition du CDE pour BANKO](#2-définition-du-cde-pour-banko)
3. [Diagramme des flux de données cartes](#3-diagramme-des-flux-de-données-cartes)
4. [Inventaire des composants in-scope](#4-inventaire-des-composants-in-scope)
5. [Composants hors-scope](#5-composants-hors-scope)
6. [Segmentation réseau](#6-segmentation-réseau)
7. [Stratégie de réduction du périmètre](#7-stratégie-de-réduction-du-périmètre)
8. [Revue annuelle du périmètre](#8-revue-annuelle-du-périmètre)

---

## 1. Objectif et contexte PCI DSS v4.0.1

### 1.1 Objectif du document

Le présent document définit formellement le périmètre du **Cardholder Data Environment (CDE)** de la plateforme BANKO. Il identifie l'ensemble des composants système, réseaux et processus qui stockent, traitent ou transmettent des données de titulaires de cartes (CHD) ou des données d'authentification sensibles (SAD).

Cette définition de périmètre constitue la **pierre angulaire** de toute démarche de conformité PCI DSS. Une délimitation inexacte entraîne soit un périmètre trop large (surcoût opérationnel), soit un périmètre trop restreint (non-conformité et risque de compromission).

### 1.2 Contexte réglementaire

Le référentiel PCI DSS v4.0.1, publié en **juin 2024**, constitue un errata de la version 4.0 (mars 2022). Il n'existe pas de version 4.1 annoncée à ce jour.

**Échéance critique** : depuis le **31 mars 2025**, toutes les exigences précédemment classées « future-dated » sont devenues **obligatoires**. Cela inclut notamment :

| Exigence | Objet | Impact BANKO |
|----------|-------|--------------|
| 3.5.1.2  | Chiffrement au niveau champ (pas uniquement disque) | PostgreSQL -- tables Payment |
| 6.4.3    | Gestion des scripts sur les pages de paiement | Frontend Astro/Svelte |
| 8.4.2    | MFA pour tout accès au CDE | Tous les opérateurs |
| 11.6.1   | Détection de modification/altération des pages de paiement | Frontend + monitoring |
| 5.4.1    | Mécanismes anti-hameçonnage | Formation + filtrage e-mail |
| 12.3.1   | Analyses de risques ciblées | Gouvernance |

### 1.3 Contexte tunisien

Dans le contexte du **Système Monétique Tunisien (SMT)**, la croissance des paiements par carte et l'essor des solutions de paiement mobile (OFT, Walletii, Kashy) renforcent la nécessité d'une conformité PCI DSS rigoureuse pour les établissements bancaires déployant BANKO.

---

## 2. Définition du CDE pour BANKO

### 2.1 Données de titulaires de cartes (CHD -- Cardholder Data)

Les données de titulaires de cartes sont classées selon le tableau suivant, conformément au glossaire PCI DSS v4.0.1 :

| Élément de donnée | Stockage autorisé | Protection requise | Présence dans BANKO |
|--------------------|--------------------|--------------------|---------------------|
| PAN (Primary Account Number) | Oui (si protégé) | Chiffrement, troncature, tokenisation ou hachage | Tokenisé -- PAN clair jamais stocké |
| Nom du titulaire | Oui | Protection si stocké avec le PAN | Module Customer (lié au compte, pas à la carte) |
| Date d'expiration | Oui | Protection si stocké avec le PAN | Non stocké dans BANKO (délégué au PSP) |
| Code de service | Oui | Protection si stocké avec le PAN | Non stocké dans BANKO |

### 2.2 Données d'authentification sensibles (SAD -- Sensitive Authentication Data)

Les données d'authentification sensibles ne doivent **jamais** être stockées après autorisation, même chiffrées :

| Élément de donnée | Stockage post-autorisation | Présence dans BANKO |
|--------------------|---------------------------|---------------------|
| Données de piste magnétique (Track 1/2) | **Interdit** | Jamais présent |
| CAV2 / CVC2 / CVV2 / CID | **Interdit** | Transit uniquement vers PSP, jamais stocké |
| PIN / Bloc PIN | **Interdit** | Jamais présent (saisie côté terminal/PSP) |

### 2.3 Identification des composants système concernés

Un composant système est **in-scope** s'il remplit au moins l'un des critères suivants :

1. **Stocke** des données CHD ou SAD (même temporairement en mémoire)
2. **Traite** des données CHD ou SAD (transformation, validation, routage)
3. **Transmet** des données CHD ou SAD (canal réseau)
4. **Se connecte** à un système qui stocke, traite ou transmet des CHD/SAD
5. **Fournit des services de sécurité** au CDE (authentification, logging, pare-feu)
6. **Pourrait impacter la sécurité** du CDE en cas de compromission (segmentation)

---

## 3. Diagramme des flux de données cartes

### 3.1 Flux principal -- Paiement par carte via PSP externe

```
                          PÉRIMÈTRE CDE BANKO
    ┌─────────────────────────────────────────────────────────────┐
    │                                                             │
    │  ┌──────────┐    TLS 1.3     ┌──────────────┐              │
    │  │ Navigateur├──────────────►│   Traefik     │              │
    │  │ Client    │               │  (TLS Term.)  │              │
    │  └──────────┘               └──────┬───────┘              │
    │                                     │ HTTP interne          │
    │                                     ▼                       │
    │                             ┌──────────────┐               │
    │                             │  Actix-web   │               │
    │                             │  Payment BC  │               │
    │                             │  (Handler)   │               │
    │                             └──────┬───────┘              │
    │                                     │                       │
    │                    ┌────────────────┼────────────────┐      │
    │                    │                │                │      │
    │                    ▼                ▼                ▼      │
    │            ┌──────────────┐ ┌────────────┐ ┌─────────────┐ │
    │            │ PostgreSQL   │ │ Vault de   │ │ PSP Externe │ │
    │            │ (tokenisé)  │ │ Tokens     │ │ (HTTPS/mTLS)│ │
    │            │ Payment DB   │ │            │ │             │ │
    │            └──────────────┘ └────────────┘ └─────────────┘ │
    │                                                             │
    └─────────────────────────────────────────────────────────────┘

    Légende :
    ─────►  Flux de données chiffrées (TLS 1.3 / mTLS)
    PAN clair : JAMAIS stocké -- tokenisé avant persistance
    SAD (CVV, PIN) : JAMAIS stocké -- transit uniquement vers PSP
```

### 3.2 Flux secondaire -- Consultation historique de transactions

```
    ┌──────────┐    TLS 1.3     ┌──────────┐         ┌──────────────┐
    │ Opérateur├──────────────►│ Traefik  ├────────►│  Actix-web   │
    │ Banque   │  + MFA (8.4.2)│          │         │  Payment BC  │
    └──────────┘               └──────────┘         └──────┬───────┘
                                                            │
                                                            ▼
                                                    ┌──────────────┐
                                                    │ PostgreSQL   │
                                                    │ PAN tronqué  │
                                                    │ (6 premiers/ │
                                                    │  4 derniers) │
                                                    └──────────────┘

    Note : L'opérateur ne voit JAMAIS le PAN complet.
    Affichage : 4532 76** **** 1234
```

---

## 4. Inventaire des composants in-scope

### 4.1 Tableau d'inventaire

| # | Composant | Rôle | Stocke CHD ? | Traite CHD ? | Transmet CHD ? | In-scope ? | Justification |
|---|-----------|------|:------------:|:------------:|:--------------:|:----------:|---------------|
| 1 | **Traefik** (reverse proxy) | Terminaison TLS 1.3, routage HTTP | Non | Non | **Oui** (transit chiffré) | **Oui** | Transmet les requêtes contenant potentiellement des données CHD |
| 2 | **Actix-web -- Payment Handler** | Traitement des requêtes de paiement | Non (mémoire transitoire) | **Oui** | **Oui** | **Oui** | Traite le PAN pour tokenisation et routage vers le PSP |
| 3 | **PostgreSQL -- tables payment** | Persistance des transactions | **Oui** (tokens uniquement) | Non | Non | **Oui** | Stocke les tokens de PAN et métadonnées de transaction |
| 4 | **Vault de tokens** | Mapping token ↔ PAN | **Oui** (PAN chiffré AES-256) | **Oui** | Non | **Oui** | Composant critique du CDE -- stocke la correspondance |
| 5 | **Frontend Astro/Svelte** (pages paiement) | Saisie des données carte | Non | **Oui** (transit JS) | **Oui** | **Oui** | Exigences 6.4.3 et 11.6.1 sur les pages de paiement |
| 6 | **MinIO** (stockage objets) | Stockage de documents | Non | Non | Non | **Non** (\*) | Ne stocke aucune donnée CHD -- voir section 5 |
| 7 | **Docker Engine / K8s** | Orchestration des conteneurs | Non | Non | Non | **Oui** | Fournit l'environnement d'exécution du CDE |
| 8 | **Réseau Docker / K8s namespace** | Segmentation réseau | Non | Non | **Oui** (réseau) | **Oui** | Canal de communication entre composants CDE |
| 9 | **Système de logging** (ELK/Prometheus) | Collecte et analyse des logs | Non (\*\*) | Non | Non | **Oui** | Fournit des services de sécurité au CDE (Req. 10) |
| 10 | **Serveur de sauvegarde** | Backup des bases de données | **Oui** (tokens chiffrés) | Non | **Oui** | **Oui** | Contient des copies des données tokenisées |

(\*) MinIO est hors-scope sous condition de validation annuelle -- voir [section 5](#5-composants-hors-scope).

(\*\*) Les logs ne doivent **jamais** contenir de PAN complet (Req. 3.4.1). Des mécanismes de masquage sont implémentés.

### 4.2 Composants réseau in-scope

| Composant réseau | Type | Fonction | In-scope ? |
|------------------|------|----------|:----------:|
| Pare-feu périmétrique | Firewall L3/L4 | Filtrage trafic entrant/sortant | **Oui** |
| Réseau `payment-net` (Docker) | Réseau overlay isolé | Communication Payment BC ↔ PostgreSQL | **Oui** |
| Réseau `frontend-net` (Docker) | Réseau overlay | Communication Traefik ↔ Frontend | **Oui** |
| Load balancer externe | L7 (si applicable) | Distribution de charge | **Oui** |
| DNS interne | Service DNS | Résolution de noms dans le CDE | **Oui** |

---

## 5. Composants hors-scope

### 5.1 Tableau des composants hors-scope

| # | Composant | Justification de l'exclusion | Contrôle de segmentation |
|---|-----------|------------------------------|--------------------------|
| 1 | **MinIO** (stockage objets) | Ne stocke aucune donnée CHD ; contient uniquement des documents administratifs (KYC, justificatifs) | Réseau isolé `storage-net`, aucune route vers `payment-net` |
| 2 | **Actix-web -- Customer Handler** | Bounded Context Customer : gère l'identité client, pas les données carte | Pas d'accès aux tables `payment_*` de PostgreSQL |
| 3 | **Actix-web -- Account Handler** | Bounded Context Account : gestion des comptes courants/épargne | Pas de traitement de données carte |
| 4 | **Actix-web -- Credit Handler** | Bounded Context Credit : octroi et suivi de prêts | Aucun lien avec les flux monétiques |
| 5 | **Actix-web -- AML Handler** | Bounded Context AML : détection blanchiment | Reçoit uniquement des métadonnées anonymisées de transactions |
| 6 | **Actix-web -- Reporting Handler** | Bounded Context Reporting : rapports réglementaires | Accès uniquement aux données agrégées, PAN tronqués |
| 7 | **Actix-web -- Accounting Handler** | Bounded Context Accounting : écritures comptables | Montants et références uniquement, pas de CHD |
| 8 | **Postes de développement** | Aucun accès aux données de production | Environnements de dev isolés, données synthétiques |
| 9 | **CI/CD (GitHub Actions)** | Pipeline de build et test | Aucun accès au CDE de production |

### 5.2 Conditions de maintien hors-scope

Pour qu'un composant reste hors-scope, les conditions suivantes doivent être **continuellement** satisfaites :

1. Aucune donnée CHD ne transite par le composant, même temporairement
2. Le composant n'a pas de connectivité réseau directe avec le CDE
3. Le composant ne fournit pas de services de sécurité au CDE
4. La segmentation est vérifiée lors de chaque test de pénétration (Req. 11.4.5)

---

## 6. Segmentation réseau

### 6.1 Principes de segmentation

La segmentation réseau est le mécanisme principal de réduction du périmètre PCI DSS. BANKO utilise une stratégie de segmentation à plusieurs niveaux :

| Niveau | Technologie | Objectif |
|--------|-------------|----------|
| **L1 -- Infrastructure** | Pare-feu périmétrique | Isolation du CDE vis-à-vis d'Internet et du réseau corporate |
| **L2 -- Conteneurs** | Réseaux Docker / Namespaces K8s | Isolation entre bounded contexts |
| **L3 -- Application** | Middleware Actix-web + politiques RBAC | Contrôle d'accès applicatif aux endpoints sensibles |
| **L4 -- Données** | Chiffrement au niveau champ + tokenisation | Protection en profondeur même en cas de compromission réseau |

### 6.2 Architecture réseau Docker (développement)

```
    ┌─────────────────────────────────────────────────┐
    │                    Host Docker                    │
    │                                                   │
    │  ┌─────────────┐    ┌───────────────────────┐    │
    │  │ frontend-net│    │     payment-net        │    │
    │  │             │    │  (CDE -- ISOLÉ)         │    │
    │  │  Traefik ◄──┼────┤  Payment Handler      │    │
    │  │  Frontend   │    │  PostgreSQL (payment)  │    │
    │  └─────────────┘    │  Token Vault           │    │
    │                      └───────────────────────┘    │
    │  ┌─────────────┐                                  │
    │  │ backend-net │    ┌───────────────────────┐    │
    │  │             │    │     storage-net        │    │
    │  │  Customer   │    │  (HORS SCOPE)          │    │
    │  │  Account    │    │  MinIO                 │    │
    │  │  Credit     │    └───────────────────────┘    │
    │  │  AML        │                                  │
    │  │  ...        │                                  │
    │  └─────────────┘                                  │
    └─────────────────────────────────────────────────┘
```

### 6.3 Architecture réseau Kubernetes (production)

| Namespace | Bounded Contexts | Network Policy | Accès au CDE |
|-----------|-----------------|----------------|:------------:|
| `banko-payment` | Payment, ForeignExchange | Deny-all par défaut, allow-list explicite | **Oui** (CDE) |
| `banko-core` | Customer, Account, Credit | Deny-all par défaut | Non |
| `banko-compliance` | AML, Sanctions, Prudential | Deny-all par défaut | Non |
| `banko-operations` | Accounting, Reporting | Deny-all par défaut | Non |
| `banko-security` | Governance, Identity | Allow vers tous (services de sécurité) | **Oui** (sécurité CDE) |
| `banko-infra` | Traefik, monitoring, logging | Politiques spécifiques par service | **Oui** (sécurité CDE) |

### 6.4 Règles de pare-feu inter-segments

| Source | Destination | Port | Protocole | Action | Justification |
|--------|------------|------|-----------|--------|---------------|
| Internet | Traefik | 443 | HTTPS/TLS 1.3 | **Autoriser** | Point d'entrée unique |
| Internet | Tout autre | * | * | **Refuser** | Principe du moindre accès |
| Traefik | Payment Handler | 8080 | HTTP | **Autoriser** | Routage des requêtes paiement |
| Payment Handler | PostgreSQL (payment) | 5432 | PostgreSQL/TLS | **Autoriser** | Persistance des données |
| Payment Handler | PSP externe | 443 | HTTPS/mTLS | **Autoriser** | Communication avec le processeur |
| Payment Handler | Token Vault | 8200 | HTTPS | **Autoriser** | Tokenisation/dé-tokenisation |
| backend-net | payment-net | * | * | **Refuser** | Segmentation CDE |
| storage-net | payment-net | * | * | **Refuser** | Segmentation CDE |
| Monitoring | payment-net | 9090 | Prometheus | **Autoriser** | Collecte métriques (lecture seule) |

---

## 7. Stratégie de réduction du périmètre

### 7.1 Tokenisation (stratégie principale)

La tokenisation constitue le levier principal de réduction du périmètre CDE de BANKO. Le principe est de remplacer le PAN par un token irréversible dès la réception, de sorte que seul le vault de tokens et le handler de paiement manipulent le PAN clair.

| Aspect | Approche BANKO |
|--------|----------------|
| **Moment de tokenisation** | Dès réception par le Payment Handler, avant toute persistance |
| **Type de token** | Token aléatoire (pas de préservation de format) |
| **Stockage du mapping** | Vault dédié, chiffré AES-256-GCM, accès restreint |
| **Dé-tokenisation** | Uniquement pour communication vers le PSP, jamais pour affichage |
| **Impact périmètre** | Réduit les composants in-scope aux seuls Payment Handler + Vault |

Voir [03-tokenization-and-encryption-guide.md](./03-tokenization-and-encryption-guide.md) pour les détails techniques.

### 7.2 Externalisation PSP

Pour les banques déployant BANKO, l'externalisation du traitement carte vers un PSP certifié PCI DSS Level 1 est **fortement recommandée** :

| Modèle | Description | Impact périmètre |
|--------|-------------|------------------|
| **Redirection complète** | Le client est redirigé vers la page du PSP | Périmètre minimal (SAQ A) |
| **iFrame / hosted fields** | Champs de saisie carte hébergés par le PSP | Périmètre réduit (SAQ A-EP) |
| **API directe** | BANKO reçoit le PAN et le transmet au PSP | Périmètre complet (SAQ D / ROC) |

BANKO supporte les trois modèles. Le modèle **iFrame / hosted fields** est recommandé comme compromis entre expérience utilisateur et réduction du périmètre.

### 7.3 Point-to-Point Encryption (P2PE)

Pour les paiements en agence (terminaux physiques), l'utilisation de solutions **P2PE validées** par le PCI SSC permet d'exclure le terminal et le réseau de transport du périmètre.

---

## 8. Revue annuelle du périmètre

### 8.1 Fréquence et déclencheurs

La revue du périmètre CDE doit être effectuée :

| Déclencheur | Fréquence | Responsable |
|-------------|-----------|-------------|
| Revue planifiée | **Annuelle** (minimum) | RSSI + Équipe sécurité |
| Changement d'architecture | À chaque modification significative | Architecte sécurité |
| Ajout d'un bounded context | Avant mise en production | Équipe développement + RSSI |
| Changement de PSP ou d'hébergeur | Avant migration | RSSI + Direction IT |
| Suite à un incident de sécurité | Immédiatement après l'incident | Équipe réponse aux incidents |
| Résultats de tests de pénétration | Après chaque campagne de tests | Auditeur + RSSI |

### 8.2 Processus de revue

1. **Inventaire** : Mettre à jour la liste des composants système (section 4)
2. **Flux de données** : Revalider les diagrammes de flux (section 3)
3. **Segmentation** : Vérifier l'efficacité de la segmentation réseau via tests de pénétration
4. **Hors-scope** : Confirmer que les composants exclus respectent toujours les critères d'exclusion
5. **Documentation** : Mettre à jour le présent document avec la nouvelle version
6. **Approbation** : Faire valider par le RSSI et le comité sécurité

### 8.3 Registre des revues

| Date | Version | Périmètre modifié | Raison | Approuvé par |
|------|---------|-------------------|--------|--------------|
| 06/04/2026 | 1.0.0 | Définition initiale | Création du document | RSSI |
| _Prochaine revue prévue : 06/04/2027_ | | | | |

---

## Références

| Document | Lien |
|----------|------|
| PCI DSS v4.0.1 (juin 2024) | [PCI SSC Document Library](https://www.pcisecuritystandards.org/document_library/) |
| Guide de tokenisation -- BANKO | [03-tokenization-and-encryption-guide.md](./03-tokenization-and-encryption-guide.md) |
| Matrice des responsabilités | [04-responsibility-matrix.md](./04-responsibility-matrix.md) |
| Mapping des exigences PCI DSS | [02-requirements-mapping.md](./02-requirements-mapping.md) |
| Architecture BANKO | [CLAUDE.md](../../../CLAUDE.md) |
| Référentiel légal et normatif | [REFERENTIEL_LEGAL_ET_NORMATIF.md](../../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) |

---

*Document généré dans le cadre du programme de conformité PCI DSS de la plateforme BANKO. Toute modification doit suivre le processus de revue documentaire décrit en section 8.*
