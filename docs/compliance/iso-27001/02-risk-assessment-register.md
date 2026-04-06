# BANKO — Registre d'Évaluation des Risques

> **Version** : 1.0.0 — 6 avril 2026
> **Statut** : Document initial
> **Classification** : Confidentiel — Usage interne et auditeurs
> **Licence** : AGPL-3.0
> **Auteur** : Projet BANKO
> **Norme de référence** : ISO/IEC 27001:2022 (clause 6.1.2 — Appréciation des risques)
> **Méthodologie** : Alignée ISO 31000:2018

---

## Table des matières

1. [Méthodologie d'évaluation des risques](#1-méthodologie-dévaluation-des-risques)
2. [Critères d'évaluation](#2-critères-dévaluation)
3. [Registre des risques](#3-registre-des-risques)
4. [Matrice de risques](#4-matrice-de-risques)
5. [Plans de traitement prioritaires](#5-plans-de-traitement-prioritaires)
6. [Calendrier de revue](#6-calendrier-de-revue)

---

## 1. Méthodologie d'évaluation des risques

### 1.1 Cadre de référence

La méthodologie d'évaluation des risques de BANKO est alignée sur :

- **ISO/IEC 27001:2022**, clause 6.1.2 — Appréciation des risques de sécurité de l'information
- **ISO 31000:2018** — Management du risque — Lignes directrices
- **ISO/IEC 27005:2022** — Lignes directrices pour la gestion des risques liés à la sécurité de l'information

### 1.2 Processus d'évaluation

Le processus d'évaluation des risques suit les étapes suivantes :

1. **Identification des actifs** : Inventaire des actifs informationnels couverts par le SMSI (cf. [01-scope-and-statement-of-applicability.md](01-scope-and-statement-of-applicability.md), section 2)
2. **Identification des menaces** : Catalogue des menaces internes et externes applicables au contexte bancaire tunisien
3. **Identification des vulnérabilités** : Faiblesses exploitables dans l'architecture, le code, les processus
4. **Évaluation de la vraisemblance** : Probabilité d'occurrence sur une échelle de 1 à 5
5. **Évaluation de l'impact** : Conséquences sur la confidentialité, l'intégrité et la disponibilité, sur une échelle de 1 à 5
6. **Calcul du niveau de risque** : Risque = Vraisemblance x Impact
7. **Traitement du risque** : Sélection de la stratégie (réduction, transfert, acceptation, évitement)
8. **Surveillance continue** : Revue trimestrielle et après chaque incident significatif

### 1.3 Périmètre de l'évaluation

L'évaluation couvre l'intégralité des 12 bounded contexts de BANKO, l'infrastructure technique (PostgreSQL 16, Actix-web, Traefik, MinIO, Docker/K8s) et les processus organisationnels associés au développement et à l'exploitation de la plateforme.

---

## 2. Critères d'évaluation

### 2.1 Échelle de vraisemblance

| Niveau | Intitulé | Description | Fréquence estimée |
|---|---|---|---|
| 1 | Très improbable | Événement exceptionnel, jamais observé dans le secteur | Moins d'une fois en 10 ans |
| 2 | Improbable | Événement rare, quelques cas documentés | Une fois tous les 5 à 10 ans |
| 3 | Possible | Événement occasionnel, déjà observé dans le secteur bancaire tunisien | Une fois tous les 1 à 5 ans |
| 4 | Probable | Événement fréquent, observé régulièrement dans le secteur | Une fois par an |
| 5 | Quasi certain | Événement très fréquent, attendu dans les mois à venir | Plusieurs fois par an |

### 2.2 Échelle d'impact

| Niveau | Intitulé | Confidentialité | Intégrité | Disponibilité | Impact financier | Impact réglementaire |
|---|---|---|---|---|---|---|
| 1 | Négligeable | Données publiques exposées | Erreur mineure sans conséquence | Interruption < 1h | < 10 000 TND | Aucun |
| 2 | Mineur | Données internes exposées | Erreur corrigeable rapidement | Interruption 1-4h | 10 000 - 100 000 TND | Observation de la BCT |
| 3 | Modéré | Données confidentielles d'un client exposées | Altération de données nécessitant restauration | Interruption 4-24h | 100 000 - 1 M TND | Mise en demeure BCT |
| 4 | Majeur | Données sensibles de nombreux clients exposées | Corruption de données comptables ou transactionnelles | Interruption 1-7 jours | 1 M - 10 M TND | Sanction BCT, amende INPDP |
| 5 | Critique | Compromission massive (données bancaires, biométriques) | Perte irréversible de données | Interruption > 7 jours | > 10 M TND | Retrait d'agrément, poursuites pénales |

### 2.3 Matrice de calcul du niveau de risque

Le niveau de risque est calculé par la formule : **Risque = Vraisemblance x Impact**

| Score | Niveau de risque | Code couleur | Action requise |
|---|---|---|---|
| 1 - 4 | **Faible** | Vert | Acceptation ou surveillance. Revue annuelle. |
| 5 - 9 | **Moyen** | Jaune | Réduction recommandée. Plan d'action dans les 6 mois. |
| 10 - 16 | **Élevé** | Orange | Réduction obligatoire. Plan d'action dans les 3 mois. |
| 17 - 25 | **Critique** | Rouge | Traitement immédiat. Escalade à la direction. Plan d'action dans les 30 jours. |

---

## 3. Registre des risques

### 3.1 Risques réglementaires

| ID | Catégorie | Description | V | I | Niveau | Contrôle ISO | Plan de traitement | Propriétaire | Statut |
|---|---|---|---|---|---|---|---|---|---|
| R-REG-01 | Réglementaire | **Inscription sur la liste grise du GAFI (évaluation mutuelle prévue nov. 2026)** — La Tunisie pourrait être maintenue ou inscrite sur la liste grise si les lacunes du dispositif LBC/FT ne sont pas comblées. Impact direct sur les banques utilisatrices de BANKO : restrictions sur les correspondants bancaires internationaux. | 4 | 5 | **Critique (20)** | A.5.31, A.5.5 | Implémentation complète du module AML et Sanctions avec filtrage temps réel, conformité à la circulaire BCT 2025-17 | RSSI / Responsable conformité | Planned |
| R-REG-02 | Réglementaire | **Non-conformité aux circulaires BCT (2006-19, 2021-05)** — Absence de système de contrôle interne permanent, défaillance des trois lignes de défense, rapports réglementaires incomplets ou tardifs. | 3 | 4 | **Élevé (12)** | A.5.31, A.5.36 | Développement du module Governance avec workflows de contrôle interne, module Reporting automatisé | Responsable conformité | Planned |
| R-REG-03 | Réglementaire | **Retard dans l'adoption d'IFRS 9** — Les banques tunisiennes sont en transition vers IFRS 9 (provisionnement des pertes de crédit attendues). Un module Accounting non conforme IFRS 9 expose les banques à des sanctions BCT. | 3 | 4 | **Élevé (12)** | A.5.31 | Intégration du modèle de provisionnement IFRS 9 (Expected Credit Loss) dans le module Accounting | Responsable conformité | Planned |
| R-REG-04 | Réglementaire | **Non-conformité à la loi données personnelles 2025** — Absence de DPO, défaut de notification de violation dans les 72h, absence de DPIA, données non chiffrées. Amendes et sanctions pénales à compter de juillet 2026. | 4 | 4 | **Élevé (16)** | A.5.34, A.8.24 | Désignation d'un DPO, implémentation du chiffrement end-to-end, procédure de notification 72h, registre des traitements | DPO | Planned |
| R-REG-05 | Réglementaire | **Non-conformité tests d'intrusion e-KYC (circulaire BCT 2025-06)** — Les services d'onboarding numérique (e-KYC) doivent faire l'objet de tests d'intrusion par un prestataire accrédité ANCS. Défaut de conformité entraîne la suspension du service e-KYC. | 3 | 4 | **Élevé (12)** | A.8.29, A.5.35 | Planification de tests d'intrusion e-KYC par un prestataire accrédité ANCS, intégration dans le cycle de release | RSSI | Planned |

### 3.2 Risques cyber

| ID | Catégorie | Description | V | I | Niveau | Contrôle ISO | Plan de traitement | Propriétaire | Statut |
|---|---|---|---|---|---|---|---|---|---|
| R-CYB-01 | Cyber | **Injection SQL** — Exploitation de vulnérabilités dans les requêtes de base de données pour accéder, modifier ou supprimer des données bancaires. | 2 | 5 | **Élevé (10)** | A.8.28, A.8.26 | Utilisation systématique de SQLx avec requêtes typées à la compilation (prévention structurelle en Rust), revue de code obligatoire, tests SAST | Équipe développement | In Progress |
| R-CYB-02 | Cyber | **Cross-Site Scripting (XSS)** — Injection de scripts malveillants dans le frontend pour voler des sessions utilisateurs ou exfiltrer des données. | 3 | 4 | **Élevé (12)** | A.8.28, A.8.26 | Sanitisation des entrées côté frontend (Svelte escape par défaut), Content Security Policy (CSP), validation côté backend | Équipe développement | Planned |
| R-CYB-03 | Cyber | **Attaque par déni de service distribué (DDoS)** — Saturation des ressources serveur rendant la plateforme bancaire indisponible. Impact sur la continuité des opérations bancaires. | 4 | 4 | **Élevé (16)** | A.8.6, A.8.20 | Rate limiting Traefik, autoscaling K8s, protection DDoS au niveau hébergeur/CDN, plan de continuité | Équipe infrastructure | Planned |
| R-CYB-04 | Cyber | **Compromission des credentials** — Vol d'identifiants administrateurs (base de données, serveurs, K8s) par phishing, force brute ou fuite. | 3 | 5 | **Critique (15)** | A.5.17, A.8.5 | MFA obligatoire, rotation automatique des secrets, gestion centralisée des credentials (Vault), monitoring des connexions suspectes | RSSI | Planned |
| R-CYB-05 | Cyber | **Attaque sur la chaîne d'approvisionnement (supply chain)** — Compromission d'une dépendance Rust (crate) ou npm contenant du code malveillant. | 3 | 5 | **Critique (15)** | A.5.21, A.8.7 | `cargo audit` et `npm audit` dans le pipeline CI, verrouillage des versions (Cargo.lock, package-lock.json), revue des nouvelles dépendances | Équipe développement | In Progress |
| R-CYB-06 | Cyber | **Exploitation de vulnérabilités zero-day** — Exploitation d'une faille non corrigée dans Actix-web, PostgreSQL, Traefik ou une autre dépendance critique. | 2 | 5 | **Élevé (10)** | A.8.8, A.5.7 | Veille active sur les CVE, abonnement aux listes de diffusion sécurité, procédure de patch d'urgence (< 24h pour les critiques) | RSSI | Planned |
| R-CYB-07 | Cyber | **Interception de communications (Man-in-the-Middle)** — Interception des échanges entre le frontend et le backend, ou entre les microservices internes. | 2 | 4 | **Moyen (8)** | A.8.21, A.8.24 | TLS 1.3 obligatoire, mTLS entre services K8s, HSTS, certificate pinning | Équipe infrastructure | Planned |
| R-CYB-08 | Cyber | **Ransomware** — Chiffrement malveillant des données PostgreSQL et des sauvegardes MinIO, avec demande de rançon. | 3 | 5 | **Critique (15)** | A.8.13, A.8.7 | Sauvegardes hors ligne (air-gapped), réplication géographique, tests de restauration mensuels, segmentation réseau | Équipe infrastructure | Planned |

### 3.3 Risques sur les données

| ID | Catégorie | Description | V | I | Niveau | Contrôle ISO | Plan de traitement | Propriétaire | Statut |
|---|---|---|---|---|---|---|---|---|---|
| R-DAT-01 | Données | **Fuite de données clients** — Exfiltration de données personnelles et bancaires (identités, comptes, soldes, transactions) suite à une intrusion ou une erreur de configuration. | 3 | 5 | **Critique (15)** | A.8.12, A.5.34 | Chiffrement des données au repos (AES-256) et en transit (TLS 1.3), DLP (Data Leakage Prevention), classification des données, monitoring des accès | RSSI / DPO | Planned |
| R-DAT-02 | Données | **Violation de la loi 2025 sur les données personnelles** — Traitement de données personnelles sans base légale, absence de registre des traitements, pas de DPIA pour les traitements à haut risque (biométrie e-KYC). | 4 | 4 | **Élevé (16)** | A.5.34, A.8.11 | Registre des traitements, DPIA pour chaque bounded context traitant des données personnelles, mécanismes de consentement, droit à l'effacement | DPO | Planned |
| R-DAT-03 | Données | **Perte d'intégrité des données comptables** — Altération non détectée des écritures comptables, soldes, ou rapports réglementaires, compromettant la fiabilité des états financiers. | 2 | 5 | **Élevé (10)** | A.8.15, A.8.4 | Piste d'audit immuable (append-only logs), contrôles d'intégrité par checksum, double validation des écritures comptables, reconciliation automatique | Responsable comptable | Planned |
| R-DAT-04 | Données | **Exposition de données de test contenant des données réelles** — Utilisation de données de production dans les environnements de développement ou de test. | 3 | 4 | **Élevé (12)** | A.8.33, A.8.11 | Anonymisation obligatoire des données de test, `make seed` avec données fictives uniquement, politique d'interdiction de copie prod vers dev | Équipe développement | Planned |
| R-DAT-05 | Données | **Non-respect de la conservation des données LBC/FT (10 ans)** — Suppression prématurée des données de vigilance (KYC, transactions, alertes AML) avant l'expiration du délai légal de 10 ans. | 2 | 4 | **Moyen (8)** | A.5.33, A.8.10 | Politique de rétention automatisée par module, archivage sécurisé, contrôles de purge avec validation juridique | Responsable conformité | Planned |

### 3.4 Risques opérationnels

| ID | Catégorie | Description | V | I | Niveau | Contrôle ISO | Plan de traitement | Propriétaire | Statut |
|---|---|---|---|---|---|---|---|---|---|
| R-OPS-01 | Opérationnel | **Indisponibilité de la base de données PostgreSQL** — Panne du serveur PostgreSQL entraînant l'arrêt complet des services bancaires (comptes, paiements, comptabilité). | 3 | 5 | **Critique (15)** | A.8.14, A.5.30 | Réplication PostgreSQL (streaming replication), failover automatique, monitoring avec alertes, RPO < 1min, RTO < 15min | Équipe infrastructure | Planned |
| R-OPS-02 | Opérationnel | **Perte des sauvegardes MinIO** — Corruption ou suppression des sauvegardes stockées dans MinIO (documents KYC, pièces justificatives), rendant impossible la restauration après sinistre. | 2 | 5 | **Élevé (10)** | A.8.13 | Stratégie 3-2-1 (3 copies, 2 supports, 1 hors site), tests de restauration mensuels, chiffrement des sauvegardes, versioning MinIO | Équipe infrastructure | Planned |
| R-OPS-03 | Opérationnel | **Erreur de migration de base de données** — Migration SQLx défaillante corrompant le schéma ou les données en production, entraînant une indisponibilité prolongée. | 3 | 4 | **Élevé (12)** | A.8.32, A.8.31 | Migrations réversibles (up/down), test préalable en staging, snapshot avant migration, procédure de rollback documentée | Équipe développement | Planned |
| R-OPS-04 | Opérationnel | **Défaillance du reverse proxy Traefik** — Panne de Traefik rendant inaccessible l'ensemble de l'API et du frontend. | 2 | 4 | **Moyen (8)** | A.8.14, A.8.20 | Déploiement Traefik en haute disponibilité (replicas K8s), health checks, failover automatique | Équipe infrastructure | Planned |
| R-OPS-05 | Opérationnel | **Saturation des ressources conteneur** — Épuisement des ressources CPU/mémoire des conteneurs Docker/K8s entraînant une dégradation des performances ou un arrêt des services. | 3 | 3 | **Moyen (9)** | A.8.6 | Resource limits et requests K8s, Horizontal Pod Autoscaler, monitoring Prometheus, alertes sur seuils | Équipe infrastructure | Planned |
| R-OPS-06 | Opérationnel | **Défaillance du pipeline CI/CD** — Compromission ou dysfonctionnement du pipeline GitHub Actions permettant le déploiement de code non vérifié en production. | 2 | 4 | **Moyen (8)** | A.8.25, A.8.19 | Branch protection rules, revue obligatoire, tests automatisés bloquants, signature des artefacts, environnements de déploiement protégés | Équipe développement | In Progress |

### 3.5 Risques humains

| ID | Catégorie | Description | V | I | Niveau | Contrôle ISO | Plan de traitement | Propriétaire | Statut |
|---|---|---|---|---|---|---|---|---|---|
| R-HUM-01 | Humain | **Accès non autorisé par un développeur** — Un contributeur disposant d'accès élevés (administrateur DB, accès K8s production) abuse de ses privilèges pour consulter ou exfiltrer des données bancaires. | 2 | 5 | **Élevé (10)** | A.8.2, A.5.3 | Principe du moindre privilège, séparation des rôles (dev/ops/admin), revue trimestrielle des accès, journalisation de toutes les actions privilégiées | RSSI | Planned |
| R-HUM-02 | Humain | **Ingénierie sociale (phishing ciblé)** — Attaque de spear phishing ciblant un membre de l'équipe de développement ou d'exploitation pour obtenir des credentials d'accès à l'infrastructure. | 4 | 4 | **Élevé (16)** | A.6.3, A.8.5 | Formation annuelle anti-phishing, simulations de phishing, MFA sur tous les accès critiques, procédure de signalement rapide | RSSI | Planned |
| R-HUM-03 | Humain | **Départ d'un personnel clé** — Perte de connaissance critique suite au départ d'un développeur senior ou de l'architecte principal, sans transfert de compétences adéquat. | 3 | 3 | **Moyen (9)** | A.6.5, A.5.37 | Documentation technique exhaustive (CLAUDE.md, guides), pair programming, bus factor > 2, revue de code systématique | Direction technique | Planned |
| R-HUM-04 | Humain | **Erreur humaine lors d'une opération de production** — Exécution accidentelle de `make reset-db` en production, suppression de données, erreur de configuration K8s. | 3 | 5 | **Critique (15)** | A.5.37, A.8.18 | Procédures d'exploitation documentées (runbooks), double validation pour les opérations destructrices, environnements protégés, sauvegardes avant toute opération | Équipe infrastructure | Planned |
| R-HUM-05 | Humain | **Contribution malveillante au code open source** — Insertion de code malveillant (backdoor, exfiltration de données) dans une pull request par un contributeur externe. | 2 | 5 | **Élevé (10)** | A.8.4, A.8.30 | Revue de code obligatoire par 2 mainteneurs, CI avec tests de sécurité automatisés, analyse statique (SAST), DCO sign-off obligatoire | Mainteneurs du projet | In Progress |

### 3.6 Risques climatiques et environnementaux

| ID | Catégorie | Description | V | I | Niveau | Contrôle ISO | Plan de traitement | Propriétaire | Statut |
|---|---|---|---|---|---|---|---|---|---|
| R-ENV-01 | Environnemental | **Vagues de chaleur affectant les centres de données** — Dépassement des capacités de refroidissement des centres de données tunisiens lors de canicules prolongées (> 45 °C). | 3 | 3 | **Moyen (9)** | A.7.5, A.7.11 | Sélection d'hébergeurs avec PUE optimisé et refroidissement redondant, réplication vers un site en zone tempérée | Équipe infrastructure | Planned |
| R-ENV-02 | Environnemental | **Inondations côtières détruisant l'infrastructure** — Montée des eaux et inondations affectant un centre de données situé en zone littorale (Tunis, Sousse). | 2 | 5 | **Élevé (10)** | A.7.5, A.5.30 | Exigence contractuelle de localisation hors zone inondable, réplication géographique vers un site intérieur | Équipe infrastructure | Planned |

---

## 4. Matrice de risques

La matrice suivante présente la distribution des risques identifiés selon leur vraisemblance et leur impact :

```
                           IMPACT
              1           2           3           4           5
          Négligeable   Mineur     Modéré      Majeur    Critique
       ┌───────────┬───────────┬───────────┬───────────┬───────────┐
   5   │           │           │           │           │           │
Quasi  │           │           │           │           │           │
certain│           │           │           │           │           │
       ├───────────┼───────────┼───────────┼───────────┼───────────┤
   4   │           │           │           │ R-REG-04  │ R-REG-01  │
       │           │           │           │ R-DAT-02  │           │
Probable           │           │           │ R-CYB-03  │           │
       │           │           │           │ R-HUM-02  │           │
       ├───────────┼───────────┼───────────┼───────────┼───────────┤
   3   │           │           │ R-OPS-05  │ R-REG-02  │ R-CYB-04  │
       │           │           │ R-HUM-03  │ R-REG-03  │ R-CYB-05  │
Possible           │           │ R-ENV-01  │ R-CYB-02  │ R-CYB-08  │
       │           │           │           │ R-REG-05  │ R-DAT-01  │
       │           │           │           │ R-OPS-03  │ R-OPS-01  │
       │           │           │           │ R-DAT-04  │ R-HUM-04  │
       ├───────────┼───────────┼───────────┼───────────┼───────────┤
   2   │           │           │           │ R-OPS-04  │ R-CYB-01  │
       │           │           │           │ R-OPS-06  │ R-CYB-06  │
Improbable         │           │           │ R-DAT-05  │ R-DAT-03  │
       │           │           │           │ R-CYB-07  │ R-OPS-02  │
       │           │           │           │           │ R-HUM-01  │
       │           │           │           │           │ R-HUM-05  │
       │           │           │           │           │ R-ENV-02  │
       ├───────────┼───────────┼───────────┼───────────┼───────────┤
   1   │           │           │           │           │           │
Très   │           │           │           │           │           │
improb.│           │           │           │           │           │
       └───────────┴───────────┴───────────┴───────────┴───────────┘

Légende :  ■ Faible (1-4)  ■ Moyen (5-9)  ■ Élevé (10-16)  ■ Critique (17-25)
```

### 4.1 Synthèse de la distribution des risques

| Niveau de risque | Nombre de risques | Pourcentage | Action requise |
|---|---|---|---|
| **Critique** (17-25) | 1 | 4 % | Traitement immédiat (30 jours) |
| **Élevé** (10-16) | 18 | 72 % | Plan d'action sous 3 mois |
| **Moyen** (5-9) | 6 | 24 % | Plan d'action sous 6 mois |
| **Faible** (1-4) | 0 | 0 % | Surveillance |
| **Total** | **25** | **100 %** | — |

La prédominance des risques de niveau élevé reflète le stade initial du projet (phase de développement) et le contexte réglementaire exigeant du secteur bancaire tunisien. La mise en oeuvre progressive des contrôles ISO 27001 réduira mécaniquement le nombre de risques élevés.

---

## 5. Plans de traitement prioritaires

### 5.1 Risque R-REG-01 — Inscription sur la liste grise du GAFI (Critique — Score 20)

| Élément | Détail |
|---|---|
| **Description** | La Tunisie fait l'objet d'une évaluation mutuelle du GAFI/MENAFATF prévue fin 2026. Un défaut d'efficacité du dispositif LBC/FT pourrait conduire au maintien ou à l'inscription sur la liste grise, avec des conséquences systémiques pour le secteur bancaire. |
| **Impact sur BANKO** | Les banques utilisatrices de BANKO doivent disposer d'un système LBC/FT conforme aux 40 recommandations du GAFI et à la circulaire BCT 2025-17. |
| **Contrôles ISO** | A.5.31 (Exigences légales), A.5.5 (Relations avec les autorités) |
| **Modules concernés** | AML, Sanctions, Customer (KYC), Reporting |
| **Actions de traitement** | 1. Implémentation du module AML avec détection d'opérations suspectes en temps réel |
|  | 2. Implémentation du module Sanctions avec filtrage multi-listes (ONU, UE, OFAC, listes nationales) |
|  | 3. Module KYC conforme à la circulaire BCT 2025-17 (fiche KYC, vigilance renforcée, PEP) |
|  | 4. Module de déclarations de soupçon (STR) vers la CTAF |
|  | 5. Conservation des données de vigilance pendant 10 ans minimum |
|  | 6. Piste d'audit exhaustive et immuable |
| **Échéance** | M6 (phase 2 du plan d'implémentation) |
| **Risque résiduel cible** | Moyen (score 8 — vraisemblance réduite à 2) |
| **Propriétaire** | RSSI / Responsable conformité |
| **Indicateur de suivi** | Taux de couverture des recommandations GAFI dans le code |

### 5.2 Risque R-CYB-03 — Attaque DDoS (Élevé — Score 16)

| Élément | Détail |
|---|---|
| **Description** | Les services bancaires en ligne constituent une cible privilégiée pour les attaques DDoS, en particulier lors de périodes de tension géopolitique. |
| **Impact sur BANKO** | Indisponibilité de l'API et du frontend, impossibilité pour les clients d'accéder à leurs comptes et d'effectuer des paiements. |
| **Contrôles ISO** | A.8.6 (Dimensionnement), A.8.20 (Sécurité réseau), A.5.30 (Continuité TIC) |
| **Modules concernés** | Infrastructure (Traefik, K8s), tous modules |
| **Actions de traitement** | 1. Configuration du rate limiting dans Traefik (par IP, par endpoint) |
|  | 2. Déploiement d'un WAF (Web Application Firewall) en amont |
|  | 3. Configuration de l'autoscaling K8s (HPA) pour absorber les pics |
|  | 4. Mise en place d'une protection DDoS au niveau de l'hébergeur ou du CDN |
|  | 5. Plan de basculement vers un site de secours |
|  | 6. Tests de charge réguliers pour valider les seuils |
| **Échéance** | M9 (phase 2 du plan d'implémentation) |
| **Risque résiduel cible** | Moyen (score 8 — impact réduit à 2 grâce au failover) |
| **Propriétaire** | Équipe infrastructure |
| **Indicateur de suivi** | RTO mesuré lors des tests de charge, disponibilité mensuelle (cible : 99,95 %) |

### 5.3 Risque R-DAT-02 — Violation de la loi données personnelles 2025 (Élevé — Score 16)

| Élément | Détail |
|---|---|
| **Description** | La nouvelle loi sur la protection des données personnelles adoptée en juin 2025 (application juillet 2026) introduit des obligations strictes : désignation d'un DPO, notification de violation sous 72h, réalisation de DPIA, chiffrement obligatoire. |
| **Impact sur BANKO** | Les bounded contexts Customer (données personnelles, KYC), Identity (biométrie, authentification) et Account (données financières) traitent massivement des données à caractère personnel. |
| **Contrôles ISO** | A.5.34 (Vie privée), A.8.11 (Masquage des données), A.8.10 (Suppression d'informations) |
| **Modules concernés** | Customer, Identity, Account, tous modules |
| **Actions de traitement** | 1. Désignation d'un DPO et création d'un registre des traitements |
|  | 2. Réalisation d'une DPIA pour chaque bounded context traitant des données personnelles |
|  | 3. Implémentation du droit à l'effacement (A.8.10) dans le module Customer |
|  | 4. Mécanisme de consentement explicite pour les traitements non obligatoires |
|  | 5. Chiffrement AES-256 de toutes les données personnelles au repos |
|  | 6. Procédure de notification de violation (72h) avec template préétabli |
|  | 7. Masquage des données (A.8.11) dans les environnements de test |
| **Échéance** | M6 (avant juillet 2026 — entrée en application de la loi) |
| **Risque résiduel cible** | Faible (score 4 — vraisemblance réduite à 1, impact réduit à 4) |
| **Propriétaire** | DPO |
| **Indicateur de suivi** | Nombre de DPIA réalisées, couverture du chiffrement |

### 5.4 Risque R-HUM-02 — Ingénierie sociale (Élevé — Score 16)

| Élément | Détail |
|---|---|
| **Description** | Le phishing ciblé (spear phishing) est le vecteur d'attaque le plus efficace contre les organisations. Les développeurs et administrateurs de BANKO détiennent des accès critiques à l'infrastructure et au code source. |
| **Impact sur BANKO** | Compromission d'un compte développeur ou administrateur pouvant mener à l'injection de code malveillant, l'exfiltration de données, ou la compromission de l'infrastructure de production. |
| **Contrôles ISO** | A.6.3 (Formation sécurité), A.8.5 (Authentification sécurisée) |
| **Modules concernés** | Identity, Infrastructure, tous modules |
| **Actions de traitement** | 1. MFA obligatoire sur tous les accès (GitHub, serveurs, K8s, bases de données) |
|  | 2. Formation annuelle de sensibilisation anti-phishing pour toute l'équipe |
|  | 3. Simulations de phishing trimestrielles avec suivi des résultats |
|  | 4. Clés de sécurité matérielles (FIDO2/WebAuthn) pour les comptes administrateurs |
|  | 5. Procédure de signalement des e-mails suspects |
|  | 6. Politique de mots de passe forts (minimum 16 caractères, gestionnaire de mots de passe) |
| **Échéance** | M4 (début de la phase 2) |
| **Risque résiduel cible** | Moyen (score 8 — vraisemblance réduite à 2) |
| **Propriétaire** | RSSI |
| **Indicateur de suivi** | Taux de réussite des simulations de phishing (cible : < 5 % de clics) |

### 5.5 Risque R-OPS-01 — Indisponibilité de PostgreSQL (Critique — Score 15)

| Élément | Détail |
|---|---|
| **Description** | PostgreSQL constitue le coeur du stockage de données de BANKO. Son indisponibilité entraîne l'arrêt complet de tous les services bancaires (comptes, paiements, comptabilité, reporting). |
| **Impact sur BANKO** | Perte de service totale, impossibilité d'effectuer des transactions, risque de corruption de données en cas d'arrêt brutal. |
| **Contrôles ISO** | A.8.14 (Redondance), A.5.30 (Continuité TIC), A.8.13 (Sauvegarde) |
| **Modules concernés** | Tous les 12 bounded contexts |
| **Actions de traitement** | 1. Déploiement de PostgreSQL en haute disponibilité (streaming replication + failover automatique) |
|  | 2. RPO (Recovery Point Objective) cible : < 1 minute |
|  | 3. RTO (Recovery Time Objective) cible : < 15 minutes |
|  | 4. Sauvegardes continues (WAL archiving) + snapshots quotidiens |
|  | 5. Tests de failover mensuels en environnement de staging |
|  | 6. Monitoring des métriques PostgreSQL (connexions, requêtes, réplication lag) via Prometheus |
|  | 7. Procédure de restauration documentée et testée |
| **Échéance** | M9 (phase 2 du plan d'implémentation) |
| **Risque résiduel cible** | Moyen (score 6 — vraisemblance réduite à 2, impact réduit à 3) |
| **Propriétaire** | Équipe infrastructure |
| **Indicateur de suivi** | Disponibilité PostgreSQL (cible : 99,99 %), réplication lag (cible : < 100ms) |

---

## 6. Calendrier de revue

### 6.1 Fréquence des revues

| Type de revue | Fréquence | Participants | Livrables |
|---|---|---|---|
| **Revue trimestrielle du registre des risques** | Tous les 3 mois | RSSI, DPO, Direction technique, Responsable conformité | Registre mis à jour, nouveaux risques identifiés, risques clôturés |
| **Revue après incident** | Après chaque incident de sécurité significatif | RSSI, Équipe concernée | Analyse post-incident, mise à jour du registre, actions correctives |
| **Revue annuelle complète** | Annuelle (avril) | Direction générale, RSSI, DPO, Responsable conformité, Auditeur | Rapport annuel de sécurité, tendances, comparaison N-1 |
| **Revue ad hoc** | Sur événement déclencheur | RSSI, parties prenantes concernées | Évaluation ponctuelle, ajustement des contrôles |

### 6.2 Événements déclencheurs d'une revue ad hoc

- Publication d'une nouvelle circulaire BCT affectant la sécurité de l'information
- Modification significative de l'architecture technique de BANKO
- Incident de sécurité dans le secteur bancaire tunisien ou international
- Résultats d'un test d'intrusion révélant de nouvelles vulnérabilités
- Changement majeur dans l'environnement de menaces (nouveau type d'attaque, CVE critique)
- Évolution du cadre réglementaire (nouvelle loi, entrée en application de la loi données personnelles 2025)
- Évaluation mutuelle du GAFI/MENAFATF concernant la Tunisie

### 6.3 Calendrier prévisionnel 2026-2027

| Date | Action | Responsable |
|---|---|---|
| Avril 2026 | Établissement du registre initial (présent document) | RSSI |
| Juillet 2026 | 1re revue trimestrielle + revue ad hoc (entrée en application loi données 2025) | RSSI, DPO |
| Octobre 2026 | 2e revue trimestrielle | RSSI |
| Novembre 2026 | Revue ad hoc (évaluation mutuelle GAFI si confirmée) | RSSI, Responsable conformité |
| Janvier 2027 | 3e revue trimestrielle | RSSI |
| Avril 2027 | Revue annuelle complète | Direction, RSSI, DPO |

---

> **Prochaine revue prévue** : Juillet 2026
>
> **Documents associés** :
> - [01-scope-and-statement-of-applicability.md](01-scope-and-statement-of-applicability.md) — Périmètre du SMSI et SoA
> - [03-controls-annex-a-mapping.md](03-controls-annex-a-mapping.md) — Mapping détaillé des contrôles Annexe A
> - [04-implementation-plan.md](04-implementation-plan.md) — Plan d'implémentation ISO 27001
> - [Référentiel légal et normatif](../../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) — Cadre réglementaire tunisien
>
> **Approbation** : Ce registre doit être validé par le RSSI et la direction générale avant utilisation dans le cadre d'un audit ISO 27001.
