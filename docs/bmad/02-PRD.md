# PRD — BANKO
## Méthode Maury — Phase TOGAF B-C (Business + SI)

**Stack** : Rust/Actix-web + Astro/Svelte + PostgreSQL
**Disciplines** : SOLID + DDD + Hexagonal + BDD + TDD
**Organisation** : Scrum → Nexus → SAFe → ITIL
**Dev** : Agents IA supervisés

**Version** : 2.0.0 — 4 avril 2026
**Auteur** : GILMRY / Projet BANKO
**Consommateur** : Étape 3 (Architecte)

---

## 1. Résumé exécutif

BANKO est un système bancaire open source (AGPL-3.0) conçu pour les banques tunisiennes. Il met en œuvre de manière irréfutable la conformité réglementaire BCT, CTAF, INPDP, et les normes internationales (Bâle III, GAFI).

**Objectif** : Fournir un Core Banking System transparent, auditable, modulaire, où chaque opération est traçable vers un texte légal [REF-XX]. Une action illégale en droit bancaire tunisien ne compile tout simplement pas.

**MVP** : Gestion des clients (KYC), comptes, crédits, calcul prudentiel, AML, sanctions, comptabilité NCT, audit trail.

**Cible MVP** : Petites banques tunisiennes, établissements financiers, startups fintech.

---

## 2. Objectifs produit (mesurables)

| Objectif | Métrique | Cible |
|---|---|---|
| Conformité réglementaire | Couverture exigences BCT P0 | 100% |
| Couverture tests domain | Tarpaulin | 100% |
| Scénarios BDD | Count gherkin | ≥ 100 (MVP) |
| Performance API | P95 latence | < 200ms |
| Disponibilité | Uptime | 99.9% |
| Piste d'audit | Complétude opérations | 100% |
| Sécurité | Vulnérabilités critiques non mitigées | 0 |
| Accessibilité i18n | Langues | AR (RTL) + FR + EN |

---

## 3. Périmètre (MVP / Hors scope)

### 3.1 MVP (Jalons 0-2 — Semaines 1-12)

**P0 — Blocantes** :
- C1: Gestion des clients (KYC/CDD/EDD complet Circ. 2025-17)
- C2: Gestion des comptes (courant, épargne, DAT)
- C3: Gestion des crédits (octroi, suivi, classification, provisionnement)
- C4: Calcul prudentiel en temps réel (ratios solvabilité, Tier 1, C/D, concentration)
- C5: AML — Surveillance transactionnelle (scénarios P0)
- C6: Sanctions — Filtrage listes ONU/nationales
- C7: Comptabilité bancaire NCT (écritures, balance, GL)
- C10: Governance — Piste d'audit immutable, 3LoD
- C12: Authentification sécurisée (2FA, RBAC, sessions)

**P1 — Importantes** :
- C6: Déclarations de soupçon (DOS) workflow
- C8: Gel des avoirs
- C9: Reporting réglementaire BCT (états prudentiels)
- C11: Opérations de paiement nationales simples
- C16: Protections INPDP (consentement, droits)

**P2 — Secondaires (post-MVP)** :
- Virements SWIFT / ISO 20022
- Opérations de change (Loi 76-18)
- Monétique (ISO 8583) — dépriorisé P2, complexité ISO 8583 + dépendance acquéreur/processeur
- Portail d'audit BCT (inspecteurs)
- E-banking portail client
- Provisionnement IFRS 9 (modèle ECL)
- Reporting automatisé avancé

### 3.2 Hors scope MVP

- Courtage, assurance
- Gestion d'actifs (hedge funds, fonds mutuels)
- Dérivés complexes
- Intégrations fintech avancées (APIs ouvertes)
- Blockchain, stablecoins
- Monnaies numériques de banque centrale (CBDC)

---

## 4. Glossaire métier (Ubiquitous Language DDD)

| Terme | Définition | Contexte DDD |
|---|---|---|
| **Compte** | Instrument de dépôt/crédit identifié par RIB, détenu par client | Account BC |
| **Client** | PP/PM titulaire, identifié par fiche KYC | Customer BC |
| **Fiche KYC** | Enregistrement d'identification conforme Annexe 1 Circ. 2025-17 [REF-31][REF-33] | Customer BC |
| **Bénéficiaire effectif** | PP qui possède/contrôle ≥25% capital ou contrôle de fait | Customer BC |
| **PEP** | Personne Politiquement Exposée — risque AML élevé, EDD obligatoire | Customer BC |
| **Créance** | Engagement de crédit classifiable en 5 classes (0-4) | Credit BC |
| **Classe de créance** | Classification réglementaire (Circ. 91-24): 0=courant, 1=suivi, 2=incertain (20%), 3=préoccupant (50%), 4=compromis (100%) | Credit BC |
| **Provision** | Montant comptabilisé pour risque de perte | Credit BC |
| **ECL** | Expected Credit Loss — IFRS 9 sur 12m ou durée de vie | Credit BC |
| **Ratio de solvabilité** | FPN / RWA ≥ 10% [REF-17][REF-19] | Prudential BC |
| **Tier 1** | Fonds propres de base (CET1 + AT1) ≥ 7% [REF-17][REF-19] | Prudential BC |
| **Ratio C/D** | Crédits / Dépôts ≤ 120% [REF-21] | Prudential BC |
| **Ratio de concentration** | Risque / bénéficiaire ≤ 25% FPN [REF-14] | Prudential BC |
| **Déclaration de soupçon** | Signalement CTAF d'opération suspecte [REF-28] art. 125 | AML BC |
| **Gel des avoirs** | Blocage irrévocable sans autorisation CTAF [REF-33] | AML BC |
| **Piste d'audit** | Enregistrement immutable de toutes opérations | Governance BC |
| **Écriture comptable** | Enregistrement journal selon plan comptable NCT | Accounting BC |
| **RIB** | Relevé d'Identité Bancaire — identifiant compte tunisien | Account BC |
| **DAT** | Dépôt À Terme — placement à durée fixe | Account BC |
| **FPN** | Fonds Propres Nets — base ratios prudentiels | Prudential BC |
| **RWA** | Risk-Weighted Assets — actifs pondérés par risque | Prudential BC |
| **NCT** | Norme Comptable Tunisienne | Accounting BC |

---

## 5. Bounded Contexts → Modules

| # | Contexte (BC) | Responsabilité | Priorité | Entités principales |
|---|---|---|---|---|
| BC1 | **Customer** | Clients (PP/PM), KYC/CDD/EDD, bénéficiaires, PEP, scoring risque | P0 | Customer, KycProfile, Beneficiary, PepCheck, RiskScore |
| BC2 | **Account** | Comptes (courant, épargne, DAT), soldes, mouvements | P0 | Account, Balance, Movement, AccountType, InterestSchedule |
| BC3 | **Credit** | Crédits (octroi, suivi, remboursement, classification, provisionnement) | P0 | Loan, LoanSchedule, AssetClass, Provision, LoanClassification |
| BC4 | **AML** | Surveillance transactionnelle, alertes, investigations, DOS, gel avoirs | P0 | Transaction, Alert, Investigation, SuspicionReport, AssetFreeze |
| BC5 | **Sanctions** | Filtrage listes sanctions (ONU, UE, OFAC, nationales) | P0 | SanctionList, SanctionEntry, ScreeningResult, ScreeningMatch |
| BC6 | **Prudential** | Ratios solvabilité, Tier 1, C/D, concentration en temps réel | P0 | PrudentialRatio, RiskWeightedAsset, RegulatoryCapital, RatioBreachAlert |
| BC7 | **Accounting** | Comptabilité NCT (écritures, journal, GL, balance, provisionnement) | P0 | JournalEntry, Ledger, ChartOfAccounts, AccountingPeriod, TrialBalance |
| BC8 | **Reporting** | États réglementaires BCT (prudentiels, AML, financiers) | P1 | RegulatoryReport, ReportTemplate, ReportSubmission, RegulatoryReportingPeriod |
| BC9 | **Payment** | Virements nationaux/SWIFT, compensation, ISO 20022 | P1 | PaymentOrder, Transfer, SwiftMessage, Clearing, PaymentRoute |
| BC10 | **ForeignExchange** | Opérations change, position change, conformité Loi 76-18 | P1 | FxOperation, FxPosition, ExchangeRate, FxLimit, FxCompliance |
| BC11 | **Governance** | Contrôle interne, 3LoD, comités, piste d'audit | P0 | AuditTrail, Committee, ControlCheck, ComplianceReport, DocumentaryEvidence |
| BC12 | **Identity** | Authentification, autorisations, RBAC, sessions, 2FA, MFA | P0 | User, Role, Permission, Session, TwoFactorAuth, UserConsent |

---

## 6. Exigences fonctionnelles (30+ FRs avec BDD Gherkin)

### 6.1 BC1 — Customer (Gestion des clients / KYC)

#### FR-001: Création de fiche KYC — Personne Physique
- **Description** : Un chargé de clientèle doit pouvoir créer une fiche KYC complète pour un client personne physique conforme Annexe 1 Circ. 2025-17 [REF-31][REF-33].
- **Priorité** : MUST | P0
- **User Story** : En tant que Karim (chargé clientèle), je veux enregistrer les données de KYC d'un nouveau client (identité, profession, revenus, adresse, PEP, source des fonds) afin de respecter les obligations de vigilance.
- **Entité DDD** : `Customer` aggregate avec `KycProfile` value object.
- **Invariants** :
  - INV-01 : Fiche KYC requise avant ouverture de compte [REF-31]
  - INV-10 : Données conservées 10 ans après clôture [REF-28]
  - INV-13 : Consentement INPDP avant traitement [REF-54]
- **SOLID** :
  - **S** : `KycRepository` ≠ `KycValidator` (SRP)
  - **O** : `KycProfileFactory` extensible pour nouvelles normes
  - **L** : `KycProfile` → `KycProfileV2025` sans breaking change
  - **I** : `IKycProvider` pour multiples sources données
  - **D** : `KycValidator` reçoit `ICompletenesChecker` en injection
- **BDD Gherkin** :

```gherkin
Scenario: Création KYC personne physique avec tous les champs obligatoires
  Given Un nouvel client personne physique sans KYC
  When Je renseigne les champs obligatoires (CIN, nom, prénom, date naissance, profession, adresse)
  And Je confirme la création
  Then Une fiche KYC est créée avec statut "EN_COURS_VALIDATION"
  And Un enregistrement d'audit est généré [Qui: Karim, Quand: T0, Quoi: FR-001]

Scenario: Création KYC échoue si CIN déjà existant
  Given Une fiche KYC existante pour CIN = 12345678
  When Je tente de créer une nouvelle KYC avec CIN = 12345678
  Then L'opération est rejetée avec erreur "CIN_ALREADY_EXISTS"
  And Aucune fiche n'est créée

Scenario: KYC nécessite consentement INPDP explicite
  Given Un formulaire KYC rempli
  When Je n'active pas la case "J'accepte le traitement de mes données"
  Then La création est bloquée avec message "CONSENT_REQUIRED"
  And Aucun enregistrement n'est persévéré

Scenario: Détection automatique PEP lors de création KYC
  Given Un nouvel client nommé "Ali Ben Ali" avec profession "Ministre"
  When J'enregistre la KYC
  Then Une vérification PEP est lancée automatiquement
  And La fiche est marquée "PEP_FLAGGED_AUTO"
  And Un workflow d'EDD renforcée est déclenché
```

- **Dépendances** : BC12 (Identity — vérifier permissions créateur)

---

#### FR-002: Validation de fiche KYC par Compliance
- **Description** : Sonia (CMLCO) doit valider ou rejeter une fiche KYC en EN_COURS_VALIDATION.
- **Priorité** : MUST | P0
- **User Story** : En tant que Sonia, je veux examiner une fiche KYC et décider "VALIDÉE" ou "REJETÉE" avec motif.
- **Entité DDD** : `KycProfile` avec state machine (EN_COURS_VALIDATION → VALIDÉE ou REJETÉE).
- **Invariants** :
  - Transition d'état conforme au workflow réglementaire
  - Tout rejet doit être documenté (REF-31)
- **BDD Gherkin** :

```gherkin
Scenario: Validation réussie d'une KYC
  Given Une fiche KYC en statut "EN_COURS_VALIDATION"
  And Tous les champs sont conformes
  When Sonia clique "VALIDER"
  Then Le statut devient "VALIDÉE"
  And Un email de confirmation est envoyé au client
  And La fiche est verrouillée (read-only)

Scenario: Rejet de KYC avec motif
  Given Une fiche KYC avec données incomplètes
  When Sonia sélectionne "REJETER" avec motif "ADRESSE_NON_CONFORME"
  Then Le statut devient "REJETÉE"
  And La fiche retourne à l'état éditable
  And Un message est envoyé à Karim avec motif

Scenario: Workflow AML déclenché après validation KYC
  Given Une fiche KYC validée
  When La validation est complétée
  Then Un appel à BC4-AML est déclenché pour initialiser la surveillance
  And Un appel à BC5-Sanctions est déclenché pour filtrage
```

- **Dépendances** : BC4 (AML — déclenchement surveillance), BC5 (Sanctions — filtrage initial)

---

#### FR-003: Enrichissement bénéficiaires effectifs
- **Description** : Pour un client personne morale, Karim doit enregistrer les bénéficiaires effectifs (≥25% capital) avec KYC individuelle.
- **Priorité** : MUST | P0
- **Entité DDD** : `Customer` (PM) → liste `Beneficiary` aggregate.
- **Invariants** :
  - Chaque bénéficiaire effectif nécessite une KYC propre
  - Bénéficiaire = PP avec ≥25% capital ou contrôle de fait
- **BDD Gherkin** :

```gherkin
Scenario: Ajout d'un bénéficiaire effectif à une PM
  Given Une fiche KYC pour SARL "TransTech" validée
  When Je clique "AJOUTER BENEFICIAIRE"
  And Je renseigne identité de Mohamed (40% capital)
  Then Le bénéficiaire est enregistré
  And Une sous-KYC est créée pour Mohamed
  And La somme des % capital = 100%

Scenario: Rejet si bénéficiaire sans KYC complète
  Given Un bénéficiaire enregistré avec donnees partielles
  When Je tente de confirmer
  Then Le système rejette avec "KYC_INCOMPLETE_FOR_BENEFICIARY"

Scenario: Détection PEP cascadée sur bénéficiaire
  Given Un bénéficiaire effectif dont le nom matche "Benali" (ministre)
  When La KYC du bénéficiaire est validée
  Then Le client parent hérite du flag "PEP_EXPOSURE"
  And Une EDD renforcée est déclenchée
```

---

#### FR-004: Vérification PEP et EDD (Enhanced Due Diligence)
- **Description** : Vérification automatique + manuelle si client ou bénéficiaire est Personne Politiquement Exposée.
- **Priorité** : MUST | P0
- **Entité DDD** : `PepCheck` aggregate, `EddProfile` value object.
- **Invariants** : Tout PEP détecté déclenche EDD obligatoire avant ouverture de compte [REF-31].
- **BDD Gherkin** :

```gherkin
Scenario: Détection PEP automatique basée sur liste interne BCT
  Given Une liste PEP interne maintenue par Sonia
  When Un nouvel client nommé "Chadli Bennour" avec profession "Magistrat"
  Then Une alerte PEP est générée immédiatement
  And Le client est marqué "EDD_REQUIRED"
  And Sonia reçoit une notification

Scenario: EDD renforcée pour PEP avec questionnaire
  Given Un client marqué PEP
  When Sonia initialise l'EDD
  Then Un questionnaire EDD (source revenus, patrimoine, bénéficiaires) est envoyé
  And Documents justificatifs sont demandés
  And Une timeline de 10 jours ouvrables est fixée

Scenario: Rejet si EDD incomplet après délai
  Given EDD initié depuis 10 jours sans réponse
  When Le délai expire
  Then La fiche KYC devient "REJET_AUTO_EDD_EXPIRE"
  And Karim et client reçoivent notification de rejet
```

---

#### FR-005: Scoring de risque client (RiskScore)
- **Description** : Calcul automatique d'un score de risque (0-100) basé sur secteur, pays origine, montants, antécédents AML.
- **Priorité** : SHOULD | P0
- **Entité DDD** : `RiskScore` value object (immutable, calculé à la demande).
- **BDD Gherkin** :

```gherkin
Scenario: Calcul RiskScore pour client de secteur normal
  Given Un client salarié d'une entreprise formelle
  When Le score est calculé
  Then RiskScore est entre 10-30 (bas risque)
  And Catégorie = "GREEN"

Scenario: RiskScore élevé pour secteur risqué
  Given Un client avec profession "Transports de valeurs" (secteur GAFI sensible)
  When Le score est calculé
  Then RiskScore est entre 60-80 (risque moyen-élevé)
  And Catégorie = "ORANGE"

Scenario: RiskScore maximum si antécédent AML
  Given Un client avec 3 alertes AML antérieures résolues
  When Le score est recalculé
  Then RiskScore ≥ 85 (risque élevé)
  And Catégorie = "RED"
  And Surveillance renforcée est activée
```

---

### 6.2 BC2 — Account (Gestion des comptes)

#### FR-006: Ouverture de compte courant
- **Description** : Ouverture d'un compte courant (RIB) pour un client avec KYC validée [REF-31].
- **Priorité** : MUST | P0
- **User Story** : En tant que Karim, je veux ouvrir un compte courant pour un client ayant une KYC validée.
- **Entité DDD** : `Account` aggregate avec state (OUVERT, SUSPENDU, CLÔTURÉ).
- **Invariants** :
  - INV-01 : Compte nécessite KYC validée avant ouverture
  - Solde initial = 0 TND
  - RIB unique par compte
- **SOLID** :
  - **S** : `AccountRepository` ≠ `RibGenerator` (SRP)
  - **O** : Types de comptes extensibles (courant → crédit → DAT)
  - **L** : Tout Account hérite des règles d'immuabilité
  - **I** : `IAccountValidator` pour règles pluggables
  - **D** : `RibGenerator` injecté dans factory
- **BDD Gherkin** :

```gherkin
Scenario: Ouverture réussie de compte courant
  Given Un client avec KYC validée (status = VALIDÉE)
  When Je clique "OUVRIR_COMPTE_COURANT"
  And Je sélectionne type = "COURANT"
  Then Un compte est créé avec:
    - RIB généré et unique
    - Solde = 0 TND
    - Devise = TND
    - Statut = OUVERT
    - DateOuverture = now
  And Une écriture comptable débite le compte actif (2011)

Scenario: Ouverture refusée si KYC non validée
  Given Un client avec KYC en statut EN_COURS_VALIDATION
  When Je clique "OUVRIR_COMPTE_COURANT"
  Then L'opération est bloquée avec message "KYC_NOT_VALIDATED"

Scenario: Ouverture refusée si client RiskScore trop élevé sans approbation
  Given Un client avec RiskScore = 90 (RED)
  When Je tente l'ouverture sans approbation direction
  Then L'opération est bloquée
  And Un workflow d'approbation direction est créé

Scenario: Génération audit trail pour ouverture
  Given Un compte créé avec succès
  When L'ouverture est complétée
  Then Un enregistrement audit immutable est généré:
    - Qui: Karim (user_id)
    - Quand: T0 (timestamp cryptographique)
    - Quoi: ACCOUNT_OPENED (action)
    - Où: RIB (ressource)
```

- **Dépendances** : BC1 (Customer — vérifier KYC), BC12 (Identity — permissions), BC11 (Governance — audit trail)

---

#### FR-007: Ouverture de compte DAT (Dépôt À Terme)
- **Description** : DAT avec taux garanti, durée fixe (6/12/24 mois), intérêts capitalisés.
- **Priorité** : MUST | P0
- **Entité DDD** : `Account` type = DAT, `InterestSchedule` value object.
- **Invariants** : Montant > 1000 TND, taux immutable pendant durée.
- **BDD Gherkin** :

```gherkin
Scenario: Ouverture DAT avec durée et taux
  Given Client avec solde suffisant dans compte courant
  When Je crée un DAT:
    - Montant = 50 000 TND
    - Durée = 12 mois
    - Taux = 8% par an
  Then Le DAT est créé avec:
    - DateEchéance = now + 12 mois
    - TauxFixe = 8.00%
    - IntérêtsCapitalisés = MONTHLY
  And Une écriture comptable débite 1027 (DAT) et crédite 1011 (courant)

Scenario: Calcul intérêts mensuels DAT
  Given Un DAT actif depuis 2 mois (50k @ 8%)
  When Le calcul d'intérêts s'exécute
  Then IntérêtsAccumulés = 50000 * 0.08 / 12 * 2 = 667 TND
  And Le solde du DAT passe à 50667 TND
  And Une écriture comptable crédite les intérêts (4010)

Scenario: Restitution DAT à échéance
  Given Un DAT arrivé à échéance
  When La date d'échéance est atteinte
  Then Le DAT est automatiquement restitué au compte courant
  And Le solde courant augmente de principal + intérêts
  And Une notification de restitution est envoyée au client
```

---

#### FR-008: Consultation et recherche de comptes
- **Description** : Karim doit pouvoir consulter les comptes d'un client, soldes, mouvements récents.
- **Priorité** : MUST | P0
- **BDD Gherkin** :

```gherkin
Scenario: Consultation solde compte
  Given Un compte ouvert depuis 3 jours avec solde = 15000 TND
  When Je clique "CONSULTER" sur le compte
  Then Je vois:
    - RIB: 01-234-0001234-56
    - Solde: 15000.00 TND
    - DateOuverture: 2026-04-01
    - Devise: TND
    - Statut: OUVERT

Scenario: Filtrage mouvements par période
  Given Un compte avec 50 mouvements (derniers 90 jours)
  When Je filtre par période "Depuis 7 jours"
  Then Seulement 12 mouvements s'affichent
  And Chaque mouvement montre (Date, Type, Montant, Solde après)

Scenario: Recherche compte par RIB
  Given L'interface de recherche
  When Je saisis RIB = "01-234-0001234-56"
  Then Le compte s'affiche avec toutes ses données
```

---

### 6.3 BC3 — Credit (Gestion des crédits)

#### FR-009: Demande de crédit et analyse risque
- **Description** : Octroi d'un crédit avec analyse risque, classification et provisionnement.
- **Priorité** : MUST | P0
- **User Story** : En tant que Karim, je veux instruire une demande de crédit. En tant que Rachid, je veux analyser le dossier et proposer une classification.
- **Entité DDD** : `Loan` aggregate, `LoanSchedule`, `LoanClassification`.
- **Invariants** :
  - INV-05 : Risque / bénéficiaire ≤ 25% FPN [REF-14]
  - INV-06 : Créance classée dans exactement une classe (0-4)
  - INV-07 : Provisionnement minimum par classe [REF-14]
- **BDD Gherkin** :

```gherkin
Scenario: Création demande crédit
  Given Un client avec KYC validée et RiskScore acceptable
  When Karim crée une demande:
    - Montant: 100 000 TND
    - Durée: 60 mois
    - Objet: Crédit immobilier
    - Garanties: Hypothèque sur bien
  Then La demande passe en statut "EN_INSTRUCTION"
  And Un dossier est créé
  And Rachid reçoit une alerte d'analyse

Scenario: Analyse risque par Rachid
  Given Une demande en EN_INSTRUCTION
  When Rachid analyse:
    - Capacité de remboursement (ratio D/E)
    - Garanties (LGD)
    - Probabilité défaut (PD estimée)
  Then Il propose une classification initiale:
    - Classe proposée: 0 (courant) si bon profil
    - Taux proposé: 7.5%
  And Un comité crédit est créé pour approbation

Scenario: Comité crédit approuve
  Given Une analyse risque proposée par Rachid
  When Le comité (3 membres) vote "APPROUVE"
  Then La demande passe en "APPROUVEE"
  And Un contrat de crédit est généré
  And Déblocage peut s'effectuer

Scenario: Déblocage crédit transfère montant
  Given Un crédit approuvé
  When Karim clique "DEBLOQUER"
  And Montant = 100 000 TND
  Then Le solde du compte client augmente de 100 000
  And Une créance est enregistrée au bilan (classe 0)
  And Un échéancier de remboursement est généré (60 mensualités)
  And Écritures comptables:
    - Débit 2111 (Crédits de consommation)
    - Crédit 1012 (Compte courant client)
```

- **Dépendances** : BC1 (Customer), BC6 (Prudential — vérifier concentration), BC7 (Accounting)

---

#### FR-010: Classification de créance (Circ. 91-24)
- **Description** : Classification automatique et manuelle des créances en classe 0-4 selon Circ. 91-24 + Circ. 2023-02 [REF-14][REF-24].
- **Priorité** : MUST | P0
- **Entité DDD** : `LoanClassification` aggregate (déclenché par analyse retards).
- **Invariants** :
  - Classe 0 : En jour
  - Classe 1 : Retard 1-30 jours
  - Classe 2 : Retard 31-90 jours → provision 20%
  - Classe 3 : Retard 91-180 jours → provision 50%
  - Classe 4 : Retard > 180 jours → provision 100%
- **BDD Gherkin** :

```gherkin
Scenario: Classification automatique — créance en jour
  Given Un crédit déblocké depuis 5 mois, sans aucun retard
  When L'analyse de classification s'exécute (mensuel)
  Then Classe = 0 (courant)
  And Provision = 0 TND
  And Aucune écriture comptable de provision

Scenario: Rétrogradation classe due à retard
  Given Un crédit avec 2 échéances impayées (45 jours de retard)
  When L'analyse s'exécute
  Then Classe rétrograde de 0 → 2
  And Provision minimum = 20% du solde restant dû
  And Amina reçoit une alerte pour écriture comptable

Scenario: Classification manuelle Rachid avec justification
  Given Une créance en classe 2 sans raison apparent
  When Rachid change manuellement la classe:
    - Nouvelle classe: 1
    - Justification: "Accord de restructuration signé"
  Then La classification change
  And Un enregistrement audit documente la décision
  And Email notification à Amina

Scenario: Provision recalculée après reclassification
  Given Créance 100 000 TND classe 0 (provision = 0)
  When Elle est reclassée classe 3 (retard 120j)
  Then Provision passe à 50 000 (50% minimum)
  And Différentielle de provision (+50 000) est comptabilisée
  And Écriture: Débit 6125 (Dotation), Crédit 3012 (Provision)
```

---

#### FR-011: Calcul et suivi d'échéancier de remboursement
- **Description** : Génération automatique de l'échéancier avec mensualité fixe (amortissement constant).
- **Priorité** : MUST | P0
- **Entité DDD** : `LoanSchedule` immutable après génération.
- **BDD Gherkin** :

```gherkin
Scenario: Génération échéancier crédit
  Given Crédit déblocké:
    - Principal: 120 000 TND
    - Durée: 60 mois
    - Taux: 7.5% annuel
  When L'échéancier est généré
  Then Mensualité = 2 429.94 TND (fixe)
  And 60 lignes d'échéance sont créées:
    - Mois 1: Intérêts = 750, Principal = 1679.94, Solde = 118320.06
    - Mois 2: Intérêts = 737.63, Principal = 1692.31, Solde = 116627.75
    - ...
    - Mois 60: Intérêts = 15.09, Principal = 2414.85, Solde = 0

Scenario: Suivi d'une échéance impayée
  Given Échéance due le 2026-05-01 de 2 429.94 TND
  When La date d'échéance passe sans paiement
  Then:
    - Jour 1-30: Statut = "RETARD_1_30J"
    - Jour 31-90: Statut = "RETARD_31_90J"
    - Jour 91+: Statut = "RETARD_90J_PLUS"
  And Emails de relance sont envoyés J+15, J+45
  And Rachid reçoit alerte risque

Scenario: Remboursement anticipé
  Given Client qui rembourse 60 000 (50%) avant terme
  When Le paiement est reçu
  Then L'échéancier est réamorcé sur 30 mois restants
  And Nouvelle mensualité = 1 215.89
  And Intérêts économisés = ~9000 TND
  And Écriture comptable débit 2111 (réduction créance)
```

---

### 6.4 BC4 — AML (Lutte anti-blanchiment)

#### FR-012: Surveillance transactionnelle (monitoring)
- **Description** : Détection automatique d'opérations suspectes selon scénarios AML [REF-28][REF-33].
- **Priorité** : MUST | P0
- **Entité DDD** : `Transaction` aggregate, `Alert` aggregate.
- **Invariants** :
  - INV-08 : Opération ≥ 5 000 TND espèces déclenche vérification [REF-28]
  - Tout virement ≥ 10 000 TND filtré sanctions [REF-33]
- **SOLID** :
  - **S** : `AmlEngine` ≠ `AlertRepository` (SRP)
  - **O** : Scénarios AML pluggables (stratégie pattern)
  - **L** : Tout Alert hérite des règles d'immutabilité
  - **I** : `IAmlScenario` pour scénarios extensibles
  - **D** : Scénarios injectés dans engine
- **BDD Gherkin** :

```gherkin
Scenario: Détection transaction > 5000 TND espèces
  Given Un client retirant 7 500 TND en espèces
  When La transaction est saisie
  Then Un scénario AML "ESPECES_IMPORTANTS" est déclenché
  And Une alerte "ESPECES_IMPORTANTS_ALERT" est créée
  And Statut alerte = "EN_INVESTIGATION"
  And Sonia reçoit une notification

Scenario: Détection structuring (plusieurs petits retrait < seuil)
  Given Un client effectuant 4 retraits de 4 800 TND (j+1, j+2, j+3, j+4)
  When Le 4ème retrait est effectué
  Then Un scénario "STRUCTURING" détecte la pattern (total = 19 200)
  And Une alerte STRUCTURING_ALERT est générée
  And Sonia reçoit notification avec contexte

Scenario: Détection virement à destination haut-risque
  Given Un virement de 15 000 TND vers pays GAFI gris
  When Le virement est initié
  Then AML check déclenche vérification destination
  And Si score risque destination > 70, alerte "HR_DESTINATION"
  And Virement mis en attente validation Sonia

Scenario: Détection premier virement client nouveau
  Given Client ouvert depuis 5j, premier virement 25 000 TND
  When Le virement est initié
  Then Alerte "ANOMALOUS_BEHAVIOR" créée (premier op large = risque)
  And Sonia doit valider avant envoi
```

- **Dépendances** : BC2 (Account), BC5 (Sanctions), BC4 (AML — autre scenario)

---

#### FR-013: Investigation d'alerte AML
- **Description** : Sonia doit pouvoir enquêter sur une alerte, collecter docs, décider transmission CTAF.
- **Priorité** : MUST | P0
- **Entité DDD** : `Investigation` aggregate, state machine.
- **BDD Gherkin** :

```gherkin
Scenario: Sonia ouvre investigation sur alerte
  Given Une alerte AML_ESPECES_IMPORTANTS
  When Sonia clique "OUVRIR_INVESTIGATION"
  Then Une investigation est créée:
    - Statut: EN_COURS
    - AlertId: référence
    - Investigateur: Sonia
    - DateOuverture: now
  And Sonia peut ajouter des notes et documents

Scenario: Collecte de documents pour investigation
  Given Une investigation ouverte
  When Sonia:
    - Consulte les transactions du client (3 mois)
    - Demande des justificatifs au client
    - Consulte les rapports d'alerte précédents
  Then Tous les documents sont attachés à l'investigation
  And Historique des consultations est tracé (audit)

Scenario: Conclusion investigation — Innocence
  Given Une investigation sur structuring avec justification métier valide
  When Sonia conclut "BENIN" (pas de soupçon)
  Then:
    - Statut investigation: CLOTUREE_BENIN
    - Alerte fermée
    - Aucune DOS transmise
    - Client reste sous surveillance normale

Scenario: Conclusion investigation — Soupçon justifié
  Given Une investigation confirmant pattern blanchiment
  When Sonia conclut "SOUPCON_FONDE"
  Then:
    - Une DOS (Déclaration de Soupçon) est créée
    - Workflow transmission CTAF s'initialise
    - Client est marqué "SOUPCON_DECLAIRE"
    - Compte peut être gelé si demandé CTAF
```

- **Dépendances** : BC4 (AML), BC11 (Governance — audit)

---

#### FR-014: Déclaration de soupçon (DOS) — transmission CTAF
- **Description** : Transmission structurée d'une DOS à la CTAF conformément art. 125 Loi 2015-26 [REF-28].
- **Priorité** : MUST | P0
- **Entité DDD** : `SuspicionReport` aggregate, immutable après génération.
- **Invariants** : Tout soupçon = DOS obligatoire à la CTAF dans délai réglementaire.
- **BDD Gherkin** :

```gherkin
Scenario: Génération DOS structurée
  Given Une investigation conclue "SOUPCON_FONDE"
  When Sonia génère une DOS
  Then Le formulaire DOS contient:
    - Identité client (CIN/Registre commerce)
    - Description opérations suspectes
    - Montants et dates
    - Motif du soupçon
    - Déclarant (Sonia)
    - DateDéclaration
  And Le format respecte le modèle CTAF officiel

Scenario: Transmission DOS à CTAF
  Given Une DOS générée et signée électroniquement
  When Sonia clique "TRANSMETTRE_CTAF"
  Then:
    - Connexion sécurisée CTAF (si API disponible) ou envoi protégé
    - Numéro de transmission généré
    - Statut DOS: TRANSMISE
    - DateTransmission: now
    - Récépissé numéroté archivé

Scenario: Notification client de DOS
  Given Une DOS transmise (sauf si court-circuitage pour raisons de sécurité)
  When La transmission est confirmée
  Then Un notif est envoyée au client:
    - "Vos opérations font l'objet d'une investigation"
    - Pas de détails (secret professionnel CTAF)
    - Client peut contacter agence

Scenario: Gel des avoirs déclenché par DOS
  Given Une DOS transmise avec demande de gel (priorité haute)
  When Sonia coche "DEMANDER_GEL_IMMEDIAT"
  Then Le gel est immédiat sans attendre CTAF [REF-33]
  And Les avoirs du client sont bloqués (mouvements interdits)
  And Notification CTAF transmise parallèlement
```

- **Dépendances** : BC4 (AML), BC2 (Account — blocage), BC5 (Sanctions — gel appliqué)

---

### 6.5 BC5 — Sanctions (Filtrage listes sanctions)

#### FR-015: Filtrage sanctions listes ONU/EU/OFAC/nationales
- **Description** : Screening automatique des clients/bénéficiaires/créanciers contre listes sanctions.
- **Priorité** : MUST | P0
- **Entité DDD** : `SanctionList` (données), `ScreeningResult` (résultats).
- **Invariants** :
  - Match = blocage immédiat sans approbation
  - Tous les mouvements filtrent avant exécution [REF-33]
- **SOLID** :
  - **S** : `SanctionRepository` ≠ `MatchingEngine` (SRP)
  - **O** : Multiples algorithmes matching (exact, fuzzy, phonétique)
  - **L** : Tout ScreeningResult immutable
  - **I** : `ISanctionListProvider` pluggable
  - **D** : Providers injectés dans engine
- **BDD Gherkin** :

```gherkin
Scenario: Screening client à la création KYC
  Given Un nouvel client "Jamal Al-Wazir" (nom arabe)
  When La KYC est créée
  Then Screening automatique contre:
    - Liste ONU (UN_OFAC, UN_UNSC)
    - Liste EU (CONSOLIDATED_LIST)
    - Liste OFAC (SDN, Sanctions_Names)
    - Liste nationale BCT
  And Si match = pas de création, erreur "SANCTIONED_PERSON"
  And Si fuzzy match (Score > 85%) = alerte manuelle Sonia

Scenario: Screening exact match — blocage immédiat
  Given Un virement vers bénéficiaire "Ali Al-Qaeda" (sur SDN OFAC)
  When Le virement est initié
  Then Screening exact match (Levenshtein distance = 0)
  And Virement BLOQUE immédiatement
  And Message d'erreur: "SANCTIONED_DESTINATION_BLOCKED"
  And Audit trail généré avec raison bloc

Scenario: Fuzzy match — escalade manuelle
  Given Un virement vers "Ahmed El-Ansari" (fuzzy match à 87% avec liste)
  When Screening détecte fuzzy match > 85%
  Then Virement mis en attente manuelle
  And Sonia reçoit alerte avec:
    - Nom saisi: Ahmed El-Ansari
    - Nom liste: Ahmad Al-Insari (SDN)
    - Confiance: 87%
  And Sonia peut approuver ou bloquer après investigation

Scenario: Renouvellement lists sanctions (daily batch)
  Given Les listes sont mises à jour quotidiennement (T+1)
  When Les nouvelles listes sont téléchargées depuis source officielle
  Then:
    - Avant 06:00 UTC: nouvelles listes en staging
    - À 06:00: activation atomique des listes
    - Tous les clients actifs sont re-screenés contre nouvelles listes
    - Matches nouveaux génèrent alertes
```

---

#### FR-016: Gel des avoirs (Asset Freeze)
- **Description** : Blocage irrévocable des avoirs en cas match sanctions ou DOS [REF-33].
- **Priorité** : MUST | P0
- **Entité DDD** : `AssetFreeze` aggregate, immutable, documentée.
- **Invariants** :
  - Gel = immédiat, irrévocable sans autorisation CTAF
  - Tous mouvements compte gelé = rejetés
  - Durée = jusqu'à déblocage explicite CTAF
- **BDD Gherkin** :

```gherkin
Scenario: Gel automatique suite match sanctions
  Given Un client avec match exact à liste ONU
  When Le screening détecte le match
  Then Le gel est appliqué immédiatement:
    - Compte mis en statut "GELE"
    - Tous les mouvements (débits, virements) rejetés
    - Crédit des intérêts DAT suspendu
    - Message d'erreur au client: "Compte gelé conformément régulation"

Scenario: Gel manuel par Sonia après investigation
  Given Une investigation AML conclue "SOUPCON_FONDE"
  When Sonia décide de geler les avoirs
  Then:
    - AssetFreeze créée avec motif "DOS_INVESTIGATION"
    - Compte gelé immédiatement
    - Notification CTAF transmise
    - Client notifié (sans détails)
    - Gel enregistré à l'audit trail

Scenario: Déblocage suite décision CTAF
  Given Un compte gelé depuis 3 mois
  When CTAF envoie l'ordre de déblocage (API ou notification officielle)
  Then:
    - Gel levé
    - Compte retourne en statut OUVERT
    - Mouvements remis en service
    - Audit trail documenta déblocage CTAF

Scenario: Requête client — accès avoirs gelés
  Given Un client avec compte gelé (soupçon)
  When Client demande accès aux fonds
  Then:
    - Message standard: "Vos avoirs sont soumis à mesures de conformité"
    - Pas de détails motif (secret investigation)
    - Direction apporte assistance si déblocage justifié
```

---

### 6.6 BC6 — Prudential (Ratios prudentiels en temps réel)

#### FR-017: Calcul ratio de solvabilité (10% minimum)
- **Description** : Fonds propres réglementaires / RWA ≥ 10% [REF-17][REF-19].
- **Priorité** : MUST | P0
- **Entité DDD** : `PrudentialRatio`, `RiskWeightedAsset`, `RegulatoryCapital`.
- **Invariants** : INV-02 ratio ≥ 10% (violation = alerte P0).
- **SOLID** :
  - **S** : `RwaCaclulator` ≠ `RatioAggregator` (SRP)
  - **O** : Pondérations risque extensibles par circulaire
  - **L** : Ratios immuables post-calcul
  - **I** : `IRwaMethodology` (standard / advanced IRB)
  - **D** : Méthodologies injectées
- **BDD Gherkin** :

```gherkin
Scenario: Calcul RWA crédits clientèle
  Given Portefeuille crédits:
    - Crédit A: 50M TND, classe 0, pondération 100% → RWA = 50M
    - Crédit B: 30M TND, classe 2, pondération 100% → RWA = 30M
    - Crédit C: 20M TND, classe 4, pondération 150% → RWA = 30M
  When RWA est calculé
  Then RWA total crédits = 110M TND

Scenario: Calcul ratio solvabilité
  Given:
    - FPN (fonds propres nets) = 15M TND
    - RWA total (crédits + marché + opérationnel) = 130M TND
  When Ratio est calculé
  Then Ratio = 15 / 130 = 11.54%
  And Status = "CONFORME" (≥ 10%)
  And Alerte = NONE

Scenario: Alerte ratio solvabilité sous seuil
  Given Ratio solvabilité calculé = 9.8% (< 10%)
  When Le calcul complète
  Then:
    - Status = "NON_CONFORME"
    - Une RatioBreachAlert est créée P0
    - Rachid (Risques) + Directeur reçoivent alerte urgente
    - Plan d'action remédiation déclenché

Scenario: Simulation avant octroi crédit
  Given Directeur analyse demande crédit de 5M
  When Il demande "simulation impact ratio"
  Then Le système calcule:
    - Ratio actuel: 11.54%
    - RWA nouveau: 130M + 5M = 135M
    - Ratio projeté: 15 / 135 = 11.11%
  And Décision d'octroi = OK, ratio reste conforme
```

- **Dépendances** : BC3 (Credit), BC7 (Accounting)

---

#### FR-018: Calcul Tier 1 ratio (7% minimum)
- **Description** : Fonds propres de base (CET1 + AT1) / RWA ≥ 7% [REF-17][REF-19].
- **Priorité** : MUST | P0
- **Invariants** : INV-03 Tier1 ≥ 7%.
- **BDD Gherkin** :

```gherkin
Scenario: Composition Tier 1
  Given:
    - Capital social: 10M TND
    - Réserves légales: 2M TND
    - Bénéfices non distribués: 2.5M TND
    - Autres ajustements: -0.5M TND
  When Tier 1 est calculé
  Then CET1 = 14M, AT1 = 0, Tier 1 = 14M

Scenario: Tier 1 ratio conforme
  Given Tier 1 = 14M, RWA = 200M
  When Ratio calculé
  Then Ratio = 14 / 200 = 7%
  And Status = "CONFORME_LIMITE" (= 7% exactly)
  And Vigilance recommandée

Scenario: Tier 1 ratio critique
  Given Tier 1 = 12M, RWA = 200M
  When Ratio calculé
  Then Ratio = 6%
  And Status = "NON_CONFORME"
  And Alerte P0 "TIER1_BELOW_MIN"
  And Direction doit augmenter capital ou réduire RWA
```

---

#### FR-019: Ratio Crédits/Dépôts ≤ 120%
- **Description** : Suivi liquidité [REF-21].
- **Priorité** : MUST | P0
- **Invariants** : INV-04 C/D ≤ 120%.
- **BDD Gherkin** :

```gherkin
Scenario: Calcul ratio C/D
  Given:
    - Total crédits actifs: 240M TND
    - Total dépôts: 200M TND
  When Ratio calculé
  Then C/D = 240 / 200 = 120%
  And Status = "CONFORME_LIMITE"

Scenario: Dépassement C/D
  Given C/D = 122%
  When Ratio calculé
  Then:
    - Status = "NON_CONFORME"
    - Alerte "CD_RATIO_EXCEEDED"
    - Plan d'action trimestriel requis

Scenario: Prévention dépassement C/D avant octroi
  Given C/D actuellement = 119%
  When Demande crédit +5M initiée
  Then Simulation: C/D futur = 121%
  And Octroi bloqué jusqu'à réduction dépôts
```

---

#### FR-020: Ratio de concentration ≤ 25% FPN
- **Description** : Risque sur un même bénéficiaire ≤ 25% des fonds propres nets [REF-14].
- **Priorité** : MUST | P0
- **Invariants** : INV-05.
- **BDD Gherkin** :

```gherkin
Scenario: Vérification concentration avant octroi crédit
  Given:
    - FPN = 20M TND
    - Limite concentration = 25% * 20M = 5M TND
    - Expositions existantes sur bénéficiaire X: 4.5M
  When Demande crédit +1M sur même bénéficiaire
  Then:
    - Exposition future = 5.5M (> 5M limit)
    - Octroi BLOQUE
    - Message: "CONCENTRATION_LIMIT_EXCEEDED"

Scenario: Concentration conforme
  Given Exposition X = 4M (80% de limite)
  When Nouveau crédit +200k sur X
  Then:
    - Exposition future = 4.2M (84% limite)
    - Status = CONFORME
    - Octroi approuvé

Scenario: Dashboard concentration
  Given Dashboard Rachid
  When Il consulte "Concentration par bénéficiaire"
  Then Liste des 20 plus importants bénéficiaires:
    - Rang 1: Groupe STEG 3.8M (19% FPN) — vert
    - Rang 2: Sicar Tunis 2.1M (10.5%) — vert
    - ...
    - Rouge si > 25%
```

---

### 6.7 BC7 — Accounting (Comptabilité bancaire NCT)

#### FR-021: Plan comptable bancaire et chart of accounts
- **Description** : Implémentation du plan comptable bancaire tunisien (NCT 01-30) avec tous les comptes.
- **Priorité** : MUST | P0
- **Entité DDD** : `ChartOfAccounts` (immuable), `Account` (plan comptable).
- **Invariants** : INV-11 écritures équilibrées, INV-07 provisionnement minimum.
- **SOLID** :
  - **S** : `ChartRepository` ≠ `TrialBalanceCalculator` (SRP)
  - **O** : Plans extensibles par circulaire
  - **L** : Comptes immuables après utilisation
  - **I** : `IAccountingStandard` (NCT / IFRS)
  - **D** : Standards injectés dans factory
- **BDD Gherkin** :

```gherkin
Scenario: Vérification plan comptable bancaire complet
  Given Le système au démarrage
  When Le chart of accounts est chargé
  Then Tous les comptes NCT sont présents:
    - Classe 1 (Actifs) : 1011, 1012, 1021, 1027, 2111, 2112, etc.
    - Classe 2 (Passifs) : 4010, 4011, etc.
    - Classe 6 (Charges) : 6111, 6125, etc.
  And Chaque compte a label, code, nature (D/C)

Scenario: Consultation chart of accounts
  Given Interface Amina
  When Elle recherche compte "2111" (Crédits consommation)
  Then Affichage:
    - Intitulé: "Crédits à la clientèle — Crédits de consommation"
    - Nature: Débit (actif)
    - Classe: 2
    - Sous-classe: 211
```

---

#### FR-022: Écritures comptables automatiques — déblocage crédit
- **Description** : Chaque opération débite/crédite les comptes appropriés automatiquement.
- **Priorité** : MUST | P0
- **Entité DDD** : `JournalEntry` aggregate (immutable après validation).
- **Invariants** : INV-11 débit = crédit.
- **BDD Gherkin** :

```gherkin
Scenario: Écriture déblocage crédit de 100k
  Given Crédit déblocké pour 100 000 TND
  When L'écriture est générée
  Then:
    - Débit 2111 (Crédits clientèle) : 100 000 TND
    - Crédit 1012 (Compte courant client) : 100 000 TND
    - Total débit = Total crédit = 100 000 ✓
  And JournalEntry créée avec:
    - JournalCode: "CREANCES"
    - DateEcriture: T
    - ReferenceDocument: LoanId
    - Immutable après validation

Scenario: Écriture dépôt initial client
  Given Client dépose 50 000 TND en espèces
  When Dépôt comptabilisé
  Then:
    - Débit 1010 (Encaisse/Caisse) : 50 000
    - Crédit 1012 (Compte client) : 50 000
    - Journal: "OPERATIONS_COURANTES"

Scenario: Écriture provisionnement créance classe 2
  Given Créance 30 000 passée classe 2 (20% provision)
  When Provision est enregistrée
  Then:
    - Débit 6125 (Dotation provision) : 6 000
    - Crédit 3012 (Provision créances) : 6 000
    - Balance: 0 ✓

Scenario: Rejet écriture déséquilibrée
  Given Une écriture mal formée:
    - Débit 2111: 100 000
    - Crédit 1012: 99 000
  When Validation
  Then L'écriture est rejetée
  And Message: "IMBALANCED_ENTRY" (débits ≠ crédits)
```

---

#### FR-023: Journal, Grand Livre et Balance
- **Description** : Enregistrement chronologique, GL par compte, TB équilibrée.
- **Priorité** : MUST | P0
- **BDD Gherkin** :

```gherkin
Scenario: Journal des opérations courantes
  Given Journal du mois (avril 2026)
  When Amina consulte le journal
  Then Liste chronologique des écritures:
    - 2026-04-01: CREANCES (+100k) — Débit 2111 / Crédit 1012
    - 2026-04-02: PROVISIONS (+6k) — Débit 6125 / Crédit 3012
    - 2026-04-05: INTERETS_DAT (+667) — Débit 1027 / Crédit 4010
  And Total débits = Total crédits

Scenario: Grand Livre compte 2111
  Given Compte 2111 (Crédits clientèle)
  When Amina ouvre le GL
  Then Solde par opération:
    - Crédit A déblocage 100k: solde = 100k
    - Crédit B déblocage 80k: solde = 180k
    - Remboursement mensuel -2.4k: solde = 177.6k
    - ...
  And Solde final = 177.6k (tous les crédits nets rembours)

Scenario: Balance générale
  Given Clôture période (fin avril 2026)
  When Balance générée
  Then Tous les comptes avec solde:
    - Classe 1 (Actifs): 400M TND (débit)
    - Classe 2 (Passifs): 350M TND (crédit)
    - Classe 6 (Charges): 2.5M TND (débit)
  And Total débits = Total crédits = 400M + 350M ✓
```

---

### 6.8 BC11 — Governance (Piste d'audit)

#### FR-024: Piste d'audit immutable et cryptographique
- **Description** : Enregistrement immutable de toutes les opérations, signé cryptographiquement [REF-35][REF-28].
- **Priorité** : MUST | P0
- **Entité DDD** : `AuditTrail` append-only, `AuditLogEntry` (valeur objet).
- **Invariants** :
  - INV-12 : Chaque opération = 1 entrée immutable
  - Horodatage cryptographique (TSA — Time Stamp Authority)
  - Signature ECDSA ou RSA
- **SOLID** :
  - **S** : `AuditRepository` ≠ `CryptographicSigner` (SRP)
  - **O** : Signatures pluggables (ECDSA, RSA)
  - **L** : AuditEntry immutable, append-only
  - **I** : `ICryptographicProvider` pour HSM ou soft
  - **D** : Providers injectés
- **BDD Gherkin** :

```gherkin
Scenario: Création entrée audit pour chaque opération
  Given Une opération (ouverture compte, création crédit, etc.)
  When L'opération complète
  Then Une AuditLogEntry est créée avec:
    - Id: UUID unique
    - Timestamp: UTC ISO 8601 (TSA)
    - UserId: Karim
    - Action: "ACCOUNT_OPENED"
    - ResourceType: "Account"
    - ResourceId: RIB
    - Changes: {before: null, after: {...}}
    - IpAddress: 192.168.1.10
    - SessionId: session_abc123
    - Signature: ECDSA(entry, private_key)
    - HashPrecedent: hash(previous_entry) [chaînage]

Scenario: Immuabilité audit trail
  Given Une AuditLogEntry créée il y a 3 mois
  When Quelqu'un tente de modifier l'entrée
  Then:
    - Modification REJETEE au niveau DB (NOT NULL hash)
    - Erreur: "AUDIT_ENTRY_IMMUTABLE"
    - Tentative est loggée comme anomalie sécurité

Scenario: Vérification intégrité chaîne audit
  Given Audit trail avec 1000 entrées
  When Inspecteur BCT demande vérification intégrité
  Then Système vérifie:
    - Chaque hash_precedent = hash(entrée précédente)
    - Aucun chaînon manquant
    - Toutes les signatures cryptographiques valides
  And Rapport: "INTEGRITY_OK" ou "TAMPERING_DETECTED"

Scenario: Consultation audit trail par Rachid
  Given Audittail pour Crédit 12345 depuis déblocage
  When Rachid consulte l'historique
  Then Affichage chronologique:
    - 2026-03-15 14:23:45: LOAN_CREATED (Karim)
    - 2026-03-16 09:00:00: LOAN_APPROVED (Comité)
    - 2026-04-01 08:30:00: LOAN_DISBURSED (Karim)
    - 2026-04-05 15:00:00: LOAN_CLASSIFIED (Auto, classe 0)
    - 2026-04-20 10:30:00: REPAYMENT_RECEIVED (+2429.94)
  And Chaque entrée montre les modifications (avant/après)
```

---

#### FR-025: Accès audit trail par inspecteur BCT
- **Description** : Inspecteur BCT peut accéder audit trail en lecture seule via portail sécurisé.
- **Priorité** : SHOULD | P1
- **BDD Gherkin** :

```gherkin
Scenario: Inspecteur BCT consulte audit trail
  Given Inspecteur BCT connecté au portail d'audit
  When Il sélectionne "Audit trail" et filtre par période
  Then Il voit:
    - Opérations 2026-01-01 à 2026-04-01
    - Possibilité de filtrer par action, user, ressource
    - Export CSV possible pour analyse externe
  And Les données sont en lecture seule (pas de modification possible)
  And Chaque accès est lui-même audité
```

---

### 6.9 BC12 — Identity (Authentification et autorisation)

#### FR-026: Authentification 2FA
- **Description** : Authentification multi-facteur (mot de passe + SMS/Email/TOTP) [REF-31].
- **Priorité** : MUST | P0
- **Entité DDD** : `User`, `TwoFactorAuth`, `Session`.
- **SOLID** :
  - **S** : `AuthService` ≠ `TwoFactorProvider` (SRP)
  - **O** : Providers 2FA pluggables (SMS, Email, Authenticator)
  - **L** : Sessions immuables
  - **I** : `ITwoFactorProvider` extensible
  - **D** : Providers injectés
- **BDD Gherkin** :

```gherkin
Scenario: Login avec 2FA SMS
  Given Karim accède au login
  When Il renseigne email + mot de passe correct
  Then:
    - Authentification primaire réussit
    - SMS envoyé au +216 97 123 456 : "Code: 452789"
    - Écran "Entrer code 2FA"
  And Délai d'expiration code = 5 minutes

Scenario: Entrée code 2FA correct
  Given Code 452789 reçu par SMS
  When Karim saisit le code
  Then:
    - Validation réussit
    - Session créée (JWT + refresh token)
    - Redirection vers dashboard
    - AuditLogEntry générée (LOGIN)

Scenario: Rejet code 2FA expiré
  Given Code SMS reçu il y a 6 minutes
  When Karim saisit le code
  Then:
    - Rejet: "2FA_CODE_EXPIRED"
    - Message: "Demander un nouveau code"
    - Tentative échouée loggée

Scenario: Blocage après 3 tentatives 2FA
  Given 3 codes incorrects saisis
  When 4ème tentative
  Then:
    - Session verrouillée 30 minutes
    - Message: "Trop de tentatives. Réessayez dans 30 min"
    - Alerte sécurité envoyée à Karim
```

---

#### FR-027: Contrôle d'accès basé rôles (RBAC)
- **Description** : Permissions par rôle (Admin, Karim, Sonia, Rachid, Amina, etc.).
- **Priorité** : MUST | P0
- **Entité DDD** : `Role`, `Permission`, `User` → roles mapping.
- **BDD Gherkin** :

```gherkin
Scenario: Définition rôles et permissions
  Given Configuration des rôles
  Then Rôles définis:
    - SUPER_ADMIN: tous les droits
    - ACCOUNT_OFFICER (Karim): ouverture comptes, demandes crédit, virements
    - COMPLIANCE_OFFICER (Sonia): validation KYC, investigations AML, DOS
    - RISK_OFFICER (Rachid): analyses crédit, classification, ratios
    - ACCOUNTING (Amina): comptabilité, reporting
    - AUDIT (Inspecteur): lecture audit trail
  And Chaque rôle a permissions spécifiques

Scenario: Permission vérifiée avant action
  Given Karim (ACCOUNT_OFFICER) sans permission "APPROVE_LOAN"
  When Il tente d'approuver un crédit
  Then:
    - Action BLOQUEE
    - Message: "INSUFFICIENT_PERMISSIONS"
    - AuditLogEntry loggée (tentative non-autorisée)

Scenario: Escalade d'accès
  Given Un crédit > 500k TND nécessite approbation directeur
  When Karim crée le crédit
  Then Workflow d'escalade:
    - Crédit en statut "AWAITING_DIRECTOR_APPROVAL"
    - Email directeur
    - Seul user avec rôle DIRECTOR peut approuver
```

---

#### FR-028: Gestion des sessions et timeout
- **Description** : Sessions avec timeout, logout, révocation.
- **Priorité** : MUST | P0
- **BDD Gherkin** :

```gherkin
Scenario: Session créée avec durée de vie
  Given User Karim authentifié
  When Session créée
  Then:
    - SessionId: UUID
    - UserId: karim_id
    - CreatedAt: T0
    - ExpiresAt: T0 + 8 heures (configurable)
    - LastActivity: T0
    - IpAddress: 192.168.1.10

Scenario: Timeout session après inactivité
  Given Session créée il y a 8h02 (timeout = 8h)
  When Karim effectue une action
  Then:
    - Vérification: ExpiresAt < now
    - Session invalidée
    - Redirection login avec "Session expired"
    - AuditLogEntry: SESSION_EXPIRED

Scenario: Logout explicite
  Given Karim clique "DECONNEXION"
  When Logout déclenché
  Then:
    - Session marquée REVOKED
    - JWT refresh token supprimé
    - Redirection login
    - AuditLogEntry: LOGOUT
```

---

#### FR-029: Consentement et droits INPDP
- **Description** : Gestion des consentements pour traitement données personnelles [REF-54].
- **Priorité** : MUST | P0
- **Entité DDD** : `UserConsent` aggregate.
- **Invariants** : INV-13 consentement explicite obligatoire.
- **BDD Gherkin** :

```gherkin
Scenario: Demande consentement lors création KYC
  Given Formulaire KYC
  When Karim présente à client
  Then Affichage des consentements:
    - "J'accepte le traitement de mes données à caractère personnel"
    - "J'accepte les communications marketing" (optionnel)
    - "J'accepte l'analyse crédit et scoring de risque"
  And Client doit cocher explicitement avant envoi

Scenario: Conservation trace du consentement
  Given Client a coché consentements
  When Formulaire soumis
  Then UserConsent enregistré:
    - CustomerId: client_123
    - ConsentType: "KYC_PROCESSING"
    - Granted: true
    - GrantedAt: 2026-04-04 14:30:00
    - IpAddress: 196.168.1.5
    - UserAgent: Mozilla/5.0...
    - Version: Conditions_20260401

Scenario: Droit d'opposition à marketing
  Given Client avec consent marketing = true
  When Il se rétracte via email/portail
  Then:
    - Consent marketing changé à false
    - Aucun email marketing envoyé dorénavant
    - Trace conservée 3 ans (INPDP)

Scenario: Droit d'accès aux données personnelles
  Given Client demande "Quelles données avez-vous sur moi?"
  When Demande traitée
  Then:
    - Export JSON de toutes les données personnelles
    - Fourni dans 30 jours (INPDP)
    - Entrée audit: "DATA_ACCESS_REQUEST"
```

---

### 6.10 BC8 — Reporting (États réglementaires BCT)

#### FR-030: Génération états prudentiels mensuels
- **Description** : États prudentiels BCT (solvabilité, Tier 1, concentration, C/D) format officiel.
- **Priorité** : SHOULD | P1
- **Entité DDD** : `RegulatoryReport`, `ReportTemplate`.
- **BDD Gherkin** :

```gherkin
Scenario: Génération automatique rapport prudentiel fin de mois
  Given Clôture du mois avril 2026
  When Batch job s'exécute le 30 avril
  Then Rapport prudentiel généré:
    - RatioSolvabilité: 11.54%
    - RatioTier1: 8.2%
    - RatioCD: 118%
    - Concentration: 19% FPN
    - RiskWeightedAssets: 130M TND
  And Rapport sauvegardé en format PDF + XML officiel BCT
  And Email d'notification envoyé à Amina

Scenario: Transmission rapport à BCT
  Given Rapport prudentiel généré
  When Amina valide et clique "TRANSMETTRE_BCT"
  Then:
    - Format XML conforme schéma BCT
    - Signature numérique de la banque
    - Transmission sécurisée (SFTP ou API BCT)
    - Récépissé de réception archivé
```

---

### 6.11 BC9 — Payment (Opérations de paiement)

#### FR-031: Virement national simple
- **Description** : Transfert fonds entre comptes en TND [REF-28][REF-33].
- **Priorité** : SHOULD | P1
- **Entité DDD** : `PaymentOrder`, `Transfer`.
- **BDD Gherkin** :

```gherkin
Scenario: Initiation virement national
  Given Client Karim veut virer 15 000 TND vers autre compte
  When Karim remplit:
    - Compte origine: 01-234-0001234-56
    - RIB bénéficiaire: 02-345-0005678-90
    - Montant: 15 000 TND
    - Motif: "Paiement fournisseur"
  Then:
    - PaymentOrder créée (EN_COURS)
    - Montant débité temporairement (suspense)
    - Vérifications:
      - Solvabilité compte
      - Sanctions sur bénéficiaire
      - Limites de transaction

Scenario: Filtrage sanctions avant exécution
  Given Virement vers RIB 02-345-0005678-90
  When Screening lancé
  Then Pas de match → virement peut continuer
  And Transfert exécuté:
    - Débit 1012 compte origine
    - Crédit 1012 compte bénéficiaire
    - Compensation BCT lancée
    - DateExecution: T
    - Statut: EXECUTED
    - Email confirmation envoyée

Scenario: Rejet virement si montant > limite quotidienne
  Given Client limite quotidienne = 100 000 TND
  When Il viremaximale = 100 000 TND (déjà utilisé aujourd'hui)
  And Nouveau virement = 20 000 TND
  Then Rejet: "DAILY_LIMIT_EXCEEDED"
  And Message: "Limite quotidienne dépassée. Réessayez demain."
```

---

## 7. Exigences non-fonctionnelles

### 7.1 Performance

| Exigence | Cible | Mesure |
|---|---|---|
| **Latence API P95** | < 200ms | Prometheus endpoint |
| **Throughput** | 500 req/s | Load testing |
| **Time to First Byte (TTFB)** | < 100ms | Frontend metrics |
| **Calcul prudentiel** | < 5s pour 1000 créances | Benchmark unit test |
| **Screening sanctions** | < 100ms / transaction | Latency tracking |

### 7.2 Sécurité

| Exigence | Détail | Référence |
|---|---|---|
| **Authentification** | 2FA obligatoire, 3 tentatives max avant blocage 30min | [REF-31] |
| **Chiffrement données en transit** | TLS 1.3 obligatoire, cipher suite forte | [REF-54] |
| **Chiffrement données au repos** | AES-256-GCM ou équivalent | [REF-54] |
| **Signatures cryptographiques** | Audit trail signé ECDSA/RSA (HSM recommandé) | [REF-35] |
| **Gestion secrets** | Vault ou AWS Secrets Manager (pas en clair) | [REF-31] |
| **CORS** | Whitelist strict, pas de "*" | OWASP |
| **Rate limiting** | 100 req/min par IP pour login, 1000 pour API | OWASP |
| **SQL injection** | Prepared statements partout, jamais de string concat | OWASP |
| **XSS** | CSP headers strict, Svelte escaping automatique | OWASP |
| **CSRF** | SameSite=Strict sur tous les cookies | OWASP |
| **Audit vulnérabilités** | `cargo audit` hebdomadaire, pentest annuel | DevSecOps |

### 7.3 Conformité INPDP (Loi 2004-63) [REF-54]

| Exigence | Détail |
|---|---|
| **Consentement** | Explicite avant traitement, tracé (FR-029) |
| **Droit d'accès** | Export données personnelles en 30 jours |
| **Droit de rectification** | Client peut modifier données fausses |
| **Droit d'opposition** | Marketing, analyse crédit (FR-029) |
| **Droit à l'oubli** | Suppression après 10 ans clôture compte (INV-10) |
| **Anonymisation** | Données test non-réelles, compliance masquage |
| **DPA (Data Protection Agreement)** | Si tiers (cloud, payment provider) |
| **Breach notification** | 72h à INPDP si fuite > 10 personnes |

### 7.4 i18n — Langues et RTL

| Langue | Direction | Priorité | Domaines |
|---|---|---|---|
| **Arabe (ar-TN)** | RTL | P0 | UI complète + rapports |
| **Français (fr-FR)** | LTR | P0 | UI complète + légal |
| **Anglais (en-US)** | LTR | P1 | API docs + support |

**Format nombres/devises** : TND avec 3 décimales pour montants, virgule français.
**Formats dates** : DD/MM/YYYY (français) ou DD/MM/YYYY (arabe).

### 7.5 ITIL — Gestion de service

| Domaine | Exigence |
|---|---|
| **Incident Management** | SLA P1 = 4h, P2 = 24h, CMDB, runbooks |
| **Change Management** | CAB (Change Advisory Board), test env obligatoire |
| **Availability Management** | RTO = 4h, RPO = 1h (données), 99.9% target |
| **Capacity Planning** | Monitoring CPU/RAM/Disk, alertes à 80% |
| **Service Catalog** | Documenter tous les services (API, reporting) |

---

## 8. Frontend UX (Parcours par persona)

### 8.1 Karim — Chargé de clientèle

**Workflow principal** : Ouverture compte → Crédit → Virements

**Écrans clés** :
1. **Dashboard** : Liste clients à traiter, demandes en attente
2. **Fiche client** : KYC, comptes, crédits, mouvements, scoring risque
3. **Ouverture compte** : Formulaire structuré, validation temps réel
4. **Demande crédit** : Workflow pas à pas, upload docs, simulation intérêts
5. **Virement** : Saisie RIB bénéficiaire, validation sanctions, confirmation

**Accessibilité** : Arabe (RTL) + Français, grand texte pour agence, clavier navigation

---

### 8.2 Sonia — Conformité AML

**Workflow principal** : Validation KYC → Investigation → DOS

**Écrans clés** :
1. **Queue KYC** : Fiches en attente validation, avec statut complétude
2. **Détails KYC** : Révision documents, PEP check, validation/rejet
3. **Alertes AML** : Dashboard alerte transactionnelles, filtrage par client/type
4. **Investigation** : Timeline, documents joints, notes, décision soupçon
5. **DOS** : Formulaire pré-rempli, transmission CTAF, suivi

**Accessibilité** : Recherche rapide client, bulk validation, export CSV

---

### 8.3 Rachid — Risques (CRO)

**Workflow principal** : Analyse crédit → Classification → Ratios prudentiels

**Écrans clés** :
1. **Dashboard ratios** : Solvabilité, Tier 1, C/D, concentration — code couleur (vert/orange/rouge)
2. **Portefeuille crédits** : Tous les crédits actifs, classe, provision, retard
3. **Analyse crédit** : Dossier complet (capacité remboursement, garanties, PD), notation
4. **Classification créances** : Vue audit (avant/après), justifications
5. **Alertes concentration** : Clients > 20% FPN, projection si nouveau crédit

**Accessibilité** : Tableaux croisés dynamiques, export Excel, graphiques

---

### 8.4 Amina — Comptabilité

**Workflow principal** : Comptabilité → Reporting → Balance

**Écrans clés** :
1. **Journal** : Toutes les écritures du jour, filtrage par journal (CREANCES, DEPOTS, etc.)
2. **Grand Livre** : Solde par compte, mouvements détaillés
3. **Balance générale** : Tous les comptes, balances avant/après régularisations
4. **Provisions** : Dotations/reprises par classe créance, réconciliation
5. **Reporting BCT** : États prudentiels, transmission, historique

**Accessibilité** : Drill-down depuis grand livre vers opérations, audit trail liée

---

### 8.5 Inspecteur BCT

**Workflow principal** : Portail d'audit → Requêtes → Export

**Écrans clés** :
1. **Authentication** : Certificat numérique ou 2FA
2. **Dashboard** : Vue synthétique (ratios actuels, alertes, dernières opérations)
3. **Audit Trail** : Recherche par période/action/user, export CSV
4. **Données prudentielles** : Consultation directe ratios, RWA, FPN
5. **Conformité AML** : Nombre DOS transmises, alertes pendantes, investigations

**Sécurité** : Lecture seule, chaque accès audité, VPN/TLS obligatoire

---

## 9. Documentation Vivante (Flux critiques E2E)

Tous les flux P0 doivent être documentés en vidéos E2E + scénarios BDD :

### 9.1 Flux E2E — Ouverture de compte
**Acteur** : Karim
**Durée** : 10 min
**Description** : Du formulaire KYC à la création du compte courant.

### 9.2 Flux E2E — Octroi crédit
**Acteur** : Karim → Rachid → Karim
**Durée** : 15 min
**Description** : Demande crédit → Analyse → Approbation → Déblocage.

### 9.3 Flux E2E — Alerte AML → DOS
**Acteur** : Système → Sonia
**Durée** : 10 min
**Description** : Transaction suspecte → Alerte → Investigation → DOS CTAF.

### 9.4 Flux E2E — Calcul prudentiel
**Acteur** : Système (batch)
**Durée** : 5 min
**Description** : Calcul fin de jour RWA → Ratios → Alertes si dépassement.

---

## 10. Modèle de données (Entités DDD → Tables PostgreSQL)

### 10.1 BC1 — Customer

```sql
-- Table customers (personnes physiques et morales)
CREATE TABLE customers (
  id UUID PRIMARY KEY,
  customer_type VARCHAR(10) CHECK (customer_type IN ('PP', 'PM')), -- PP = Personne Physique
  first_name VARCHAR(100),  -- Null si PM
  last_name VARCHAR(100),   -- Null si PM
  legal_name VARCHAR(200),  -- Nom SARL si PM
  cin_passport VARCHAR(20),
  registration_number VARCHAR(50), -- Numéro registre commerce si PM
  birth_date DATE,
  nationality VARCHAR(3), -- Code ISO
  profession VARCHAR(100),
  risk_score SMALLINT CHECK (risk_score BETWEEN 0 AND 100),
  kyc_status VARCHAR(20) CHECK (kyc_status IN ('EN_COURS_VALIDATION', 'VALIDÉE', 'REJETÉE', 'SUSPENDUE')),
  created_at TIMESTAMPTZ,
  created_by UUID REFERENCES users(id),
  updated_at TIMESTAMPTZ,
  updated_by UUID REFERENCES users(id),
  deleted_at TIMESTAMPTZ, -- Soft delete
  UNIQUE(cin_passport)
);

-- Table kyc_profiles (détails KYC conformes Circ. 2025-17)
CREATE TABLE kyc_profiles (
  id UUID PRIMARY KEY,
  customer_id UUID NOT NULL REFERENCES customers(id),
  identity_document_type VARCHAR(20), -- CIN, PASSPORT
  identity_document_number VARCHAR(20),
  issue_date DATE,
  expiry_date DATE,
  issuing_country VARCHAR(3),
  address_street VARCHAR(200),
  address_city VARCHAR(100),
  address_postal_code VARCHAR(10),
  address_country VARCHAR(3),
  phone_number VARCHAR(20),
  email VARCHAR(100),
  profession VARCHAR(100),
  monthly_revenue DECIMAL(15, 3),
  revenue_source VARCHAR(200),
  pep_flag BOOLEAN DEFAULT FALSE,
  pep_details VARCHAR(500), -- Si PEP, description
  edd_status VARCHAR(20) DEFAULT 'NOT_REQUIRED', -- NOT_REQUIRED, IN_PROGRESS, COMPLETED
  consent_personal_data BOOLEAN DEFAULT FALSE,
  consent_marketing BOOLEAN DEFAULT FALSE,
  consent_granted_at TIMESTAMPTZ,
  validated_at TIMESTAMPTZ,
  validated_by UUID REFERENCES users(id),
  rejection_reason VARCHAR(500),
  rejection_date TIMESTAMPTZ,
  created_at TIMESTAMPTZ,
  PRIMARY KEY (id),
  FOREIGN KEY (customer_id) REFERENCES customers(id)
);

-- Table beneficiaries (bénéficiaires effectifs pour PM)
CREATE TABLE beneficiaries (
  id UUID PRIMARY KEY,
  customer_id UUID NOT NULL REFERENCES customers(id), -- PM parente
  beneficiary_id UUID NOT NULL REFERENCES customers(id), -- PP bénéficiaire
  capital_percentage DECIMAL(5, 2) CHECK (capital_percentage > 0 AND capital_percentage <= 100),
  control_description VARCHAR(200), -- "Droit de vote majoritaire", etc.
  kyc_validated BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ,
  UNIQUE(customer_id, beneficiary_id)
);
```

### 10.2 BC2 — Account

```sql
CREATE TABLE accounts (
  id UUID PRIMARY KEY,
  customer_id UUID NOT NULL REFERENCES customers(id),
  rib VARCHAR(30) UNIQUE NOT NULL,  -- Relevé d'Identité Bancaire
  account_type VARCHAR(20) CHECK (account_type IN ('COURANT', 'EPARGNE', 'DAT')),
  currency VARCHAR(3) DEFAULT 'TND',
  balance_current DECIMAL(15, 3) DEFAULT 0,
  balance_reserved DECIMAL(15, 3) DEFAULT 0, -- Montants gelés/en suspense
  account_status VARCHAR(20) CHECK (account_status IN ('OUVERT', 'SUSPENDU', 'GELE', 'CLÔTURÉ')),
  opened_at TIMESTAMPTZ,
  opened_by UUID REFERENCES users(id),
  closed_at TIMESTAMPTZ,
  close_reason VARCHAR(200),
  interest_rate DECIMAL(5, 3) DEFAULT 0, -- Pour DAT
  term_months SMALLINT, -- Null si COURANT
  maturity_date DATE, -- Pour DAT
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

CREATE TABLE movements (
  id UUID PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES accounts(id),
  movement_type VARCHAR(20), -- DEPOT, RETRAIT, VIREMENT_ENTRANT, VIREMENT_SORTANT, INTERET, PROVISION
  amount DECIMAL(15, 3) NOT NULL,
  balance_after DECIMAL(15, 3),
  description VARCHAR(500),
  reference_document_id UUID, -- LoanId, TransferId, etc.
  created_at TIMESTAMPTZ
);

CREATE TABLE interest_schedules (
  id UUID PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES accounts(id),
  interest_rate DECIMAL(5, 3),
  capitalization VARCHAR(20), -- MONTHLY, QUARTERLY, ANNUALLY
  accrued_interest DECIMAL(15, 3) DEFAULT 0,
  last_capitalized_at DATE
);
```

### 10.3 BC3 — Credit

```sql
CREATE TABLE loans (
  id UUID PRIMARY KEY,
  customer_id UUID NOT NULL REFERENCES customers(id),
  principal_amount DECIMAL(15, 3) NOT NULL,
  disbursed_amount DECIMAL(15, 3) DEFAULT 0,
  outstanding_amount DECIMAL(15, 3), -- Principal restant dû
  interest_rate DECIMAL(5, 3),
  term_months SMALLINT,
  disbursement_date DATE,
  maturity_date DATE,
  loan_purpose VARCHAR(200),
  loan_status VARCHAR(20) CHECK (loan_status IN ('DEMAND', 'ANALYSIS', 'APPROVED', 'DISBURSED', 'REPAID', 'DEFAULTED')),
  asset_class SMALLINT CHECK (asset_class IN (0, 1, 2, 3, 4)), -- 0=courant, ..., 4=compromis
  asset_class_date DATE,
  asset_class_set_by UUID REFERENCES users(id),
  guarantee_type VARCHAR(100), -- Hypothèque, caution, nantissement
  guarantee_value DECIMAL(15, 3),
  default_days SMALLINT DEFAULT 0, -- Jours en retard
  created_at TIMESTAMPTZ,
  created_by UUID REFERENCES users(id)
);

CREATE TABLE loan_schedules (
  id UUID PRIMARY KEY,
  loan_id UUID NOT NULL REFERENCES loans(id),
  installment_number SMALLINT,
  due_date DATE NOT NULL,
  principal_payment DECIMAL(15, 3),
  interest_payment DECIMAL(15, 3),
  total_payment DECIMAL(15, 3),
  paid_at DATE,
  paid_amount DECIMAL(15, 3) DEFAULT 0,
  status VARCHAR(20), -- PENDING, PAID, OVERDUE
  days_overdue SMALLINT DEFAULT 0
);

CREATE TABLE provisions (
  id UUID PRIMARY KEY,
  loan_id UUID NOT NULL REFERENCES loans(id),
  asset_class SMALLINT,
  provision_rate DECIMAL(5, 2), -- 0, 20, 50, 100
  provision_amount DECIMAL(15, 3),
  created_at TIMESTAMPTZ,
  set_by UUID REFERENCES users(id)
);
```

### 10.4 BC4 — AML

```sql
CREATE TABLE transactions (
  id UUID PRIMARY KEY,
  account_id UUID REFERENCES accounts(id),
  transaction_type VARCHAR(20), -- DEPOSIT, WITHDRAWAL, TRANSFER, CASH_OPERATION
  amount DECIMAL(15, 3),
  currency VARCHAR(3),
  description VARCHAR(500),
  beneficiary_info VARCHAR(500),
  initiated_at TIMESTAMPTZ,
  executed_at TIMESTAMPTZ,
  execution_status VARCHAR(20), -- PENDING, EXECUTED, REJECTED, SUSPENDED
  created_at TIMESTAMPTZ
);

CREATE TABLE aml_alerts (
  id UUID PRIMARY KEY,
  transaction_id UUID REFERENCES transactions(id),
  alert_type VARCHAR(50), -- ESPECES_IMPORTANTS, STRUCTURING, HR_DESTINATION, ANOMALOUS_BEHAVIOR
  alert_score DECIMAL(5, 2), -- 0-100
  alert_status VARCHAR(20), -- EN_INVESTIGATION, BENIN, SOUPCON_FONDE, CLOSED
  created_at TIMESTAMPTZ
);

CREATE TABLE investigations (
  id UUID PRIMARY KEY,
  alert_id UUID REFERENCES aml_alerts(id),
  customer_id UUID REFERENCES customers(id),
  investigator_id UUID REFERENCES users(id),
  status VARCHAR(20), -- EN_COURS, CLOTUREE_BENIN, SOUPCON_FONDE
  opened_at TIMESTAMPTZ,
  closed_at TIMESTAMPTZ,
  conclusion_notes TEXT
);

CREATE TABLE suspicion_reports (
  id UUID PRIMARY KEY,
  investigation_id UUID REFERENCES investigations(id),
  report_content TEXT, -- Formulaire DOS structure
  signed_at TIMESTAMPTZ,
  signed_by UUID REFERENCES users(id),
  transmitted_to_ctaf_at TIMESTAMPTZ,
  transmission_number VARCHAR(50), -- Numéro CTAF
  ctaf_receipt_date DATE
);

CREATE TABLE asset_freezes (
  id UUID PRIMARY KEY,
  customer_id UUID REFERENCES customers(id),
  account_id UUID REFERENCES accounts(id),
  freeze_reason VARCHAR(100), -- SANCTIONS_MATCH, AML_INVESTIGATION
  freeze_date TIMESTAMPTZ,
  freeze_reason_detail TEXT,
  unfrozen_at TIMESTAMPTZ,
  unfrozen_by_ctaf_order_id VARCHAR(100)
);
```

### 10.5 BC5 — Sanctions

```sql
CREATE TABLE sanction_lists (
  id UUID PRIMARY KEY,
  list_name VARCHAR(100), -- UN_OFAC, EU_CONSOLIDATED, OFAC_SDN, OFAC_SANCTIONS_NAMES, BCT_NATIONAL
  source_url VARCHAR(500),
  downloaded_at TIMESTAMPTZ,
  active BOOLEAN DEFAULT TRUE,
  entry_count INT
);

CREATE TABLE sanction_entries (
  id UUID PRIMARY KEY,
  list_id UUID REFERENCES sanction_lists(id),
  name_full VARCHAR(200),
  name_first VARCHAR(100),
  name_last VARCHAR(100),
  name_aliases VARCHAR(500), -- Séparés par ;
  designation VARCHAR(200),
  dob DATE,
  nationality VARCHAR(3),
  entry_type VARCHAR(20), -- INDIVIDUAL, ENTITY
  list_source_id VARCHAR(50), -- ID original dans liste source
  active BOOLEAN DEFAULT TRUE
);

CREATE TABLE screening_results (
  id UUID PRIMARY KEY,
  customer_id UUID REFERENCES customers(id),
  list_id UUID REFERENCES sanction_lists(id),
  match_type VARCHAR(20), -- EXACT, FUZZY
  match_score DECIMAL(5, 2), -- 0-100 pour fuzzy
  matched_entry_id UUID REFERENCES sanction_entries(id),
  matched_at TIMESTAMPTZ,
  screened_by_user_id UUID REFERENCES users(id)
);
```

### 10.6 BC6 — Prudential

```sql
CREATE TABLE prudential_ratios (
  id UUID PRIMARY KEY,
  calculation_date DATE,
  ratio_type VARCHAR(50), -- SOLVABILITY, TIER1, CD_RATIO, CONCENTRATION
  value DECIMAL(5, 2), -- en %
  required_minimum DECIMAL(5, 2),
  status VARCHAR(20), -- CONFORME, NON_CONFORME
  calculated_at TIMESTAMPTZ,
  calculated_by VARCHAR(50) -- 'SYSTEM_BATCH'
);

CREATE TABLE risk_weighted_assets (
  id UUID PRIMARY KEY,
  calculation_date DATE,
  asset_category VARCHAR(50), -- CREDIT, MARKET, OPERATIONAL
  asset_value DECIMAL(15, 3),
  risk_weight DECIMAL(3, 1), -- En %
  rwa_value DECIMAL(15, 3), -- asset_value * risk_weight
  calculated_at TIMESTAMPTZ
);

CREATE TABLE regulatory_capital (
  id UUID PRIMARY KEY,
  calculation_date DATE,
  cet1_amount DECIMAL(15, 3), -- Capital Tier 1 commun
  at1_amount DECIMAL(15, 3), -- Capital Tier 1 additionnel
  tier1_total DECIMAL(15, 3), -- CET1 + AT1
  tier2_amount DECIMAL(15, 3),
  total_regulatory_capital DECIMAL(15, 3),
  adjustments_total DECIMAL(15, 3),
  calculated_at TIMESTAMPTZ
);

CREATE TABLE ratio_breach_alerts (
  id UUID PRIMARY KEY,
  ratio_type VARCHAR(50),
  current_value DECIMAL(5, 2),
  required_value DECIMAL(5, 2),
  alert_severity VARCHAR(20), -- P0, P1, P2
  breach_date DATE,
  remediation_action VARCHAR(500),
  resolved_at DATE
);
```

### 10.7 BC7 — Accounting

```sql
CREATE TABLE chart_of_accounts (
  id UUID PRIMARY KEY,
  account_code VARCHAR(10) UNIQUE NOT NULL,
  account_label VARCHAR(200),
  account_class SMALLINT, -- 1-6
  account_subclass VARCHAR(10),
  account_nature VARCHAR(1), -- 'D' = débit, 'C' = crédit
  is_balance_sheet BOOLEAN -- TRUE pour actif/passif, FALSE pour P&L
);

CREATE TABLE journal_entries (
  id UUID PRIMARY KEY,
  journal_code VARCHAR(20), -- CREANCES, DEPOTS, OPERATIONS_COURANTES, PROVISIONS
  entry_date DATE NOT NULL,
  entry_number BIGSERIAL, -- Numéro chronologique
  reference_document_id UUID, -- LoanId, AccountId, etc.
  total_debit DECIMAL(15, 3),
  total_credit DECIMAL(15, 3),
  created_at TIMESTAMPTZ,
  created_by UUID REFERENCES users(id),
  CONSTRAINT balanced_entry CHECK (total_debit = total_credit)
);

CREATE TABLE journal_lines (
  id UUID PRIMARY KEY,
  journal_entry_id UUID NOT NULL REFERENCES journal_entries(id),
  account_code VARCHAR(10),
  debit_amount DECIMAL(15, 3) DEFAULT 0,
  credit_amount DECIMAL(15, 3) DEFAULT 0,
  line_number SMALLINT,
  description VARCHAR(200)
);

CREATE TABLE accounting_periods (
  id UUID PRIMARY KEY,
  period_start_date DATE,
  period_end_date DATE,
  period_label VARCHAR(20), -- '2026-04'
  status VARCHAR(20), -- OPEN, CLOSED
  closed_at TIMESTAMPTZ,
  closed_by UUID REFERENCES users(id)
);

CREATE TABLE trial_balance (
  id UUID PRIMARY KEY,
  accounting_period_id UUID REFERENCES accounting_periods(id),
  account_code VARCHAR(10),
  balance_debit DECIMAL(15, 3) DEFAULT 0,
  balance_credit DECIMAL(15, 3) DEFAULT 0,
  calculated_at TIMESTAMPTZ
);
```

### 10.8 BC11 — Governance

```sql
CREATE TABLE audit_log_entries (
  id UUID PRIMARY KEY,
  timestamp TIMESTAMPTZ NOT NULL, -- Horodatage TSA
  user_id UUID REFERENCES users(id),
  action VARCHAR(50), -- ACCOUNT_OPENED, LOAN_APPROVED, KYC_VALIDATED
  resource_type VARCHAR(50), -- Account, Loan, Customer
  resource_id UUID,
  changes JSONB, -- {before: {...}, after: {...}}
  ip_address VARCHAR(45),
  session_id VARCHAR(100),
  signature_algorithm VARCHAR(20), -- ECDSA, RSA
  signature_value VARCHAR(1024), -- Signature hex
  hash_previous VARCHAR(128), -- SHA-256 chaînage
  hash_current VARCHAR(128), -- SHA-256 de cette entrée
  created_at TIMESTAMPTZ
);

-- Index pour queries rapides
CREATE INDEX idx_audit_log_timestamp ON audit_log_entries(timestamp DESC);
CREATE INDEX idx_audit_log_resource ON audit_log_entries(resource_type, resource_id);
CREATE INDEX idx_audit_log_user ON audit_log_entries(user_id);
```

### 10.9 BC12 — Identity

```sql
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username VARCHAR(50) UNIQUE NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL,
  first_name VARCHAR(100),
  last_name VARCHAR(100),
  password_hash VARCHAR(255), -- Bcrypt
  is_active BOOLEAN DEFAULT TRUE,
  created_at TIMESTAMPTZ,
  last_login TIMESTAMPTZ
);

CREATE TABLE roles (
  id UUID PRIMARY KEY,
  role_name VARCHAR(50) UNIQUE NOT NULL,
  description VARCHAR(500)
);

CREATE TABLE user_roles (
  user_id UUID NOT NULL REFERENCES users(id),
  role_id UUID NOT NULL REFERENCES roles(id),
  assigned_at TIMESTAMPTZ,
  assigned_by UUID REFERENCES users(id),
  PRIMARY KEY (user_id, role_id)
);

CREATE TABLE permissions (
  id UUID PRIMARY KEY,
  permission_name VARCHAR(100) UNIQUE NOT NULL,
  description VARCHAR(500),
  module VARCHAR(50) -- BC1, BC2, etc.
);

CREATE TABLE role_permissions (
  role_id UUID NOT NULL REFERENCES roles(id),
  permission_id UUID NOT NULL REFERENCES permissions(id),
  PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE sessions (
  id VARCHAR(100) PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id),
  created_at TIMESTAMPTZ,
  expires_at TIMESTAMPTZ,
  last_activity TIMESTAMPTZ,
  ip_address VARCHAR(45),
  user_agent VARCHAR(500),
  status VARCHAR(20) -- ACTIVE, REVOKED, EXPIRED
);

CREATE TABLE two_factor_auth (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL UNIQUE REFERENCES users(id),
  auth_type VARCHAR(20), -- SMS, EMAIL, AUTHENTICATOR_APP
  phone_number VARCHAR(20),
  email_backup VARCHAR(100),
  secret_key VARCHAR(32), -- Pour TOTP
  backup_codes VARCHAR(500), -- Codes de récupération séparés par ;
  enabled BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ,
  enabled_at TIMESTAMPTZ
);

CREATE TABLE user_consents (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  customer_id UUID REFERENCES customers(id),
  consent_type VARCHAR(50), -- KYC_PROCESSING, MARKETING, CREDIT_ANALYSIS
  granted BOOLEAN,
  granted_at TIMESTAMPTZ,
  ip_address VARCHAR(45),
  user_agent VARCHAR(500),
  consent_version VARCHAR(20),
  withdrawn_at TIMESTAMPTZ,
  withdrawn_reason VARCHAR(200)
);
```

### 10.10 BC8 — Reporting

```sql
CREATE TABLE regulatory_reports (
  id UUID PRIMARY KEY,
  report_type VARCHAR(50), -- PRUDENTIAL_MONTHLY, AML_QUARTERLY, FINANCIAL_ANNUAL
  reporting_period_start DATE,
  reporting_period_end DATE,
  generated_at TIMESTAMPTZ,
  generated_by UUID REFERENCES users(id),
  report_content BYTEA, -- PDF ou XML
  format_version VARCHAR(20), -- Version schéma BCT
  submission_status VARCHAR(20), -- DRAFT, SUBMITTED, ACCEPTED, REJECTED
  submitted_to_bct_at TIMESTAMPTZ,
  bct_receipt_number VARCHAR(50),
  bct_receipt_date DATE
);
```

### 10.11 BC9 — Payment

```sql
CREATE TABLE payment_orders (
  id UUID PRIMARY KEY,
  originating_account_id UUID NOT NULL REFERENCES accounts(id),
  beneficiary_account_id UUID REFERENCES accounts(id), -- NULL si virement externe
  beneficiary_rib VARCHAR(30),
  beneficiary_name VARCHAR(200),
  amount DECIMAL(15, 3),
  currency VARCHAR(3),
  payment_type VARCHAR(20), -- NATIONAL, SWIFT, IMMEDIATE
  purpose VARCHAR(200),
  status VARCHAR(20), -- PENDING, APPROVED, EXECUTED, REJECTED
  created_at TIMESTAMPTZ,
  created_by UUID REFERENCES users(id),
  executed_at TIMESTAMPTZ,
  execution_reason_if_rejected VARCHAR(500)
);

CREATE TABLE transfers (
  id UUID PRIMARY KEY,
  payment_order_id UUID REFERENCES payment_orders(id),
  from_account_id UUID REFERENCES accounts(id),
  to_account_id UUID REFERENCES accounts(id),
  amount DECIMAL(15, 3),
  execution_date DATE,
  clearing_reference VARCHAR(50),
  status VARCHAR(20) -- CLEARED, FAILED
);

CREATE TABLE swift_messages (
  id UUID PRIMARY KEY,
  payment_order_id UUID REFERENCES payment_orders(id),
  swift_content TEXT, -- Format ISO 20022 ou MT
  swift_reference VARCHAR(50),
  sent_at TIMESTAMPTZ,
  status VARCHAR(20) -- SENT, FAILED, CONFIRMED
);
```

---

## 11. Intégrations externes

### 11.1 SWIFT / ISO 20022 (P2)

**Objectif** : Virements internationaux conformes aux standards SWIFT.
**Format** : ISO 20022 (MX) ou MT (legacy).
**Priorité** : P1-P2 selon charge.
**Conformité** : GAFI R.16 — Traçabilité bénéficiaire [REF-66].

### 11.2 CTAF (Commission Tunisienne des Analyses Financières)

**Objectif** : Transmission DOS (Déclarations de Soupçon).
**Format** : XML structuré CTAF.
**Fréquence** : Immédiat ou J+1 (art. 125 Loi 2015-26) [REF-28].
**Mode** : API CTAF (si dispo) ou SFTP/email sécurisé.

### 11.3 Listes sanctions (ONU, EU, OFAC, nationales)

**Objectif** : Téléchargement + screening quotidien.
**Sources** :
- UN OFAC: https://www.un.org/ (ONU SDN List)
- EU: https://ec.europa.eu/info/business-economy-euro/banking-and-finance/...
- OFAC: https://home.treasury.gov/policy-issues/financial-sanctions-and-embargoes (SDN, Consolidated)
- BCT : Liste nationale maintenue par régulateur

**Fréquence** : Quotidienne (06:00 UTC).
**Format** : XML ou CSV, parsing configurable.

### 11.4 BCT Reporting API (P2)

**Objectif** : Transmission automatisée états prudentiels, rapports réglementaires.
**Format** : XML conforme schéma BCT officiel.
**Sécurité** : Certificate mutuelle TLS.
**Mode** : API SFTP ou REST sécurisé.

### 11.5 Banques correspondantes (P2)

**Objectif** : Échanges SWIFT pour virements internationaux.
**Format** : MT103, MT202.
**Sécurité** : SWIFTNet PKI.

---

## 12. Contraintes et hypothèses

### 12.1 Contraintes de conception

- **Stack imposée** : Rust/Actix-web (backend) + Astro/Svelte (frontend) + PostgreSQL
- **Disciplines** : SOLID + DDD + BDD + TDD + Hexagonal + YAGNI + DRY
- **Licence** : AGPL-3.0 (copyleft fort, code ouvert)
- **Langues** : AR (RTL), FR, EN
- **Monétaire** : TND avec 3 décimales pour montants
- **Hébergement** : Souverain (Tunisie) — conformité INPDP [REF-54]
- **Sécurité** : HSM pour signatures critiques, LUKS pour chiffrement, audit trail immuable
- **Calendrier** : MVP 8-12 mois en duo, 36 mois solo side-project

### 12.2 Hypothèses

- **KYC données correctes** : Données saisies supposées valides (validation métier en ligne, pas de vérification biométrique)
- **Connexions réseau stables** : API externes (CTAF, SWIFT) supposées disponibles 99.9%
- **Réglementation stable** : Circulaires BCT supposées stables pendant MVP. Nouvelles circulaires = modules additifs
- **Pas de support rétro-changement** : Les mouvements historiques ne sont jamais modifiés (immutabilité absolue)
- **Vol de crypto-monnaies hors scope** : BANKO n'héberge pas de crypto, seulement TND

---

## 13. Critères de succès MVP

| Critère | Mesure | Seuil de succès |
|---|---|---|
| **Conformité P0** | Couverture exigences BCT P0 vs référentiel | 100% implémentées |
| **Couverture tests** | Code coverage (Tarpaulin) domain | ≥ 100% domain, ≥ 80% app |
| **Scénarios BDD** | Count exigences avec scénarios Gherkin | ≥ 80 (MVP P0) |
| **Performance** | P95 latence API < 200ms | ✓ |
| **Sécurité** | Vulnérabilités critiques non mitigées | 0 |
| **Disponibilité** | Uptime 30 jours | ≥ 99.5% |
| **Audit trail** | Complétude traçabilité | 100% opérations tracées |
| **i18n** | Couverture UI langues | FR + AR complètes |
| **Documentation** | BDD vivante | Tous flux P0 documentés |
| **Piste légale** | Traçabilité → références légales | Tous les modules référencés [REF-XX] |

---

## Fin du document PRD

**Prochaine étape** : Phase TOGAF C (Systems Architecture)
**Consommé par** : Architecte (design hexagonal, ports & adapters, DDD)
**Validé par** : PM + Experts métier BCT/CTAF/INPDP

---

**Révision** : 2.0.0
**Date** : 4 avril 2026
**Statut** : Approuvé pour développement Étape 3
