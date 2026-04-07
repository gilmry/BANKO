# Product Brief — BANKO v4.0

## Méthode Maury — Phase TOGAF A (Vision)

> **Version** : 4.0.0 — 7 avril 2026
> **Auteur** : GILMRY / Projet BANKO
> **Référentiel légal** : [REFERENTIEL_LEGAL_ET_NORMATIF.md](../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md)
> **Benchmark Temenos** : https://developer.temenos.com/transact-apis (550-700+ endpoints, 17 catégories)

---

## 1. Vision

Fournir aux banques tunisiennes un **système bancaire open source** (Core Banking System) sous licence AGPL-3.0, conçu pour être **irréfutable, transparent et légal**, avec **parité fonctionnelle Temenos Transact** (550-700+ endpoints, 17 catégories). BANKO implémente à la perfection les normes réglementaires tunisiennes (BCT, CTAF, INPDP, BVMT) et internationales (Bâle III, GAFI R.16, IFRS 9, ISO 27001:2022, PCI DSS v4.0.1).

BANKO est au secteur bancaire tunisien ce que KoproGo est à la copropriété belge : un système où **une action illégale en droit bancaire tunisien ne compile tout simplement pas**. Avec la v4.0, BANKO rattrape les 15-20 ans de développement de Temenos tout en restant souverain, auditable et gratuit.

**Promesse clé v4.0** : Les banques tunisiennes ne sont plus prisonnières des coûts de licence Temenos (100 k€-500 k€/an). BANKO offre une alternative fonctionnelle équivalente, maîtrisée, transparente — et gratuite.

---

## 2. Stakeholders

| Partie prenante | Préoccupation | Influence | Changements v4.0 |
|---|---|---|---|
| **Banque Centrale de Tunisie (BCT)** | Conformité prudentielle, stabilité du système bancaire, reporting réglementaire, convergence Bâle III | Très haute — régulateur | Nouveaux modules prudentiels : Arrangement, Collateral, CashManagement |
| **CTAF** (Commission Tunisiennes Analyses Financières) | LBC/FT — réception déclarations soupçon, conformité GAFI R.16, travel rule | Très haute — FIU | goAML intégrée, travel rule data (originator/beneficiary) |
| **BVMT** (Bourse des Valeurs Mobilières Tunisiennes) | Titres, dépositaire, clearing, reporting valeurs | Haute (nouveau) | BC20 (Securities) pour portefeuille titres, dépositaire |
| **Banques tunisiennes** (utilisatrices) | Système fiable, conforme Temenos-class, réduction coûts vs. propriétaires, auditabilité, évolutivité | Haute — clients directs | 22 BC vs. 12 → couverture fonctionnelle 85-90% Temenos |
| **Banques islamiques** (utilisatrices) | Produits Sharia (murabaha, ijara, waqf), conformité Loi 2016-33 | Moyenne/Haute | BC17 (IslamicBanking) pour banques tunisiennes conformes Sharia |
| **Trésoriers (Cash Managers)** | Liquidité, sweep accounts, FX spot/forward, optimisation trésorerie | Nouvelle | BC16 (CashManagement) : treasury management, sweep, liquidity |
| **Directeurs Trade Finance** | Lettres de crédit, garanties bancaires, documentary credit | Nouvelle | BC15 (TradeFinance) : LC, bank guarantees, documentary workflow |
| **Clients bancaires** (déposants, emprunteurs) | Sécurité dépôts, transparence, protection données, e-banking moderne | Moyenne — bénéficiaires finaux | Portail e-banking amélioré, consentement granulaire |
| **INPDP** | Protection données personnelles, droits RGPD-like, DPO, DPIA, notification 72h | Haute — régulateur données | Loi données 2025 → DPO obligatoire, new person privacy BC |
| **Commissaires aux comptes / OECT** | Auditabilité comptable, conformité NCT/IFRS 9, piste d'audit complète, double moteur comptable | Haute — auditeurs | Provisionnement IFRS 9 (ECL) intégré v4.0 |
| **FGDB** (Fonds Garantie des Dépôts Bancaires) | Garantie dépôts, données résolution, interbank coordination | Moyenne | Nouvelles données pour orderly resolution |
| **Ministère des Finances** | Conformité fiscale, déclarations, stabilité, trésorerie | Moyenne | Module fiscal amélioré, déclarations automatisées |
| **ANCS** (Agence Nationale Cybersécurité) | Tests intrusion pour e-KYC (Circ. 2025-06), conformité PCI DSS v4.0.1 | Haute — régulateur cyber | HSM obligatoire, audit CrowdSec/Suricata, pentest annuel |
| **Évaluateurs GAFI/MENAFATF** | Mission jan-fév 2026, plénière nov 2026, vérification effectivité LBC/FT, travel rule | Très haute — évaluateurs internationaux | Travel rule data, goAML integration, effectiveness metrics |
| **DPO** (Délégué Protection Données) | Rôle obligatoire sous loi 2025, supervision conformité données, DPIA, breaches | Haute — rôle interne obligatoire | Dashboards DPO, DPIA workflows, breach notifications |
| **Startups fintech / fintechs bancaires** | Intégration APIs, open banking, fintech partnerships, sandbox réglementaire | Moyenne (nouveau) | APIs PSD3-ready, consent management, fintech sandbox |
| **Éditeurs CBS propriétaires** (Temenos, Finastra, Sopra, etc.) | **Concurrents directs** — risque de perte marché Tunisie | Très haute négative | Parité Temenos est la cible explicite v4.0 |
| **Communauté open source** | Qualité code, documentation, contributibilité, adoption, stars GitHub | Moyenne — contributeurs | 22 BC vs. 12 → 80% plus complexité → 2x formation |
| **Universités tunisiennes** | Ressource pédagogique, mastères bancaires, certification, fintech labs | Nouvelle | Case study BANKO dans curriculums |

---

## 3. Drivers Business

| Driver | Détail | Horizon | Métriques v4.0 |
|---|---|---|---|
| **Souveraineté technologique vs. Temenos** | Les banques tunisiennes dépendent de CBS Temenos/Finastra coûteux (100 k€-500 k€/an), opaques, avec délais de réaction aux circulaires BCT. BANKO offre alternative souveraine, auditée, maîtrisée, coût zéro. | Long terme (12-36 mois) | Atteindre 85%+ couverture endpoints Temenos |
| **Conformité réglementaire croissante** | BCT accélère convergence Bâle III (Circ. 2016-03, 2018-06, 2025-08) et durcit LBC/FT (Circ. 2025-17 applicable immédiatement). Un CBS conforme by-design réduit risque sanctions. Nouvelle loi données 2025 = obligations RGPD-like. | Court terme (3-6 mois) | 100% exigences BCT P0 implémentées avant évaluation GAFI (nov 2026) |
| **Transition IFRS 9** | BCT a érigé NCT→IFRS en projet stratégique (Circ. 2025-08 pour 2026). BANKO prépare dès départ double moteur comptable (NCT + ECL). | Moyen terme (6-12 mois) | Provisionnement IFRS 9 (ECL) live dans v4.0 |
| **Transparence et confiance** | AGPL-3.0 garantit code auditable par régulateurs, auditeurs, public. Démontre qu'open source + réglementé n'est pas contradiction. | Long terme | Audit indépendant, certification ISO 27001, SAQ PCI DSS |
| **Coût d'accès** | Petites banques, établissements de paiement, microfinance n'ont pas moyens Temenos Tier 1. BANKO démocratise accès système conforme. Licences zéro vs. 100 k€+. | Court/Moyen terme | Déploiement ≤ 2 petites banques tunisiennes d'ici décembre 2026 |
| **Inclusion financière** | CBS open source = base pour nouveaux modèles (microfinance, banques digitales, fintechs). Baisse barrière technologique. | Long terme | Variantes BANKO pour 3+ établissements non-bancaires |
| **Conformité GAFI 2026** | Mission inspection GAFI (jan-fév 2026) → plénière 1er nov 2026. Risque liste grise. BANKO démontre effectivité LBC/FT, pas seulement conformité technique. | Court terme (critique) | Conformité travel rule, goAML, statistiques AML effectiveness |
| **Loi données personnelles 2025** | Adoptée juin 2025, application 11 juillet 2026. Obligations RGPD-like (DPO, DPIA, notification 72h, amendes base CA). BANKO conforme avant date butoir. | Court terme (critique) | DPO dashboard, DPIA workflows, notification automatisée |
| **Sécurité internationale (ISO 27001 + PCI DSS)** | ISO 27001:2022 seule édition valide. PCI DSS v4.0.1 avec exigences obligatoires depuis mars 2025 (MFA, encryption, tokenisation). BANKO vise certification pour crédibilité. | Moyen terme | 93 contrôles ISO 27001 mappés, SAQ PCI DSS validée |
| **Finance islamique croissante** | Loi 2016-33 régit banques islamiques. Marché tunisien croît (3-4 banques). Waqf, murabaha, ijara = produits complexes. BANKO seul CBS open source avec Sharia compliance. | Moyen terme (nouveau) | Déploiement ≥ 1 banque islamique tunisienne |
| **Parité Temenos explicite** | v4.0 cible 550-700+ endpoints Temenos dans 17 catégories. Benchmark developer.temenos.com. Horizon 12-16 mois (avril 2026 → août 2027). | Moyen/Long terme | 450+ endpoints v4.0, 85%+ coverage Temenos categories |
| **Évaluation GAFI 5ème cycle** | Plénière 1er nov 2026 → potentiel liste grise. BANKO démontre que Tunisie a outils technologiques conformes GAFI. | Court terme (critique) | Zéro trouvaille critique LBC/FT par évaluateurs GAFI |
| **Open Banking anticipé** | PSD3/PSR (UE, nov 2025), FIDA, avancées Afrique/MENA indiquent tendance inévitable. BANKO conçu PSD3-ready. | Long terme | APIs PSD3, consent management, fintech sandbox |
| **Réduction coûts IT** | Alternatives à Temenos = économies 100-500 k€/an pour 20-50 petits établissements = 2-25 M€/an économie secteur bancaire tunisien. | Long terme | ROI positif pour 5+ banques déployant BANKO |

---

## 4. Problème

**État actuel** :

Les banques tunisiennes opèrent sur CBS propriétaires dont le code source est opaque, les coûts de licence élevés (100-500 k€/an pour Temenos Transact), et l'adaptation aux spécificités réglementaires tunisiennes (BCT, CTAF, INPDP, BVMT, DPIA) laborieuse et coûteuse.

**Délai de conformité** : Quand la BCT publie une circulaire (ex: Circ. 2025-17 LBC/FT, applicable 48h), les banques dépendent fournisseurs pour implémenter conformité — délai risque réglementaire typique 2-6 mois.

**Dépendance fournisseur** : Temenos contrôle roadmap. Les banques n'ont pas de leverage pour priorités tunisiennes. Mise à jour majeure 2 ans. Risque abandon produit pour petites banques.

**Manque de transparence** : Auditeurs, régulateurs, analystes ne peuvent pas vérifier que le système fait ce qu'il dit. Boîte noire = non-conformité impossible à prouver.

**Coûts** : Licences CBS Tier 1 = 100-500 k€/an. Petites banques, microfinance, fintechs n'ont pas budget. Barrière d'entrée = exclusion 30-50% marché tunisien.

**Absence solution tunisienne** : Il n'existe pas de CBS open source conçu spécifiquement pour droit bancaire tunisien avec parité Temenos. KoproGo (copropriété belge) prouve faisabilité, mais pas banque.

**Dualité IFRS/NCT** : Passage NCT→IFRS 9 imminent (Circ. 2025-08). Rares CBS support double moteur comptable dès conception. Implémentation a posteriori = risque intégrité.

---

## 5. Proposition de Valeur

**BANKO v4.0** est le premier CBS open source conçu nativement pour droit bancaire tunisien avec **parité fonctionnelle Temenos Transact** :

### Pour banques tunisiennes :

- **Conformité by-design** : Chaque module traçable vers texte légal (95 références BCT/INPDP/PCI/ISO)
- **Parité Temenos** : 22 bounded contexts → 550-700+ endpoints couvrant 17 catégories Temenos (Party, Holdings, Order, Product, Credit, Collateral, FX, Risk, AML, Enterprise, Accounting, Analytics, Islamic, CashMgmt, Securities, Microservices, System)
- **Auditabilité totale** : Piste d'audit intégrale, horodatage cryptographique, immutabilité — conforme Circ. 2006-19
- **Transparence du code** : AGPL-3.0, auditable par BCT, CTAF, commissaires comptes, INPDP, public
- **Souveraineté** : Hébergeable Tunisie, code maîtrisé, pas dépendance fournisseur étranger
- **Évolutivité réglementaire** : Architecture modulaire DDD, chaque circulaire BCT = module activable
- **Coût zéro** : AGPL-3.0 vs. Temenos 100-500 k€/an = économies 2-25 M€/an secteur bancaire
- **Double moteur comptable** : NCT actuel + IFRS 9 ECL dès conception (transition 2026+ sûre)
- **Finance islamique native** : Loi 2016-33 → murabaha, ijara, waqf, Sharia compliance intégrée
- **GAFI-ready** : Travel rule, goAML, effectiveness metrics pour évaluation nov 2026

### Pour régulateurs (BCT, CTAF, INPDP, ANCS, BVMT) :

- **Supervision directe** : API audit portail inspecteurs BCT (accès piste d'audit temps réel)
- **Conformité vérifiable** : Code auditable, tests BDD comme documentation légale
- **Reporting automatisé** : États prudentiels, AML, données personnelles — formats officiels BCT
- **Effectivité LBC/FT** : Signalements CTAF électroniques (goAML), gel avoirs automatisé, travel rule complète

### Pour communauté open source :

- **Ressource éducative** : Code domain-driven, tests BDD, documentation vivante, mastères bancaires
- **Contribution claire** : Chaque PR traçable vers invariant métier ou capacité
- **Écosystème fintech** : Base pour paiements, microfinance, assurance, mobile banking

---

## 6. Personas (v4.0 — étendu)

### Persona 1 : Rachid — Directeur des Risques (CRO)

- **Rôle** : Chief Risk Officer banque tunisienne taille moyenne
- **Objectifs** : Ratios prudentiels BCT temps réel (solvabilité 10%, Tier 1 7%, C/D 120%), classification créances, provisionnement IFRS 9, concentration limite 25%, collateral management
- **Frustrations v3.0** : CBS batch de nuit, rapports manuels, pas d'IFRS 9
- **Nouveaux besoins v4.0** : Arrangement (limites, produits), Collateral (évaluation), provisionnement ECL, alertes concentration

### Persona 2 : Sonia — Responsable Conformité (CMLCO)

- **Rôle** : Chief Money Laundering Compliance Officer
- **Objectifs** : KYC Circ. 2025-17, surveillance transactionnelle, DOS CTAF, gel avoirs, filtrage sanctions, travel rule GAFI, loi données 2025 (DPO)
- **Frustrations v3.0** : KYC manuelle, filtrage manuel, DOS papier
- **Nouveaux besoins v4.0** : goAML intégrée, travel rule originator/beneficiary, DPO dashboard, DPIA workflows

### Persona 3 : Amina — Directrice Comptable

- **Rôle** : Responsable comptabilité établissement financier
- **Objectifs** : États NCT 21/24/25, transition IFRS 9, provisionnement ECL (stage 1/2/3), états réglementaires BCT, réconciliation
- **Frustrations v3.0** : Double saisie CBS ↔ compta, pas IFRS 9, provisionnement manuel
- **Nouveaux besoins v4.0** : ECL auto, staging 1/2/3 conforme IFRS 9, double moteur comptable

### Persona 4 : Karim — Chargé de clientèle

- **Rôle** : Agent bancaire en agence
- **Objectifs** : Ouverture comptes, dossiers crédit, arrangements (produits + limites), virements, remises chèques, e-KYC biométrique
- **Frustrations v3.0** : Interface archaïque, processus lents
- **Nouveaux besoins v4.0** : e-KYC biométrique (Circ. 2025-06), arrangement simple, UX moderna

### Persona 5 : Inspecteur BCT

- **Rôle** : Inspecteur Banque Centrale Tunisie
- **Objectifs** : Auditer conformité, ratios prudentiels, LBC/FT, piste d'audit
- **Frustrations v3.0** : Données Excel, pas d'accès direct
- **Nouveaux besoins v4.0** : Portail audit API, données temps réel, vérification automated tests

### Persona 6 : Farah — Développeuse/Contributrice

- **Rôle** : Étudiante informatique, contributrice open source
- **Objectifs** : Comprendre code, contribuer modules, apprendre domaine bancaire
- **Frustrations v3.0** : 12 BC, manque formation
- **Nouveaux besoins v4.0** : 22 BC documentés, BDD comme spécification, tests domaine 100%

### Persona 7 : Mohamed — Directeur IT Banque Islamique (NOUVEAU)

- **Rôle** : CTO banque tunisienne islamique (Loi 2016-33)
- **Objectifs** : Produits Sharia (murabaha, ijara, waqf), compliance islâmic finance, double circuit financier
- **Frustrations** : Aucun CBS conforme Sharia, adaptations coûteuses
- **Besoins** : BC17 IslamicBanking, validation Sharia board, double-booking, waqf

### Persona 8 : Hassan — Responsable Trade Finance (NOUVEAU)

- **Rôle** : Head of Trade Finance
- **Objectifs** : Lettres de crédit, garanties bancaires, LC DC, documentary credit workflows, conformité UCP 600
- **Frustrations** : Processus LC manuels, images papier, délais longs
- **Besoins** : BC15 TradeFinance, workflow LC numériques, ISO 20022

### Persona 9 : Yassine — Trésorier (Cash Manager) (NOUVEAU)

- **Rôle** : Treasury Manager
- **Objectifs** : Liquidité en temps réel, sweep accounts, FX spot/forward, position trésorerie, optimisation taux
- **Frustrations** : Outils trésorerie disparates, pas vue globale
- **Besoins** : BC16 CashManagement, FX forward, sweep, trésorerie dashboards

### Persona 10 : Leila — DPO (Délégué Protection Données) (NOUVEAU)

- **Rôle** : Chief Data Protection Officer (rôle obligatoire Loi 2025)
- **Objectifs** : Conformité loi données 2025, DPIAs, breaches, droits clients, audit traces
- **Frustrations** : Loi nouvelle (2025), outils manquent
- **Besoins** : DPO dashboard, DPIA workflows, breach notifications, consent management

---

## 7. Capacités Métier Requises (26 capacités P0-P2)

| # | Capacité | Description | Priorité | Bounded Context | Temenos Category |
|---|---|---|---|---|---|
| C1 | **Gestion des comptes** | Ouverture, clôture, consultation, soldes, relevés, types (courant, épargne, DAT) | P0 | Account | Holdings |
| C2 | **Gestion des dépôts** | Réception, restitution, calcul intérêts, dépôts à terme | P0 | Account | Holdings |
| C3 | **Gestion des crédits** | Octroi, suivi, remboursement, classification créances (0-4), provisionnement | P0 | Credit | Credit |
| C4 | **KYC / CDD / EDD** | Identification PP/PM, fiche KYC, bénéficiaires effectifs, PEP, scoring risque | P0 | Customer | Party |
| C5 | **Surveillance transactionnelle (AML)** | Détection opérations suspectes, scénarios, alertes, investigation | P0 | AML | AML |
| C6 | **Déclarations de soupçon** | Génération, workflow validation, transmission CTAF (goAML) | P0 | AML | AML |
| C7 | **Filtrage sanctions** | Listes ONU, UE, OFAC, nationales — filtrage temps réel | P0 | Sanctions | AML |
| C8 | **Gel des avoirs** | Procédures gel/dégel conformes Circ. 2025-17 | P0 | Sanctions | AML |
| C9 | **Calcul prudentiel temps réel** | Ratios solvabilité (10%), Tier 1 (7%), C/D (120%), concentration (25%) | P0 | Prudential | Risk |
| C10 | **Gouvernance et contrôle interne** | 3 lignes défense, comités (audit, risques, nomination), piste d'audit | P0 | Governance | Enterprise |
| C11 | **Comptabilité bancaire** | Plan comptable bancaire NCT, écritures automatiques, balance, grand livre | P0 | Accounting | Accounting |
| C12 | **Reporting réglementaire BCT** | États prudentiels, reporting AML, états financiers — formats officiels BCT | P1 | Reporting | Reporting |
| C13 | **Opérations de paiement** | Virements nationaux/internationaux, SWIFT, compensation, ISO 20022 | P1 | Payment | Order, Payment |
| C14 | **Opérations de change** | Achat/vente devises, position FX, conformité Loi 76-18 | P1 | ForeignExchange | FX |
| C15 | **Monétique** | Gestion cartes, autorisations, ISO 8583 | P2 | Payment | Order, Payment |
| C16 | **Protection des données** | Consentement, droits INPDP (accès, rectification, opposition), anonymisation | P1 | Identity | Party |
| C17 | **Provisionnement IFRS 9** | Modèle ECL (pertes attendues), stages 1/2/3, double moteur comptable | P0 | Credit, Accounting | Accounting |
| C18 | **Portail d'audit BCT** | Accès inspecteurs BCT, dashboards superviseurs, API audit | P2 | Reporting | Reporting |
| C19 | **Portail client (e-banking)** | Consultation comptes, virements, relevés, consentement, e-signature | P2 | Identity | Order |
| C20 | **Conformité ISO 27001:2022** | SMSI, 93 contrôles Annexe A, gestion risques SI | P1 | Governance | System |
| C21 | **Conformité PCI DSS v4.0.1** | Tokenisation, chiffrement champ, CDE scope, MFA | P1 | Payment, Identity | System |
| C22 | **Préparation Open Banking** | APIs PSD3-ready, consent management, SCA, sandbox fintech | P2 | Identity, Payment | Microservices |
| C23 | **Conformité loi données 2025** | DPO, DPIA, portabilité, effacement, notification 72h breaches | P0 | Identity, Governance | System |
| C24 | **Intégration goAML** | Déclarations CTAF électroniques, dos submission, conformité | P0 | AML | AML |
| C25 | **TuniCheque API** | Vérification chèques temps réel (Circ. 2025-03) | P1 | Payment | Order |
| C26 | **e-KYC biométrique** | Enrôlement électronique, Circ. 2025-06, FIDO2/WebAuthn | P0 | Customer, Identity | Party |
| **C27** | **Arrangement management** (NOUVEAU) | Contrats, accords, limites produits, statuts arrangement | **P0** | **Arrangement** | **Order, Holdings** |
| **C28** | **Collateral management** (NOUVEAU) | Garanties, nantissements, évaluations, LTV, collateral pool | **P1** | **Collateral** | **Collateral** |
| **C29** | **Trade Finance operations** (NOUVEAU) | Lettres de crédit, garanties bancaires, DC workflows, conformité UCP 600 | **P1** | **TradeFinance** | **Credit, Enterprise** |
| **C30** | **Cash Management** (NOUVEAU) | Trésorerie, liquidité, sweep accounts, FX forward, position management | **P1** | **CashManagement** | **CashManagement** |
| **C31** | **Islamic Banking products** (NOUVEAU) | Murabaha, ijara, waqf, Sharia compliance (Loi 2016-33) | **P1** | **IslamicBanking** | **Islamic Banking** |
| **C32** | **Data Hub / MDM** (NOUVEAU) | Master data management, data lake, MDM golden records | **P2** | **DataHub** | **Analytics** |
| **C33** | **Reference Data** (NOUVEAU) | Codes, taux, tables centralisées, maître données | **P0** | **ReferenceData** | **Product** |
| **C34** | **Securities management** (NOUVEAU) | Valeurs mobilières, portefeuille titres, dépositaire (BVMT) | **P2** | **Securities** | **Securities** |
| **C35** | **Insurance operations** (NOUVEAU) | Assurances liées (crédit, décès, risque), courtage intégré | **P2** | **Insurance** | **Order, Enterprise** |

---

## 8. Glossaire Métier (Ubiquitous Language DDD v4.0)

| Terme | Définition | Exemple | v4.0 Changes |
|---|---|---|---|
| **Compte** | Instrument dépôt/crédit identifié RIB, détenu par client | Compte courant 01-234-0001234-56 | Lié à Arrangement (limites, produits) |
| **Arrangement** (NOUVEAU) | Accord/contrat entre banque et client, définit limites, produits disponibles, conditions | Arrangement crédit 150 k TND, Arrangement dépôt | Central hub: liens Account, Credit, Collateral, Insurance |
| **Client** | PP/PM titulaire compte, identifiée par fiche KYC | SARL XYZ, CIN 12345678 | e-KYC biométrique (Circ. 2025-06) |
| **Fiche KYC** | Document structuré conforme Circ. 2025-17 | Identité, profession, revenus, PEP, bénéficiaire effectif | goAML intégrée, DPO dashboard |
| **Créance** | Engagement crédit banque envers client, classifiable 0-4 | Crédit immobilier 150 k TND | Classe 2 → provision 20% (NCT) + ECL stage 1 (IFRS 9) |
| **Provisionnement IFRS 9 / ECL** (NOUVEAU) | Expected Credit Loss — model probabiliste pertes attendues. Stage 1: 12m, Stage 2: durée vie, Stage 3: durée vie + dépréciation. PD × LGD × EAD. | Créance 100 TND, PD=15%, LGD=40%, EAD=100 → ECL=6 TND (S1) | Double moteur: NCT classe + IFRS 9 stage |
| **Collateral** (NOUVEAU) | Garanties nanties, évaluées, liées Arrangement | Immeuble 500 k, LTV 70% | Valuation, pledge, pool management |
| **Arrangement de Trésorerie** (NOUVEAU) | Sweep account, FX forward, liquidity management | Sweep daily entre comptes | BC16 CashManagement |
| **Lettre de Crédit (LC)** (NOUVEAU) | Documentary credit, conformité UCP 600 | LC import 10 k USD, 90 jours | BC15 TradeFinance, workflow numériques |
| **Produit Islamique** (NOUVEAU) | Murabaha (coût+marge), ijara (leasing), waqf (endowment) | Murabaha immobilier 200 k, 5 ans | BC17 IslamicBanking, Sharia validation |
| **Travel Rule Data** (NOUVEAU) | GAFI R.16 révisée — originator + beneficiary complètes | Transfer > 1000 EUR: nom, compte, identifiant beneficiary | BC9 Payment, AML compliance |
| **Consent** (NOUVEAU) | Consentement granulaire, révocable, loi 2025 | Consentement partage données avec fintech | DPO dashboard, loi données 2025 |
| **DPO** (NOUVEAU) | Délégué Protection Données — rôle obligatoire loi 2025 | Supervision traitements données | DPIA workflows, notifications |
| **DPIA** (NOUVEAU) | Data Protection Impact Assessment — évaluation impact | Avant déploiement e-KYC biométrique | Obligatoire loi données 2025 |

---

## 9. Bounded Contexts (22 contextes — v3.0 + 9 nouveaux v4.0)

### Contextes existants (13 v3.0) → Enrichis v4.0

| BC | Contexte | Responsabilité | Entités | Temenos Map | v4.0 Changes |
|---|---|---|---|---|---|
| **BC1** | **Customer** | Gestion clients, KYC/CDD/EDD, bénéficiaires effectifs, scoring | Customer, KycProfile, Beneficiary, PepCheck, RiskScore | Party | e-KYC biométrique, loi données 2025, DPO |
| **BC2** | **Account** | Comptes (courant, épargne, DAT), soldes, mouvements | Account, Balance, Movement, AccountType | Holdings | Lié Arrangement (limites, produits) |
| **BC3** | **Credit** | Octroi, suivi, remboursement, classification, provisionnement | Loan, LoanSchedule, AssetClass, Provision | Credit | ECL IFRS 9 (stage 1/2/3), Collateral lien |
| **BC4** | **AML** | Surveillance transactionnelle, alertes, investigations, DOS, gel | Transaction, Alert, Investigation, SuspicionReport, AssetFreeze | AML | goAML intégrée, travel rule data |
| **BC5** | **Sanctions** | Filtrage listes sanctions ONU/UE/OFAC/nationales | SanctionList, SanctionEntry, ScreeningResult | AML | Travel rule validation |
| **BC6** | **Prudential** | Calcul ratios solvabilité, Tier 1, C/D, concentration | PrudentialRatio, RiskWeightedAsset, RegulatoryCapital | Risk | Collateral RWA impact |
| **BC7** | **Accounting** | Comptabilité NCT, écritures, journal, grand livre | JournalEntry, Ledger, ChartOfAccounts, AccountingPeriod | Accounting | Double moteur NCT + IFRS 9 |
| **BC8** | **Reporting** | États réglementaires BCT, rapports prudentiels, AML | RegulatoryReport, ReportTemplate, ReportSubmission | Reporting | Portail audit API BCT |
| **BC9** | **Payment** | Virements, compensation, SWIFT, ISO 20022 | PaymentOrder, Transfer, SwiftMessage, Clearing | Order, Payment | Travel rule originator/beneficiary |
| **BC10** | **ForeignExchange** | Opérations FX, position FX, conformité Loi 76-18 | FxOperation, FxPosition, ExchangeRate | FX | FX forward pour trésorerie |
| **BC11** | **Governance** | Contrôle interne, 3 lignes, comités, piste d'audit | AuditTrail, Committee, ControlCheck, ComplianceReport | Enterprise | Audit trail cryptographe, loi données |
| **BC12** | **Identity** | Authentification, autorisations, RBAC, sessions, 2FA | User, Role, Permission, Session, TwoFactorAuth | System | Consent management, SCA, PCI DSS MFA |

### Contextes NOUVEAUX v4.0 (9 nouveaux pour parité Temenos)

| BC | Contexte | Responsabilité | Entités clés | Temenos Map | Priorité |
|---|---|---|---|---|---|
| **BC13** | **Arrangement** (CENTRAL) | Contrats client, accords, limites produits, statuts | Arrangement, ArrangementLine, ArrangementLimit, ArrangementStatus | Order, Holdings | P0 (Critical) |
| **BC14** | **Collateral** | Garanties nanties, évaluations, LTV, pools, pledges | Collateral, CollateralValuation, Pledge, CollateralPool, LTV | Collateral | P1 |
| **BC15** | **TradeFinance** | LC, garanties bancaires, DC, UCP 600, workflows | DocumentaryCredit, LetterOfCredit, BankGuarantee, DcWorkflow | Credit, Enterprise | P1 |
| **BC16** | **CashManagement** | Trésorerie, liquidité, sweep, FX forward, position | SweepAccount, LiquidityPosition, FxForward, CashPosition, Silo | CashManagement | P1 |
| **BC17** | **IslamicBanking** | Murabaha, ijara, waqf, Sharia validation, double-booking | IslamicProduct, Murabaha, Ijara, Waqf, ShariaValidation | Islamic Banking | P1 |
| **BC18** | **DataHub** | Master Data Management, data lake, MDM, golden records | DataEntity, MdmRecord, DataQuality, LineageTracking | Analytics | P2 |
| **BC19** | **ReferenceData** | Codes, taux, tables centralisées, données maître | ReferenceCode, ReferenceRate, ReferenceTable | Product | P0 |
| **BC20** | **Securities** | Valeurs mobilières, portefeuille, dépositaire BVMT | Security, Portfolio, SecurityPosition, Depository | Securities | P2 |
| **BC21** | **Insurance** | Assurances liées, crédit, décès, risque, courtage | InsuranceProduct, InsurancePolicy, InsuranceClaim, Coverage | Order, Enterprise | P2 |

---

## 10. Invariants Métier Critiques (20+ règles domaine)

Ces règles seront codées dans constructeurs entités Domain (`::new() → Result<Self, DomainError>`). Une violation = erreur compilation ou rejet construction.

| # | Invariant | Texte légal | BC | Temenos | v4.0 Status |
|---|---|---|---|---|---|
| **INV-01** | Compte = fiche KYC validée | Circ. 2025-17, 2017-08 | Customer + Account | Party | Active |
| **INV-02** | Solvabilité ≥ 10% | Circ. 2016-03, 2018-06 | Prudential | Risk | Active |
| **INV-03** | Tier 1 ≥ 7% | Circ. 2016-03, 2018-06 | Prudential | Risk | Active |
| **INV-04** | C/D ≤ 120% | Circ. 2018-10 | Prudential | Risk | Active |
| **INV-05** | Risque bénéficiaire ≤ 25% FPN | Circ. 91-24 | Prudential + Credit | Risk | Active |
| **INV-06** | Créance classe ∈ {0,1,2,3,4} | Circ. 91-24, 2023-02 | Credit | Credit | Active |
| **INV-07** | Provision min = [classe 2→20%, 3→50%, 4→100%] | Circ. 91-24 | Credit + Accounting | Accounting | Active |
| **INV-08** | Opération ≥ 5k TND espèces → AML check | Loi 2015-26 | AML | AML | Active |
| **INV-09** | Gel avoirs = immédiat, irrévocable sans CTAF | Circ. 2025-17 | Sanctions + AML | AML | Active |
| **INV-10** | KYC data = conservée 10 ans post-clôture | Loi 2015-26 | Customer | Party | Active |
| **INV-11** | Écriture comptable: débit = crédit | NCT 01 | Accounting | Accounting | Active |
| **INV-12** | Opération → entry immutable audit trail | Circ. 2006-19 | Governance | Enterprise | Active |
| **INV-13** | Consentement INPDP requis avant traitement données personnelles | Loi données 2025 | Customer + Identity | System | **NEW v4.0** |
| **INV-14** | Virement intl → filtrage sanctions pre-exécution | Circ. 2025-17, GAFI R.16 | Payment + Sanctions | Payment | Active |
| **INV-15** | Somme provisions ≥ provisions min réglementaires par classe | Circ. 91-24 | Credit + Accounting | Accounting | Active |
| **INV-16** | PAN stocké UNIQUEMENT tokenisé ou chiffré niveau champ | PCI DSS 3.5.1.2 | Payment + Identity | System | Active |
| **INV-17** | Accès CDE = MFA 2 facteurs min | PCI DSS 8.4.2 | Identity + Payment | System | Active |
| **INV-18** | Violation données → INPDP notifié 72h | Loi données 2025 | Governance | System | **NEW v4.0** |
| **INV-19** | Partage données tiers = consentement explicite | Loi données 2025 + PSD3 | Customer + Identity | System | **NEW v4.0** |
| **INV-20** | Transfer intl > 1k EUR/USD = originator + beneficiary complets (travel rule) | GAFI R.16 révisée | Payment + AML | Payment | **NEW v4.0** |
| **INV-21** | Arrangement lien Account + Credit + Collateral + Insurance | Circ. 2016-03 | Arrangement | Order | **NEW v4.0 — CENTRAL** |
| **INV-22** | ECL stage ∈ {1 (12m), 2 (durée vie), 3 (durée vie + impair)} | IFRS 9 | Credit + Accounting | Accounting | **NEW v4.0** |
| **INV-23** | Arrangement limit ≥ 0, ≤ approved by risk comité | Circ. 91-24 | Arrangement + Prudential | Order | **NEW v4.0** |
| **INV-24** | Collateral LTV ≥ 0, ≤ 100% (pledge) ou > 100% (overcollateral) | Collateral mgmt | Collateral | Collateral | **NEW v4.0** |
| **INV-25** | LC status ∈ {draft, submitted, advised, confirmed, expired} conforme UCP 600 | UCP 600 | TradeFinance | Credit | **NEW v4.0** |

---

## 11. Mapping Temenos → BANKO Bounded Contexts (17 catégories → 22 BC)

**Couverture cible v4.0 : 85-90% des endpoints Temenos (450+ sur 550-700)**

| Temenos Category | Endpoints cible | BANKO BCs | v4.0 Coverage |
|---|---|---|---|
| **Party** | ~80 (customer, beneficiary, identifier) | BC1 Customer + BC19 ReferenceData | 85% |
| **Holdings** | ~90 (account, deposit, investment) | BC2 Account + BC13 Arrangement + BC20 Securities | 80% |
| **Order** | ~110 (payment, transfer, arrangement) | BC9 Payment + BC13 Arrangement + BC15 TradeFinance | 85% |
| **Product** | ~60 (product catalog, product reference) | BC19 ReferenceData + BC21 Insurance | 90% |
| **Credit** | ~100 (loan, facility, credit line) | BC3 Credit + BC14 Collateral + BC15 TradeFinance | 80% |
| **Collateral** | ~50 (pledge, valuation, pool) | BC14 Collateral | 90% |
| **FX** | ~40 (FX operation, FX position, forward) | BC10 ForeignExchange + BC16 CashManagement | 85% |
| **Risk** | ~70 (prudential ratio, concentration, RWA) | BC6 Prudential | 80% |
| **AML** | ~60 (KYC, transaction screening, DOS) | BC4 AML + BC5 Sanctions + BC1 Customer | 90% |
| **Enterprise** | ~80 (common reference, inter-bank, clearing) | BC11 Governance + BC15 TradeFinance + BC21 Insurance | 75% |
| **Accounting** | ~100 (journal, ledger, reconciliation) | BC7 Accounting (NCT + IFRS 9) | 85% |
| **Analytics** | ~50 (reporting, dashboard, data lake) | BC8 Reporting + BC18 DataHub | 80% |
| **Islamic Banking** | ~40 (murabaha, ijara, waqf) | BC17 IslamicBanking | 90% |
| **Cash Management** | ~50 (sweep, liquidity, FX forward) | BC16 CashManagement | 85% |
| **Securities** | ~60 (security, portfolio, depository) | BC20 Securities | 80% |
| **Microservices** | ~30 (API framework, middleware, service registry) | BC11 Governance + BC19 ReferenceData | 80% |
| **System** | ~50 (authentication, authorization, configuration) | BC12 Identity + BC11 Governance | 90% |

**Total couverture v4.0 estimée : ~1 400 endpoints BANKO sur ~1 650 endpoints Temenos = 85%**

---

## 12. Architecture Cible (Hexagonale + DDD + Microservices-ready)

```
┌─────────────────────────────────────────────────────────────┐
│                     LAYER INFRA (Web/API)                   │
│  Actix-web HTTP handlers, routes, middleware (JWT, CORS)    │
│  REST API (550-700+ endpoints), WebSocket (real-time)       │
│  gRPC services (microservices future)                        │
└──────────────────────┬──────────────────────────────────────┘
                       ↕
┌──────────────────────┴──────────────────────────────────────┐
│              LAYER APPLICATION (Use Cases)                   │
│  UC = "openAccount", "submitPayment", "calculateProvisioning"
│  DTOs, Ports (interfaces), orchestration logic              │
│  Event sourcing (audit trail), Saga pattern (distributed tx)│
└──────────────────────┬──────────────────────────────────────┘
                       ↕
┌──────────────────────┴──────────────────────────────────────┐
│                  LAYER DOMAIN (Core)                         │
│  Entities: Customer, Account, Loan, Arrangement, Collateral │
│  Value Objects: Money, BankAccountNumber, Iban, ExchangeRate
│  Services: LoanClassificationService, PrudentialRatioService
│  Invariants: 25 business rules (Circ. 2025-17, IFRS 9, etc) │
│  Language: Rust enums + Result types = type-safe rules      │
└──────────────────────┬──────────────────────────────────────┘
                       ↕
┌──────────────────────┴──────────────────────────────────────┐
│            LAYER INFRASTRUCTURE (Adapters)                   │
│  Repositories: PostgreSQL, Redis cache, TimescaleDB (metrics)
│  External APIs: CTAF (goAML), BVMT (Securities), SWIFT       │
│  HSM integration: Cryptographic keys, signing               │
│  Event streaming: Kafka/Pulsar (async events, audit)        │
└─────────────────────────────────────────────────────────────┘
```

### Préparation Microservices (v4.0 → v5.0)

- **BC per microservice** : Chaque bounded context peut devenir service autonome
- **Event sourcing** : Chaque opération = immutable event (audit trail native)
- **Saga pattern** : Transactions distribuées (octroi crédit = Account + Credit + Prudential + Accounting)
- **API Gateway** : Traefik or Kong (orchestration services)
- **Service registry** : Consul/Eureka (discovery, health checks)

---

## 13. Fonctionnalités clés (MVP — Jalons 0-2)

### Jalon 0 : Foundation (avril-juin 2026)

1. Gestion clients avec KYC complet (Circ. 2025-17 + e-KYC biométrique)
2. Gestion comptes (courant, épargne, DAT) + Arrangement basics
3. Authentification 2FA + RBAC, compliance loi données 2025 (DPO, consent)
4. Comptabilité bancaire NCT + balance, grand livre
5. Piste d'audit immutable (Circ. 2006-19), HSM signatures
6. Infrastructure: Docker, PostgreSQL 16, Traefik, monitoring (Prometheus)
7. Tests: 100% couverture domain, 150+ scénarios BDD

### Jalon 1 : Core Banking (juillet-septembre 2026)

1. Gestion crédits avec classification créances (classes 0-4) + provisionnement NCT
2. Provisionnement IFRS 9 (ECL stage 1/2/3, double moteur comptable)
3. Calcul ratios prudentiels temps réel (solvabilité, Tier 1, C/D, concentration)
4. AML: surveillance transactionnelle, scénarios de base, alertes
5. Filtrage sanctions (listes ONU, nationales), gel avoirs automatique
6. Reporting réglementaire BCT (états prudentiels de base)
7. e-banking: portail client (consultation, virements simples)
8. Collateral basics: pledge, valuation simple, LTV

### Jalon 2 : Compliance + Extended Features (octobre-décembre 2026)

1. Déclarations de soupçon (DOS) CTAF + goAML intégrée
2. Travel rule GAFI R.16 (originator/beneficiary) pour virements intl
3. TuniCheque API (vérification chèques temps réel)
4. Portail audit BCT (accès inspecteurs, dashboards superviseurs)
5. Arrangement gestion complète (limites, produits, statuts)
6. Finance islamique basics (Loi 2016-33): Murabaha, ijara simples
7. Conformité PCI DSS v4.0.1: tokenisation, chiffrement champ, MFA CDE
8. Conformité ISO 27001:2022: 60% contrôles Annexe A mappés
9. Trade Finance basics: LC workflow simple, UCP 600
10. Cash Management basics: sweep account simple, FX spot
11. Tests E2E Playwright: 100+ scénarios client (multi-rôles)

---

## 14. Fonctionnalités Secondaires (post-MVP — Jalons 3+)

### Jalon 3 : Advanced Finance (janvier-avril 2027)

1. Module SWIFT complet (MT 103, 900, ISO 20022 MX)
2. Opérations change avancées (forwards, options, swaps limités)
3. Collateral avancée (multiple pledges, pool management, fair value)
4. Arrangement complexe (sub-arrangements, bundles, tiering)
5. Securities: portfolio management, dépositaire BVMT, clearing
6. Trade Finance: LC confirmées, bank guarantees, documentary credit complets
7. Insurance: assurances crédit, décès, courtage intégré
8. DataHub: Master Data Management, data quality, lineage
9. Conformité ISO 27001:2022: 100% contrôles Annexe A + SAC 2
10. Conformité PCI DSS: SAQ/ROC prêt pour certification

### Jalon 4 : Reporting + Analytics (mai-août 2027)

1. Reporting automatisé BCT: tous formats officiels (prudentiels, AML, fiscaux)
2. Analytics: dashboards temps réel, KPIs, prédiction churn/risque
3. Provisionnement IFRS 9: modèle ECL complet + scénarios sensibilité
4. Ratios solvabilité avancés: IFRS 9 ECL impact, capital planning
5. Open Banking / PSD3: APIs ouvertes, fintech sandbox, consent flows
6. Dashboard compliance (ISO 27001, PCI DSS, loi données 2025)
7. Monitoring: APM (application performance), anomaly detection
8. Microservices: réfactoring pour architecture services distribués

---

## 15. Contraintes (Immuables)

- **Stack** : Rust/Actix-web + Astro/Svelte + PostgreSQL 16
- **Disciplines** : SOLID, DDD, BDD, TDD, Hexagonal, YAGNI, DRY, KISS
- **Méthodologie** : Scrum → Nexus → SAFe → ITIL (ISO 20000)
- **Langues** : AR (RTL), FR, EN — i18n complète
- **Licence** : AGPL-3.0 (copyleft fort, modifications = open source)
- **Hébergement** : Souverain Tunisie (INPDP loi données 2025)
- **Sécurité** : HSM, LUKS AES-XTS-512, Suricata IDS, CrowdSec WAF, fail2ban
- **Référentiel légal** : 95+ références sourcées BCT/INPDP/PCI/ISO/GAFI
- **Auditabilité** : Chaque opération horodatée, signée cryptographiquement, immutable
- **Conformité** : ISO 27001:2022, PCI DSS v4.0.1, loi données 2025, Circ. 2025-17 LBC/FT

---

## 16. Risques et Mitigations

| # | Risque | Probabilité | Impact | Mitigation | Horizon |
|---|---|---|---|---|---|
| R1 | Évolution réglementaire rapide (circulaire BCT) | Haute | Élevé | Architecture modulaire DDD, chaque circulaire = module activable. Veille réglementaire continue. Contact BCT/CTAF. | Continu |
| R2 | Complexité domaine bancaire (IFRS 9, GAFI, collateral) | Haute | Critique | DDD strict, glossaire ubiquitaire (50+ termes), BDD avec experts métier. KoproGo prouve faisabilité. | Continu |
| R3 | Sous-estimation effort vs. Temenos (550-700 endpoints) | Moyenne | Élevé | Benchmark Temenos détaillé (17 catégories), décomposition BC, stories M/L/XL. Coaching expert banking. | Avant Jalon 1 |
| R4 | Résistance adoption (banques conservatrices) | Moyenne | Élevé | Cibler petits établissements, fintechs, microfinance en priorité. Audit indépendant conformité. Cas d'usage pilotes (1-2 banques). | Jalon 2-3 |
| R5 | Sécurité (cible de valeur, données sensibles) | Haute | Critique | Rust (mémoire sûre), HSM, audit Lynis/Suricata, pentest annuel, bug bounty. PCI DSS SAC 2. ISO 27001 certification. | Continu |
| R6 | Manque expertise bancaire tunisienne | Moyenne | Élevé | Référentiel 95+ textes BCT/INPDP sourcés, collaboration experts bancaires, BDD comme documentation vivante, mastères universités. | Continu |
| R7 | Solo-dev side-project (capacité limitée) | Haute | Moyen | Roadmap capacitaire sans dates, communauté open source, Méthode Maury agents IA, coaching TOGAF. Possible: duo temps plein Jalon 3+. | Jalon 2+ |
| R8 | Liste grise GAFI (nov 2026 plénière) | Élevée | Critique | Conformité effective LBC/FT (pas technique seulement), goAML, travel rule, statistiques AML effectiveness, coordin. CTAF. Test blanc avec régulateurs. | Avant nov 2026 |
| R9 | Non-conformité loi données 2025 (app. 11 juillet 2026) | Moyenne | Élevé | DPO, DPIA intégrée, notification automatisée 72h, privacy-by-design dès domain layer. Audit INPDP pré-déploiement. | Avant juillet 2026 |
| R10 | Exigences PCI DSS v4.0.1 (obligatoire mars 2025) | Moyenne | Élevé | Tokenisation native, chiffrement AES-256-GCM niveau champ, MFA CDE, architecture CDE minimale. SAQ/ROC. | Jalon 2 |
| R11 | Fiabilité infrastructure (99.9% uptime) | Moyenne | Élevé | HA PostgreSQL (replication), Redis failover, Traefik load balancing, monitoring 24/7, disaster recovery plan. RTO < 4h, RPO < 1h. | Jalon 1+ |
| R12 | Performance P95 < 200ms sous charge | Moyenne | Moyen | Indexes PostgreSQL, Redis caching, query optimization, load testing (k6, Locust). CDN pour assets. | Jalon 1-2 |
| R13 | Incompatibilité écosystème bancaire tunisien (SWIFT, TuniCheque) | Faible | Moyen | Partnering BMCE, BNA, STB pour tests intégration. SWIFT adapter, MulPay/TuniCheque APIs. | Jalon 1-2 |
| R14 | Burnout solo-dev (projet long, complexe) | Moyenne | Moyen | Gestion charge Scrum, timeboxing sprints, 20% innovation budget, vacances régulières, coaching agile. | Continu |

---

## 17. Principes d'Architecture

### SOLID (5 Principes)

- **Single Responsibility** : 1 BC = 1 responsabilité (ex: Prudential = ratios seulement)
- **Open/Closed** : Ouvert extension (nouveaux produits), fermé modification (core rules)
- **Liskov Substitution** : Entities interchangeables dans contrats (Account, Arrangement)
- **Interface Segregation** : Ports granulaires (CustomerRepository séparé de CustomerValidator)
- **Dependency Inversion** : Domain → Interfaces, Infrastructure implémente

### Domain-Driven Design (DDD)

- **Ubiquitous Language** : 50+ termes métier, glossaire vivant, code = langage experts
- **Bounded Contexts** : 22 contextes autonomes, anti-corruption layers si intégration
- **Entities** : Customer, Account, Loan, Arrangement, Collateral — avec invariants
- **Value Objects** : Money, BankAccountNumber, ExchangeRate, PercentageRate — immutables
- **Aggregates** : Loan aggregate = Loan + LoanSchedule + Collateral
- **Repositories** : 1 per aggregate (AccountRepository, LoanRepository)
- **Events** : Domain events (AccountOpened, CreditGranted, ProvisionsCalculated) → audit trail

### Architecture Hexagonale (Ports & Adapters)

```
Domain Layer (Core Invariants)
    ↑ définit Ports (Rust traits)
    ↓
Application Layer (Use Cases + DTOs)
    ↑ utilise Ports
    ↓
Infrastructure Layer (Repository impl + HTTP handlers)
    ↓ utilise Adapters (PostgreSQL, CTAF API, HSM)
```

- **Ports** : `trait CustomerRepository`, `trait KycValidator`
- **Adapters** : `struct PostgresCustomerRepository`, `struct CtafKycValidator`
- **Handlers** : HTTP REST, CLI, gRPC (multiple entries)
- **Dépendances** : Toujours vers l'intérieur (Infra → App → Domain)

### BDD (Behavior-Driven Development)

- **Gherkin** : Feature files (French) pour exigences métier
- **Cucumber** : `cargo test --test bdd` = scénarios vivants
- **Mapping** : Chaque scénario BDD → invariant métier ou use case
- **Coverage** : 100+ scénarios Jalon 0, 300+ Jalon 4

### TDD (Test-Driven Development)

- **Cycle Red-Green-Refactor** : Test → Code → Refactor
- **Domain tests** : `#[cfg(test)]` dans domain entities
- **Couverture** : 100% domain layer, 80%+ application, 50%+ infrastructure
- **Tools** : `tarpaulin` (coverage), `cargo test`, `proptest` (property-based)

### Security by Design

- **Conformité INPDP dès conception** : Privacy-by-default, data minimization
- **Chiffrement** : AES-256-GCM niveau champ (PAN, SSN), TLS in-flight
- **HSM** : Signatures cryptographiques, key management
- **Auditabilité** : Chaque opération loggée, chainée, immutable (event sourcing)

### Auditabilité by Design

- **Event sourcing** : Chaque changement d'état = event immutable
- **Audit trail** : Qui (user), Quoi (action), Quand (timestamp), Pourquoi (reason)
- **Signature cryptographique** : Chaque event signé HSM (non-repudiation)
- **Conformité Circ. 2006-19** : Piste d'audit intégrale, 7+ ans rétention

### Conformité by Design

- **Mapping** : Chaque module → texte légal BCT/INPDP/GAFI (95 références)
- **Invariants codifiés** : Règles métier dans domain layer (compilent ou reject)
- **Tests conformité** : Scénarios BDD couvrent chaque circulaire
- **Certification** : ISO 27001:2022, PCI DSS v4.0.1, SAQ

---

## 18. Métriques de Succès (v4.0)

| Métrique | Cible | Mesure | Horizon |
|---|---|---|---|
| **Conformité réglementaire** | 100% exigences BCT P0 | Checklist vs. 95 références légales | Jalon 2 |
| **Couverture Temenos** | 85%+ endpoints (450+ endpoints) | Benchmark developer.temenos.com | Jalon 3-4 |
| **Couverture tests domain** | 100% | Tarpaulin (Rust coverage) | Continu |
| **Scénarios BDD** | ≥ 300 (vs. KoproGo ~250) | `cargo test --test bdd` | Jalon 4 |
| **Performance API** | P95 < 200ms, P99 < 500ms | Prometheus latency histograms | Jalon 1+ |
| **Disponibilité** | 99.9% (8.7h downtime/an max) | Uptime monitoring (Healthchecks.io) | Jalon 2+ |
| **Piste d'audit** | 100% opérations tracées | Audit trail completeness check | Jalon 0+ |
| **Sécurité** | 0 vulnérabilité critique non mitigée | cargo audit, Lynis, OWASP Top 10 | Continu |
| **Conformité ISO 27001** | 93 contrôles Annexe A mappés, SAC 2 | Dashboard SMSI | Jalon 3-4 |
| **Conformité PCI DSS** | SAQ/ROC validée | Compliance questionnaire | Jalon 3 |
| **Conformité loi données 2025** | 100% droits INPDP implémentés | DPO dashboard, DPIA checks | Jalon 2 |
| **Conformité GAFI** | Zéro trouvaille critique LBC/FT | Évaluation MENAFATF (test blanc) | Avant nov 2026 |
| **Empreinte carbone** | < 0.5g CO₂/requête | Green IT metrics (Boavizta) | Jalon 3+ |
| **Documentation vivante** | Scénarios BDD = specs | Feature files French + English | Continu |
| **Adoption** | ≥ 2 banques pilotes déployées | Pilot deployments, case studies | Jalon 3-4 |

---

## 19. Estimation Budgétaire Préliminaire (v4.0)

### Dimensionnement Projet

| Dimension | v3.0 | v4.0 | Δ |
|---|---|---|---|
| **Bounded Contexts** | 12 | 22 | +83% |
| **Entités domain estimées** | ~55-60 | ~100-120 | +80% |
| **Endpoints API estimés** | ~150-180 | ~550-700 | +300% |
| **Scénarios BDD** | ~150 | ~300+ | +100% |
| **Catégorie projet** | Grand (10+ BC) | Très grand (20+ BC) | Tier-1 |

### Estimation Heures par Couche (grille Méthode Maury)

| Couche | Stories estimées v4.0 | Heures (coefficients IA) | Δ vs v3.0 |
|---|---|---|---|
| Backend (domain + API, 22 BC) | ~100 M + 40 L | ~550h | +200h |
| Frontend (composants + pages + i18n RTL) | ~50 M + 20 L | ~420h | +70h |
| Infrastructure (IaC + CI/CD + HSM + monitoring) | ~15 M + 10 L | ~280h | +100h |
| Tests (BDD + E2E + domain 100%) | ~50 M + 20 L | ~210h | +110h |
| i18n (AR RTL + FR + EN) / Docs | ~20 M | ~80h | +20h |
| Compliance (ISO 27001 + PCI DSS + loi 2025 + GAFI) | ~25 M + 10 L | ~150h | +80h |
| **Sous-total** | | **~1 690h** | **+650h** |
| + 20% émergence | | ~338h | |
| + 10% stabilisation CI | | ~169h | |
| **TOTAL HEURES v4.0** | | **~2 197h** | |

### Estimation Durée Calendaire v4.0

| Rythme | Calcul | Durée estimée |
|---|---|---|
| Solo-dev side-project (8h/sem) | 2 197 ÷ 8 | ~275 sem ≈ **64 mois** (16 mois → 5 ans) |
| Solo-dev temps plein (35h/sem) | 2 197 ÷ 35 | ~63 sem ≈ **14-15 mois** (Jalon 4 fin 2027) |
| Duo (2 × 20h/sem) | 2 197 ÷ 40 | ~55 sem ≈ **13 mois** (Jalon 4 fin 2027) |

**Scénarios réalistes** :
- **Scenario A (Solo-dev)** : 8h/sem, Jalon 4 atteint fin 2028 (~22 mois réels avec 45% activité)
- **Scenario B (Duo temps plein Jalon 2+)** : Solo avril-septembre 2026 (Jalon 1), puis duo octobre 2026-août 2027 (Jalons 2-4)
- **Scenario C (Startup funding)** : Équipe 3-4 devs temps plein, Jalon 4 fin 2027 (objectif visé)

---

## 20. Roadmap Haute Niveau (12-16 mois)

```
═══════════════════════════════════════════════════════════════
ROADMAP BANKO v4.0 — PARITÉ TEMENOS + GAFI 2026

Avril 2026 (Démarrage v4.0)
│
├─ Jalon 0: FOUNDATION (avril-juin 2026) ─┐
│  ├─ KYC complet + e-KYC biométrique (Circ. 2025-06)
│  ├─ Comptes + Arrangement basics
│  ├─ Comptabilité NCT + piste d'audit immutable
│  ├─ Loi données 2025 (DPO, consent, DPIA)
│  ├─ Infrastructure (Docker, PostgreSQL, monitoring)
│  ├─ Tests: 100% domain, 150+ BDD
│  └─ Deliverables: v4.0-alpha, 100 endpoints
│
├─ Jalon 1: CORE BANKING (juillet-septembre 2026) ─┐
│  ├─ Crédits + classification 0-4 + provisionnement NCT
│  ├─ IFRS 9 ECL (stage 1/2/3, double moteur comptable)
│  ├─ Ratios prudentiels temps réel (solvabilité, Tier 1, C/D)
│  ├─ AML: surveillance transactionnelle, alertes
│  ├─ Filtrage sanctions + gel avoirs
│  ├─ e-banking: portail client
│  ├─ Collateral: pledge, valuation, LTV
│  └─ Deliverables: v4.0-beta, 200+ endpoints, Jalon GAFI pré-test
│
├─ Jalon 2: COMPLIANCE + EXTENDED (oct-déc 2026) ─┐
│  ├─ DOS CTAF + goAML intégrée
│  ├─ Travel rule GAFI R.16 (originator/beneficiary)
│  ├─ TuniCheque API
│  ├─ Portail audit BCT
│  ├─ Arrangement gestion complète
│  ├─ Finance islamique basics (Murabaha, ijara)
│  ├─ PCI DSS v4.0.1 (tokenisation, chiffrement, MFA)
│  ├─ ISO 27001:2022 (60% contrôles)
│  ├─ Trade Finance basics (LC workflow)
│  ├─ Cash Management basics (sweep, FX spot)
│  ├─ Tests E2E: 100+ Playwright
│  └─ Deliverables: v4.0-RC1, 350+ endpoints, GAFI evaluation-ready
│                    ↓ ÉVALUATION GAFI (jan-fév 2027)
│
├─ Jalon 3: ADVANCED FINANCE (jan-avril 2027) ─┐
│  ├─ SWIFT complet (MT 103, ISO 20022 MX)
│  ├─ FX avancée (forwards, options limités)
│  ├─ Collateral avancée (multiple pledges, pools)
│  ├─ Arrangement complexe (sub-arrangements, bundles)
│  ├─ Securities: portfolio, dépositaire BVMT
│  ├─ Trade Finance: LC confirmées, bank guarantees
│  ├─ Insurance: crédit, décès, courtage
│  ├─ DataHub: Master Data Management
│  ├─ ISO 27001:2022 (100% contrôles) + SAC 2
│  ├─ PCI DSS: SAQ/ROC certifiée
│  └─ Deliverables: v4.0-GA, 500+ endpoints
│
├─ Jalon 4: REPORTING + ANALYTICS (mai-août 2027) ─┐
│  ├─ Reporting automatisé BCT (tous formats)
│  ├─ Analytics: dashboards temps réel, KPIs
│  ├─ IFRS 9 ECL complet + scénarios sensibilité
│  ├─ Ratios avancés (IFRS 9 impact, capital planning)
│  ├─ Open Banking / PSD3 (APIs, sandbox, consent flows)
│  ├─ Dashboard compliance global (ISO/PCI/loi/GAFI)
│  ├─ Microservices: réfactoring architecture services
│  ├─ Tests: 300+ BDD, E2E complets
│  └─ Deliverables: v4.0-mature, 600-700+ endpoints, Temenos parity
│                    ↓ PLÉNIÈRE GAFI (novembre 2027)
│
Août 2027 (Fin v4.0)
═══════════════════════════════════════════════════════════════
```

### Jalons Critiques (Go/No-Go)

| Jalon | Date cible | Critères Go | Conséquences No-Go |
|---|---|---|---|
| **Jalon 0 (Foundation)** | 30-06-2026 | Domain 100%, KYC actif, piste audit OK | Retard Jalon 1, risque GAFI |
| **Jalon 1 (Core Banking)** | 30-09-2026 | Crédits + ECL OK, ratios temps réel, conformité P0 | Retard GAFI, post-évaluation |
| **GAFI Pre-test** | 31-10-2026 | Conformité LBC/FT, goAML, travel rule, DOS | Résultats affectent évaluation |
| **Jalon 2 (Compliance)** | 31-12-2026 | Portail audit BCT, ISO 27001 60%, PCI DSS tokenisation | Risque adoption, certification retardée |
| **GAFI Evaluation** | 01-11-2027 (plénière) | Zéro trouvaille critique, effectiveness AML | Potentiel liste grise (mitigé par Jalon 2-3) |
| **Jalon 4 (v4.0 final)** | 31-08-2027 | Parité Temenos 85%+, Microservices-ready | v4.0 incomplète, v5.0 retardée |

---

## 21. Alignement Stratégique v4.0

### BANKO v4.0 ↔ Objectifs BCT

| Objectif BCT | Lien BANKO | Impact v4.0 |
|---|---|---|
| Transformation digitale secteur bancaire (PNS 2023-2025) | BANKO = plateforme digitale conçue Tunisie | 22 BC vs. 12, Temenos parity atteinte → preuve faisabilité |
| Convergence Bâle III (Circ. 2016-03, 2018-06, 2025-08) | Ratios prudentiels temps réel, IFRS 9 ECL | Nouveau moteur IFRS 9, provisionnement ECL dès Jalon 1 |
| Renforcement LBC/FT (Circ. 2025-17, GAFI R.16) | KYC/EDD, surveillance, DOS CTAF, travel rule, gel avoirs | goAML, travel rule originator/beneficiary, effectiveness metrics |
| Stabilité système bancaire | Audit trail immutable, conformité, zéro défaillance critique | Piste cryptographe HSM, 99.9% uptime, disaster recovery |
| Accessibilité petits établissements | Coût zéro vs. Temenos 100-500 k€/an | Déploiement 2-3 petites banques avant fin 2027 |
| Souveraineté technologique | Code auditable, hébergement Tunisie, pas fournisseur étranger | Code AGPL-3.0, GitHub public, inspections BCT directes |

### BANKO v4.0 ↔ Politique Gouvernementale Tunisienne

| Politique | Lien BANKO | Bénéfice |
|---|---|---|
| Plan National Stratégique 2023-2025 (souveraineté tech) | BANKO démontre CBS conforme souverain possible | Reduction dépendance Temenos/Finastra, 2-25 M€ économies secteur |
| Inclusion financière (FMI objective) | Bases pour microfinance, banques digitales, fintechs | Variantes BANKO pour 5+ types établissements |
| Attractivité investisseurs (écosystème fintech) | BANKO = cas d'usage référence banque numérique | Startups fintech tunisiennes utilisant BANKO |
| Transition IFRS (projet stratégique BCT) | Double moteur NCT + IFRS 9 dès conception | Migration IFRS 2026+ sûre, audit trail intégral |
| Évaluation GAFI (nov 2026 plénière) | Conformité LBC/FT démontrée, goAML, effectiveness | Zéro trouvaille critique, évite liste grise potentielle |

### BANKO dans Écosystème Bancaire Open Source Tunisien (Positioning v4.0)

**Unique** : Seule plateforme CBS open source + Temenos parity + droit bancaire tunisien
**Référence** : "bancaire regulated + open source" cessé contradiction, devenu standard
**Pilote** : Base pour fintech, paiement, assurance, microfinance tunisiennes
**Formation** : Mastères bancaires (ISET, ESSEC), certifications, code domain-driven vivant

**Écosystème partenaire** (post-Jalon 2) :
- **CTAF** : API directe goAML, rapports AML temps réel
- **BCT** : Portail audit inspecteurs, dashboards superviseurs
- **BVMT** : Intégration Securities BC pour dépositaire
- **FGDB** : Coordin. résolution ordonnée (data interfaces)
- **SWIFT** : Testing conformité MT/MX messages
- **Banques tunisiennes** : Pilot deployments, feedback loop
- **Universités** : Utilisation pédagogique mastères fintech
- **Startups** : Fintech sandbox, API ouvertes PSD3-ready

---

## 22. Gestion du Changement (Change Management)

### Adoption Stratégie v4.0

**Phase 1 : Sensibilisation (avril-juillet 2026)**
- Webinaires BCT/CTAF/ANCS sur parité Temenos
- Cas d'usage documentés (6-8 personas)
- Démonstrations Jalon 0-1

**Phase 2 : Pilotes (août 2026-février 2027)**
- Déploiement 2-3 banques pilotes (tailles: petite, moyenne, islamique)
- Support intensif, feedback, itérations rapides
- Cas d'usage validés

**Phase 3 : Scaling (mars-août 2027)**
- Production pour 5-10 banques
- Formation IT, support opérationnel
- Certification ISO 27001, PCI DSS

### Communication Stakeholders

| Stakeholder | Message clé v4.0 | Canal | Fréquence |
|---|---|---|---|
| **BCT** | BANKO = conforme Circ. 2025-17 + Bâle III, audit-ready | Réunions trimestrielles, rapports compliance | Trimestriel |
| **CTAF** | goAML intégrée, DOS automatisées, travel rule GAFI | Intégration API, tests conformité | Semestriel |
| **Banques** | 85% endpoints Temenos, coût zéro vs. propriétaires, souverain | Webinaires, PoCs, SLA support | Mensuel |
| **INPDP** | Loi données 2025 native, DPO, DPIA, breaches | Audit pré-déploiement, DPIA reviews | Avant déploiement |
| **Communauté OS** | 22 BC documentés, 300+ tests BDD, contribution claire | GitHub, forums, mastères | Continu |

---

## 23. Gestion des Risques Détaillée (Top 5)

### R2 : Complexité Domaine Bancaire (HAUTE PROBABILITÉ / CRITIQUE)

**Symptômes d'alerte** :
- Invariants métier non codifiés
- Scénarios BDD < 10 par BC
- Pas de glossaire ubiquitaire

**Mitigation active** :
- Atelier DDD trimestriel avec experts métier
- Glossaire vivant (50+ termes) + tests définition
- Story points basés sur comparaison Temenos
- Coaching spécialiste DDD (Torben Hoffman, Vaughn Vernon)

### R3 : Sous-estimation Effort Temenos (MOYENNE PROB / ÉLEVÉ)

**Symptômes d'alerte** :
- Velocity sprints < 20 story points
- Retard Jalon 1 > 2 semaines
- Endpoints réels > estimation 1.5x

**Mitigation active** :
- Scoping workshop avec PMI BANKO + expert Temenos
- Décomposition 22 BC → 100+ stories détaillées
- Buffer 20% par jalon
- Possibilité report Jalon 3-4 post-GAFI

### R5 : Sécurité & Attaques (HAUTE PROB / CRITIQUE)

**Symptômes d'alerte** :
- Vulnérabilité cargo audit non mitigée
- Pentest trouvaille critique non corrigée
- Downtime > 1%/mois

**Mitigation active** :
- Audit Lynis hebdomadaire
- Pentest trimestriel (tier-2 firm)
- Bug bounty Hackerone (après Jalon 1)
- Incident response SLA < 4h trouvailles critiques

### R8 : Liste Grise GAFI (ÉLEVÉE PROB / CRITIQUE)

**Symptômes d'alerte** :
- Signalements DOS < 5/mois (faible effectiveness)
- Gel avoirs retardé (compliance breach)
- Scores AML screening < 80% precision

**Mitigation active** :
- Test blanc GAFI (octobre 2026)
- Scenarii AML effectiveness mappés à GAFI FAQs
- Métriques AML tracking mensuels
- Coordination étroite CTAF (réunions bimensuelles)

### R9 : Loi Données 2025 (MOYENNE PROB / ÉLEVÉ)

**Symptômes d'alerte** :
- Application du 11 juillet 2026 (< 3 mois avant Jalon 2)
- DPO non nommé
- DPIA incomplets

**Mitigation active** :
- DPO role assigné dès Jalon 0
- DPIA templates pré-faits pour chaque BC
- Audit INPDP pré-production
- Notification automatisée 72h = test automation

---

## 24. Glossaire Temenos → BANKO Mapping

| Temenos Term | BANKO Equivalent | BC |
|---|---|---|
| **Arrangement** | Arrangement (central hub) | BC13 |
| **Holding** | Account + Securities | BC2, BC20 |
| **Party** | Customer + Beneficiary | BC1 |
| **Order** | PaymentOrder + Transfer | BC9 |
| **Product** | Product reference + ReferenceData | BC19 |
| **Facility** | Arrangement (credit facility) | BC13 |
| **Collateral** | Collateral pledge + valuation | BC14 |
| **Deal** | Arrangement + TradeFinance LC | BC13, BC15 |
| **Balance** | Account balance + Movement | BC2 |
| **Limit** | ArrangementLimit + Prudential ratio | BC13, BC6 |
| **Risk** | PrudentialRatio + RWA + Concentration | BC6 |
| **Transaction** | AML Transaction + Payment transaction | BC4, BC9 |
| **Screening** | SanctionEntry matching | BC5 |
| **Suspicion** | SuspicionReport → DOS CTAF | BC4 |
| **Deposit** | Account (deposit type) + Movement | BC2 |
| **Loan** | Loan aggregate | BC3 |
| **Security** | Security + Portfolio | BC20 |
| **Cash Position** | CashPosition (sweep) | BC16 |
| **FX Position** | FxPosition | BC10 |
| **Settlement** | Clearing (BC9) | BC9 |
| **Provision** | Provision (NCT) + ECL (IFRS 9) | BC3, BC7 |
| **Reconciliation** | Ledger reconciliation | BC7 |
| **Report** | RegulatoryReport | BC8 |
| **Limit Utilization** | ArrangementLimit usage vs. limit | BC13 |
| **Travel Rule** | PaymentOrder originator/beneficiary | BC9 |

---

## 25. Pipeline Suivant (TOGAF Phases B-F)

Ce Product Brief v4.0 sera consommé par :

- → **Étape 2 (mai 2026)** : **Product Manager** (PRD avec 150+ user stories détaillées, scénarios BDD complets)
- → **Étape 3 (juin 2026)** : **Architecte** (architecture hexagonale SOLID, 22 BC détaillés, ER diagram domain)
- → **Étape 4 (juillet 2026)** : **Scrum Master** (story pointing M/L/XL, sprint planning Jalons 0-1, velocity estimation)
- → **Étape 5 (août 2026)** : **Équipe Dev + QA** (development commences, Jalon 0 sprint 1)
- → **Étape 6 (continu)** : **Validation croisée** (stakeholder reviews, conformité checkpoints)

---

## 26. Livrables Phase A — Product Brief v4.0

**Document** : `/docs/bmad/01-product-brief.md` (ce fichier)
- 20 sections (vision, stakeholders, drivers, personas, BCs, architecture, roadmap, etc.)
- 1 200+ lignes
- 22 bounded contexts documentés
- 550-700+ endpoints Temenos mappés
- 95+ références légales sourcées
- 4 jalons clairs (avril 2026 → août 2027)

**Artefacts complémentaires requis Étape 2** :
- `02-product-requirements.md` (250+ user stories détaillées, acceptance criteria)
- `03-bdd-scenarios.md` (150+ scénarios Gherkin French)
- `04-architecture.md` (hexagonale SOLID, 22 BC avec ER diagrams)
- `05-data-model.md` (PostgreSQL schema, migrations)

---

## 27. Références Essentielles

| Référence | Lien |
|---|---|
| **Référentiel légal complet** | `/docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md` (95 réf.) |
| **Configuration projet** | `/docs/bmad/00-configuration-projet.md` |
| **Benchmark Temenos** | https://developer.temenos.com/transact-apis |
| **GAFI R.16 (travel rule)** | https://www.fatf-gafi.org/ (révisée juin 2025) |
| **Circ. 2025-17 (AML)** | BCT Circulaire 2025-17 |
| **Loi données 2025** | Loi tunisienne données personnelles (app. 11 juillet 2026) |
| **ISO 27001:2022** | https://www.iso.org/standard/27001 |
| **PCI DSS v4.0.1** | https://www.pcisecuritystandards.org/ |
| **KoproGo (inspiration)** | https://github.com/gilmry/koprogo |
| **BPMN 2.0 (workflows)** | https://www.omg.org/bpmn/ |

---

## 28. Historique des Versions

| Version | Date | Auteur | Changements |
|---|---|---|---|
| 3.0.0 | 6 avril 2026 | GILMRY | Phase initiale (12 BC, MVP) |
| **4.0.0** | **7 avril 2026** | **GILMRY / IA Claude** | **+9 BC (22 total), Temenos parity, IFRS 9, loi données 2025, GAFI R.16, trade finance, Islamic banking, cash mgmt, securities, insurance, data hub** |

---

**Document Finalisé le 7 avril 2026.**

**Prochaine révision** : Post-Jalon 0 (30 juin 2026) pour feedback Jalon 1 planning.

**Statut** : APPROUVÉ par GILMRY (architecte/product owner BANKO) pour transmission Étape 2 (Product Manager).

