# BANKO — Matrice de conformite globale

> **Version** : 1.0.0 — 6 avril 2026
> **Statut** : Document de reference — Suivi de conformite transversal
> **Licence** : AGPL-3.0
> **Auteur** : GILMRY / Projet BANKO

---

## Table des matieres

1. [Objectif](#1-objectif)
2. [Legende des statuts](#2-legende-des-statuts)
3. [Matrice de conformite globale](#3-matrice-de-conformite-globale)
4. [Synthese par statut](#4-synthese-par-statut)
5. [Prochaines echeances critiques](#5-prochaines-echeances-critiques)

---

## 1. Objectif

Ce document constitue la **source unique de verite** (*single source of truth*) pour le suivi de la conformite du projet BANKO a l'ensemble des normes legales, reglementaires et internationales applicables a un systeme bancaire tunisien.

Chaque exigence normative est croisee avec son implementation dans BANKO, son statut de realisation, le ou les modules concernes (parmi les 12 bounded contexts) et sa priorite. Ce document est concu pour etre **audit-ready** : il peut etre presente tel quel a un auditeur externe, au regulateur (BCT, CTAF, CMF) ou au comite de conformite interne.

La matrice est alimentee par les documents de conformite detailles :
- `docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md` — Referentiel legal fondateur
- `docs/legal/legal-references-index.md` — Index des references
- `docs/compliance/iso-27001/` — Conformite ISO 27001:2022
- `docs/compliance/pci-dss/` — Conformite PCI DSS v4.0.1
- `docs/compliance/open-banking-psd2/` — Readiness Open Banking / PSD3

---

## 2. Legende des statuts

| Statut | Symbole | Definition |
|---|---|---|
| **Done** | `Done` | Exigence implementee dans le code, testee (tests unitaires + BDD) et documentee |
| **In Progress** | `In Progress` | Developpement en cours, partiellement implemente |
| **Planned** | `Planned` | Planifie dans la roadmap, pas encore commence |
| **N/A** | `N/A` | Non applicable au perimetre actuel de BANKO |

**Priorites** :
| Priorite | Definition |
|---|---|
| **P0 — Critique** | Prerequis legal obligatoire, bloquant pour la mise en production |
| **P1 — Elevee** | Exigence reglementaire a respecter dans les 6 mois suivant le lancement |
| **P2 — Moyenne** | Exigence a satisfaire a moyen terme (6-18 mois) |
| **P3 — Basse** | Anticipation, bonnes pratiques, exigences futures |

---

## 3. Matrice de conformite globale

### A. Reglementation bancaire tunisienne

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| Loi 2016-48 — Operations bancaires | [REF-09] Art. 1-22 | Definition et encadrement des operations bancaires (depots, credits, moyens de paiement) | Architecture metier : bounded contexts Account, Credit, Payment couvrant les 3 categories d'operations bancaires | Planned | Account, Credit, Payment | P0 — Critique |
| Loi 2016-48 — Agrement | [REF-09] Art. 23-35 | Conditions d'agrement et d'exercice, categories d'etablissements | Configuration multi-tenant permettant de parametrer le type d'etablissement (banque, EF, EP) | Planned | Governance | P0 — Critique |
| Loi 2016-48 — Gouvernance | [REF-09] Art. 49-51 | Comites obligatoires : audit, risques, nomination/remuneration | Workflows d'approbation multi-niveaux, separation des pouvoirs, piste d'audit integrale | Planned | Governance | P0 — Critique |
| Loi 2016-48 — Resolution | [REF-09] Art. 100-116 | Dispositif de resolution et de liquidation bancaire | Mecanismes de gel de comptes, reporting de resolution vers BCT/FGDB | Planned | Account, Reporting | P2 — Moyenne |
| Circ. BCT 91-24 — Division risques | [REF-14] | Ratio de concentration par beneficiaire <= 25% FPN | Moteur de calcul des engagements par contrepartie, alerte automatique au seuil de 20% et blocage a 25% | Planned | Credit, Prudential | P0 — Critique |
| Circ. BCT 91-24 — Classification creances | [REF-14] | Classification classes 0-4 avec provisionnement gradue (0%, 20%, 50%, 100%) | Moteur de classification automatique des creances, calcul des provisions reglementaires | Planned | Credit, Accounting | P0 — Critique |
| Circ. BCT 91-24 — Grands risques | [REF-14] | Total grands risques (>= 5% FPN) <= 3x FPN ; risques >= 15% FPN <= 1,5x FPN | Tableau de bord des grands risques en temps reel, reporting BCT | Planned | Prudential, Reporting | P0 — Critique |
| Circ. BCT 2016-03 — Ratio solvabilite | [REF-17] | Ratio de solvabilite global minimum 10%, Tier 1 minimum 7% | Calcul automatise des RWA et ratios de solvabilite, simulation d'impact | Planned | Prudential | P0 — Critique |
| Circ. BCT 2018-06 — Adequation fonds propres | [REF-19] | Exigences fonds propres par type de risque (credit, marche, operationnel 15% PNB) | Module de calcul prudentiel multi-risques, convergence Bale III | Planned | Prudential | P0 — Critique |
| Circ. BCT 2018-10 — Ratio Credits/Depots | [REF-21] | Ratio C/D <= 120%, mesures correctives trimestrielles si depassement | Calcul en temps reel du ratio C/D, alertes de depassement, reporting trimestriel | Planned | Prudential, Reporting | P0 — Critique |
| Circ. BCT 2023-02 — Provisionnement modifie | [REF-24] | Nouvelles regles de provisionnement (modification circulaire 91-24) | Integration des nouveaux taux et criteres dans le moteur de provisionnement | Planned | Credit, Accounting | P0 — Critique |
| Circ. BCT 2006-19 — Controle interne | [REF-35] | Systeme de controle interne permanent, comite d'audit, securite des operations | Framework de controle interne integre, journalisation exhaustive, alertes | Planned | Governance | P0 — Critique |
| Circ. BCT 2021-05 — Gouvernance | [REF-37] | 3 lignes de defense (audit, risque, conformite), comites obligatoires | Segregation des roles par ligne de defense, workflows de validation, reporting gouvernance | Planned | Governance | P0 — Critique |
| Circ. BCT 2018-09 — Reporting reglementaire | [REF-41] | Formats et frequences de reporting vers la BCT | Generateur de rapports reglementaires automatise (mensuel, trimestriel, annuel) | Planned | Reporting | P1 — Elevee |
| Circ. BCT 2025-01 — Modification annexe III risques | [REF-71] | Mise a jour de l'annexe III relative aux risques (circ. 91-24) | Adaptation des modeles de risque et des etats de reporting | Planned | Prudential, Reporting | P1 — Elevee |
| Circ. BCT 2025-03 — TuniCheque | [REF-72] | Plateforme nationale de compensation des cheques | Connecteur TuniCheque pour compensation electronique des cheques | Planned | Payment | P2 — Moyenne |
| Circ. BCT 2025-06 — e-KYC | [REF-73] | Enrolement electronique des clients, verification biometrique, tests ANCS | Module e-KYC avec verification d'identite numerique, integration biometrie et ANCS | Planned | Customer, Identity | P0 — Critique |
| Circ. BCT 2025-08 — Reforme prudentielle | [REF-74] | Transition IFRS 9, nouvelles normes d'adequation du capital applicables 2026 | Double moteur comptable NCT/IFRS 9, recalcul des exigences en capital | Planned | Prudential, Accounting | P0 — Critique |
| Circ. BCT 2025-12 — Plateforme numerique change | [REF-75] | Plateforme numerique pour les operations de change | API d'integration avec la plateforme BCT de change numerique | Planned | ForeignExchange | P1 — Elevee |
| Circ. BCT 2025-17 — LBC/FT/FP | [REF-76] | Refonte complete du dispositif LBC/FT, inclusion lutte contre la proliferation (FP), goAML, gel avoirs | Moteur de surveillance transactionnelle, integration goAML (CTAF), procedures de gel, screening proliferation | Planned | AML, Sanctions, Customer | P0 — Critique |
| Circ. BCT 2026-02 — LBC/FT bureaux de change | [REF-77] | Extension LBC/FT aux bureaux de change, screening sanctions obligatoire | Screening sanctions etendu aux operations de change manuel | Planned | Sanctions, ForeignExchange | P1 — Elevee |
| Circ. BCT 2026-04 — Autofinancement importations | [REF-78] | Exigences d'autofinancement pour importations non-prioritaires | Controles de conformite change sur les operations d'importation | Planned | ForeignExchange, Payment | P2 — Moyenne |

### B. LBC/FT (Anti-blanchiment et financement du terrorisme)

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| Loi 2015-26 — Criminalisation blanchiment | [REF-28] Art. 62-98 | Incrimination du blanchiment d'argent et du financement du terrorisme | Detection des typologies de blanchiment, alertes automatiques | Planned | AML | P0 — Critique |
| Loi 2015-26 — Creation CTAF | [REF-28] Art. 118 | Institution de la CTAF comme cellule de renseignement financier | Interface de communication securisee avec la CTAF | Planned | AML, Reporting | P0 — Critique |
| Loi 2015-26 — Declaration de soupcon | [REF-28] Art. 125 | Obligation de declaration de soupcon aupres de la CTAF | Module de generation et transmission des declarations de soupcon (DOS) | Planned | AML | P0 — Critique |
| Loi 2015-26 — Conservation 10 ans | [REF-28] Art. 130 | Conservation des donnees et documents pendant 10 ans minimum | Politique de retention des donnees (10 ans), archivage securise, non-suppression | Planned | AML, Customer | P0 — Critique |
| Loi 2015-26 — Vigilance CDD | [REF-28] Art. 99-115 | Obligations de vigilance (CDD) : identification, verification, suivi continu | Pipeline KYC complet : identification, verification documentaire, profilage risque | Planned | Customer, AML | P0 — Critique |
| Loi 2019-9 — Renforcement LBC/FT | [REF-30] | Extension des obligations de vigilance, elargissement des infractions sous-jacentes | Mise a jour des scenarios de surveillance, couverture elargie des infractions | Planned | AML | P0 — Critique |
| GAFI 40 Recommandations — Approche risques | [REF-64] R.1 | Evaluation des risques BC/FT a l'echelle de l'etablissement, mesures proportionnees | Cartographie des risques LBC/FT integree, scoring risque client (faible/moyen/eleve/tres eleve) | Planned | AML, Customer | P0 — Critique |
| GAFI R.1 mise a jour fev 2025 — Inclusion financiere | [REF-65] | Mesures simplifiees pour les produits a faible risque, promotion inclusion financiere | Parcours KYC simplifie pour les produits a risque limite (comptes de base) | Planned | Customer, AML | P1 — Elevee |
| GAFI R.10-12 — CDD/EDD | [REF-64] | Vigilance standard et renforcee, personnes politiquement exposees (PPE) | Detection automatique des PPE, EDD avec approbation hierarchique, revue periodique | Planned | Customer, AML | P0 — Critique |
| GAFI R.16 revisee juin 2025 — Travel rule | [REF-83] | Travel rule elargie : donnees originator/beneficiary obligatoires pour tous les virements | Enrichissement des messages de paiement avec donnees originator/beneficiary conformes R.16 | Planned | Payment, AML | P1 — Elevee |
| GAFI R.20 — Declaration operations suspectes | [REF-64] | Declaration rapide et protegee des operations suspectes | Workflow DOS automatise, protection du declarant, delai de transmission < 24h | Planned | AML | P0 — Critique |
| CMF Strategie 2026 — Controles LBC/FT | [REF-81] | 60+ missions de controle prevues, guides sectoriels | Documentation de conformite preparee pour inspections, piste d'audit complete | Planned | AML, Governance | P1 — Elevee |

### C. Protection des donnees personnelles

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| Loi 2004-63 — Protection donnees | [REF-54] | Base legale pour tout traitement, consentement, droits d'acces/rectification/opposition | Module de gestion du consentement, API exercice des droits (acces, rectification, opposition) | Planned | Customer, Identity | P1 — Elevee |
| Loi 2004-63 — Declaration INPDP | [REF-54] Art. 7 | Declaration prealable des traitements aupres de l'INPDP | Registre des traitements automatise, templates de declaration INPDP | Planned | Governance | P1 — Elevee |
| Loi 2004-63 — Donnees sensibles | [REF-54] Art. 14 | Interdiction de traitement des donnees sensibles sauf exceptions limitees | Classification automatique des donnees sensibles, controles d'acces renforces | Planned | Customer, Identity | P1 — Elevee |
| Nouvelle loi donnees 2025 — DPO obligatoire | [REF-79] | Designation d'un delegue a la protection des donnees (DPO) | Role DPO dans le module de gouvernance, tableau de bord dedie | Planned | Governance | P0 — Critique |
| Nouvelle loi donnees 2025 — DPIA | [REF-79] | Analyse d'impact relative a la protection des donnees pour traitements a risque | Outil de DPIA integre, templates d'evaluation d'impact | Planned | Governance | P0 — Critique |
| Nouvelle loi donnees 2025 — Notification 72h | [REF-79] | Notification de violation de donnees dans les 72 heures | Systeme d'alerte et de notification de brèches, workflow de signalement | Planned | Identity, Governance | P0 — Critique |
| Nouvelle loi donnees 2025 — Amendes CA | [REF-79] | Sanctions financieres proportionnelles au chiffre d'affaires | Controles preventifs renforces, audit trail exhaustif pour demonstration de conformite | Planned | Governance | P0 — Critique |
| Nouvelle loi donnees 2025 — Effacement et portabilite | [REF-79] | Droit a l'effacement et droit a la portabilite des donnees | API d'effacement (avec conservation legale LBC/FT), export de donnees en format structure | Planned | Customer, Identity | P0 — Critique |
| Convention 108+ Conseil de l'Europe | [REF-57] | Principes de protection des donnees transfrontaliers | Conformite aux principes de la Convention 108+ dans l'architecture privacy-by-design | Planned | Customer, Identity | P2 — Moyenne |

### D. Normes comptables

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| NCT 21 — Etats financiers bancaires | [REF-46] | Presentation normalisee des etats financiers des etablissements bancaires | Generateur d'etats financiers conforme NCT 21 (bilan, resultat, flux) | Planned | Accounting, Reporting | P0 — Critique |
| NCT 22 — Controle interne comptable | [REF-45] | Organisation du controle interne comptable dans les etablissements bancaires | Controles de coherence comptable automatises, rapprochements, piste d'audit | Planned | Accounting | P0 — Critique |
| NCT 24 — Engagements bancaires | [REF-46] | Comptabilisation des engagements et revenus y afferents | Moteur comptable pour enregistrement des engagements (credits, garanties, hors-bilan) | Planned | Accounting, Credit | P0 — Critique |
| NCT 25 — Portefeuille-titres | [REF-46] | Traitement comptable du portefeuille-titres des banques | Module de comptabilisation des titres (transaction, placement, investissement) | Planned | Accounting | P2 — Moyenne |
| Loi 96-112 — Systeme comptable | [REF-13] | Cadre conceptuel comptable tunisien, obligations de tenue comptable | Architecture comptable conforme au cadre conceptuel tunisien | Planned | Accounting | P0 — Critique |
| IFRS 9 — Pertes attendues ECL | [REF-47] | Modele de depreciation ECL (Expected Credit Losses) en 3 stages | Moteur de calcul ECL : stage 1 (12 mois), stage 2 (lifetime), stage 3 (defaut), modeles PD/LGD/EAD | Planned | Credit, Accounting | P0 — Critique |
| IFRS 9 — Classification instruments | [REF-47] | Classification et evaluation des instruments financiers (cout amorti, FVOCI, FVTPL) | Moteur de classification automatique selon le modele economique et les flux contractuels | Planned | Accounting | P1 — Elevee |
| IFRS 7 — Informations a fournir | [REF-48] | Obligations de divulgation sur les instruments financiers | Generateur de notes annexes IFRS 7, informations sur le risque de credit | Planned | Reporting | P2 — Moyenne |

### E. Changes

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| Loi 76-18 — Code des changes | [REF-50] | Toutes operations de change via BCT ou intermediaires agrees, controle mouvements de capitaux | Module de gestion des operations en devises avec controles de conformite change | Planned | ForeignExchange | P1 — Elevee |
| Decret 77-608 — Application code changes | [REF-49] | Conditions d'application de la loi 76-18 | Parametrage des regles de change (plafonds, autorisations requises) | Planned | ForeignExchange | P1 — Elevee |
| Circ. BCT 2018-07 — Change manuel | [REF-53] | Reglementation de l'activite de change manuel | Gestion des operations de change manuel, taux directeurs BCT | Planned | ForeignExchange | P1 — Elevee |
| Loi Finances 2026 — Comptes devises residents | [REF-80] | Ouverture de comptes en devises pour residents sans autorisation BCT | Configuration des comptes multi-devises pour residents, controles allegees | Planned | Account, ForeignExchange | P1 — Elevee |
| Circ. BCT 2025-12 — Plateforme change numerique | [REF-75] | Plateforme numerique centralisee pour les operations de change | Connecteur API vers la plateforme BCT de change numerique | Planned | ForeignExchange | P1 — Elevee |

### F. ISO 27001:2022 — Securite de l'information

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| ISO 27001 — Clause 4 | [REF-86] 4.1-4.4 | Contexte de l'organisme, parties interessees, perimetre du SMSI | Documentation du perimetre SMSI couvrant les 12 bounded contexts | Planned | Governance | P1 — Elevee |
| ISO 27001 — Clause 5 | [REF-86] 5.1-5.3 | Leadership, politique de securite, roles et responsabilites | Politique de securite documentee, matrice RACI securite | Planned | Governance | P1 — Elevee |
| ISO 27001 — Clause 6 | [REF-86] 6.1-6.3 | Planification : appreciation des risques, traitement des risques, objectifs | Registre des risques SI, plan de traitement, objectifs securite mesurables | Planned | Governance | P1 — Elevee |
| ISO 27001 — Clause 7 | [REF-86] 7.1-7.5 | Support : ressources, competences, sensibilisation, communication, documentation | Programmes de formation securite, gestion documentaire du SMSI | Planned | Governance | P2 — Moyenne |
| ISO 27001 — Clause 8 | [REF-86] 8.1-8.3 | Realisation : planification operationnelle, appreciation risques, traitement | Processus operationnels de securite integres dans le pipeline CI/CD | Planned | Governance | P1 — Elevee |
| ISO 27001 — Clause 9 | [REF-86] 9.1-9.3 | Evaluation des performances : surveillance, audit interne, revue de direction | Metriques de securite sur /metrics, audits internes programmes | Planned | Governance | P1 — Elevee |
| ISO 27001 — Clause 10 | [REF-86] 10.1-10.2 | Amelioration : non-conformites, actions correctives, amelioration continue | Registre des non-conformites, workflow de correction, suivi | Planned | Governance | P1 — Elevee |
| ISO 27001 Annexe A — A.5 Controles organisationnels | [REF-86] A.5.1-A.5.37 | 37 controles organisationnels (politiques, roles, classification, fournisseurs, continuite) | Politiques de securite codifiees, classification des actifs informationnels, gestion fournisseurs | Planned | Governance | P1 — Elevee |
| ISO 27001 Annexe A — A.6 Controles personnes | [REF-86] A.6.1-A.6.8 | 8 controles lies au personnel (selection, T&C, sensibilisation, disciplinaire, fin de contrat) | Procedures RH de securite, habilitations liees au cycle de vie employe | Planned | Governance, Identity | P2 — Moyenne |
| ISO 27001 Annexe A — A.7 Controles physiques | [REF-86] A.7.1-A.7.14 | 14 controles physiques (perimetres, acces physique, bureaux, stockage, utilites) | N/A pour le logiciel open source ; guide de recommandations pour les deploiements | N/A | — | P3 — Basse |
| ISO 27001 Annexe A — A.8 Controles technologiques | [REF-86] A.8.1-A.8.34 | 34 controles technologiques (endpoint, acces privilegies, code source, chiffrement, logs, reseaux) | Chiffrement AES-256 au repos, TLS 1.3 en transit, RBAC, journalisation exhaustive, SAST/DAST | Planned | Identity, Governance | P0 — Critique |
| ISO 27001:2022/Amd 1:2024 — Changement climatique | [REF-87] | Prise en compte du changement climatique dans l'analyse du contexte (clause 4.1/4.2) | Analyse des risques climatiques sur l'infrastructure (centres de donnees, continuite) | Planned | Governance | P3 — Basse |
| ISO 27701:2025 — Vie privee | [REF-89] | Extension vie privee de l'ISO 27001, devenue norme standalone, roles PII controller/processor | Privacy Information Management System (PIMS) integre, registre des traitements PII | Planned | Customer, Identity, Governance | P1 — Elevee |

### G. PCI DSS v4.0.1

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| PCI DSS Req 1 — Securite reseau | [REF-90] Req 1 | Installation et maintenance de controles de securite reseau (firewalls, segmentation) | Configuration Traefik avec regles de segmentation, isolation du CDE | Planned | Payment | P1 — Elevee |
| PCI DSS Req 2 — Configurations securisees | [REF-90] Req 2 | Application de configurations securisees a tous les composants systeme | Hardening des conteneurs Docker, suppression des defaults, CIS benchmarks | Planned | Payment | P1 — Elevee |
| PCI DSS Req 3 — Protection donnees stockees | [REF-90] Req 3 | Chiffrement des donnees de carte stockees, chiffrement au niveau du champ obligatoire (v4.0.1) | Chiffrement AES-256 au niveau du champ pour PAN, tokenisation des donnees carte | Planned | Payment, Account | P0 — Critique |
| PCI DSS Req 4 — Chiffrement en transit | [REF-90] Req 4 | Protection des donnees en transit via TLS 1.2+ (TLS 1.3 recommande) | TLS 1.3 obligatoire sur toutes les connexions, HSTS, certificate pinning | Planned | Payment | P0 — Critique |
| PCI DSS Req 5 — Protection contre les malwares | [REF-90] Req 5 | Protection de tous les systemes contre les logiciels malveillants | Scanning des images conteneurs, signatures de vulnerabilites, mise a jour automatique | Planned | Payment | P1 — Elevee |
| PCI DSS Req 6 — Developpement securise | [REF-90] Req 6 | Developpement et maintenance de logiciels securises | Pipeline SAST (clippy, cargo-audit), DAST, revue de code obligatoire, dependances verifiees | Planned | Payment | P0 — Critique |
| PCI DSS Req 7 — Controle d'acces | [REF-90] Req 7 | Restriction de l'acces aux donnees de carte au besoin strict | RBAC granulaire, principe du moindre privilege, revue d'acces periodique | Planned | Payment, Identity | P0 — Critique |
| PCI DSS Req 8 — Authentification | [REF-90] Req 8 | Identification et authentification de l'acces aux composants systeme, MFA pour acces CDE | MFA obligatoire pour acces CDE, politiques de mots de passe, session timeout | Planned | Identity | P0 — Critique |
| PCI DSS Req 9 — Acces physique | [REF-90] Req 9 | Restriction de l'acces physique aux donnees de carte | N/A pour le logiciel ; guide de recommandations pour les deploiements sur site | N/A | — | P3 — Basse |
| PCI DSS Req 10 — Journalisation et surveillance | [REF-90] Req 10 | Journalisation et surveillance de tous les acces aux ressources reseau et aux donnees de carte | Journalisation structuree de tous les acces, integration SIEM, retention 12 mois, alertes | Planned | Payment, Governance | P0 — Critique |
| PCI DSS Req 11 — Tests de securite | [REF-90] Req 11 | Tests reguliers des systemes et reseaux de securite (pentest, scans vulnerabilites) | Scans de vulnerabilites automatises (CI/CD), pentests semestriels, programme de bug bounty | Planned | Payment | P1 — Elevee |
| PCI DSS Req 12 — Politiques organisationnelles | [REF-90] Req 12 | Politique de securite de l'information, gestion des risques, sensibilisation | Politiques de securite documentees, programme de sensibilisation, gestion des incidents | Planned | Governance | P1 — Elevee |

### H. Open Banking / PSD3

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| APIs standardisees Open Banking | [REF-91] | APIs ouvertes et documentees pour l'acces aux comptes et aux paiements (XS2A) | API REST documentee OpenAPI 3.1, endpoints Account Information et Payment Initiation | Planned | Account, Payment | P2 — Moyenne |
| Consent Management | [REF-91] | Gestion du consentement client pour le partage de donnees avec les TPP | Module de gestion des consentements (creation, revocation, expiration, historique) | Planned | Customer, Identity | P2 — Moyenne |
| SCA — Authentification forte | [REF-91] | Authentification forte du client (2 facteurs parmi connaissance, possession, inherence) | Framework SCA : OTP + biometrie, exemptions basees sur le risque (TRA) | Planned | Identity | P1 — Elevee |
| TPP Onboarding | [REF-91] | Enregistrement et gestion des prestataires tiers (AISP, PISP, CISP) | Portail d'enregistrement TPP, verification des agrements, gestion des certificats eIDAS | Planned | Identity, Governance | P3 — Basse |
| FIDA — Open Finance etendu | [REF-92] | Extension de l'acces aux donnees au-dela des comptes de paiement (epargne, credits, assurance) | Architecture extensible preparee pour l'exposition de donnees multi-produits | Planned | Account, Credit | P3 — Basse |

### I. Bale III

| Norme | Reference | Exigence cle | Implementation dans BANKO | Statut | Module concerne | Priorite |
|---|---|---|---|---|---|---|
| Bale III — Pilier 1 CET1 | [REF-61] | Ratio Common Equity Tier 1 minimum 4,5% des RWA | Calcul automatise du CET1, decomposition par composante | Planned | Prudential | P0 — Critique |
| Bale III — Pilier 1 Tier 1 | [REF-61] | Ratio Tier 1 minimum 6% des RWA (7% en Tunisie par Circ. 2016-03) | Calcul du ratio Tier 1 avec seuil tunisien de 7% | Planned | Prudential | P0 — Critique |
| Bale III — Pilier 1 Total Capital | [REF-61] | Ratio total de capital minimum 8% des RWA (10% en Tunisie) | Calcul du ratio total avec seuil tunisien de 10%, buffer de conservation | Planned | Prudential | P0 — Critique |
| Bale III — Pilier 1 RWA Credit | [REF-61] | Calcul des actifs ponderes par le risque pour le risque de credit | Moteur de calcul RWA : approche standard, ponderations par categorie d'actif | Planned | Prudential, Credit | P0 — Critique |
| Bale III — Pilier 1 Risque Operationnel | [REF-61] | Exigence en fonds propres pour le risque operationnel | Calcul par methode indicateur de base (15% PNB moyen 3 ans) | Planned | Prudential | P0 — Critique |
| Bale III — Pilier 1 Risque de Marche | [REF-61] | Exigence en fonds propres pour les risques de marche | Calcul des exigences pour risque de taux d'interet et positions en actions | Planned | Prudential | P1 — Elevee |
| Bale III — Pilier 2 ICAAP | [REF-61] | Processus interne d'adequation des fonds propres | Framework ICAAP avec stress tests, buffer de capital supplementaire | Planned | Prudential, Governance | P1 — Elevee |
| Bale III — Pilier 3 Discipline de marche | [REF-61] | Publication d'informations sur les risques, les fonds propres et la gouvernance | Generateur de rapports Pilier 3 (composition du capital, RWA, exposition au risque) | Planned | Reporting | P2 — Moyenne |
| Bale III — LCR | [REF-61] | Ratio de couverture de liquidite : HQLA / sorties nettes 30j >= 100% | Calcul du LCR, classification des actifs liquides (Level 1, 2A, 2B), projection des flux | Planned | Prudential | P1 — Elevee |
| Bale III — NSFR | [REF-61] | Ratio de financement stable net : ASF / RSF >= 100% | Calcul du NSFR, classification des financements stables disponibles et requis | Planned | Prudential | P2 — Moyenne |

---

## 4. Synthese par statut

| Statut | Nombre d'exigences | Pourcentage |
|---|---|---|
| **Done** | 0 | 0,0% |
| **In Progress** | 0 | 0,0% |
| **Planned** | 83 | 96,5% |
| **N/A** | 3 | 3,5% |
| **Total** | **86** | **100%** |

### Repartition par priorite

| Priorite | Nombre | Pourcentage |
|---|---|---|
| **P0 — Critique** | 46 | 53,5% |
| **P1 — Elevee** | 27 | 31,4% |
| **P2 — Moyenne** | 8 | 9,3% |
| **P3 — Basse** | 5 | 5,8% |
| **Total** | **86** | **100%** |

### Repartition par categorie normative

| Categorie | Nombre d'exigences | P0 | P1 | P2 | P3 |
|---|---|---|---|---|---|
| A. Reglementation bancaire tunisienne | 22 | 14 | 5 | 3 | 0 |
| B. LBC/FT | 12 | 9 | 3 | 0 | 0 |
| C. Protection des donnees | 9 | 5 | 2 | 1 | 0 | 1 |
| D. Normes comptables | 8 | 5 | 1 | 2 | 0 |
| E. Changes | 5 | 0 | 4 | 0 | 1 |
| F. ISO 27001:2022 | 13 | 1 | 8 | 2 | 2 |
| G. PCI DSS v4.0.1 | 12 | 6 | 4 | 0 | 2 |
| H. Open Banking / PSD3 | 5 | 0 | 1 | 2 | 2 |
| I. Bale III | 10 | 5 | 3 | 2 | 0 |

### Repartition par module BANKO

| Module | Nombre d'exigences concernees |
|---|---|
| **Prudential** | 18 |
| **Governance** | 17 |
| **AML** | 14 |
| **Payment** | 13 |
| **Customer** | 12 |
| **Accounting** | 10 |
| **Identity** | 10 |
| **Credit** | 9 |
| **Reporting** | 9 |
| **ForeignExchange** | 7 |
| **Account** | 6 |
| **Sanctions** | 3 |

> **Note** : Un meme exigence peut concerner plusieurs modules ; les totaux ci-dessus excedent donc 86.

---

## 5. Prochaines echeances critiques

| Date | Echeance | Norme | Impact | Action requise |
|---|---|---|---|---|
| **11 juillet 2026** | Application de la nouvelle loi sur les donnees personnelles | [REF-79] Nouvelle loi donnees 2025 | **Critique** — Amendes proportionnelles au CA, DPO obligatoire, notification 72h | Implementer les modules DPIA, DPO, notification de breches, droit a l'effacement et portabilite avant le 1er juillet 2026 |
| **1er novembre 2026** | Pleniere GAFI — Evaluation mutuelle de la Tunisie | [REF-85] 5eme cycle evaluations mutuelles | **Critique** — Risque d'inscription sur liste grise GAFI si deficiences detectees | Conformite complete LBC/FT/FP (Circ. 2025-17), integration goAML, documentation des procedures |
| **2026 (S2)** | Nouvelles normes d'adequation du capital | [REF-74] Circ. BCT 2025-08 | **Eleve** — Nouvelles exigences de capital, transition IFRS 9 | Deployer le double moteur comptable NCT/IFRS 9, recalculer les exigences en capital |
| **2027** | Nouvelles regles de classification des risques | [REF-71] Circ. BCT 2025-01 | **Eleve** — Modification de l'annexe III, impact sur le provisionnement | Adapter les modeles de classification et les etats de reporting |
| **31 mars 2025 (passe)** | Exigences PCI DSS v4.0.1 obligatoires | [REF-90] PCI DSS v4.0.1 | **Eleve** — Chiffrement au niveau du champ, MFA pour CDE | Implementer le chiffrement PAN au niveau du champ et le MFA pour tout acces au CDE |
| **2030** | Conformite complete R.16 travel rule | [REF-83] GAFI R.16 revisee | **Moyen** — Donnees originator/beneficiary obligatoires pour tous les virements | Integration progressive dans les messages de paiement SEPA/SWIFT |
| **H1 2026** | Adoption FIDA (Open Finance) | [REF-92] FIDA proposition | **Faible** — Pas encore reglemente en Tunisie, anticipation | Veille reglementaire, architecture extensible |

---

> **Historique des modifications**
>
> | Version | Date | Auteur | Description |
> |---|---|---|---|
> | 1.0.0 | 6 avril 2026 | GILMRY | Creation initiale — 86 exigences, 9 categories normatives |
>
> **Document suivant** : [`docs/compliance-dashboard.md`](../compliance-dashboard.md) — Tableau de bord executif
