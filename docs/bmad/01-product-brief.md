# Product Brief — BANKO

## Méthode Maury — Phase TOGAF A (Vision)

> **Version** : 3.0.0 — 6 avril 2026
> **Auteur** : GILMRY / Projet BANKO
> **Référentiel légal** : [REFERENTIEL_LEGAL_ET_NORMATIF.md](../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md)

---

## 1. Vision

Fournir aux banques tunisiennes un **système bancaire open source** (Core Banking System) sous licence AGPL-3.0, conçu pour être **irréfutable, transparent et légal**. BANKO implémente à la perfection les normes réglementaires tunisiennes (BCT) et internationales (Bâle III, GAFI), de sorte que chaque opération soit auditable par la Banque Centrale de Tunisie, le gouvernement et tous les acteurs de la sécurité bancaire.

BANKO est au secteur bancaire tunisien ce que KoproGo est à la copropriété belge : un système où **une action illégale en droit bancaire tunisien ne compile tout simplement pas**.

---

## 2. Stakeholders

| Partie prenante | Préoccupation | Influence |
|---|---|---|
| **Banque Centrale de Tunisie (BCT)** | Conformité prudentielle, stabilité du système bancaire, reporting réglementaire | Très haute — régulateur |
| **CTAF** (Commission Tunisienne des Analyses Financières) | LBC/FT — réception des déclarations de soupçon, conformité GAFI | Très haute — FIU |
| **Banques tunisiennes** (utilisatrices) | Système fiable, conforme, réduction des coûts IT, auditabilité | Haute — clients directs |
| **Clients bancaires** (déposants, emprunteurs) | Sécurité des dépôts, transparence, protection des données | Moyenne — bénéficiaires finaux |
| **INPDP** | Protection des données à caractère personnel des clients | Haute — régulateur données |
| **Commissaires aux comptes / OECT** | Auditabilité comptable, conformité NCT/IFRS, piste d'audit | Haute — auditeurs |
| **FGDB** (Fonds de Garantie des Dépôts) | Garantie des dépôts, données de résolution | Moyenne |
| **Ministère des Finances** | Conformité fiscale, déclarations, stabilité | Moyenne |
| **Communauté open source** | Qualité du code, documentation, contributibilité | Moyenne — contributeurs |
| **ANCS** (Agence Nationale de la Cybersécurité) | Tests d'intrusion obligatoires pour e-KYC (Circ. 2025-06) | Haute — régulateur cyber |
| **Évaluateurs GAFI/MENAFATF** | Mission jan-fév 2026, plénière nov 2026. Vérification effectivité LBC/FT. | Très haute — évaluateurs internationaux |
| **DPO** (Délégué Protection Données) | Rôle obligatoire sous la nouvelle loi 2025. Supervision conformité données personnelles. | Haute — rôle interne obligatoire |

---

## 3. Drivers Business

| Driver | Détail |
|---|---|
| **Souveraineté technologique** | Les banques tunisiennes dépendent de CBS propriétaires (Temenos, Finastra, etc.) coûteux et opaques. BANKO offre une alternative souveraine, auditée, maîtrisée. |
| **Conformité réglementaire croissante** | La BCT accélère la convergence vers Bâle III (Circ. 2016-03, 2018-06) et durcit le dispositif LBC/FT (Circ. 2025-17 applicable immédiatement). Un CBS conforme by-design réduit le risque de sanctions. |
| **Transition IFRS** | La BCT a érigé la transition NCT→IFRS en projet stratégique. BANKO prépare dès le départ un double moteur comptable. |
| **Transparence et confiance** | L'AGPL-3.0 garantit que le code est auditable. Les régulateurs, auditeurs et le public peuvent vérifier que le système fait ce qu'il dit. |
| **Coût d'accès** | Les petites banques et établissements financiers n'ont pas les moyens d'un CBS Tier 1. BANKO démocratise l'accès à un système conforme. |
| **Inclusion financière** | Un CBS open source peut servir de base pour de nouveaux modèles bancaires (microfinance, banques digitales, établissements de paiement). |
| **Conformité GAFI 2026** | Mission d'inspection GAFI (jan-fév 2026) suivie de la plénière du 1er novembre 2026. Risque de liste grise. BANKO doit démontrer l'effectivité LBC/FT, pas seulement la conformité technique. |
| **Nouvelle loi données personnelles** | Adoptée juin 2025, application 11 juillet 2026. Obligations RGPD-like (DPO, DPIA, notification 72h, amendes sur CA). BANKO doit être conforme avant la date butoir. |
| **Sécurité internationale (ISO 27001 + PCI DSS)** | ISO 27001:2022 seule édition valide. PCI DSS v4.0.1 avec exigences obligatoires depuis mars 2025. BANKO vise la certification pour crédibiliser l'offre. |
| **Open Banking anticipé** | Pas encore réglementé en Tunisie, mais PSD3/PSR (UE, nov 2025), FIDA, et avancées Nigeria/Arabie Saoudite indiquent une tendance inévitable. BANKO conçu open-banking-ready. |

---

## 4. Problème

Les banques tunisiennes opèrent sur des CBS propriétaires dont le code source est opaque, les coûts de licence élevés, et l'adaptation aux spécificités réglementaires tunisiennes (BCT, CTAF, INPDP, NCT) laborieuse et coûteuse. Quand la BCT publie une nouvelle circulaire (ex: Circ. 2025-17 LBC/FT, applicable immédiatement), les banques dépendent de leurs fournisseurs pour implémenter la conformité, créant un délai de risque réglementaire.

Il n'existe pas de CBS open source conçu spécifiquement pour le cadre réglementaire tunisien.

---

## 5. Proposition de valeur

**BANKO** est le premier CBS open source conçu nativement pour le droit bancaire tunisien :
- **Conformité by-design** : chaque module est traçable vers un texte légal (95 références sourcées)
- **Auditabilité totale** : piste d'audit intégrale, horodatage cryptographique, immutabilité
- **Transparence du code** : AGPL-3.0, auditable par la BCT, les commissaires aux comptes, le public
- **Souveraineté** : hébergeable en Tunisie, code maîtrisé, pas de dépendance fournisseur
- **Évolutivité réglementaire** : architecture modulaire, chaque circulaire BCT = un module activable

---

## 6. Personas

### Persona 1 : Rachid — Directeur des Risques (CRO)

- **Rôle** : Chief Risk Officer d'une banque tunisienne de taille moyenne
- **Objectifs** : S'assurer que la banque respecte les ratios prudentiels BCT (solvabilité 10%, Tier 1 7%, C/D 120%), classifier correctement les créances, provisionner adéquatement
- **Frustrations** : Le CBS actuel calcule les ratios en batch de nuit — impossible d'avoir une vue temps réel. Les rapports BCT sont manuels, sources d'erreurs.

### Persona 2 : Sonia — Responsable Conformité (CMLCO)

- **Rôle** : Chief Money Laundering Compliance Officer
- **Objectifs** : KYC conforme à la Circ. 2025-17, surveillance transactionnelle, déclarations de soupçon à la CTAF, gel des avoirs, filtrage listes sanctions
- **Frustrations** : Le CBS actuel n'a pas de module KYC conforme à la nouvelle circulaire. Le filtrage est manuel. Les déclarations de soupçon sont sur papier.

### Persona 3 : Amina — Directrice Comptable

- **Rôle** : Responsable comptabilité d'un établissement financier
- **Objectifs** : Produire des états financiers conformes NCT 21/24/25, préparer la transition IFRS 9, générer les états réglementaires BCT
- **Frustrations** : Double saisie entre le CBS et le système comptable. Pas de provisionnement automatique ECL (IFRS 9). Réconciliation manuelle.

### Persona 4 : Karim — Chargé de clientèle

- **Rôle** : Agent bancaire en agence
- **Objectifs** : Ouvrir des comptes, instruire des dossiers de crédit, effectuer des opérations courantes (virements, retraits, remises chèques)
- **Frustrations** : Interface archaïque du CBS. Processus d'ouverture de compte laborieux. Pas de vue unifiée du client.

### Persona 5 : Inspecteur BCT

- **Rôle** : Inspecteur de la Banque Centrale de Tunisie
- **Objectifs** : Auditer la conformité de la banque, vérifier les ratios prudentiels, contrôler le dispositif LBC/FT, accéder aux pistes d'audit
- **Frustrations** : Les données sont fournies en Excel par la banque. Pas d'accès direct. Vérification longue et sujette à manipulation.

### Persona 6 : Farah — Développeuse/Contributrice

- **Rôle** : Étudiante en informatique, contributrice open source
- **Objectifs** : Comprendre le code, contribuer des modules, apprendre le domaine bancaire
- **Frustrations** : Les CBS propriétaires sont des boîtes noires. Pas de documentation, pas de tests, pas d'architecture claire.

---

## 7. Capacités métier requises

| # | Capacité | Description | Priorité |
|---|---|---|---|
| C1 | **Gestion des comptes** | Ouverture, clôture, consultation, soldes, relevés, types de comptes (courant, épargne, DAT) | P0 |
| C2 | **Gestion des dépôts** | Réception, restitution, calcul intérêts, dépôts à terme | P0 |
| C3 | **Gestion des crédits** | Octroi, suivi, remboursement, classification créances (classes 0-4), provisionnement | P0 |
| C4 | **KYC / CDD / EDD** | Identification clients (PP/PM), fiche KYC, bénéficiaires effectifs, PEP, scoring risque | P0 |
| C5 | **Surveillance transactionnelle (AML)** | Détection opérations suspectes, scénarios, alertes, investigation | P0 |
| C6 | **Déclarations de soupçon** | Génération, workflow validation, transmission CTAF | P0 |
| C7 | **Filtrage sanctions** | Listes ONU, UE, OFAC, nationales — filtrage temps réel | P0 |
| C8 | **Gel des avoirs** | Procédures de gel et dégel conformes Circ. 2025-17 | P0 |
| C9 | **Calcul prudentiel** | Ratios solvabilité (10%), Tier 1 (7%), C/D (120%), concentration (25% FPN) | P0 |
| C10 | **Gouvernance et contrôle interne** | 3 lignes de défense, comités (audit, risques, nomination), piste d'audit | P0 |
| C11 | **Comptabilité bancaire** | Plan comptable bancaire NCT, écritures automatiques, balance, grand livre | P0 |
| C12 | **Reporting réglementaire BCT** | États prudentiels, reporting AML, états financiers — formats BCT | P1 |
| C13 | **Opérations de paiement** | Virements nationaux/internationaux, SWIFT, compensation, ISO 20022 | P1 |
| C14 | **Opérations de change** | Achat/vente devises, position de change, conformité Loi 76-18 | P1 |
| C15 | **Monétique** | Gestion cartes, autorisations, ISO 8583 | P2 |
| C16 | **Protection des données** | Consentement, droits INPDP (accès, rectification, opposition), anonymisation | P1 |
| C17 | **Provisionnement IFRS 9** | Modèle ECL (pertes attendues), stages 1/2/3 | P2 |
| C18 | **Portail d'audit BCT** | Accès direct inspecteurs BCT, dashboards superviseurs, API audit | P2 |
| C19 | **Portail client (e-banking)** | Consultation comptes, virements, relevés en ligne | P2 |
| C20 | **Conformité ISO 27001:2022** | SMSI, 93 contrôles Annexe A, gestion risques SI | P1 |
| C21 | **Conformité PCI DSS v4.0.1** | Tokenisation, chiffrement champ, CDE scope | P1 |
| C22 | **Préparation Open Banking** | APIs PSD3-ready, consent management, SCA | P2 |
| C23 | **Conformité nouvelle loi données 2025** | DPO, DPIA, portabilité, effacement, notification 72h | P0 |
| C24 | **Intégration goAML** | Déclarations CTAF électroniques | P0 |
| C25 | **TuniCheque API** | Vérification chèques temps réel, Circ. 2025-03 | P1 |
| C26 | **e-KYC biométrique** | Enrôlement électronique, Circ. 2025-06 | P0 |

---

## 8. Glossaire métier (Ubiquitous Language DDD)

| Terme métier | Définition | Exemple |
|---|---|---|
| **Compte** | Instrument de dépôt ou de crédit identifié par un RIB, détenu par un client | Compte courant 01-234-0001234-56 |
| **Client** | Personne physique ou morale titulaire d'un compte, identifiée par une fiche KYC | SARL XYZ, CIN 12345678 |
| **Fiche KYC** | Document structuré de connaissance du client conforme Annexe 1 Circ. 2017-08/2025-17 | Identité, profession, revenus, PEP, bénéficiaire effectif |
| **Bénéficiaire effectif** | Personne physique qui possède ou contrôle in fine le client PM (≥25% capital ou contrôle de fait) | Actionnaire majoritaire SARL |
| **PEP** | Personne Politiquement Exposée — risque AML élevé, EDD obligatoire | Ministre, parlementaire, magistrat |
| **Créance** | Engagement de crédit de la banque envers un client, classifiable en 5 classes (0-4) | Crédit immobilier 150 000 TND |
| **Classe de créance** | Classification réglementaire (Circ. 91-24) : 0=courant, 1=suivi, 2=incertain, 3=préoccupant, 4=compromis | Créance classe 2 → provision 20% |
| **Provision** | Montant MINIMUM réglementaire selon Circ. 91-24 par classe (2=20%, 3=50%, 4=100%). Approche comptable déterministe, basée sur la classe d'actif. Code Rust: `struct Provision { amount: Money, asset_class: AssetClass, regulatory_min_pct: Percentage }` | Créance classe 2 de 100 TND → provision min 20 TND (20%) |
| **ECL** | Expected Credit Loss — modèle IFRS 9 probabiliste de pertes attendues. Stage 1: 12 mois, Stage 2: durée de vie, Stage 3: durée de vie + dépréciation. Approche statistique (PD × LGD × EAD). Code Rust: `struct ExpectedCreditLoss { stage: EclStage, amount: Money, probability_of_default: Decimal, loss_given_default: Percentage, exposure_at_default: Money }` | Créance 100 TND, PD=15%, LGD=40%, EAD=100 → ECL = 6 TND (Stage 1) |
| **Ratio de solvabilité** | Fonds propres réglementaires / Actifs pondérés par le risque (RWA) — minimum 10% | Ratio = 14.5% → conforme |
| **Tier 1** | Fonds propres de base (CET1 + AT1) — minimum 7% | Tier 1 = 9.2% → conforme |
| **Ratio C/D** | Crédits / Dépôts — maximum 120% (Circ. 2018-10) | C/D = 115% → conforme |
| **Ratio de concentration** | Risque sur un même bénéficiaire / FPN — maximum 25% | 22% FPN → conforme |
| **Déclaration de soupçon (DOS)** | Signalement à la CTAF d'une opération suspecte (art. 125 Loi 2015-26) | DOS transmise pour transaction atypique |
| **Gel des avoirs** | Blocage des fonds d'une personne/entité figurant sur une liste de sanctions | Gel immédiat, notification CTAF |
| **Piste d'audit** | Enregistrement chronologique immutable de toutes les opérations et accès | Qui a fait quoi, quand, sur quel compte |
| **Écriture comptable** | Enregistrement au journal d'une opération selon le plan comptable bancaire NCT | Débit 2011 / Crédit 1011 |
| **Virement SWIFT** | Transfert international via le réseau SWIFT au format ISO 20022 (MT/MX) | Virement 5 000 EUR vers BNP Paribas |
| **RIB** | Relevé d'Identité Bancaire — identifiant unique d'un compte en Tunisie | 01-234-0001234-56 |
| **DAT** | Dépôt À Terme — placement à durée fixe avec taux d'intérêt garanti | DAT 50 000 TND à 8% sur 12 mois |
| **FPN** | Fonds Propres Nets — base de calcul des ratios prudentiels | FPN = 120 M TND |
| **RWA** | Risk-Weighted Assets — actifs pondérés par le risque de crédit, marché, opérationnel | RWA = 800 M TND |
| **PNB** | Produit Net Bancaire — marge d'intermédiation + commissions | PNB = 45 M TND |
| **NCT** | Norme Comptable Tunisienne | NCT 24 = engagements bancaires |
| **SMSI** | Système de Management de la Sécurité de l'Information (ISO 27001:2022). Code : `struct Smsi { scope: Vec<BoundedContext>, controls: Vec<AnnexAControl>, risk_register: RiskRegister }` | SMSI couvrant les 12 bounded contexts BANKO |
| **CDE** | Cardholder Data Environment — périmètre PCI DSS où transitent les données cartes | Serveurs de tokenisation + passerelle paiement |
| **Tokenisation** | Remplacement du PAN par un jeton irréversible pour réduire le périmètre PCI DSS | PAN 4111-XXXX-XXXX-1234 → token tkn_a8f3... |
| **SCA** | Strong Customer Authentication — authentification à 2 facteurs minimum (connaissance + possession + inhérence) | Mot de passe + OTP mobile |
| **Consent** | Consentement granulaire, révocable, conforme loi 2025. Code : `struct Consent { id: ConsentId, customer_id: CustomerId, permissions: Vec<Permission>, granted_at: DateTime, expires_at: DateTime, status: ConsentStatus }` | Consentement partage données avec partenaire fintech |
| **DPO** | Délégué à la Protection des Données — rôle obligatoire (loi 2025) | DPO supervise traitements données personnelles |
| **DPIA** | Data Protection Impact Assessment — évaluation d'impact obligatoire pour traitements à haut risque | DPIA avant déploiement module e-KYC biométrique |
| **goAML** | Plateforme CTAF pour les déclarations de soupçon électroniques | DOS transmise via goAML à la CTAF |
| **TuniCheque** | Plateforme électronique unifiée des chèques (Circ. 2025-03) | Vérification provision chèque en temps réel |

---

## 9. Bounded Contexts identifiés (DDD)

| # | Contexte | Responsabilité | Entités principales |
|---|---|---|---|
| **BC1** | **Customer** | Gestion des clients, KYC/CDD/EDD, bénéficiaires effectifs, scoring risque | Customer, KycProfile, Beneficiary, PepCheck, RiskScore |
| **BC2** | **Account** | Gestion des comptes (courant, épargne, DAT), soldes, mouvements | Account, Balance, Movement, AccountType |
| **BC3** | **Credit** | Octroi, suivi, remboursement, classification créances, provisionnement | Loan, LoanSchedule, AssetClass, Provision |
| **BC4** | **AML** (Anti-Money Laundering) | Surveillance transactionnelle, alertes, investigations, DOS, gel avoirs | Transaction, Alert, Investigation, SuspicionReport, AssetFreeze |
| **BC5** | **Sanctions** | Filtrage listes sanctions (ONU, UE, OFAC, nationales), matching | SanctionList, SanctionEntry, ScreeningResult |
| **BC6** | **Prudential** | Calcul des ratios réglementaires (solvabilité, Tier 1, C/D, concentration) | PrudentialRatio, RiskWeightedAsset, RegulatoryCapital |
| **BC7** | **Accounting** | Comptabilité bancaire NCT, écritures, journal, grand livre, balance | JournalEntry, Ledger, ChartOfAccounts, AccountingPeriod |
| **BC8** | **Reporting** | États réglementaires BCT, rapports prudentiels, reporting AML | RegulatoryReport, ReportTemplate, ReportSubmission |
| **BC9** | **Payment** | Virements, compensation, SWIFT, ISO 20022 | PaymentOrder, Transfer, SwiftMessage, Clearing |
| **BC10** | **ForeignExchange** | Opérations de change, position de change, conformité Loi 76-18 | FxOperation, FxPosition, ExchangeRate |
| **BC11** | **Governance** | Contrôle interne, 3 lignes de défense, comités, piste d'audit | AuditTrail, Committee, ControlCheck, ComplianceReport |
| **BC12** | **Identity** | Authentification, autorisations, RBAC, sessions, 2FA | User, Role, Permission, Session, TwoFactorAuth |

---

## 10. Invariants métier critiques

Ces règles seront codées dans les constructeurs des entités Domain (`::new() → Result<Self, DomainError>`). Une violation = erreur de compilation ou rejet à la construction.

| # | Invariant | Texte légal | Bounded Context |
|---|---|---|---|
| **INV-01** | Un compte ne peut être ouvert sans fiche KYC validée | Circ. 2025-17, Circ. 2017-08 [REF-31][REF-33] | Customer + Account |
| **INV-02** | Le ratio de solvabilité ne peut être inférieur à 10% | Circ. 2016-03, 2018-06 [REF-17][REF-19] | Prudential |
| **INV-03** | Le ratio Tier 1 ne peut être inférieur à 7% | Circ. 2016-03, 2018-06 [REF-17][REF-19] | Prudential |
| **INV-04** | Le ratio Crédits/Dépôts ne peut excéder 120% | Circ. 2018-10 [REF-21] | Prudential |
| **INV-05** | Le risque sur un même bénéficiaire ne peut excéder 25% des FPN | Circ. 91-24 [REF-14] | Prudential + Credit |
| **INV-06** | Une créance doit être classée dans exactement une classe (0, 1, 2, 3 ou 4) | Circ. 91-24, 2023-02 [REF-14][REF-24] | Credit |
| **INV-07** | Le provisionnement minimum est : classe 2=20%, classe 3=50%, classe 4=100% | Circ. 91-24 [REF-14] | Credit + Accounting |
| **INV-08** | Toute opération ≥ 5 000 TND en espèces déclenche une vérification AML | Loi 2015-26 [REF-28] | AML |
| **INV-09** | Un gel des avoirs est immédiat et irrévocable sans autorisation CTAF | Circ. 2025-17 [REF-33] | AML + Sanctions |
| **INV-10** | Les données KYC doivent être conservées 10 ans minimum après clôture | Loi 2015-26 art. 125 [REF-28] | Customer |
| **INV-11** | Toute écriture comptable est équilibrée (débit = crédit) | NCT 01 [REF-44] | Accounting |
| **INV-12** | Chaque opération produit une entrée immutable dans la piste d'audit | Circ. 2006-19 [REF-35] | Governance |
| **INV-13** | Le consentement INPDP est requis avant tout traitement de données personnelles | Loi 2004-63 [REF-54] | Customer + Identity |
| **INV-14** | Un virement international requiert le filtrage sanctions avant exécution | Circ. 2025-17, GAFI R.16 [REF-33][REF-66] | Payment + Sanctions |
| **INV-15** | La somme des provisions ≥ provisions minimales réglementaires par classe | Circ. 91-24 [REF-14] | Credit + Accounting |
| **INV-16** | Données cartes (PAN) stockées UNIQUEMENT sous forme tokenisée ou chiffrée niveau champ | PCI DSS v4.0.1 Req 3.5.1.2 [REF-90] | Payment + Identity |
| **INV-17** | Accès au CDE requiert MFA (2 facteurs minimum) | PCI DSS Req 8.4.2 [REF-90] | Identity + Payment |
| **INV-18** | Toute violation de données personnelles notifiée à l'INPDP sous 72 heures | Loi données 2025 [REF-79] | Customer + Governance |
| **INV-19** | Consentement explicite requis avant tout partage de données avec un tiers | Loi données 2025 + PSD3 [REF-79][REF-91] | Customer + Identity |
| **INV-20** | Transfert international > 1000 EUR/USD inclut données originator ET beneficiary complètes | GAFI R.16 révisée [REF-83] | Payment + AML |

---

## 11. Flux multi-acteurs

| # | Capacité | Initiateur | Validateur | Consommateur | Workflow |
|---|---|---|---|---|---|
| **F1** | Ouverture de compte | Karim (chargé clientèle) | Sonia (conformité KYC) | Client | Collecte docs → KYC → Validation → Création compte |
| **F2** | Octroi de crédit | Karim | Rachid (risques) → Comité crédit | Client + Amina (compta) | Demande → Analyse risque → Classe → Décision → Déblocage |
| **F3** | Déclaration de soupçon | Système AML (alerte auto) | Sonia (investigation) → Directeur | CTAF | Alerte → Investigation → Décision → Transmission CTAF |
| **F4** | Gel des avoirs | Système Sanctions (match) | Sonia (confirmation) | CTAF + Client | Match → Vérification → Gel immédiat → Notification |
| **F5** | Classification créance | Système (analyse auto) | Rachid (validation) | Amina (provisionnement) | Analyse retards → Classification → Provision → Écriture comptable |
| **F6** | Virement international | Karim / Client e-banking | Système (filtrage sanctions) → Sonia si alerte | Banque bénéficiaire | Saisie → Filtrage → Validation → SWIFT → Compensation |
| **F7** | Reporting BCT | Système (génération auto) | Amina (vérification) → Direction | Inspecteur BCT | Collecte données → Calcul ratios → Génération → Soumission |
| **F8** | Audit BCT | Inspecteur BCT | — | BCT | Connexion portail → Requêtes → Extraction → Rapport |
| **F9** | Provisionnement IFRS 9 | Système (calcul ECL) | Amina → Commissaire aux comptes | Direction + BCT | Calcul ECL → Staging → Écriture → Validation auditeur |

---

## 12. Fonctionnalités clés (MVP — Jalons 0-2)

1. Gestion des clients avec KYC complet (Circ. 2025-17)
2. Gestion des comptes (courant, épargne, DAT)
3. Gestion des crédits avec classification créances (classes 0-4)
4. Provisionnement réglementaire automatique
5. Calcul des ratios prudentiels en temps réel (solvabilité, Tier 1, C/D, concentration)
6. Surveillance transactionnelle AML (scénarios de base)
7. Filtrage sanctions (listes ONU, nationales)
8. Piste d'audit intégrale immutable
9. Comptabilité bancaire NCT (écritures automatiques, balance, grand livre)
10. Authentification sécurisée (2FA, RBAC, sessions)

---

## 13. Fonctionnalités secondaires (post-MVP — Jalons 3+)

1. Module SWIFT / ISO 20022 complet
2. Opérations de change (Loi 76-18)
3. Monétique (ISO 8583) — dépriorisé de P1 à P2 (complexité ISO 8583, dépendance acquéreur)
4. Portail d'audit BCT (accès direct inspecteurs)
5. E-banking (portail client)
6. Provisionnement IFRS 9 (modèle ECL complet)
7. Reporting automatisé BCT (formats officiels)
8. API ouverte pour intégrations tierces

---

## 14. Contraintes

- **Stack imposée** : Rust/Actix-web + Astro/Svelte + PostgreSQL (cf. Configuration Projet)
- **Disciplines** : SOLID + DDD + BDD + TDD + Hexagonal + YAGNI + DRY
- **Organisation** : Scrum → Nexus → SAFe → ITIL
- **Langues** : AR (RTL), FR, EN
- **Licence** : AGPL-3.0 (copyleft fort)
- **Hébergement** : Souverain (Tunisie) — conformité INPDP
- **Sécurité** : HSM pour signatures cryptographiques, LUKS, Suricata, CrowdSec
- **Référentiel légal** : 95 références sourcées (docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md)
- **Auditabilité** : Chaque opération horodatée, signée cryptographiquement, immutable

---

## 15. Risques

| # | Risque | Probabilité | Impact | Mitigation |
|---|---|---|---|---|
| R1 | Évolution réglementaire rapide (nouvelle circulaire BCT) | Haute | Élevé | Architecture modulaire — chaque circulaire = module activable. Veille réglementaire continue. |
| R2 | Complexité du domaine bancaire | Haute | Élevé | DDD strict, glossaire ubiquitaire, BDD avec experts métier. KoproGo prouve la faisabilité sur un domaine juridique complexe. |
| R3 | Résistance à l'adoption (banques conservatrices) | Moyenne | Élevé | Cibler d'abord les petits établissements financiers. Prouver la conformité par audit indépendant. |
| R4 | Sécurité (cible de valeur) | Haute | Critique | Rust (mémoire sûre), HSM, audit Lynis/Suricata/CrowdSec, pentest, bug bounty. |
| R5 | Manque d'expertise bancaire | Moyenne | Élevé | Référentiel légal sourcé, collaboration avec des experts bancaires tunisiens, BDD comme documentation vivante. |
| R6 | Solo-dev en side-project | Haute | Moyen | Roadmap capacitaire (pas de dates), communauté open source, Méthode Maury avec agents IA. |
| R7 | Liste grise GAFI | Élevée | Critique | Plénière nov 2026. Mitigation : conformité effective LBC/FT, goAML, statistiques mesurables. |
| R8 | Non-conformité loi données 2025 | Moyenne | Élevé | Application juillet 2026. Mitigation : DPO, DPIA intégrée, notification automatisée. |
| R9 | Exigences PCI DSS v4.0.1 | Moyenne | Élevé | Obligatoire depuis mars 2025. Mitigation : tokenisation native, chiffrement champ, architecture CDE minimale. |

---

## 16. Principes d'architecture

- **SOLID** : 5 principes dans chaque couche
- **Architecture hexagonale stricte** : Ports & Adapters, dépendances → vers l'intérieur
- **DDD** : domaine métier protégé (droit bancaire tunisien codé dans le domain), sans dépendance externe
- **BDD** : spécifications vivantes en Gherkin — chaque exigence BCT testable
- **TDD** : test-first, couverture domain 100%
- **Sécurité by design** : conformité INPDP dès la conception, chiffrement, HSM
- **Auditabilité by design** : piste d'audit intégrale, horodatage cryptographique
- **Conformité by design** : chaque module traçable vers un texte légal [REF-XX]

---

## 17. Métriques de succès

| Métrique | Cible | Mesure |
|---|---|---|
| Conformité réglementaire | 100% des exigences BCT P0 implémentées | Checklist vs référentiel légal |
| Couverture tests domain | 100% | Tarpaulin (Rust coverage) |
| Scénarios BDD | ≥ 200 (objectif KoproGo-level) | `cargo test --test bdd` |
| Performance API | P95 < 200ms | Prometheus |
| Disponibilité | 99.9% (8.7h downtime/an max) | Uptime monitoring |
| Piste d'audit | 100% des opérations tracées | Audit trail completeness check |
| Sécurité | 0 vulnérabilité critique non mitigée | cargo audit + Lynis + pentest |
| Empreinte carbone | < 0.5g CO₂/requête | Green IT metrics |
| Conformité ISO 27001 | 93 contrôles Annexe A mappés | Dashboard SMSI |
| Conformité PCI DSS | 12 exigences validées | SAQ/ROC ready |
| Conformité loi données 2025 | 100% droits INPDP implémentés | DPO dashboard |

---

## 18. Estimation budgétaire préliminaire

### Dimensionnement projet

| Dimension | Valeur estimée |
|---|---|
| Bounded Contexts | 12 |
| Entités domain estimées | ~55-60 (4-5 par BC) |
| Endpoints API estimés | ~150-180 (12-15 par BC) |
| Catégorie projet | **Grand** (10+ BC) |

### Estimation heures par couche (grille Méthode Maury)

| Couche | Stories estimées | Heures (coefficients IA) |
|---|---|---|
| Backend (domain + API) | ~60 M + 20 L | ~280h |
| Frontend (composants + pages + i18n RTL) | ~40 M + 15 L | ~350h |
| Infrastructure (IaC + CI/CD + HSM) | ~10 M + 5 L | ~180h |
| Tests (BDD + E2E Documentation Vivante) | ~30 M + 10 L | ~100h |
| i18n (AR RTL + FR + EN) / Docs | ~15 M | ~60h |
| Compliance (ISO 27001 + PCI DSS + Open Banking) | ~15 M + 5 L | ~70h |
| **Sous-total** | | **~1 040h** |
| + 20% émergence | | ~208h |
| + 10% stabilisation CI | | ~104h |
| **TOTAL HEURES** | | **~1 352h** |

### Estimation durée calendaire

| Rythme | Calcul | Durée |
|---|---|---|
| Solo-dev side-project (8h/sem) | 1 352 ÷ 8 | ~169 sem ≈ **39 mois** |
| Solo-dev temps plein (35h/sem) | 1 352 ÷ 35 | ~39 sem ≈ **9 mois** |
| Duo (2 × 20h/sem) | 1 352 ÷ 40 | ~34 sem ≈ **8 mois** |

> Note : Cette estimation est PRÉLIMINAIRE. Elle sera affinée par le Scrum Master (étape 4) avec des stories détaillées S/M/L.

---

## 18. Alignement Stratégique

### BANKO ↔ Objectifs BCT (Banque Centrale de Tunisie)

| Objectif BCT | Lien BANKO | Implémentation |
|---|---|---|
| **Transformation digitale du secteur bancaire** | BANKO est la première plateforme de core banking digitale conçue pour la Tunisie | CBS open source, auditée, évolutive via circulaires BCT |
| **Convergence Bâle III / Circ. 2016-03, 2018-06** | Calcul des ratios prudentiels en temps réel (solvabilité, Tier 1, C/D, concentration) | BC6 (Prudential) implémente toutes les exigences de capital réglementaire |
| **Renforcement du dispositif LBC/FT (Circ. 2025-17)** | KYC/CDD/EDD conforme, surveillance transactionnelle, signalement CTAF, gel avoirs | BC1 (Customer) + BC4 (AML) + BC5 (Sanctions) |
| **Stabilité du système bancaire** | Audit trail intégrale, immutabilité, conformité, zéro point de défaillance critique | Piste d'audit cryptographique (BC11 Governance), architecture microservices résiliente |
| **Accessibilité pour petits établissements** | CBS à coût zéro (AGPL-3.0 = libre) vs. Temenos/Finastra 100 k€+/an | Licence AGPL-3.0, déploiement sur infrastructure légère |

### BANKO ↔ Politique Gouvernementale Tunisienne

| Politique | Lien BANKO | Bénéfice |
|---|---|---|
| **Plan National Stratégique (PNS) 2023-2025** | Objectif "Souveraineté technologique" — réduire dépendance aux fournisseurs étrangers | BANKO démontre qu'un CBS conforme tunisien est possible, maîtrisable localement |
| **Inclusion Financière (objectif FMI)** | Réduction coûts CBS = accès pour microfinance, banques digitales, établissements de paiement | BANKO peut servir de base à des variantes pour inclusion |
| **Attractivité pour investisseurs** | Écosystème fintech stable, régulé, transparent | BANKO = cas d'usage de référence pour startup fintech tunisiennes |
| **Transition IFRS** | BANKO prépare dès le départ double moteur comptable (NCT + IFRS 9) | Facilite passage à IFRS quand BCT l'imposera |

### BANKO dans l'Écosystème Bancaire Open Source Tunisien

**Positionnement** :
- **Unique** : Seule plateforme de core banking open source + conforme droit tunisien
- **Référence** : Démontre que "bancaire regulated + open source" n'est pas une contradiction
- **Pilote** : Peut servir de base pour autres modules (fintech, paiement, assurance)
- **Formation** : Ressource éducative pour experts bancaires tunisiens (code readable, BDD documentation)

**Écosystème partenaire** (potentiel post-MVP) :
- **CTAF** : Intégration API directe pour signalements de soupçon (DOS), rapports AML
- **Auditeurs BCT** : API portail audit (lecture de piste d'audit, rapports prudentiels)
- **Fournisseurs locaux** : Hébergement, support, intégration SWIFT/MulPay (paiements)
- **Universités** : Utilisation pédagogique (mastères bancaires, certification)
- **Startups fintech** : Utilisation comme base pour applications de paiement/crédit

---

## Pipeline suivant

Ce brief sera consommé par :
- → **Étape 2** : Product Manager (PRD avec scénarios BDD)
- → **Étape 3** : Architecte (architecture hexagonale SOLID)
- → **Étape 4** : Scrum Master (stories TDD pour agents IA)
- → **Étape 5** : Validation croisée
