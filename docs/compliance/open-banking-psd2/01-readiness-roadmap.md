# Feuille de Route Open Banking -- Preparedness BANKO

| Metadata | Valeur |
|---|---|
| **Version** | 1.0.0 |
| **Date** | 6 avril 2026 |
| **Statut** | En vigueur |
| **Classification** | Interne -- Diffusion restreinte |
| **Auteur** | Equipe Architecture BANKO |
| **Approbation** | Comite de conformite |

---

## Table des matieres

1. [Contexte -- L'Open Banking dans le monde en 2026](#1-contexte--lopen-banking-dans-le-monde-en-2026)
2. [Etat des lieux en Tunisie](#2-etat-des-lieux-en-tunisie)
3. [Pourquoi BANKO doit etre Open-Banking-Ready](#3-pourquoi-banko-doit-etre-open-banking-ready)
4. [Roadmap de preparation 2026-2030](#4-roadmap-de-preparation-2026-2030)
5. [Modele PSD3/PSR comme reference](#5-modele-psd3psr-comme-reference)
6. [Benchmarks regionaux](#6-benchmarks-regionaux)
7. [Risques et prerequis](#7-risques-et-prerequis)

---

## 1. Contexte -- L'Open Banking dans le monde en 2026

L'Open Banking connait en 2026 une acceleration sans precedent a l'echelle mondiale. Le cadre reglementaire de l'Union europeenne, longtemps considere comme le modele de reference, entre dans une phase de maturite avec l'adoption de PSD3 et du reglement FIDA, tandis que de nouvelles juridictions en Afrique, au Moyen-Orient et en Amerique du Nord adoptent leurs propres approches.

### 1.1 Union europeenne -- PSD3, PSR et FIDA

L'accord provisoire sur le paquet PSD3/PSR a ete conclu le 27 novembre 2025. La publication au Journal officiel de l'UE est prevue pour le deuxieme trimestre 2026. Ce nouveau cadre :

- **Abroge PSD2 et la Directive Monnaie Electronique (EMD)** pour les fusionner en un cadre unique.
- **Standardise les APIs** avec des obligations anti-obstruction renforcees pour les banques.
- **Introduit les tableaux de bord client** (customer dashboards) pour la gestion centralisee des consentements.
- **Impose la verification IBAN-nom** (Verification of Payee -- VoP) pour lutter contre la fraude aux virements.
- **Ameliore le SCA** (Strong Customer Authentication) avec des exemptions mieux calibrees.

En parallele, le reglement **FIDA** (Financial Data Access), dont l'adoption est attendue au premier semestre 2026, etend le partage de donnees au-dela des comptes de paiement vers le credit, l'assurance, les investissements et les retraites. Il cree une nouvelle categorie d'acteurs : les **FISP** (Financial Information Service Providers).

| Texte | Statut (avril 2026) | Entree en application prevue | Perimetre |
|---|---|---|---|
| PSD3 (Directive) | Accord provisoire (nov. 2025) | ~2028 (apres transposition) | Services de paiement, statut TPP |
| PSR (Reglement) | Accord provisoire (nov. 2025) | ~2027 (application directe) | SCA, droits consommateurs, API |
| FIDA (Reglement) | Adoption attendue H1 2026 | ~2028 | Partage donnees financieres elargi |

### 1.2 Royaume-Uni

Le Royaume-Uni, pionnier mondial de l'Open Banking depuis 2018 (CMA Order), evolue vers un modele **Smart Data** avec le Data Protection and Digital Information Act. L'Open Banking Implementation Entity (OBIE) a ete remplacee par Open Banking Limited, et le perimetre s'etend progressivement vers l'Open Finance sous la supervision de la FCA.

### 1.3 Etats-Unis

La regle finale de la Section 1033 du Consumer Financial Protection Act, publiee par le CFPB en octobre 2024, etablit pour la premiere fois un droit d'acces aux donnees financieres personnelles. L'implementation s'echelonne entre 2026 et 2030, en commencant par les plus grands etablissements.

### 1.4 Afrique et MENA

| Pays/Region | Statut Open Banking | Autorite | Annee cle |
|---|---|---|---|
| **Nigeria** | Lignes directrices publiees | CBN (Central Bank of Nigeria) | 2021 |
| **Afrique du Sud** | Approche phasee en cours | SARB / FSCA | 2023 |
| **Rwanda** | Cadre publie | BNR | 2024 |
| **Arabie Saoudite** | Premiere licence delivree | SAMA | Mars 2026 |
| **Emirats Arabes Unis** | Cadre ADGM operationnel | CBUAE / ADGM | 2023 |
| **Bahrein** | Operationnel | CBB | 2020 |
| **Tunisie** | Pas de reglementation formelle | BCT | -- |

---

## 2. Etat des lieux en Tunisie

### 2.1 Absence de reglementation formelle

A la date de redaction de ce document (avril 2026), la Tunisie ne dispose d'**aucun cadre reglementaire formel** regissant l'Open Banking. Le concept de "DSP2 tunisifiee" est discute depuis 2018 dans les cercles institutionnels et professionnels, mais aucun texte legislatif ou reglementaire n'a ete promulgue.

### 2.2 Initiatives de la BCT en matiere de fintech

La Banque Centrale de Tunisie (BCT) a neanmoins pose des jalons significatifs :

| Initiative | Description | Date | Impact |
|---|---|---|---|
| **BCT Sandbox** | Environnement de test reglementaire pour les fintechs | Actif depuis janvier 2020 | Innovation encadree |
| **BCT-Lab** | Laboratoire d'innovation de la BCT | Operationnel | Accompagnement fintechs |
| **Portail fintech.bct.gov.tn** | Plateforme d'information et de candidature | En ligne | Transparence reglementaire |
| **Agrements paiement** | 16 etablissements de paiement agrees | Cumul 2020-2026 | Diversification acteurs |

### 2.3 Dynamique du marche tunisien

Le secteur fintech tunisien connait une croissance remarquable malgre l'absence de cadre Open Banking :

- **Croissance des paiements mobiles** : +81% en volume sur 2025, portee par les plateformes OFT Walletii, Kashy et d'autres acteurs agrees.
- **TuniCheque** (Circulaire BCT 2025-03) : dematerialisation du cheque, partage d'information sur les comptes.
- **e-KYC** (Circulaire BCT 2025-06) : identification electronique avec biometrie, tests d'intrusion ANCS obligatoires.
- **Plateforme change numerique** : autorisation de transactions de change en ligne.
- **Adoption ISO 20022** : migration progressive des formats de messages financiers.

### 2.4 Nouvelle loi sur la protection des donnees personnelles

La loi sur la protection des donnees personnelles adoptee en **juin 2025** (entree en vigueur des sanctions en **juillet 2026**) constitue un socle essentiel pour l'Open Banking :

| Disposition | Description | Pertinence Open Banking |
|---|---|---|
| Consentement explicite | Obligation de consentement libre, eclaire, specifique | Fondement du partage de donnees |
| Designation d'un DPO | Obligation pour les operateurs de donnees | Gouvernance des donnees partagees |
| DPIA obligatoire | Etude d'impact avant traitements a risque | APIs de partage de donnees |
| Notification 72h | Obligation de notification en cas de violation | Incidents securite API |
| Droit a la portabilite | Droit de recuperer ses donnees | Coeur de l'Open Banking |

---

## 3. Pourquoi BANKO doit etre Open-Banking-Ready

### 3.1 Avantage competitif

L'adoption proactive des standards Open Banking confere a BANKO et a ses banques utilisatrices un avantage strategique :

| Avantage | Description | Beneficiaires |
|---|---|---|
| **Differenciation** | Premiere plateforme core banking tunisienne avec APIs ouvertes | Banques utilisatrices |
| **Ecosysteme** | Capacite d'integration avec fintechs et TPP | Fintechs, clients finaux |
| **Export regional** | Conformite aux standards africains et MENA | Expansion geographique |
| **Attractivite** | Attirance de talents et partenaires internationaux | Equipe, investisseurs |

### 3.2 Anticipation reglementaire

L'experience internationale demontre que les reglementations Open Banking sont adoptees avec des delais de mise en conformite courts (12-24 mois typiquement). Les banques preparees beneficient d'un avantage temporel considerable.

### 3.3 Inclusion financiere

La Tunisie affiche un taux de bancarisation d'environ 37% de la population adulte. L'Open Banking, en favorisant l'emergence de services financiers accessibles via mobile, constitue un levier majeur d'inclusion financiere.

---

## 4. Roadmap de preparation 2026-2030

### 4.1 Vue d'ensemble

| Phase | Periode | Objectif principal | Statut |
|---|---|---|---|
| **Phase 1** | 2026 | APIs internes standardisees, architecture ouverte | En cours |
| **Phase 2** | 2027 | APIs tierces en sandbox, consent management | Planifie |
| **Phase 3** | 2028 | TPP onboarding, support AISP/PISP | Planifie |
| **Phase 4** | 2029-2030 | Open Finance (credit, FX, data sharing elargi) | Vision |

### 4.2 Phase 1 -- Fondations (2026)

**Objectif** : Construire les fondations techniques pour l'ouverture future.

| Livrable | Description | Bounded Context | Priorite |
|---|---|---|---|
| API Gateway | Mise en place d'un gateway API avec Traefik/Kong | Infrastructure | Critique |
| APIs REST standardisees | Endpoints conformes aux patterns NextGenPSD2 | Account, Payment, Customer | Critique |
| Documentation OpenAPI | Specifications OpenAPI 3.1 pour toutes les APIs | Transversal | Haute |
| Modele de consentement | Schema de donnees ConsentRecord v1 | Governance | Haute |
| Journalisation audit | Logs immutables d'acces aux donnees | Governance | Haute |
| Formats ISO 20022 | Support des messages pacs.008, camt.053 | Payment, Accounting | Moyenne |
| Portail developpeur (MVP) | Documentation interactive des APIs | Infrastructure | Moyenne |

### 4.3 Phase 2 -- Ouverture controlee (2027)

**Objectif** : Permettre l'acces tiers en environnement controle.

| Livrable | Description | Bounded Context | Priorite |
|---|---|---|---|
| Sandbox TPP | Environnement de test isole pour les tiers | Infrastructure | Critique |
| Consent Management Service | Service complet de gestion du consentement | Governance, Identity | Critique |
| OAuth 2.0 + PKCE | Flux d'autorisation pour les TPP | Identity | Critique |
| Dashboard client | Interface de gestion des consentements | Customer, Governance | Haute |
| mTLS pour TPP | Authentification mutuelle par certificats | Infrastructure | Haute |
| Rate limiting par TPP | Controle de debit par tiers | Infrastructure | Haute |
| Webhook notifications | Notifications evenementielles pour les TPP | Transversal | Moyenne |

### 4.4 Phase 3 -- Conformite TPP (2028)

**Objectif** : Supporter le cycle de vie complet des TPP.

| Livrable | Description | Bounded Context | Priorite |
|---|---|---|---|
| TPP Registry | Registre des tiers autorises | Governance | Critique |
| AISP Support | Acces aux informations de compte pour les TPP | Account | Critique |
| PISP Support | Initiation de paiement par les TPP | Payment | Critique |
| SCA dynamique | Lien dynamique montant-beneficiaire | Identity | Haute |
| VoP (Verification of Payee) | Verification IBAN-nom avant virement | Payment | Haute |
| Dispute Resolution | Mecanisme de gestion des litiges TPP | Governance | Moyenne |
| Reporting BCT | Rapports d'activite Open Banking pour la BCT | Reporting | Moyenne |

### 4.5 Phase 4 -- Open Finance (2029-2030)

**Objectif** : Etendre le partage de donnees au-dela des comptes de paiement.

| Livrable | Description | Bounded Context | Priorite |
|---|---|---|---|
| Credit Data Sharing | Partage de donnees de credit (scoring, historique) | Credit | Haute |
| FX Data APIs | APIs de taux de change et operations | ForeignExchange | Haute |
| FISP Support | Nouveau role de fournisseur de services d'information | Governance | Haute |
| Multi-bank aggregation | Agregation de comptes multi-banques | Account | Moyenne |
| Open Insurance readiness | Preparation partage donnees assurance | Extension | Basse |
| Open Investment readiness | Preparation partage donnees investissement | Extension | Basse |

---

## 5. Modele PSD3/PSR comme reference

### 5.1 Dispositions cles et correspondance BANKO

| Disposition PSD3/PSR | Description | Composant BANKO | Phase |
|---|---|---|---|
| APIs standardisees obligatoires | Interface dediee performante pour les TPP | API Gateway + OpenAPI | Phase 1 |
| Anti-obstruction | Interdiction de degrader l'acces API vs canal direct | Monitoring de performance | Phase 2 |
| Customer dashboards | Tableau de bord client pour gerer les consentements | Dashboard client (frontend Svelte) | Phase 2 |
| Verification of Payee (VoP) | Verification IBAN-nom avant execution | Payment BC + service VoP | Phase 3 |
| SCA ameliore | Exemptions mieux calibrees, delegation | Identity BC | Phase 3 |
| IBAN portability | Portabilite du numero de compte | Account BC | Phase 4 |
| Acces aux especes via TPP | Retrait d'especes initie par TPP | Payment BC | Phase 4 |
| Responsabilite clarifiee | Regime de responsabilite entre PSP et TPP | Governance BC | Phase 3 |

### 5.2 Lecons de PSD2 pour l'implementation BANKO

| Lecon PSD2 | Implication pour BANKO |
|---|---|
| APIs de mauvaise qualite ont freine l'adoption | Investir dans la qualite de documentation et les sandbox |
| Screen scraping persiste faute d'alternative fiable | Fournir des APIs performantes des le depart |
| Complexite SCA nuit a l'experience utilisateur | Implementer des exemptions intelligentes basees sur le risque |
| Fragmentation des standards en Europe | Adopter un standard unique (Berlin Group NextGenPSD2) |
| Manque de monitoring en temps reel | Integrer le monitoring de disponibilite et performance |

---

## 6. Benchmarks regionaux

### 6.1 Nigeria -- Central Bank of Nigeria (CBN)

Le Nigeria est le leader africain de l'Open Banking avec les lignes directrices publiees par la CBN en 2021 :

| Aspect | Detail |
|---|---|
| **Cadre** | Regulatory Guidelines on Open Banking in Nigeria |
| **Approche** | Market-driven avec supervision CBN |
| **Niveaux d'acces** | 4 niveaux (product info, customer info, transactions, financial transactions) |
| **Categorie TPP** | AISP, PISP, CISP (Card Issuer Service Provider) |
| **Pertinence BANKO** | Modele applicable pour le contexte africain, classification par niveaux de risque |

### 6.2 Arabie Saoudite -- SAMA

| Aspect | Detail |
|---|---|
| **Cadre** | Saudi Open Banking Framework |
| **Premiere licence** | Delivree en mars 2026 |
| **Approche** | Centralisee avec plateforme nationale |
| **Standards** | Bases sur UK Open Banking adaptes au contexte local |
| **Pertinence BANKO** | Reference pour le marche MENA, interoperabilite potentielle |

### 6.3 Afrique du Sud -- SARB/FSCA

| Aspect | Detail |
|---|---|
| **Cadre** | Approche phasee, pas de legislation dediee encore |
| **Approche** | Collaborative industrie-regulateur |
| **Particularite** | Integration avec le programme de modernisation des paiements (NPPS) |
| **Pertinence BANKO** | Approche pragmatique adaptee aux marches en developpement |

### 6.4 Tableau comparatif synthetique

| Critere | Nigeria | Arabie Saoudite | Afrique du Sud | Tunisie (objectif BANKO) |
|---|---|---|---|---|
| Cadre legal | Lignes directrices CBN | Framework SAMA | En cours | A etablir |
| Standard API | API Specifications v1 | UK OB adapte | Non defini | NextGenPSD2 adapte |
| Registre TPP | Operationnel | En cours | Non | Phase 3 |
| Sandbox | Oui (CBN) | Oui (SAMA) | Partiellement | Oui (BCT existante) |
| Maturite | Avance | Emergent | En cours | Pre-reglementaire |

---

## 7. Risques et prerequis

### 7.1 Risques identifies

| Risque | Probabilite | Impact | Mitigation |
|---|---|---|---|
| Absence prolongee de cadre legal tunisien | Haute | Moyen | Construire sur les standards internationaux, rester adaptable |
| Reglementation divergente du modele PSD3 | Moyenne | Haut | Architecture modulaire, couche d'abstraction reglementaire |
| Faible adoption par les TPP locaux | Moyenne | Moyen | Sandbox attractive, documentation excellente, hackathons |
| Risques de securite lies a l'ouverture | Moyenne | Critique | SCA robuste, monitoring continu, tests de penetration ANCS |
| Resistance des banques au partage de donnees | Haute | Haut | Demontrer la valeur ajoutee, modeles economiques viables |
| Insuffisance de l'infrastructure telecom | Basse | Moyen | Conception resiliente, gestion des timeouts, mode degrade |

### 7.2 Prerequis techniques

| Prerequis | Description | Responsabilite | Statut |
|---|---|---|---|
| API Gateway production-ready | Traefik configure avec TLS 1.3, rate limiting | Equipe infrastructure | En cours |
| PostgreSQL 16 avec partitionnement | Performance adequate pour les volumes Open Banking | Equipe DBA | Operationnel |
| Pipeline CI/CD securise | Tests de securite automatises (OWASP, SAST, DAST) | Equipe DevSecOps | En cours |
| Monitoring Prometheus/Grafana | Metriques de disponibilite et performance API | Equipe SRE | Planifie |
| Environnement sandbox isole | Isolation reseau et donnees pour les TPP | Equipe infrastructure | Phase 2 |

### 7.3 Prerequis organisationnels

| Prerequis | Description | Responsabilite |
|---|---|---|
| Equipe Open Banking dediee | Chef de projet, architecte API, juriste conformite | Direction |
| Relations avec la BCT | Dialogue regulier sur les evolutions reglementaires | Conformite |
| Partenariats fintechs | Identification et onboarding de TPP pilotes | Business development |
| Formation equipes | Montee en competence sur les standards Open Banking | RH / Architecture |
| Budget securite | Tests de penetration reguliers (exigence ANCS) | Direction / Securite |

---

## Annexe A -- Glossaire

| Terme | Definition |
|---|---|
| **AISP** | Account Information Service Provider -- Prestataire de services d'information sur les comptes |
| **PISP** | Payment Initiation Service Provider -- Prestataire de services d'initiation de paiement |
| **CISP** | Card Issuer Service Provider -- Prestataire de services d'emission de carte |
| **FISP** | Financial Information Service Provider -- Prestataire de services d'information financiere (FIDA) |
| **TPP** | Third Party Provider -- Prestataire tiers |
| **SCA** | Strong Customer Authentication -- Authentification forte du client |
| **VoP** | Verification of Payee -- Verification du beneficiaire |
| **FIDA** | Financial Data Access -- Reglement europeen sur l'acces aux donnees financieres |
| **PSD3** | Payment Services Directive 3 -- Troisieme directive sur les services de paiement |
| **PSR** | Payment Services Regulation -- Reglement sur les services de paiement |
| **BCT** | Banque Centrale de Tunisie |
| **ANCS** | Agence Nationale de la Cybersecurite (Tunisie) |

---

## Annexe B -- References

| Reference | Lien / Source |
|---|---|
| PSD3/PSR accord provisoire | Conseil de l'UE, 27 novembre 2025 |
| FIDA proposition | Commission europeenne, COM(2023) 360 |
| CBN Open Banking Guidelines | Central Bank of Nigeria, 2021 |
| Saudi Open Banking Framework | SAMA, 2023 (mis a jour 2026) |
| BCT Sandbox | fintech.bct.gov.tn |
| Loi donnees personnelles Tunisie | Loi organique n. 2025-xx, juin 2025 |
| Section 1033 CFPB | Federal Register, octobre 2024 |
| NextGenPSD2 Framework | Berlin Group, v1.3.12 |

---

*Document suivant : [02 -- Gestion du consentement](./02-consent-management.md)*
