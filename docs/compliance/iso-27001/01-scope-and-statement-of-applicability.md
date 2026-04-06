# BANKO — Périmètre du SMSI et Déclaration d'Applicabilité (SoA)

> **Version** : 1.0.0 — 6 avril 2026
> **Statut** : Document initial
> **Classification** : Confidentiel — Usage interne et auditeurs
> **Licence** : AGPL-3.0
> **Auteur** : Projet BANKO
> **Norme de référence** : ISO/IEC 27001:2022 (seule édition en vigueur depuis octobre 2025)
> **Amendement** : ISO/IEC 27001:2022/Amd 1:2024 (changement climatique)

---

## Table des matières

1. [Objectif du document](#1-objectif-du-document)
2. [Périmètre du SMSI BANKO](#2-périmètre-du-smsi-banko)
3. [Parties intéressées](#3-parties-intéressées)
4. [Évaluation du changement climatique](#4-évaluation-du-changement-climatique)
5. [Déclaration d'applicabilité (SoA)](#5-déclaration-dapplicabilité-soa)
6. [Exclusions justifiées](#6-exclusions-justifiées)
7. [Références](#7-références)

---

## 1. Objectif du document

Le présent document constitue la **Déclaration d'Applicabilité** (*Statement of Applicability* — SoA) du Système de Management de la Sécurité de l'Information (SMSI) de la plateforme BANKO. Il remplit les exigences suivantes de la norme ISO/IEC 27001:2022 :

- **Clause 4.3** : Détermination du périmètre du SMSI
- **Clause 4.2** : Compréhension des besoins et attentes des parties intéressées
- **Clause 6.1.3 d)** : Production d'une déclaration d'applicabilité incluant les contrôles nécessaires, leur justification d'inclusion et leur état d'implémentation
- **Amendement 1:2024** : Prise en compte des enjeux climatiques dans les clauses 4.1 et 4.2

Ce document est destiné à servir de référence pour :

- Les auditeurs internes et externes dans le cadre de la certification ISO 27001:2022
- La Banque Centrale de Tunisie (BCT) lors des inspections de conformité
- L'équipe de développement de BANKO pour le pilotage de l'implémentation des contrôles de sécurité
- Les établissements bancaires tunisiens déployant BANKO dans leur environnement de production

---

## 2. Périmètre du SMSI BANKO

### 2.1 Description générale

Le SMSI couvre l'intégralité de la plateforme bancaire open source BANKO, comprenant la conception, le développement, les tests, le déploiement et l'exploitation de l'ensemble des composants logiciels et infrastructurels.

### 2.2 Bounded Contexts (domaines métier)

L'architecture de BANKO repose sur les principes du Domain-Driven Design (DDD) et de l'architecture hexagonale. Le périmètre du SMSI englobe les **12 bounded contexts** suivants :

| N° | Bounded Context | Description | Données sensibles |
|---|---|---|---|
| 1 | **Customer** | Gestion des clients : onboarding, KYC (Know Your Customer), profil client, statut réglementaire | Données d'identité, pièces justificatives, données biométriques (e-KYC) |
| 2 | **Account** | Gestion des comptes bancaires : ouverture, clôture, solde, types de comptes (courant, épargne, DAT) | Numéros de compte, soldes, historique de mouvements |
| 3 | **Credit** | Octroi de crédit et gestion des prêts : scoring, décaissement, échéancier, recouvrement | Capacité d'endettement, revenus, garanties |
| 4 | **AML** | Anti-blanchiment d'argent : détection d'opérations suspectes, alertes, seuils réglementaires, déclarations CTAF | Profils de risque, alertes de soupçon, rapports d'investigation |
| 5 | **Sanctions** | Contrôle des sanctions internationales : filtrage des listes (ONU, UE, OFAC, listes nationales), gel des avoirs | Résultats de filtrage, correspondances, décisions de gel |
| 6 | **Prudential** | Exigences prudentielles : ratios de solvabilité (Bâle III/BCT), ratio Crédits/Dépôts, ratio de concentration | Données agrégées de risques, calculs de fonds propres |
| 7 | **Accounting** | Comptabilité générale : plan comptable bancaire, journaux, écritures comptables, rapprochement | Écritures comptables, balances, états financiers |
| 8 | **Reporting** | Rapports réglementaires : reporting BCT, CTAF, rapports statistiques et tableaux de bord | Rapports agrégés, indicateurs de performance |
| 9 | **Payment** | Virements et paiements : SEPA, SWIFT, paiements domestiques, compensation interbancaire | Ordres de virement, coordonnées bénéficiaires, montants |
| 10 | **ForeignExchange** | Opérations de change : taux de change, conversions, opérations en devises | Positions de change, contreparties |
| 11 | **Governance** | Gouvernance : rôles et permissions, piste d'audit, workflows d'approbation, séparation des pouvoirs | Matrices d'habilitations, journaux d'audit |
| 12 | **Identity** | Gestion des identités : authentification (MFA, JWT), biométrie, gestion des sessions | Credentials, tokens, données biométriques |

### 2.3 Périmètre technique et infrastructurel

| Composant | Technologie | Version | Rôle dans le SMSI |
|---|---|---|---|
| Backend API | Rust + Actix-web | 4.x | Traitement métier, enforcement des règles de sécurité |
| Base de données | PostgreSQL | 16 (Alpine) | Stockage persistant des données bancaires |
| Reverse Proxy | Traefik | Dernière stable | Routage HTTP/HTTPS, terminaison TLS, rate limiting |
| Stockage objet | MinIO | Dernière stable | Stockage S3-compatible (documents KYC, pièces justificatives) |
| Frontend | Astro 4.x + Svelte | 4.x | Interface utilisateur (SSG + islands architecture) |
| Conteneurisation | Docker + Docker Compose | Dernière stable | Environnement de développement et d'intégration |
| Orchestration (prod) | Kubernetes (K8s) | 1.28+ | Déploiement en production, haute disponibilité |
| Runtime asynchrone | Tokio | Dernière stable | Traitement concurrent des requêtes |
| ORM / requêtes | SQLx | Dernière stable | Requêtes typées à la compilation (prévention injection SQL) |
| Sérialisation | serde + serde_json | Dernière stable | Validation et transformation des données |

### 2.4 Périmètre organisationnel

Le SMSI couvre les activités suivantes :

- Développement du code source (contribution open source sous AGPL-3.0)
- Revue de code et intégration continue (CI/CD via GitHub Actions)
- Gestion des dépendances et audit de sécurité (`cargo audit`, `npm audit`)
- Documentation technique et réglementaire
- Tests de sécurité (unitaires, BDD, E2E, tests d'intrusion)
- Gestion des incidents de sécurité
- Gestion des accès et des habilitations

---

## 3. Parties intéressées

Conformément à la clause 4.2 de la norme ISO/IEC 27001:2022, les parties intéressées suivantes ont été identifiées, ainsi que leurs exigences pertinentes en matière de sécurité de l'information :

| Partie intéressée | Catégorie | Exigences et attentes | Référence réglementaire |
|---|---|---|---|
| **Banque Centrale de Tunisie (BCT)** | Régulateur principal | Conformité aux circulaires prudentielles, contrôle interne, reporting réglementaire, tests d'intrusion accrédités ANCS | Circulaires 2006-19, 2021-05, 2025-06 |
| **Commission Tunisienne des Analyses Financières (CTAF)** | Autorité LBC/FT | Déclarations de soupçon, filtrage sanctions, conservation données 10 ans | Loi 2015-26, Circulaire BCT 2025-17 |
| **Instance Nationale de Protection des Données Personnelles (INPDP)** | Autorité protection données | Conformité à la loi sur la protection des données personnelles, désignation DPO, notification 72h, chiffrement | Loi données personnelles 2025 (application juillet 2026) |
| **Clients des banques** | Utilisateurs finaux | Confidentialité des données personnelles et bancaires, disponibilité des services, intégrité des transactions | Code des obligations et contrats, Loi 2016-48 |
| **Évaluateurs GAFI / MENAFATF** | Organismes internationaux | Conformité aux 40 recommandations GAFI, dispositif LBC/FT efficace, supervision basée sur les risques | Recommandations GAFI, évaluation mutuelle prévue 2026-2027 |
| **Développeurs et contributeurs** | Communauté open source | Sécurité du processus de contribution, revue de code, absence de vulnérabilités dans le code source | CONTRIBUTING.md, SECURITY.md |
| **Banques utilisatrices de BANKO** | Clients institutionnels | Conformité réglementaire intégrée, auditabilité, haute disponibilité, SLA de sécurité | Exigences contractuelles, SLA |
| **Hébergeurs et prestataires cloud** | Fournisseurs | Conformité SLA, localisation des données, certifications (ISO 27001, SOC 2) | Contrats de prestation, clauses de sécurité |
| **Auditeurs externes** | Tiers de confiance | Accès aux preuves d'audit, traçabilité complète, documentation à jour | Normes ISA, ISO 19011 |
| **Agence Nationale de la Cybersécurité (ANCS)** | Autorité cybersécurité | Accréditation des prestataires de tests d'intrusion, conformité aux normes nationales | Circulaire BCT 2025-06 |

---

## 4. Évaluation du changement climatique

### 4.1 Contexte normatif

L'amendement ISO/IEC 27001:2022/Amd 1:2024, publié en février 2024, exige que les organismes déterminent si le changement climatique est un enjeu pertinent pour leur SMSI. Cette évaluation doit être intégrée aux analyses des clauses 4.1 (contexte de l'organisme) et 4.2 (parties intéressées).

### 4.2 Analyse des risques climatiques pour BANKO en contexte tunisien

La Tunisie est classée parmi les pays les plus vulnérables au changement climatique dans la région méditerranéenne. Les risques identifiés susceptibles d'affecter le SMSI de BANKO sont les suivants :

| Risque climatique | Description | Impact potentiel sur le SMSI | Niveau |
|---|---|---|---|
| **Désertification et stress hydrique** | Avancée du désert, raréfaction des ressources en eau dans les régions intérieures | Instabilité de l'alimentation électrique des centres de données, risque de coupures prolongées | Moyen |
| **Inondations côtières** | Montée du niveau de la mer affectant les zones côtières (Tunis, Sousse, Sfax) | Destruction physique d'infrastructures d'hébergement situées en zone littorale | Élevé |
| **Vagues de chaleur extrême** | Épisodes caniculaires dépassant 50 °C dans le sud, 45 °C dans le nord | Défaillance des systèmes de refroidissement des centres de données, dégradation des performances matérielles | Élevé |
| **Perturbations du réseau électrique** | Surcharge du réseau national lors des pics de consommation estivaux | Coupures d'alimentation des serveurs, corruption de données en cas d'arrêt brutal | Moyen |
| **Événements météorologiques extrêmes** | Tempêtes, inondations soudaines affectant les infrastructures de télécommunications | Interruption de la connectivité réseau, indisponibilité des services bancaires | Moyen |

### 4.3 Mesures d'atténuation prévues

| Mesure | Contrôle ISO associé | Responsable |
|---|---|---|
| Sélection d'hébergeurs disposant de plans de continuité intégrant les risques climatiques | A.5.30 (Préparation TIC pour la continuité d'activité) | RSSI |
| Réplication géographique des données hors zones à risque côtier | A.8.14 (Redondance des moyens de traitement de l'information) | Équipe infrastructure |
| Alimentation électrique de secours (UPS, générateurs) pour les sites critiques | A.7.11 (Services généraux) | Hébergeur / Exploitant |
| Surveillance de la température des salles serveurs avec alertes automatisées | A.7.13 (Maintenance du matériel) | Équipe exploitation |
| Intégration des scénarios climatiques dans les plans de reprise d'activité | A.5.29 (Sécurité de l'information durant une perturbation) | RSSI |

### 4.4 Conclusion de l'évaluation climatique

Le changement climatique constitue un enjeu pertinent pour le SMSI de BANKO, en particulier pour les déploiements en Tunisie. Les risques identifiés sont intégrés au registre des risques (voir [02-risk-assessment-register.md](02-risk-assessment-register.md)) et feront l'objet d'une revue annuelle.

---

## 5. Déclaration d'applicabilité (SoA)

### 5.1 Méthodologie

La présente déclaration d'applicabilité couvre les **93 contrôles** de l'Annexe A de la norme ISO/IEC 27001:2022, organisés en **4 thèmes** :

- **Contrôles organisationnels** (A.5) : 37 contrôles
- **Contrôles relatifs aux personnes** (A.6) : 8 contrôles
- **Contrôles physiques** (A.7) : 14 contrôles
- **Contrôles technologiques** (A.8) : 34 contrôles

Pour chaque contrôle, les colonnes suivantes sont renseignées :

| Colonne | Description |
|---|---|
| **ID** | Identifiant du contrôle selon l'Annexe A |
| **Contrôle** | Intitulé officiel du contrôle |
| **Applicable** | Oui / Non — Applicabilité au périmètre du SMSI |
| **Justification** | Raison de l'inclusion ou de l'exclusion |
| **Module BANKO** | Bounded context(s) ou composant(s) concerné(s) |
| **Statut** | Planned / In Progress / Done |

### 5.2 Contrôles organisationnels (A.5.1 — A.5.37)

| ID | Contrôle | Applicable | Justification | Module BANKO | Statut |
|---|---|---|---|---|---|
| A.5.1 | Politiques de sécurité de l'information | Oui | Fondation du SMSI, exigence BCT circulaire 2006-19 | Governance, tous modules | Planned |
| A.5.2 | Fonctions et responsabilités liées à la sécurité de l'information | Oui | Séparation des pouvoirs exigée par BCT 2021-05 | Governance | Planned |
| A.5.3 | Séparation des tâches | Oui | Principe fondamental de contrôle interne bancaire | Governance, Identity | Planned |
| A.5.4 | Responsabilités de la direction | Oui | Engagement de la direction requis par ISO 27001 clause 5.1 | Governance | Planned |
| A.5.5 | Relations avec les autorités | Oui | Relations avec BCT, CTAF, INPDP, ANCS | Governance, Reporting | Planned |
| A.5.6 | Relations avec les groupes d'intérêt spécialisés | Oui | Veille sécurité, partage d'informations sur les menaces | Governance | Planned |
| A.5.7 | Renseignement sur les menaces | Oui | **Nouveau contrôle 2022** — Essentiel pour le secteur bancaire | AML, Sanctions, Identity | Planned |
| A.5.8 | Sécurité de l'information dans la gestion de projet | Oui | Intégration sécurité dans le cycle de développement BANKO | Tous modules | Planned |
| A.5.9 | Inventaire des informations et autres actifs associés | Oui | Cartographie des actifs informationnels bancaires | Tous modules | Planned |
| A.5.10 | Utilisation correcte des informations et autres actifs associés | Oui | Politique d'utilisation acceptable des données bancaires | Tous modules | Planned |
| A.5.11 | Restitution des actifs | Oui | Procédure de restitution lors du départ d'un collaborateur | Governance, Identity | Planned |
| A.5.12 | Classification des informations | Oui | Classification des données bancaires (public, interne, confidentiel, secret) | Tous modules | Planned |
| A.5.13 | Étiquetage des informations | Oui | Marquage des documents et données selon classification | Tous modules | Planned |
| A.5.14 | Transfert des informations | Oui | Sécurisation des échanges (SWIFT, SEPA, API) | Payment, ForeignExchange | Planned |
| A.5.15 | Contrôle d'accès | Oui | RBAC et contrôle d'accès granulaire | Identity, Governance | Planned |
| A.5.16 | Gestion des identités | Oui | Cycle de vie des identités utilisateurs et systèmes | Identity | Planned |
| A.5.17 | Informations d'authentification | Oui | Gestion sécurisée des credentials (JWT, MFA) | Identity | Planned |
| A.5.18 | Droits d'accès | Oui | Attribution, revue et révocation des droits | Identity, Governance | Planned |
| A.5.19 | Sécurité de l'information dans les relations avec les fournisseurs | Oui | Sécurisation des intégrations tierces (prestataires cloud, API) | Tous modules | Planned |
| A.5.20 | Prise en compte de la sécurité de l'information dans les accords avec les fournisseurs | Oui | Clauses de sécurité dans les contrats d'hébergement et prestation | Infrastructure | Planned |
| A.5.21 | Gestion de la sécurité de l'information dans la chaîne d'approvisionnement TIC | Oui | Sécurité des dépendances (crates Rust, packages npm) | Tous modules | Planned |
| A.5.22 | Surveillance, revue et gestion des changements des services fournisseurs | Oui | Suivi des SLA et changements des prestataires | Infrastructure | Planned |
| A.5.23 | Sécurité de l'information pour l'utilisation de services en nuage | Oui | **Nouveau contrôle 2022** — Déploiement K8s cloud | Infrastructure, tous modules | Planned |
| A.5.24 | Planification et préparation de la gestion des incidents de sécurité de l'information | Oui | Procédure de gestion des incidents, exigence BCT | Governance, tous modules | Planned |
| A.5.25 | Évaluation et décision concernant les événements de sécurité de l'information | Oui | Triage et qualification des événements de sécurité | Governance, AML | Planned |
| A.5.26 | Réponse aux incidents de sécurité de l'information | Oui | Plan de réponse aux incidents, notification 72h (loi données 2025) | Governance, tous modules | Planned |
| A.5.27 | Enseignements tirés des incidents de sécurité de l'information | Oui | Retour d'expérience et amélioration continue | Governance | Planned |
| A.5.28 | Collecte de preuves | Oui | Préservation de la chaîne de preuve pour investigations | Governance, AML | Planned |
| A.5.29 | Sécurité de l'information durant une perturbation | Oui | Continuité de la sécurité lors des incidents | Tous modules | Planned |
| A.5.30 | Préparation des TIC pour la continuité d'activité | Oui | **Nouveau contrôle 2022** — PCA/PRA pour services bancaires critiques | Tous modules | Planned |
| A.5.31 | Exigences légales, statutaires, réglementaires et contractuelles | Oui | Conformité BCT, CTAF, INPDP, loi 2016-48, loi données 2025 | Tous modules | Planned |
| A.5.32 | Droits de propriété intellectuelle | Oui | Licence AGPL-3.0, gestion des dépendances open source | Tous modules | Planned |
| A.5.33 | Protection des enregistrements | Oui | Conservation des données 10 ans (LBC/FT), archivage légal | Tous modules | Planned |
| A.5.34 | Vie privée et protection des données à caractère personnel | Oui | Conformité loi données personnelles 2025, DPO, DPIA | Customer, Identity, tous modules | Planned |
| A.5.35 | Revue indépendante de la sécurité de l'information | Oui | Audit interne et externe, tests d'intrusion ANCS | Governance | Planned |
| A.5.36 | Conformité aux politiques, règles et normes de sécurité de l'information | Oui | Vérification de conformité continue | Governance | Planned |
| A.5.37 | Procédures d'exploitation documentées | Oui | Documentation des procédures d'exploitation (runbooks) | Infrastructure, tous modules | Planned |

### 5.3 Contrôles relatifs aux personnes (A.6.1 — A.6.8)

| ID | Contrôle | Applicable | Justification | Module BANKO | Statut |
|---|---|---|---|---|---|
| A.6.1 | Présélection | Oui | Vérification des antécédents des contributeurs ayant accès aux données sensibles | Governance | Planned |
| A.6.2 | Termes et conditions d'emploi | Oui | Clauses de confidentialité et de sécurité dans les contrats | Governance | Planned |
| A.6.3 | Sensibilisation, enseignement et formation à la sécurité de l'information | Oui | Formation obligatoire en sécurité pour l'équipe de développement | Governance | Planned |
| A.6.4 | Processus disciplinaire | Oui | Sanctions en cas de violation de la politique de sécurité | Governance | Planned |
| A.6.5 | Responsabilités après la fin ou le changement d'emploi | Oui | Révocation des accès, restitution des actifs | Identity, Governance | Planned |
| A.6.6 | Accords de confidentialité ou de non-divulgation | Oui | NDA pour les contributeurs ayant accès aux données bancaires | Governance | Planned |
| A.6.7 | Travail à distance | Oui | Sécurisation du développement à distance (VPN, MFA) | Identity, Infrastructure | Planned |
| A.6.8 | Signalement des événements de sécurité de l'information | Oui | Canal de signalement des vulnérabilités (SECURITY.md) | Governance | Planned |

### 5.4 Contrôles physiques (A.7.1 — A.7.14)

| ID | Contrôle | Applicable | Justification | Module BANKO | Statut |
|---|---|---|---|---|---|
| A.7.1 | Périmètres de sécurité physique | Oui | Délégué à l'hébergeur ; exigences contractuelles | Infrastructure (hébergeur) | Planned |
| A.7.2 | Contrôles physiques des accès | Oui | Délégué à l'hébergeur ; audit des contrôles d'accès | Infrastructure (hébergeur) | Planned |
| A.7.3 | Sécurisation des bureaux, des salles et des équipements | Oui | Délégué à l'hébergeur pour les centres de données | Infrastructure (hébergeur) | Planned |
| A.7.4 | Surveillance de la sécurité physique | **Non** | **Nouveau contrôle 2022** — Déploiement cloud uniquement, pas de locaux propres à surveiller. Délégué intégralement à l'hébergeur certifié. | N/A | N/A |
| A.7.5 | Protection contre les menaces physiques et environnementales | Oui | Protection contre incendie, inondation, surchauffe — risques climatiques (cf. section 4) | Infrastructure (hébergeur) | Planned |
| A.7.6 | Travail dans les zones sécurisées | Oui | Procédures d'accès aux salles serveurs — délégué à l'hébergeur | Infrastructure (hébergeur) | Planned |
| A.7.7 | Bureau propre et écran verrouillé | Oui | Politique de bureau propre pour l'équipe de développement | Governance | Planned |
| A.7.8 | Emplacement et protection du matériel | Oui | Délégué à l'hébergeur ; vérification lors des audits | Infrastructure (hébergeur) | Planned |
| A.7.9 | Sécurité des actifs hors des locaux | Oui | Protection des postes de développement mobiles | Governance | Planned |
| A.7.10 | Supports de stockage | Oui | Chiffrement des supports, destruction sécurisée | Infrastructure, Identity | Planned |
| A.7.11 | Services généraux | Oui | Alimentation électrique, climatisation — délégué à l'hébergeur, intégrant les risques climatiques tunisiens | Infrastructure (hébergeur) | Planned |
| A.7.12 | Sécurité du câblage | Oui | Délégué à l'hébergeur | Infrastructure (hébergeur) | Planned |
| A.7.13 | Maintenance du matériel | Oui | Maintenance préventive des serveurs — délégué à l'hébergeur | Infrastructure (hébergeur) | Planned |
| A.7.14 | Mise au rebut ou réutilisation sécurisée du matériel | Oui | Effacement sécurisé des données avant mise au rebut | Infrastructure (hébergeur) | Planned |

### 5.5 Contrôles technologiques (A.8.1 — A.8.34)

| ID | Contrôle | Applicable | Justification | Module BANKO | Statut |
|---|---|---|---|---|---|
| A.8.1 | Terminaux utilisateurs | Oui | Sécurisation des postes de développement et d'administration | Identity, Infrastructure | Planned |
| A.8.2 | Droits d'accès privilégiés | Oui | Gestion des comptes administrateurs (DB, K8s, serveurs) | Identity, Governance | Planned |
| A.8.3 | Restriction d'accès aux informations | Oui | Contrôle d'accès aux données bancaires par module | Identity, tous modules | Planned |
| A.8.4 | Accès au code source | Oui | Gestion des accès au dépôt Git, revue de code obligatoire | Governance | In Progress |
| A.8.5 | Authentification sécurisée | Oui | JWT + refresh tokens, MFA, gestion des sessions | Identity | Planned |
| A.8.6 | Dimensionnement des capacités | Oui | Monitoring des ressources, autoscaling K8s | Infrastructure | Planned |
| A.8.7 | Protection contre les programmes malveillants | Oui | Analyse statique du code, audit des dépendances | Tous modules | Planned |
| A.8.8 | Gestion des vulnérabilités techniques | Oui | `cargo audit`, `npm audit`, veille CVE | Tous modules | Planned |
| A.8.9 | Gestion de la configuration | Oui | **Nouveau contrôle 2022** — IaC (Docker, K8s manifests), gestion des configurations | Infrastructure | Planned |
| A.8.10 | Suppression d'informations | Oui | **Nouveau contrôle 2022** — Droit à l'effacement (loi données 2025), purge des données expirées | Customer, tous modules | Planned |
| A.8.11 | Masquage des données | Oui | **Nouveau contrôle 2022** — Anonymisation/pseudonymisation pour environnements de test et reporting | Customer, Account, tous modules | Planned |
| A.8.12 | Prévention de la fuite de données | Oui | **Nouveau contrôle 2022** — Protection contre l'exfiltration de données bancaires | Tous modules | Planned |
| A.8.13 | Sauvegarde des informations | Oui | Sauvegarde PostgreSQL, MinIO, stratégie 3-2-1 | Infrastructure, tous modules | Planned |
| A.8.14 | Redondance des moyens de traitement de l'information | Oui | Haute disponibilité K8s, réplication PostgreSQL | Infrastructure | Planned |
| A.8.15 | Journalisation | Oui | Journaux d'audit exhaustifs (exigence BCT 2006-19), piste d'audit | Governance, tous modules | Planned |
| A.8.16 | Activités de surveillance | Oui | **Nouveau contrôle 2022** — Monitoring continu (Prometheus /metrics), alertes | Infrastructure, tous modules | Planned |
| A.8.17 | Synchronisation des horloges | Oui | NTP pour horodatage des transactions bancaires et audit | Infrastructure | Planned |
| A.8.18 | Utilisation de programmes utilitaires privilégiés | Oui | Restriction des outils d'administration système | Infrastructure | Planned |
| A.8.19 | Installation de logiciels sur les systèmes en exploitation | Oui | Déploiement contrôlé via CI/CD, images Docker signées | Infrastructure | Planned |
| A.8.20 | Sécurité des réseaux | Oui | Segmentation réseau K8s (Network Policies), pare-feu Traefik | Infrastructure | Planned |
| A.8.21 | Sécurité des services réseau | Oui | TLS 1.3, certificats gérés, mTLS inter-services | Infrastructure | Planned |
| A.8.22 | Séparation des réseaux | Oui | Isolation des namespaces K8s (dev, staging, production) | Infrastructure | Planned |
| A.8.23 | Filtrage web | Oui | **Nouveau contrôle 2022** — Filtrage des accès web sortants, WAF | Infrastructure | Planned |
| A.8.24 | Utilisation de la cryptographie | Oui | Chiffrement AES-256 (données au repos), TLS 1.3 (données en transit), hachage bcrypt/argon2 | Identity, tous modules | Planned |
| A.8.25 | Cycle de vie du développement sécurisé | Oui | SDLC sécurisé : revue de code, tests de sécurité, CI/CD | Tous modules | In Progress |
| A.8.26 | Exigences de sécurité des applications | Oui | Validation des entrées au niveau Domain, sanitisation | Tous modules | In Progress |
| A.8.27 | Architecture de systèmes sécurisés et principes d'ingénierie | Oui | Architecture hexagonale, DDD, principle of least privilege | Tous modules | In Progress |
| A.8.28 | Codage sécurisé | Oui | **Nouveau contrôle 2022** — Rust : sécurité mémoire native (ownership, borrowing), SQLx requêtes typées, absence de buffer overflow | Tous modules | **In Progress** |
| A.8.29 | Tests de sécurité dans le développement et l'acceptation | Oui | Tests unitaires, BDD (Cucumber), E2E (Playwright), tests d'intrusion ANCS | Tous modules | Planned |
| A.8.30 | Développement externalisé | Oui | Contributions open source soumises à revue de code et CI | Tous modules | Planned |
| A.8.31 | Séparation des environnements de développement, de test et de production | Oui | Environnements isolés : dev (Docker Compose), staging, production (K8s) | Infrastructure | Planned |
| A.8.32 | Gestion des changements | Oui | Processus de gestion des changements (Git flow, PR, revue) | Tous modules | In Progress |
| A.8.33 | Informations de test | Oui | Données de test anonymisées, `make seed` avec données fictives | Tous modules | Planned |
| A.8.34 | Protection des systèmes d'information durant les tests d'audit | Oui | Isolation des tests d'audit, environnement dédié | Infrastructure | Planned |

---

## 6. Exclusions justifiées

Conformément à la clause 6.1.3 d) de la norme ISO/IEC 27001:2022, les exclusions suivantes sont documentées et justifiées :

| Contrôle exclu | Intitulé | Justification de l'exclusion |
|---|---|---|
| A.7.4 | Surveillance de la sécurité physique | BANKO est une plateforme logicielle déployée exclusivement en environnement cloud ou sur des infrastructures hébergées par des tiers certifiés. Le projet ne dispose pas de locaux physiques dédiés nécessitant une surveillance vidéo ou de capteurs physiques propres. Ce contrôle est intégralement délégué aux hébergeurs, dont la conformité est vérifiée contractuellement (contrôle A.5.19 et A.5.20). |

**Note** : L'exclusion est limitée à un seul contrôle. Les contrôles physiques restants (A.7.1 à A.7.3, A.7.5 à A.7.14) sont maintenus comme applicables car ils font l'objet d'exigences contractuelles envers les hébergeurs et de vérifications lors des audits de fournisseurs.

---

## 7. Références

### 7.1 Documents internes BANKO

| Document | Chemin | Description |
|---|---|---|
| Référentiel légal et normatif | [docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md](../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) | Inventaire exhaustif des textes réglementaires tunisiens |
| Registre des risques ISO 27001 | [02-risk-assessment-register.md](02-risk-assessment-register.md) | Analyse des risques et plans de traitement |
| Mapping contrôles Annexe A | [03-controls-annex-a-mapping.md](03-controls-annex-a-mapping.md) | Correspondance détaillée contrôles / implémentation BANKO |
| Plan d'implémentation ISO 27001 | [04-implementation-plan.md](04-implementation-plan.md) | Roadmap de certification sur 18 mois |
| Guide d'architecture | [.claude/guides/architecture-guide.md](../../../.claude/guides/architecture-guide.md) | Architecture hexagonale et DDD |
| Politique de sécurité | [SECURITY.md](../../../SECURITY.md) | Politique de divulgation des vulnérabilités |
| Guide de contribution | [CONTRIBUTING.md](../../../CONTRIBUTING.md) | Processus de contribution sécurisé |

### 7.2 Normes internationales

| Référence | Intitulé |
|---|---|
| ISO/IEC 27001:2022 | Technologies de l'information — Techniques de sécurité — Systèmes de management de la sécurité de l'information — Exigences |
| ISO/IEC 27001:2022/Amd 1:2024 | Amendement 1 : Action pour le changement climatique |
| ISO/IEC 27002:2022 | Sécurité de l'information, cybersécurité et protection de la vie privée — Mesures de sécurité de l'information |
| ISO/IEC 27701:2025 | Gestion de la protection de la vie privée (norme autonome, couvrant IA, biométrie, IoT) |
| ISO 31000:2018 | Management du risque — Lignes directrices |
| ISO 22301:2019 | Sécurité et résilience — Systèmes de management de la continuité d'activité |

### 7.3 Réglementation tunisienne

| Référence | Intitulé | Pertinence SMSI |
|---|---|---|
| Circulaire BCT n° 2006-19 | Contrôle interne | Exigence de système de contrôle interne permanent |
| Circulaire BCT n° 2021-05 | Cadre de gouvernance | Trois lignes de défense, comités obligatoires |
| Circulaire BCT n° 2025-06 | Tests d'intrusion e-KYC | Obligation de tests d'intrusion accrédités ANCS |
| Circulaire BCT n° 2025-17 | Nouveau cadre LBC/FT/FP | Filtrage, surveillance transactionnelle, conservation 10 ans |
| Loi données personnelles 2025 | Protection des données personnelles | DPO, notification 72h, DPIA, chiffrement (application juillet 2026) |
| Loi n° 2016-48 | Loi bancaire | Cadre fondamental de l'activité bancaire en Tunisie |
| Loi organique n° 2015-26 | LBC/FT | Obligations de vigilance, déclarations de soupçon |

---

> **Prochaine revue prévue** : Juillet 2026 (alignement avec l'entrée en application de la loi sur la protection des données personnelles)
>
> **Approbation** : Ce document doit être approuvé par le RSSI et la direction générale de l'organisme déployant BANKO avant toute utilisation dans le cadre d'un audit de certification.
