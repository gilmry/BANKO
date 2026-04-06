# BANKO — Tableau de bord de conformite

> **Version** : 1.0.0 — 6 avril 2026
> **Statut** : Tableau de bord executif — Suivi de la conformite reglementaire
> **Licence** : AGPL-3.0
> **Auteur** : GILMRY / Projet BANKO

---

## Table des matieres

1. [Vue executive](#1-vue-executive)
2. [Scorecards par norme](#2-scorecards-par-norme)
3. [Risques critiques](#3-risques-critiques)
4. [Prochaines echeances](#4-prochaines-echeances)
5. [Actions prioritaires Q2-Q3 2026](#5-actions-prioritaires-q2-q3-2026)
6. [Indicateurs de progression (KPIs)](#6-indicateurs-de-progression-kpis)
7. [Liens vers la documentation detaillee](#7-liens-vers-la-documentation-detaillee)

---

## 1. Vue executive

Le projet BANKO a identifie **86 exigences de conformite** reparties sur 9 categories normatives couvrant la reglementation bancaire tunisienne, la lutte anti-blanchiment, la protection des donnees, les normes comptables, la reglementation des changes, la securite de l'information (ISO 27001), la securite des paiements (PCI DSS), l'Open Banking et les normes prudentielles de Bale III.

A la date du 6 avril 2026, l'ensemble des exigences est au statut **Planned**. Deux echeances critiques imposent une acceleration immediate : l'application de la nouvelle loi sur les donnees personnelles le **11 juillet 2026** et l'evaluation mutuelle du GAFI prevue le **1er novembre 2026**, qui expose la Tunisie a un risque d'inscription sur la liste grise en cas de deficiences dans le dispositif LBC/FT.

---

## 2. Scorecards par norme

### 2.1 Vue d'ensemble

| Norme | Controles total | Done | In Progress | Planned | N/A | % Couverture |
|---|---|---|---|---|---|---|
| **Reglementation bancaire tunisienne (BCT)** | 22 | 0 | 0 | 22 | 0 | 0% |
| **LBC/FT (Anti-blanchiment)** | 12 | 0 | 0 | 12 | 0 | 0% |
| **Protection des donnees personnelles** | 9 | 0 | 0 | 9 | 0 | 0% |
| **Normes comptables (NCT / IFRS)** | 8 | 0 | 0 | 8 | 0 | 0% |
| **Reglementation des changes** | 5 | 0 | 0 | 5 | 0 | 0% |
| **ISO 27001:2022** | 13 | 0 | 0 | 11 | 2 | 0% |
| **PCI DSS v4.0.1** | 12 | 0 | 0 | 10 | 2 | 0% |
| **Open Banking / PSD3** | 5 | 0 | 0 | 5 | 0 | 0% |
| **Bale III** | 10 | 0 | 0 | 10 | 0 | 0% |
| **TOTAL** | **96** | **0** | **0** | **92** | **4** | **0%** |

> **Note** : Les 4 elements N/A concernent les controles physiques (ISO 27001 A.7 et PCI DSS Req 9) qui ne s'appliquent pas a un logiciel open source mais a son deploiement en environnement de production.

### 2.2 Detail ISO 27001:2022

| Categorie de controles | Nombre de controles (Annexe A) | Couverts | En cours | Planifies | % |
|---|---|---|---|---|---|
| A.5 — Controles organisationnels | 37 | 0 | 0 | 37 | 0% |
| A.6 — Controles lies au personnel | 8 | 0 | 0 | 8 | 0% |
| A.7 — Controles physiques | 14 | 0 | 0 | 0 | N/A |
| A.8 — Controles technologiques | 34 | 0 | 0 | 34 | 0% |
| **Total Annexe A** | **93** | **0** | **0** | **79** | **0%** |

### 2.3 Detail PCI DSS v4.0.1

| Requirement | Description | Sous-exigences | Done | In Progress | Planned | % |
|---|---|---|---|---|---|---|
| Req 1 | Securite reseau | 8 | 0 | 0 | 8 | 0% |
| Req 2 | Configurations securisees | 6 | 0 | 0 | 6 | 0% |
| Req 3 | Protection donnees stockees | 7 | 0 | 0 | 7 | 0% |
| Req 4 | Chiffrement en transit | 3 | 0 | 0 | 3 | 0% |
| Req 5 | Protection malwares | 4 | 0 | 0 | 4 | 0% |
| Req 6 | Developpement securise | 5 | 0 | 0 | 5 | 0% |
| Req 7 | Controle d'acces | 4 | 0 | 0 | 4 | 0% |
| Req 8 | Authentification | 6 | 0 | 0 | 6 | 0% |
| Req 9 | Acces physique | 5 | 0 | 0 | 0 | N/A |
| Req 10 | Journalisation | 7 | 0 | 0 | 7 | 0% |
| Req 11 | Tests de securite | 6 | 0 | 0 | 6 | 0% |
| Req 12 | Politiques organisationnelles | 10 | 0 | 0 | 10 | 0% |
| **Total** | — | **71** | **0** | **0** | **66** | **0%** |

### 2.4 Detail Open Banking / PSD3

| Composante | Controles | Done | In Progress | Planned | % |
|---|---|---|---|---|---|
| APIs standardisees (XS2A) | 5 | 0 | 0 | 5 | 0% |
| Gestion du consentement | 4 | 0 | 0 | 4 | 0% |
| SCA (Authentification forte) | 3 | 0 | 0 | 3 | 0% |
| TPP Onboarding | 3 | 0 | 0 | 3 | 0% |
| FIDA (Open Finance) | 2 | 0 | 0 | 2 | 0% |
| **Total** | **17** | **0** | **0** | **17** | **0%** |

---

## 3. Risques critiques

Les risques ci-dessous sont classes par severite decroissante. Les indicateurs de feux tricolores representent le niveau de risque actuel.

| # | Indicateur | Risque | Description | Impact | Probabilite | Mesure d'attenuation |
|---|---|---|---|---|---|---|
| 1 | ROUGE | **Evaluation GAFI novembre 2026** | La Tunisie sera evaluee par le GAFI dans le cadre du 5eme cycle d'evaluations mutuelles. Tout systeme bancaire deploye devra demontrer une conformite complete LBC/FT/FP. | **Critique** — Risque d'inscription sur la liste grise GAFI, impact reputationnel et restrictions de correspondance bancaire | Elevee | Priorite absolue sur l'implementation du module AML/Sanctions, integration goAML (CTAF), conformite Circ. 2025-17, documentation complete des procedures |
| 2 | ROUGE | **Loi donnees personnelles — Application 11 juillet 2026** | La nouvelle loi sur la protection des donnees personnelles entre en application le 11 juillet 2026 avec des exigences significatives (DPO, DPIA, notification 72h, droit effacement/portabilite, amendes proportionnelles au CA). | **Critique** — Amendes proportionnelles au chiffre d'affaires, suspension des traitements, risque d'injonction | Elevee | Implementation prioritaire des modules privacy (DPIA, DPO, notification de breches, droits des personnes), architecture privacy-by-design |
| 3 | JAUNE | **Reforme prudentielle BCT 2025-08** | Les nouvelles normes d'adequation du capital (transition IFRS 9) entrent en vigueur courant 2026. Le double moteur comptable NCT/IFRS 9 est necessaire. | **Eleve** — Non-conformite prudentielle, risque de sanctions BCT, impact sur la adequation du capital | Moyenne | Developpement du moteur de calcul ECL IFRS 9 en parallele du moteur NCT, tests de recalcul du capital |
| 4 | JAUNE | **PCI DSS v4.0.1 — Exigences obligatoires** | Les exigences v4.0.1 (chiffrement au niveau du champ pour les PAN, MFA pour tout acces au CDE) sont devenues obligatoires depuis le 31 mars 2025. | **Eleve** — Non-conformite PCI, impossibilite de traiter des cartes, risque d'amende par les schemes | Moyenne | Implementation du chiffrement PAN au niveau du champ, deploiement MFA pour les acces CDE, tokenisation |
| 5 | VERT | **Open Banking / PSD3** | Le cadre Open Banking n'est pas encore reglemente en Tunisie. L'anticipation permet de se positionner favorablement. | **Faible** — Pas de contrainte reglementaire immediate, avantage competitif | Faible | Architecture API-first preparee pour l'ouverture, veille sur la reglementation fintech BCT (portail fintech.bct.gov.tn) |

### Matrice des risques

|  | Impact Faible | Impact Moyen | Impact Eleve | Impact Critique |
|---|---|---|---|---|
| **Probabilite Elevee** | | | | (1) GAFI, (2) Loi donnees |
| **Probabilite Moyenne** | | | (3) Reforme prudentielle, (4) PCI DSS | |
| **Probabilite Faible** | (5) Open Banking | | | |

---

## 4. Prochaines echeances

### 4.1 Timeline 2026-2030

| Date | Echeance | Norme | Impact | Responsable |
|---|---|---|---|---|
| **Avril 2026** | Lancement du projet BANKO — Phase de conception | Toutes | Fondation | Equipe projet |
| **30 juin 2026** | Date limite de preparation loi donnees personnelles | [REF-79] | Critique | DPO / Equipe Privacy |
| **11 juillet 2026** | **APPLICATION** — Nouvelle loi donnees personnelles | [REF-79] | Critique | DPO / Equipe Privacy |
| **Septembre 2026** | Preparation dossier evaluation GAFI | [REF-76] [REF-85] | Critique | RCLI / Equipe Conformite |
| **1er novembre 2026** | **PLENIERE GAFI** — Evaluation mutuelle Tunisie | [REF-85] | Critique | RCLI / Direction Generale |
| **S2 2026** | Nouvelles normes adequation capital (Circ. 2025-08) | [REF-74] | Eleve | Equipe Prudentielle |
| **S2 2026** | Adoption prevue FIDA (Open Finance EU) | [REF-92] | Faible | Equipe Produit |
| **2027** | Nouvelles regles classification risques (annexe III) | [REF-71] | Eleve | Equipe Risques |
| **2027** | Premiere revue ISO 27001 (si certification visee) | [REF-86] | Moyen | RSSI |
| **2028** | Audit PCI DSS externe (si traitement cartes) | [REF-90] | Eleve | RSSI / QSA |
| **2030** | Conformite complete GAFI R.16 travel rule | [REF-83] | Moyen | Equipe Paiements |

### 4.2 Jalons internes du projet

| Jalon | Date cible | Modules concernes | Dependances |
|---|---|---|---|
| MVP Customer + Identity (KYC de base) | Q2 2026 | Customer, Identity | Circ. 2025-06 (e-KYC) |
| MVP Account + Accounting | Q3 2026 | Account, Accounting | NCT 21/22/24, Loi 2016-48 |
| Module AML/Sanctions operationnel | Q3 2026 | AML, Sanctions | Circ. 2025-17, Loi 2015-26 |
| Module Prudential (ratios BCT) | Q4 2026 | Prudential | Circ. 91-24, 2016-03, 2018-06, 2018-10 |
| Module Payment (SEPA/SWIFT) | Q4 2026 | Payment | R.16, PCI DSS |
| Module Credit (octroi + classification) | Q1 2027 | Credit | Circ. 91-24, 2023-02 |
| Module Reporting (etats BCT) | Q1 2027 | Reporting | Circ. 2018-09 |
| Module ForeignExchange | Q2 2027 | ForeignExchange | Loi 76-18, Circ. 2025-12 |
| Module Governance (3 lignes defense) | Q2 2027 | Governance | Circ. 2021-05, 2006-19 |
| Double moteur NCT / IFRS 9 | Q3 2027 | Accounting, Credit | Circ. 2025-08 |
| Certification ISO 27001 (optionnelle) | Q4 2027 | Tous | ISO 27001:2022 |

---

## 5. Actions prioritaires Q2-Q3 2026

Les actions ci-dessous sont classees par ordre de priorite et doivent etre lancees immediatement pour respecter les echeances critiques.

### 5.1 Actions immediates (Avril-Mai 2026)

1. **Nommer un Responsable Conformite LBC/FT (RCLI)** — Requis par la Circ. BCT 2025-17, cette personne pilotera la mise en conformite LBC/FT en vue de l'evaluation GAFI.

2. **Designer un DPO (Delegue a la Protection des Donnees)** — Obligatoire des le 11 juillet 2026 (nouvelle loi donnees), cette nomination doit etre effectuee en avance pour permettre la mise en place des processus.

3. **Implementer le module Customer avec KYC de base** — Premier bounded context a developper, il conditionne la conformite a la Circ. 2025-06 (e-KYC) et a la Circ. 2025-17 (KYC renforce LBC/FT).

4. **Mettre en place l'architecture privacy-by-design** — Chiffrement AES-256 au repos, TLS 1.3 en transit, journalisation des acces aux donnees personnelles, registre des traitements.

5. **Cartographier les risques LBC/FT** — Elaborer la cartographie des risques de blanchiment et de financement du terrorisme conformement a la Recommandation 1 du GAFI et a la Circ. 2025-17.

### 5.2 Actions Q2 2026 (Mai-Juin)

6. **Developper le module AML** — Moteur de surveillance transactionnelle, scenarios d'alerte, seuils parametrables, workflow de traitement des alertes.

7. **Developper le module Sanctions** — Screening des listes de sanctions (ONU, UE, OFAC, listes nationales), integration temps reel, procedures de gel.

8. **Integrer goAML (CTAF)** — Interface de communication avec le systeme goAML de la CTAF pour la transmission des declarations de soupcon (DOS).

9. **Implementer les droits des personnes (donnees)** — API d'exercice du droit d'acces, de rectification, d'effacement (avec exceptions LBC/FT) et de portabilite.

10. **Preparer la DPIA (Data Protection Impact Assessment)** — Evaluation d'impact pour les traitements a risque (KYC biometrique, surveillance transactionnelle, scoring).

### 5.3 Actions Q3 2026 (Juillet-Septembre)

11. **Deployer le module Account + Accounting** — Gestion des comptes, ecritures comptables conformes NCT 21/22/24, plan comptable bancaire.

12. **Implementer le calcul des ratios prudentiels** — Ratio de solvabilite (10%), Tier 1 (7%), ratio Credits/Depots (120%), ratios de concentration.

13. **Documenter les procedures LBC/FT pour l'evaluation GAFI** — Preparation du dossier de conformite LBC/FT/FP a presenter lors de l'evaluation mutuelle.

14. **Lancer les tests de penetration** — Premiers pentests sur les modules deployes, correction des vulnerabilites identifiees.

15. **Mettre en place le reporting reglementaire BCT** — Generateur d'etats reglementaires automatise (formats Circ. 2018-09).

---

## 6. Indicateurs de progression (KPIs)

### 6.1 KPIs de conformite

| Indicateur | Cible Q2 2026 | Cible Q3 2026 | Cible Q4 2026 | Cible Q2 2027 |
|---|---|---|---|---|
| % exigences P0 couvertes (Done) | 5% | 25% | 50% | 80% |
| % exigences P1 couvertes (Done) | 0% | 10% | 30% | 60% |
| Nombre de modules operationnels | 2 | 5 | 8 | 11 |
| Nombre de controles ISO 27001 implementes | 5 | 20 | 40 | 60 |
| Nombre de requirements PCI DSS satisfaits | 1 | 4 | 8 | 11 |
| Tests de securite executes | 1 | 3 | 6 | 10 |
| Couverture de tests unitaires (backend) | 60% | 75% | 80% | 85% |

### 6.2 KPIs LBC/FT (preparation GAFI)

| Indicateur | Cible | Date |
|---|---|---|
| Cartographie des risques LBC/FT completee | 100% | 31 mai 2026 |
| Module KYC/CDD operationnel | Oui | 30 juin 2026 |
| Module AML (surveillance transactionnelle) operationnel | Oui | 31 aout 2026 |
| Integration goAML (CTAF) testee | Oui | 30 septembre 2026 |
| Procedures de gel des avoirs implementees | Oui | 30 septembre 2026 |
| Screening sanctions temps reel | Oui | 31 octobre 2026 |
| Documentation conformite GAFI prete | 100% | 15 octobre 2026 |

### 6.3 KPIs Protection des donnees (preparation loi juillet 2026)

| Indicateur | Cible | Date |
|---|---|---|
| DPO designe | Oui | 30 avril 2026 |
| Registre des traitements complete | 100% | 31 mai 2026 |
| DPIA pour traitements a risque | Terminee | 30 juin 2026 |
| Mecanisme de notification 72h operationnel | Oui | 30 juin 2026 |
| API droits des personnes deployee | Oui | 30 juin 2026 |
| Politique de retention des donnees documentee | Oui | 30 juin 2026 |
| Chiffrement donnees personnelles au repos | 100% | 30 juin 2026 |

### 6.4 Tableau de suivi mensuel

| Mois | Exigences Done | Exigences In Progress | % Couverture globale | Risques ouverts | Actions en retard |
|---|---|---|---|---|---|
| Avril 2026 | 0 | 0 | 0% | 5 | 0 |
| Mai 2026 | — | — | — | — | — |
| Juin 2026 | — | — | — | — | — |
| Juillet 2026 | — | — | — | — | — |
| Aout 2026 | — | — | — | — | — |
| Septembre 2026 | — | — | — | — | — |
| Octobre 2026 | — | — | — | — | — |
| Novembre 2026 | — | — | — | — | — |
| Decembre 2026 | — | — | — | — | — |

> Ce tableau sera mis a jour mensuellement lors de la revue de conformite.

---

## 7. Liens vers la documentation detaillee

### 7.1 Documents fondateurs

| Document | Chemin | Description |
|---|---|---|
| Referentiel legal et normatif | [`docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md`](legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) | Inventaire exhaustif et source des textes legaux, reglementaires et normatifs |
| Index des references legales | [`docs/legal/legal-references-index.md`](legal/legal-references-index.md) | Index navigable de toutes les references (REF-01 a REF-95) avec URLs et dates de verification |
| Matrice de conformite globale | [`docs/compliance/overall-compliance-matrix.md`](compliance/overall-compliance-matrix.md) | Matrice croisee de toutes les exigences avec statut, module et priorite (86 exigences) |

### 7.2 Conformite ISO 27001:2022

| Document | Chemin | Description |
|---|---|---|
| Perimetre et declaration d'applicabilite | [`docs/compliance/iso-27001/01-scope-and-statement-of-applicability.md`](compliance/iso-27001/01-scope-and-statement-of-applicability.md) | Perimetre du SMSI et SoA pour les 12 bounded contexts |
| Registre d'appreciation des risques | [`docs/compliance/iso-27001/02-risk-assessment-register.md`](compliance/iso-27001/02-risk-assessment-register.md) | Registre des risques de securite de l'information |
| Mapping controles Annexe A | [`docs/compliance/iso-27001/03-controls-annex-a-mapping.md`](compliance/iso-27001/03-controls-annex-a-mapping.md) | Mapping des 93 controles de l'Annexe A avec l'implementation BANKO |
| Plan d'implementation | [`docs/compliance/iso-27001/04-implementation-plan.md`](compliance/iso-27001/04-implementation-plan.md) | Plan de mise en oeuvre du SMSI |

### 7.3 Conformite PCI DSS v4.0.1

| Document | Chemin | Description |
|---|---|---|
| Definition du perimetre CDE | [`docs/compliance/pci-dss/01-cde-scope-definition.md`](compliance/pci-dss/01-cde-scope-definition.md) | Definition du Cardholder Data Environment |
| Mapping des requirements | [`docs/compliance/pci-dss/02-requirements-mapping.md`](compliance/pci-dss/02-requirements-mapping.md) | Mapping des 12 requirements PCI DSS avec l'implementation |
| Guide tokenisation et chiffrement | [`docs/compliance/pci-dss/03-tokenization-and-encryption-guide.md`](compliance/pci-dss/03-tokenization-and-encryption-guide.md) | Guide technique de tokenisation et chiffrement des donnees carte |
| Matrice de responsabilite | [`docs/compliance/pci-dss/04-responsibility-matrix.md`](compliance/pci-dss/04-responsibility-matrix.md) | Matrice RACI pour la conformite PCI DSS |

### 7.4 Open Banking / PSD3

| Document | Chemin | Description |
|---|---|---|
| Roadmap de readiness | [`docs/compliance/open-banking-psd2/01-readiness-roadmap.md`](compliance/open-banking-psd2/01-readiness-roadmap.md) | Plan de preparation a l'Open Banking |
| Gestion du consentement | [`docs/compliance/open-banking-psd2/02-consent-management.md`](compliance/open-banking-psd2/02-consent-management.md) | Architecture de gestion du consentement client |
| SCA — Authentification forte | [`docs/compliance/open-banking-psd2/03-sca-strong-customer-authentication.md`](compliance/open-banking-psd2/03-sca-strong-customer-authentication.md) | Specifications de l'authentification forte du client |
| Specifications securite API | [`docs/compliance/open-banking-psd2/04-api-security-specifications.md`](compliance/open-banking-psd2/04-api-security-specifications.md) | Securite des APIs Open Banking |
| Mapping contexte tunisien | [`docs/compliance/open-banking-psd2/05-tunisian-open-banking-mapping.md`](compliance/open-banking-psd2/05-tunisian-open-banking-mapping.md) | Adaptation au contexte reglementaire tunisien |

### 7.5 Documentation BMAD

| Document | Chemin | Description |
|---|---|---|
| Configuration projet | [`docs/bmad/00-configuration-projet.md`](bmad/00-configuration-projet.md) | Configuration initiale du projet BMAD |
| Product Brief | [`docs/bmad/01-product-brief.md`](bmad/01-product-brief.md) | Brief produit BANKO |
| PRD (Product Requirements Document) | [`docs/bmad/02-PRD.md`](bmad/02-PRD.md) | Exigences produit detaillees |
| Architecture | [`docs/bmad/03-architecture.md`](bmad/03-architecture.md) | Architecture technique hexagonale |
| Epics et Stories | [`docs/bmad/04-epics-and-stories.md`](bmad/04-epics-and-stories.md) | Decoupage en epics et user stories |
| Rapport de validation | [`docs/bmad/05-validation-report.md`](bmad/05-validation-report.md) | Rapport de validation BMAD |

---

> **Historique des modifications**
>
> | Version | Date | Auteur | Description |
> |---|---|---|---|
> | 1.0.0 | 6 avril 2026 | GILMRY | Creation initiale — Tableau de bord executif de conformite |
>
> **Prochaine revue** : 6 mai 2026 (revue mensuelle de conformite)
>
> **Contact** : Pour toute question relative a la conformite, contacter le Responsable Conformite ou le DPO du projet.
