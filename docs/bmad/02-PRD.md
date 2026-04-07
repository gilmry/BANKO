# PRD — BANKO v4.0
## Méthode Maury — Phase TOGAF B-C (Business + SI)

**Version** : 4.0.0 — 7 avril 2026
**Auteur** : GILMRY / Projet BANKO
**Horizon** : 12-16 mois (avril 2026 → août 2027)
**Profil** : Solo-dev side-project, 8h/sem moyenne, coefficients IA appliqués

---

## 1. Résumé exécutif

BANKO v4.0 est un système bancaire open source (AGPL-3.0) conçu pour atteindre **parité fonctionnelle Temenos Transact** (550-700+ endpoints, 17 catégories) tout en implémentant à la perfection la conformité réglementaire tunisienne (BCT, CTAF, INPDP, BVMT) et internationale (Bâle III, GAFI R.16, IFRS 9, ISO 27001:2022, PCI DSS v4.0.1).

**Objectif stratégique** : Libérer les banques tunisiennes de la dépendance Temenos (100-500 k€/an/licence) en fournissant une alternative souveraine, auditable, gratuite, avec **22 bounded contexts** couvrant 85-90% des capacités Temenos.

**Cible MVP** : Petites/moyennes banques tunisiennes, établissements financiers, banques islamiques (Loi 2016-33), startups fintech, superviseurs (BCT, CTAF, INPDP).

**Promesse clé** : Une action illégale en droit bancaire tunisien ne compile tout simplement pas. Chaque opération est traçable vers un texte légal (95 références BCT/INPDP/PCI/ISO mappées).

---

## 2. Objectifs produit (mesurables)

| Objectif | Métrique | Cible v4.0 | Jalon |
|---|---|---|---|
| **Couverture endpoints Temenos** | # endpoints implémentés / 550-700 | 450+ (80%) | Jalon 3 |
| **Conformité BCT P0** | % exigences critiques implémentées | 100% | Jalon 0 |
| **Tests domain (coverage)** | Tarpaulin cobertura | 95%+ | Chaque jalon |
| **Scénarios BDD** | Count gherkin (Cucumber) | ≥400 | Jalon 4 |
| **Performance API P99** | Latence interne | <5ms | Chaque jalon |
| **Performance API E2E** | Latence incluant persistance | <200ms | Chaque jalon |
| **Disponibilité** | Uptime SLA | 99.9% | Production |
| **Piste d'audit** | Couverture opérations | 100% immutable | Jalon 0 |
| **Sécurité critiques** | Vulnérabilités non mitigées | 0 | Chaque release |
| **Accessibilité i18n** | Langues complètement supportées | AR (RTL) + FR + EN | Jalon 1 |
| **ISO 27001:2022** | Contrôles Annexe A mappés | 93/93 (100%) | Avant certification |
| **PCI DSS v4.0.1** | Exigences obligatoires | 100% | Avant SAQ |
| **Loi données 2025** | Conformité RGPD-like | 100% avant 11-07-2026 | Avant deadline |
| **GAFI R.16 travel rule** | Effectivité données originator/beneficiary | 100% | Avant nov 2026 |
| **IFRS 9 ECL** | Provisionnement stage 1/2/3 | Live operational | Jalon 2 |
| **Déploiements production** | Banques tunisiennes live | ≥2 | Avant déc 2026 |

---

## 3. Périmètre par Jalon

### 3.1 Jalon 0 (Fondations — Semaines 1-6)

**Objectif** : Socle technique sécurisé, identité, compliance obligatoire, audit trail immutable.

**Contextes P0 (blocants)** :
- **BC1-Customer** : KYC/CDD/EDD complet (Circ. 2025-17)
- **BC2-Account** : Comptes courant, épargne, DAT
- **BC7-Accounting** : Journal comptable NCT, balance générale
- **BC11-Governance** : Audit trail cryptographique, 3LoD, contrôle interne
- **BC12-Identity** : Authentification FIDO2/WebAuthn, 2FA, RBAC, sessions sécurisées
- **BC13-Compliance** (NOUVEAU) : SMSI ISO 27001:2022 (93 contrôles Annexe A), PCI DSS v4.0.1 token, loi données 2025 (DPO, DPIA, consentement)

**Capacités P0** :
- C1: Gestion clients KYC/CDD/EDD complet
- C2: Gestion comptes (courant, épargne, DAT)
- C7: Comptabilité NCT (journal, GL, balance, provision)
- C12: Authentification sécurisée 2FA/MFA
- C26: e-KYC biométrique (Circ. 2025-06)
- C23: Loi données 2025 (DPO dashboard, DPIA, consentement, portabilité, effacement)

**FRs** : ~60 FRs (FR-001 à FR-060)

**Durée estimée** : 48h (÷3 IA) = 16h solo-dev = 2 semaines @ 8h/sem

---

### 3.2 Jalon 1 (Core Banking essentiel — Semaines 7-14)

**Objectif** : Crédit, prudentiel, AML, sanctions, paiement, change, arrangements.

**Contextes P1 (importants)** :
- **BC3-Credit** : Octroi, suivi, classification (0-4), provisionnement
- **BC4-AML** : Surveillance transactionnelle, alertes, investigations, DOS workflow, gel avoirs
- **BC5-Sanctions** : Filtrage ONU/UE/OFAC/nationales, screening, matches
- **BC6-Prudential** : Ratios solvabilité (10%), Tier 1 (7%), C/D (120%), concentration (25%), RWA
- **BC8-Reporting** : États BCT prudentiels, AML, financiers
- **BC9-Payment** : Virements nationaux, compensation, ISO 20022 ready
- **BC10-ForeignExchange** : FX spot/forward, position, taux, conformité Loi 76-18
- **BC14-Arrangement** (NOUVEAU) : Contrats, conditions, limites, produits associés, bundles, simulation, négociation

**Capacités P1** :
- C3: Gestion crédits (octroi, classification, provision)
- C4: Calcul prudentiel temps réel
- C5: AML surveillance transactionnelle
- C6: Sanctions filtrage
- C9: Reporting BCT
- C11: Virements nationaux simples
- C18: Operations change de base (Loi 76-18)
- Arrangement management (contrats, limites, produits)

**FRs** : ~100 FRs (FR-061 à FR-160)

**Durée estimée** : 120h (÷3 IA) = 40h solo-dev = 5 semaines @ 8h/sem

---

### 3.3 Jalon 2 (Compliance avancé + Trade Finance — Semaines 15-20)

**Objectif** : Collateral, trade finance, cash management, IFRS 9 ECL complète, goAML, travel rule.

**Contextes P2** :
- **BC15-TradeFinance** : L/C (création, dénouement, documentaire), garanties bancaires, workflows UCP 600
- **BC16-CashManagement** : Sweeps, pooling, liquidité, FX forwards, trésorerie en temps réel
- **BC17-IslamicBanking** : Murabaha, ijara, waqf, wakala, musharaka, sukuk, Sharia board compliance
- **BC19-ReferenceData** : Master data management, devises, pays, codes, taux BCT, jours fériés
- **BC22-Compliance** (extension) : goAML intégrée (CTAF déclarations), travel rule (originator/beneficiary), effectiveness metrics, TuniCheque

**Capacités P2** :
- Trade finance (L/C, bank guarantees)
- Cash management (sweeps, pooling)
- Islamic banking produits Sharia
- IFRS 9 ECL stage 1/2/3
- goAML intégrée
- Travel rule data
- Reference data master

**FRs** : ~90 FRs (FR-161 à FR-250)

**Durée estimée** : 110h (÷3 IA) = 37h solo-dev = 5 semaines @ 8h/sem

---

### 3.4 Jalon 3 (Analytics + Securities + Insurance — Semaines 21-26)

**Objectif** : Data lake, securities, insurance, parité Temenos 80%+.

**Contextes P3** :
- **BC18-DataHub** : ODS (Operational Data Store), ADS (Analytical Data Store), MDM (Master Data Management), data quality, real-time analytics
- **BC20-Securities** : Valeurs mobilières, portefeuille titres, dépositaire, custody, ordres bourse, BVMT conformité
- **BC21-Insurance** : Assurances liées (crédit, décès, risque), polices, sinistres, taux

**Capacités P3** :
- Data lake operational + analytical
- Securities portfolio management
- Insurance integration
- Advanced analytics dashboards

**FRs** : ~60 FRs (FR-251 à FR-310)

**Durée estimée** : 80h (÷3 IA) = 27h solo-dev = 3.4 semaines @ 8h/sem

---

### 3.5 Jalon 4 (Maturité + Microservices — Semaines 27-32)

**Objectif** : Microservices production-ready, open banking PSD3-ready, certification, hardening.

**Capacités P4** :
- Microservices orchestration (Kubernetes-ready)
- Open Banking APIs (PSD3)
- Certification ISO 27001
- SAQ PCI DSS validation
- Performance optimization (P99 <5ms)

**FRs** : ~40 FRs (FR-311 à FR-350+)

**Durée estimée** : 60h (÷3 IA) = 20h solo-dev = 2.5 semaines @ 8h/sem

---

### 3.6 Hors scope v4.0
- Dérivés complexes (swaps, options)
- Gestion d'actifs (hedge funds, fonds mutuels)
- Blockchain, stablecoins, CBDC
- Courtage avancé (actions, options)
- Intégrations fintech au-delà PSD3

---

## 4. Functional Requirements (FRs) — 250+ Exigences

### 4.1 BC1 — Customer (Gestion clients / KYC / PEP / EDD)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-001 | KYC personne physique complète | Création fiche KYC conforme Annexe 1 Circ. 2025-17 (CIN, identité, profession, revenu, adresse, source fonds) | Circ. 2025-17 art. 5-7 | Party/Individual | P0 | J0 |
| FR-002 | KYC personne morale + bénéficiaires | Création SARL/SACS/EIRL avec bénéficiaires ≥25% capital, KYC individuelle chaque bénéficiaire | Circ. 2025-17 art. 8-12 | Party/Organization | P0 | J0 |
| FR-003 | Validation KYC par Compliance | Workflow Compliance : VALIDÉE / REJETÉE avec motif, verrouillage post-validation | Circ. 2025-17 art. 15 | Party/Validation | P0 | J0 |
| FR-004 | PEP detection (auto + manual) | Détection automatique sur liste interne + recherche manuelle, flagging EDD_REQUIRED | Circ. 2025-17 art. 13 | Party/PEP | P0 | J0 |
| FR-005 | EDD renforcée (Enhanced Due Diligence) | Questionnaire EDD, documents justificatifs, timeline 10 jours ouvrables, rejet si expire | Circ. 2025-17 art. 14 | Party/EDD | P0 | J0 |
| FR-006 | Risk scoring client (0-100) | Score basé profil (secteur, pays, montants), antécédents AML → GREEN/ORANGE/RED | Circ. 2025-17 | Party/Risk | P1 | J0 |
| FR-007 | Données sensibles INPDP (chiffrement) | Chiffrement AES-256-GCM des donnees PII (CIN, passeport, adresse), accès audit trail | Loi données 2025 art. 2-4 | Party/Privacy | P0 | J0 |
| FR-008 | Consentement INPDP explicite | Checkbox consentement obligatoire avant KYC, traçabilité consentement, droit retrait | Loi données 2025 art. 7 | Party/Consent | P0 | J0 |
| FR-009 | Droit portabilité données | Export données client JSON/CSV (identité, comptes, mouvements) sur demande | Loi données 2025 art. 20 | Party/DataPortability | P1 | J0 |
| FR-010 | Droit effacement ("oubli") | Suppression données non-critiques après clôture compte (90j délai, légal : 10 ans conservation) | Loi données 2025 art. 21 | Party/Erasure | P1 | J0 |
| FR-011 | e-KYC biométrique (Circ. 2025-06) | Enrôlement électronique empreintes, visage, signature électronique FIDO2/WebAuthn | Circ. 2025-06 | Party/eKYC | P0 | J0 |
| FR-012 | Profiling client (income, assets, behaviour) | Profil complet : revenus, patrimoine, comportement transactionnel, expositions | Circ. 2025-17 | Party/Profile | P1 | J0 |
| FR-013 | Statut client (actif/suspendu/clôturé) | State machine : NOUVEAU→VALIDÉ→ACTIF→SUSPENDU/CLÔTURÉ, transitions auditées | Circ. 2025-17 | Party/Status | P0 | J0 |
| FR-014 | Audit trail client (chaque modification) | Hash chain immutable : création, validations, modifications KYC, sanctions, AML | Circ. 2006-19 | Party/Audit | P0 | J0 |
| FR-015 | Vérification OFAC + listes nationales | Filtrage automatique création client vs. OFAC, UE, listes BCT nationales, blocage si match | Circ. 2025-17 | Party/Sanctions | P0 | J1 |

**Total BC1 : 15 FRs**

---

### 4.2 BC2 — Account (Gestion comptes / Soldes / Mouvements)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-016 | Ouverture compte courant | RIB généré, devise TND, solde 0, statut OUVERT, audit trail | Circ. 2025-17 | Holdings/Account | P0 | J0 |
| FR-017 | Ouverture compte épargne | Taux d'intérêt applicable, calcul mensuel composé | Circ. 2025-17 | Holdings/Savings | P0 | J0 |
| FR-018 | Ouverture DAT (Dépôt À Terme) | Montant ≥1000 TND, durée 6/12/24 mois, taux fixe immutable, intérêts capitalisés | Circ. 2025-17 | Holdings/TermDeposit | P0 | J0 |
| FR-019 | Consultation soldes/mouvements | Requête temps réel solde, historique 90 jours, filtrage période | Circ. 2025-17 | Holdings/Statement | P0 | J0 |
| FR-020 | Calcul intérêts (épargne + DAT) | Formule : Principal × Taux × Jours / 365, capitalisation mensuelle, audit trail | NCT art. 32 | Holdings/Interest | P0 | J0 |
| FR-021 | Restitution DAT à échéance | Automatique : principal + intérêts → compte courant, notification client | Circ. 2025-17 | Holdings/Maturity | P0 | J0 |
| FR-022 | Clôture de compte | Solde résiduel remboursé, compte passe CLÔTURÉ, données conservées 10 ans | Circ. 2025-17 | Holdings/Closure | P0 | J0 |
| FR-023 | Suspension de compte | Transition SUSPENDU (mouvements bloqués sauf virements entrants), cause auditée | Circ. 2025-17 | Holdings/Suspension | P1 | J1 |
| FR-024 | Recherche compte (RIB, client) | Requête par RIB ou client_id, affichage complet | Circ. 2025-17 | Holdings/Search | P0 | J0 |
| FR-025 | Conciliation solde GL ↔ mouvements | Vérification quotidienne : somme mouvements = solde GL, alertes si divergence | Circ. 91-24 | Holdings/Reconciliation | P1 | J1 |
| FR-026 | Compte multi-devise (futur) | Support USD/EUR/GBP via FX conversions réglementaires | Circ. 2025-17 | Holdings/MultiCurrency | P2 | J4 |
| FR-027 | Limite débit (overdraft) | Limite de découvert configurable par client, alertes dépassement, taux intérêt supp. | Circ. 2025-17 | Holdings/Overdraft | P1 | J1 |
| FR-028 | Blocages/déblocages (Gel des avoirs) | Gel irrévocable sans autorisation CTAF, déblocage après levée, audit | Circ. 2025-17 | Holdings/Freeze | P0 | J1 |

**Total BC2 : 13 FRs**

---

### 4.3 BC3 — Credit (Crédits / Octroi / Classification / Provisionnement)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-029 | Demande crédit (création + analyse) | Création dossier : montant, durée, objet, garanties, passage EN_INSTRUCTION | Circ. 91-24 | Credit/Application | P0 | J1 |
| FR-030 | Analyse risque (PD, LGD, EAD) | Évaluation probabilité défaut, loss given default, exposure at default → classification proposée | Bâle III | Credit/RiskAnalysis | P1 | J1 |
| FR-031 | Classification créance (classes 0-4) | Classe 0 (courant) → 4 (compromis), provisionnement % obligatoire [REF-14] | Circ. 91-24 | Credit/Classification | P0 | J1 |
| FR-032 | Comité crédit (approbation multi-niveaux) | Vote 3 membres minimum, décision APPROUVÉE/REJETÉE, quorum | Circ. 91-24 | Credit/Committee | P0 | J1 |
| FR-033 | Déblocage crédit (transfusion montant) | Transfert montant → compte client, enregistrement créance classe 0, génération échéancier | Circ. 91-24 | Credit/Disbursement | P0 | J1 |
| FR-034 | Échéancier remboursement (calcul) | Mensualités fixes ou variables, amortissement, intérêts calculés, solde restant dû | NCT art. 32 | Credit/Schedule | P0 | J1 |
| FR-035 | Paiement mensualité (enregistrement) | Débit compte client, crédit compte crédit, intérêts, principal, frais | Circ. 91-24 | Credit/Payment | P0 | J1 |
| FR-036 | Défaut paiement (moratoire + pénalité) | Retard >30j → classe 2, >60j → classe 3, >90j → classe 4, frais pénalité | Circ. 91-24 | Credit/Default | P0 | J1 |
| FR-037 | Provision créance (% par classe) | Classe 0=0%, 1=20%, 2=50%, 3=75%, 4=100%, débit P&L | Circ. 91-24 | Credit/Provisioning | P0 | J1 |
| FR-038 | IFRS 9 ECL (stage 1/2/3) | Stage 1 : 12m loss, Stage 2/3 : durée vie, calcul PD/LGD/EAD probabiliste | IFRS 9 | Credit/ECL | P1 | J2 |
| FR-039 | Concentration limite (25% FPN) | Alerte et blocage si risque/client >25% fonds propres nets | Circ. 91-24 | Credit/Concentration | P0 | J1 |
| FR-040 | Restructuration crédit (nouveau plan) | Rééchelonnement/réduction montant/taux, reclassification, nouveau contrat | Circ. 91-24 | Credit/Restructuring | P1 | J1 |
| FR-041 | Remboursement anticipé | Paiement total/partiel avant terme, calcul intérêts jusqu'à remboursement | Circ. 91-24 | Credit/Prepayment | P1 | J1 |
| FR-042 | Créances douteuses (recovery) | Dossiers classe 3/4, plans récupération, assignations, saisies | Circ. 91-24 | Credit/Recovery | P1 | J2 |

**Total BC3 : 14 FRs**

---

### 4.4 BC4 — AML (Anti-blanchiment / Surveillance / DOS / Gel)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-043 | Scénarios surveillance transactionnelle (P0) | Montant >200k TND, fréquence anormale, pays risque, secteur risque → alerte AML | Circ. 2025-17 | Risk/AML | P0 | J1 |
| FR-044 | Investigation AML (workflow) | Alerte créée, assignée analyste AML, enquête documentée, CLOSE/ESCALATE/SAR | Circ. 2025-17 | Risk/Investigation | P0 | J1 |
| FR-045 | Déclaration de soupçon (DOS) → goAML | Signalement électronique CTAF si transaction suspecte, numéro suivi, confirmation | Circ. 2025-17 art. 125 | Risk/SAR | P0 | J1 |
| FR-046 | Gel des avoirs (Asset Freeze) | Blocage irrévocable sur ordre CTAF, traçabilité ordre, déblocage post-levée | Circ. 2025-17 | Risk/Freeze | P0 | J1 |
| FR-047 | Travel rule (originator/beneficiary) | Copie données originator (CIN, nom, compte) → bénéficiaire pour transferts >250k TND | GAFI R.16 | Risk/TravelRule | P0 | J2 |
| FR-048 | Sanctions screening (création client) | Filtrage automatique OFAC/UE/nationales à chaque KYC, blocage si match | Circ. 2025-17 | Risk/Sanctions | P0 | J1 |
| FR-049 | Scénarios surveillance avancés (P1) | Structuring (multiplicité petits montants), round-tripping, splits, activités sans justif | Circ. 2025-17 | Risk/AdvancedScenarios | P1 | J1 |
| FR-050 | CTR (Déclaration Transactionnel Restreint) | Signalement automatique montants >200k TND pour fichier CTAF hebdomadaire | Circ. 2025-17 | Risk/CTR | P0 | J1 |
| FR-051 | Blocage/déblocage manuel (Sonia) | Interface override manuel, approbation CRO, trace d'audit | Circ. 2025-17 | Risk/ManualBlock | P1 | J1 |
| FR-052 | Dashboard AML (KPIs, alertes) | Nombre alertes/jour, SAR volume, gel avoirs, clients de risque, trends | Circ. 2025-17 | Risk/Dashboard | P1 | J2 |
| FR-053 | Statistiques AML (effectiveness) | Nombre investigations, SAR soumis, montants gélés, taux clôture, impact GAFI eval | GAFI R.16 | Risk/Statistics | P0 | J2 |

**Total BC4 : 11 FRs**

---

### 4.5 BC5 — Sanctions (Filtrage listes / Screening / Matches)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-054 | Listes sanctions (OFAC/UE/nationales) | Téléchargement quotidien listes, maintenance base locale, versioning | Circ. 2025-17 | Risk/SanctionList | P0 | J1 |
| FR-055 | Screening KYC vs. sanctions | Matching fuzzy (nom, date naissance, adresse) à création client, score confidence | Circ. 2025-17 | Risk/Screening | P0 | J1 |
| FR-056 | Gestion matches (résolution) | Match détecté → CONFIRMED/FALSE_POSITIVE/ESCALATE, documenter, audit | Circ. 2025-17 | Risk/MatchResolution | P0 | J1 |
| FR-057 | Blocage automatique (match confirmed) | Compte bloqué si match confirmed, déblocage après levée sanctions | Circ. 2025-17 | Risk/AutoBlock | P0 | J1 |
| FR-058 | Rapports sanctions (BCT/CTAF) | Export liste clients gélés, dates déblocage, motifs, transmission BCT trimestrielle | Circ. 2025-17 | Risk/SanctionReport | P1 | J2 |

**Total BC5 : 5 FRs**

---

### 4.6 BC6 — Prudential (Ratios / Capital / RWA / Solvabilité)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-059 | Calcul ratio solvabilité (FPN/RWA ≥10%) | Fonds propres nets / actifs pondérés risque, quotidien, alerte si <12% | Circ. 2025-08 | Risk/Solvency | P0 | J1 |
| FR-060 | Calcul Tier 1 (CET1+AT1 ≥7%) | Capital core tier 1 + Additional tier 1, quotidien, alerte si <8% | Circ. 2025-08 | Risk/Tier1 | P0 | J1 |
| FR-061 | Ratio C/D (Crédits/Dépôts ≤120%) | Somme crédits / somme dépôts, quotidien, alerte si >110% | Circ. 2025-08 | Risk/CD | P0 | J1 |
| FR-062 | Concentration par bénéficiaire (≤25% FPN) | Somme risques/client ≤ 25% fonds propres, quotidien, alerte si >20% | Circ. 91-24 | Risk/Concentration | P0 | J1 |
| FR-063 | RWA par asset class (crédits, opérations de marché) | Pondération 0% (contenu) → 150% (acquis), calculé quotidien | Bâle III | Risk/RWA | P0 | J1 |
| FR-064 | Breach alerts (auto-notification) | SMS/email Rachid si ratio franchit seuil réglementaire | Circ. 2025-08 | Risk/Alert | P1 | J1 |
| FR-065 | Rapport prudentiel quotidien | PDF/Excel synthèse : FPN, RWA, tous ratios, trends 30j | Circ. 2025-08 | Risk/Report | P1 | J2 |

**Total BC6 : 7 FRs**

---

### 4.7 BC7 — Accounting (Comptabilité NCT / Journal / GL / Balance)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-066 | Plan comptable NCT | 7 classes comptables, comptes standard NCT (1xxx dépôts, 2xxx crédits, 3xxx dettes, 4xxx produits, 5xxx charges, 6xxx résultat) | NCT art. 1-5 | Accounting/CoA | P0 | J0 |
| FR-067 | Écriture comptable double (débit/crédit) | Montant D=C, devise, journaux séparés (OD = opérations diverses, OC = opérations crédit) | NCT art. 10 | Accounting/JournalEntry | P0 | J0 |
| FR-068 | Journal par opération (OD, OC, OI, OP) | OD=opérations diverses, OC=crédit, OI=intérêts, OP=provisions, traçabilité montant | NCT art. 12 | Accounting/Journal | P0 | J0 |
| FR-069 | Comptabilisation automatique (hooks) | Chaque mouvement compte → écriture JE automatique (débit 1xxx, crédit 2111 par ex.) | NCT art. 12 | Accounting/AutoPosting | P0 | J0 |
| FR-070 | Balance générale (extraction GL) | Somme débits/crédits par compte, soldes période, matching T+0 | NCT art. 18 | Accounting/TrialBalance | P0 | J0 |
| FR-071 | Closings périodiques (fin mois) | État "OPEN" → "POSTING" → "CLOSED", pas modifications post-closed, audit trail | NCT art. 25 | Accounting/Closing | P0 | J0 |
| FR-072 | Provisionnement créances (journalisation) | Débit 5900 (charge provision) / Crédit 1900 (contre-provision) pour chaque classe 0-4 | Circ. 91-24 | Accounting/Provisioning | P0 | J0 |
| FR-073 | Intérêts comptabilisés (journalisation) | Débit 1xxx / Crédit 4010 (produits intérêts), mensuel, calcul composé | NCT art. 32 | Accounting/Interest | P0 | J0 |
| FR-074 | État de synthèse (bilan simplifié) | Actif (1xxx+2xxx) = Passif (3xxx) + Équité (9xxx), vérification quotidienne | NCT art. 35 | Accounting/BalanceSheet | P1 | J0 |
| FR-075 | Exigences IFRS 9 pré-ECL | Double-booking : NCT actuel + colonnes IFRS 9 pour stage 1/2/3 (v4.0) | IFRS 9 | Accounting/IFRS9 | P1 | J2 |
| FR-076 | Audit trail écritures (immuabilité) | Hash chain : date, montant, comptes, user, journaux, traces de modification | Circ. 2006-19 | Accounting/AuditTrail | P0 | J0 |
| FR-077 | Requête GL (filtrage période/compte) | Export CSV : comptes, débits, crédits, soldes période, drill-down | NCT art. 18 | Accounting/Query | P0 | J0 |

**Total BC7 : 12 FRs**

---

### 4.8 BC8 — Reporting (États BCT / Prudentiels / Financiers)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-078 | État prudentiel (Circ. 2025-08) | Ratios solvabilité, Tier 1, C/D, concentration, PDF/Excel, mensuel → BCT | Circ. 2025-08 | Analytics/Prudential | P1 | J1 |
| FR-079 | État AML (Circ. 2025-17) | Nombre alertes, SAR volume, montants gélés, clients suivi EDD, SAR soumis CTAF | Circ. 2025-17 | Analytics/AML | P1 | J1 |
| FR-080 | Bilan NCT (états financiers) | Synthèse actif/passif/capital, conforme NCT, attestation, audit trail | NCT art. 35 | Analytics/Balance | P1 | J1 |
| FR-081 | Compte résultat (P&L) | Produits - charges = résultat net, mensuel, tendances | NCT art. 38 | Analytics/PL | P1 | J1 |
| FR-082 | Rapport crédit (classification + provisions) | Créances par classe, provisions, ratios non-performing, provisions couvrant % | Circ. 91-24 | Analytics/Credit | P1 | J1 |
| FR-083 | Tableau de bord rapports (generation auto) | Paramétrage rapports, planification hebdo/mensuel, export email stakeholders | Circ. 2025-08 | Analytics/Scheduler | P1 | J2 |

**Total BC8 : 6 FRs**

---

### 4.9 BC9 — Payment (Virements / Compensation / ISO 20022)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-084 | Virement national intra-banque | Transfert compte débordement → crédit T+0, montant débité/crédité, frais transaction | Circ. 2025-17 | Payment/Domestic | P0 | J1 |
| FR-085 | Virement national inter-banque | Initiation XML ISO 20022, transmission clearing national, compensation T+1 | Circ. 2025-17 | Payment/Interbank | P1 | J1 |
| FR-086 | Virement SWIFT (international) | Messagerie SWIFT MT103, conforme UCP/SWIFT, débouchés devises | ISO 20022 | Payment/SWIFT | P2 | J2 |
| FR-087 | Mandat de paiement (ordre permanent) | Virement récurrent (loyer, salaire), fréquence, montant, dates, édition/suppression | Circ. 2025-17 | Payment/Standing | P1 | J1 |
| FR-088 | Frais transaction | Calcul tarif par montant/type, débité compte client, revenus comptabilisés | NCT art. 32 | Payment/Fees | P0 | J1 |
| FR-089 | Compensation (clearing) | Agrégation virements du jour, transmission clearing, règlement brut T+1, réconciliation | Circ. 2025-17 | Payment/Clearing | P1 | J2 |
| FR-090 | Statuts virement (tracking) | INITIÉ → VALIDÉ → TRANSMIS → COMPENSÉ → RÉGLÉ, audit trail | Circ. 2025-17 | Payment/Status | P0 | J1 |

**Total BC9 : 7 FRs**

---

### 4.10 BC10 — ForeignExchange (FX / Spot / Forward / Position / Loi 76-18)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-091 | Opération FX spot (achat/vente) | Conversion devises, taux BCP (quotidien), débit/crédit comptes montants, frais change | Loi 76-18 | FX/Spot | P1 | J1 |
| FR-092 | Taux de change (import BCP) | Mise à jour quotidienne (avant 16h30), utilisation for all FX ops du jour | Loi 76-18 | FX/Rate | P1 | J1 |
| FR-093 | Opération FX forward (future) | Vente/achat forward à date fixe, taux forward calculé (spot + swap points) | Loi 76-18 | FX/Forward | P1 | J2 |
| FR-094 | Position FX (expo devises) | Somme achats/ventes devises, calcul expo court/long, limite RWA par devise | Loi 76-18 | FX/Position | P1 | J2 |
| FR-095 | Conformité devise TND (limitation) | Montants USD/EUR plafonnés, autorisation BCP pour gros montants (>500k) | Loi 76-18 | FX/Compliance | P1 | J1 |
| FR-096 | Confirmation FX (client) | Email/SMS confirmation opération FX, taux appliqué, frais, montants | Loi 76-18 | FX/Confirmation | P1 | J1 |

**Total BC10 : 6 FRs**

---

### 4.11 BC11 — Governance (Audit Trail / 3LoD / Contrôle Interne / Comités)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-097 | Audit trail immutable (hash chain) | Chaque opération : timestamp, user, action, ressource, avant/après, SHA256 chainé | Circ. 2006-19 | Governance/AuditTrail | P0 | J0 |
| FR-098 | 3 Lignes de Défense (3LoD) | 1LoD : opérateurs (Karim), 2LoD : compliance/risque (Sonia/Rachid), 3LoD : audit interne (inspecteur BCT) | BCT governance | Governance/3LoD | P0 | J0 |
| FR-099 | Contrôles internes (4-eyes, segregation) | Approbations multi-niveaux : comité crédit, validation KYC, comité prudentiel | BCT governance | Governance/Controls | P0 | J0 |
| FR-100 | Comités (crédit, audit, risque) | Comité crédit (3 membres), comité audit (2 indépendants), comité risque (CRO+Risk Manager) | BCT governance | Governance/Committee | P0 | J0 |
| FR-101 | Dashboard conformité (DPO, auditeurs) | KPIs : AML alerts, SAR, gels, ratios prudentiels, clients risque, conformité données | Circ. 2025-17 | Governance/Dashboard | P1 | J1 |
| FR-102 | Rapports audit (extraction périodique) | Export audit trail JSON, requêtes avancées (filtrer par user/action/période), drill-down | Circ. 2006-19 | Governance/Reports | P0 | J0 |

**Total BC11 : 6 FRs**

---

### 4.12 BC12 — Identity (Authentification / RBAC / Sessions / 2FA)

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-103 | Authentification 2FA (OTP SMS+FIDO2) | Mot de passe + OTP SMS 6d + biométrie FIDO2/WebAuthn, fallback OTP TOTP | Circ. 2025-06 | Identity/Auth2FA | P0 | J0 |
| FR-104 | Gestion rôles (RBAC) | Rôles : OPERATEUR, COMPLIANCE, CRO, DPO, ADMIN, permissions granulaires | BCT governance | Identity/RBAC | P0 | J0 |
| FR-105 | Sessions sécurisées (timeout) | Timeout 30 min inactivité, révocation manuelle, token JWT signé HSM | PCI DSS v4.0.1 | Identity/Session | P0 | J0 |
| FR-106 | MFA pour opérations critiques | Confirmations MFA : virements >100k, déblocages crédit, validations KYC | PCI DSS v4.0.1 | Identity/CriticalMFA | P0 | J0 |
| FR-107 | Audit login/logout | Trace : user, timestamp, IP, succès/échec auth, raison rejets | Circ. 2006-19 | Identity/LoginAudit | P0 | J0 |
| FR-108 | Gestion mots de passe (complexité) | Min 12 chars, majuscule+minuscule+chiffre+spécial, pas noms clients, expiration 90j | PCI DSS v4.0.1 | Identity/PWPolicy | P0 | J0 |

**Total BC12 : 6 FRs**

---

### 4.13 BC13 — Compliance (SMSI ISO 27001:2022 / PCI DSS v4.0.1 / Loi données 2025)

| ID | Exigence | Description | Référence | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-109 | ISO 27001:2022 (93 contrôles Annexe A) | Mapping 22 BC vers 93 contrôles (A.5 org, A.6 people, A.7 phys, A.8 crypto, A.9 access, A.10 cryptog, A.11 comms, A.12 systems, A.13 suppliers, A.14 incident, A.15 business continuity) | ISO 27001:2022 | Compliance/ISO | P0 | J0 |
| FR-110 | PCI DSS v4.0.1 (tokenisation) | Tokenisation PAN, chiffrement AES-256-GCM niveau champ, MFA CDE | PCI DSS v4.0.1 | Compliance/PCI | P1 | J0 |
| FR-111 | Loi données 2025 — DPO (rôle obligatoire) | Désignation DPO (rôle user), accès dashboards conformité, notifications breaches | Loi données 2025 | Compliance/DPO | P0 | J0 |
| FR-112 | Loi données 2025 — DPIA (Data Protection Impact Assessment) | Questionnaire DPIA avant traitement risqué (biométrie, géolocalisation), validation DPO | Loi données 2025 | Compliance/DPIA | P0 | J0 |
| FR-113 | Loi données 2025 — Notification breach (72h) | Détection incident, notification INPDP/clients dans 72h, logging breach events | Loi données 2025 | Compliance/Breach | P0 | J0 |
| FR-114 | Consentement granulaire (cookie banner) | Banneau consentement AR/FR/EN, choix granulaires (analytics/marketing/essential), traçabilité | Loi données 2025 | Compliance/Consent | P0 | J0 |
| FR-115 | HSM (Hardware Security Module) | Signatures cryptographiques clés privées stockées HSM (Thales Luna), accès MFA | PCI DSS v4.0.1 | Compliance/HSM | P0 | J0 |
| FR-116 | Chiffrement données sensibles (AES-256-GCM) | CIN, passeport, adresse, salaires = chiffrement AES-256-GCM, clés rotation 90j | ISO 27001:2022 | Compliance/Encryption | P0 | J0 |

**Total BC13 : 8 FRs**

---

### 4.14 BC14 — Arrangement (Contrats / Conditions / Limites / Produits / Bundles) [NOUVEAU]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-117 | Création arrangement (contrat) | Définir arrangement : client, produits associés (compte, crédit, DAT), conditions générales, limites | Circ. 2025-17 | Arrangement/Create | P0 | J1 |
| FR-118 | Conditions arrangement (taux, frais, durée) | Paramètres : taux (%, montant), frais (montant, %valeur), durée (mois), renouvellement | Circ. 2025-17 | Arrangement/Terms | P0 | J1 |
| FR-119 | Limites arrangement (crédit, débit) | Limit crédit (montant max), limit débit (découvert), limit transfer (virement), alert % | Circ. 2025-17 | Arrangement/Limits | P0 | J1 |
| FR-120 | Produits associés arrangement | Bundle : 1 compte courant + 1 DAT + 1 crédit, éléments optionnels, pricing bundle | Circ. 2025-17 | Arrangement/Products | P1 | J1 |
| FR-121 | Simulation arrangement (pricing) | Calcul montant + frais + intérêts, total cost of ownership, scenarios alternatifs | Circ. 2025-17 | Arrangement/Simulation | P1 | J1 |
| FR-122 | Négociation arrangement (workflow) | Client → offre → acceptation → arrangement actif, versioning offres | Circ. 2025-17 | Arrangement/Negotiation | P1 | J1 |
| FR-123 | Modification arrangement (amendment) | Changement taux/frais/durée, approbation, nouvel enregistrement, audit trail | Circ. 2025-17 | Arrangement/Amendment | P1 | J1 |
| FR-124 | État arrangement (lifecycle) | PROPOSITION → OFFERTE → ACCEPTÉE → ACTIVE → SUSPENDUE/CLÔTURÉE | Circ. 2025-17 | Arrangement/Status | P0 | J1 |
| FR-125 | Termination arrangement (clôture) | Clôture anticipée/normale, settlement dernier paiement, archivage contrat | Circ. 2025-17 | Arrangement/Termination | P1 | J1 |

**Total BC14 : 9 FRs**

---

### 4.15 BC15 — TradeFinance (L/C / Garanties / Documentary / UCP 600) [NOUVEAU]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-126 | Lettre de crédit (L/C creation) | L/C import : client, applicant, beneficiary, montant, devise, documents required (invoice, B/L, insurance), validité | UCP 600 | TradeFinance/LC | P1 | J2 |
| FR-127 | Workflow LC (négociation, dénouement) | Presentation documents → vérification conformité → payment → retour docs exportateur | UCP 600 | TradeFinance/LCFlow | P1 | J2 |
| FR-128 | Garantie bancaire (Bank Guarantee) | Garantie : soumissionnaire, bénéficiaire, montant, type (soumission, avance, performance, restitution), validité | UCP 600 | TradeFinance/BG | P1 | J2 |
| FR-129 | Remise documentaire (Documentary Credit) | Vente documents contre paiement, échange docs/cash, escompte traites | UCP 600 | TradeFinance/DocCredit | P1 | J2 |
| FR-130 | Conformité documents L/C | Vérification : facture vs L/C montants, dates, descriptions, signatures, certificats | UCP 600 | TradeFinance/DocCheck | P1 | J2 |
| FR-131 | Frais trade finance (commission) | Commission L/C (%, plafond), frais dossier, frais pénalité non-conformité | Circ. 2025-17 | TradeFinance/Fees | P1 | J2 |
| FR-132 | Reporting trade finance (statistiques) | Nombre L/C/jour, montants, devises, durée moyenne, taux de conformité, incidents | Circ. 2025-17 | TradeFinance/Report | P1 | J2 |

**Total BC15 : 7 FRs**

---

### 4.16 BC16 — CashManagement (Sweeps / Pooling / Liquidity / FX Forward) [NOUVEAU]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-133 | Sweep account (liquidity optimization) | Transfert automatique soldes >X vers master, collecte fonds inter-company, optimisation taux | Circ. 2025-17 | CashManagement/Sweep | P1 | J2 |
| FR-134 | Pooling (notionnel) | Regroupement soldes logiques, calcul intérêts groupe, allocation par étape | Circ. 2025-17 | CashManagement/Pooling | P1 | J2 |
| FR-135 | Prévision liquidité (forecast) | Projection flux 7/14/30j, alertes liquidité insuffisante, recommendations actions | Circ. 2025-17 | CashManagement/Forecast | P1 | J2 |
| FR-136 | FX forward (future rates) | Achat/vente devises à future date, taux forward, settlement T+7/T+30, RWA impact | Loi 76-18 | CashManagement/FXForward | P1 | J2 |
| FR-137 | Position trésorerie (dashboard temps réel) | Vue globale cash par devise, expositions, limites, actions recommandées | Circ. 2025-17 | CashManagement/Dashboard | P1 | J2 |
| FR-138 | Optimization pricing (sweep rates) | Calcul intérêts sweep, compensation taux, overhead allocation | Circ. 2025-17 | CashManagement/Pricing | P1 | J2 |
| FR-139 | Inter-company loans | Prêt inter-entités groupe, taux marché, conformité Loi 76-18, documentations | Circ. 2025-17 | CashManagement/IntercompanyLoan | P2 | J3 |

**Total BC16 : 7 FRs**

---

### 4.17 BC17 — IslamicBanking (Murabaha / Ijara / Waqf / Sharia / Musharaka) [NOUVEAU]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-140 | Murabaha (cost-plus financing) | Financement achat bien : coût + markup, versement à livraison, amortissement mensuel | Loi 2016-33 | IslamicBanking/Murabaha | P1 | J2 |
| FR-141 | Ijara (Islamic lease) | Location bien : loyers mensuels, option achat fin de période, valeur résiduelle, Sharia compliant | Loi 2016-33 | IslamicBanking/Ijara | P1 | J2 |
| FR-142 | Waqf (charitable endowment) | Fonds immobilisés caritatifs, revenus → organisations agréées, gestion perpétuelle, exonérations | Loi 2016-33 | IslamicBanking/Waqf | P1 | J2 |
| FR-143 | Wakala (agency) | Client mandate banque pour investissement, banque agit comme agent, frais base %, pas intérêt | Loi 2016-33 | IslamicBanking/Wakala | P1 | J2 |
| FR-144 | Musharaka (partnership) | Investissement partenaire : client + banque cotisent capital, PnL partagé (%), sortie clause | Loi 2016-33 | IslamicBanking/Musharaka | P1 | J2 |
| FR-145 | Sukuk (Islamic bond) | Émission obligations Sharia, coupon % (sans intérêt), rangement, cote BVMT | Loi 2016-33 | IslamicBanking/Sukuk | P2 | J3 |
| FR-146 | Sharia board (validation) | Validation produits par sharia board (3 érudits minimum), documentation fatwa, audit Sharia | Loi 2016-33 | IslamicBanking/Sharia | P1 | J2 |
| FR-147 | Double-booking (Islamic accounting) | Enregistrement dual : NCT + Sharia accounting (mudaraba %), allocation revenus conforme Sharia | Loi 2016-33 | IslamicBanking/DoubleBook | P1 | J2 |
| FR-148 | Zakat calculation (obligatoire) | Calcul zakat 2.5% assets musulmans, retenue/virement organisme agrégé, traçabilité | Loi 2016-33 | IslamicBanking/Zakat | P1 | J2 |

**Total BC17 : 9 FRs**

---

### 4.18 BC18 — DataHub (ODS / ADS / MDM / Data Quality / Real-time Analytics) [NOUVEAU]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-149 | ODS (Operational Data Store) | Replication temps quasi-réel (T+1 max) données opérationnelles : comptes, mouvements, crédits | Circ. 2025-17 | DataHub/ODS | P1 | J3 |
| FR-150 | ADS (Analytical Data Store) | Historique complet (10 ans), agrégations par dimensions (client, produit, région, risque) | Circ. 2025-17 | DataHub/ADS | P1 | J3 |
| FR-151 | MDM (Master Data Management) | Single source truth : clients, comptes, produits, organisations, hiérarchies | Circ. 2025-17 | DataHub/MDM | P1 | J3 |
| FR-152 | Data quality (validation) | Complétude, conformité type, unicité clés, freshness, alertes anomalies | Circ. 2025-17 | DataHub/Quality | P1 | J3 |
| FR-153 | Real-time dashboards (analytics) | KPI dashboards : clients actifs, dépôts/crédits/FX volumes, ratios prudentiels, AML trends | Circ. 2025-17 | DataHub/Dashboard | P1 | J3 |
| FR-154 | ETL pipelines (ELT moderne) | Extraction source → transformation → chargement ADS, scheduling orchestration, monitoring | Circ. 2025-17 | DataHub/ETL | P1 | J3 |
| FR-155 | GDPR compliance (data retention) | Purge automatique données archivées >10 ans, conformité INPDP, logs destruction | Loi données 2025 | DataHub/Retention | P1 | J3 |
| FR-156 | Data governance (lineage) | Traçabilité données source → destination, transformations, audit trail, DPO oversight | Loi données 2025 | DataHub/Lineage | P1 | J3 |

**Total BC18 : 8 FRs**

---

### 4.19 BC19 — ReferenceData (Master codes / Taux / Calendrier / Pays) [NOUVEAU]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-157 | Taux de change (taux BCP quotidien) | Import taux USD/EUR/GBP vs TND quotidien, versioning, utilisation FX ops | Loi 76-18 | ReferenceData/FXRate | P1 | J2 |
| FR-158 | Calendrier jours fériés (Tunisie) | Jours fériés nationaux/régionaux, impacte calcul jours mouvements/intérêts | Circ. 2025-17 | ReferenceData/Calendar | P0 | J1 |
| FR-159 | Codes pays / devises / banques | ISO 3166 (pays), ISO 4217 (devises), BIC/IBAN/RIB normalisés, mappages internes | ISO 20022 | ReferenceData/CodeLists | P0 | J1 |
| FR-160 | Tables de référence secteurs/professions | GAFI 40 recommandations risque, matching clients secteurs NPL élevés | GAFI R.16 | ReferenceData/RiskSectors | P1 | J1 |
| FR-161 | Taux prudentiels (capital, RWA) | Pondérations risques (0% contenu, 20% banques, 100% crédits), mise à jour circulaires | Circ. 2025-08 | ReferenceData/RiskWeights | P0 | J1 |
| FR-162 | Paramètres légaux (limites, minima) | Limite concentration 25%, ratio C/D 120%, solvabilité 10%, Tier 1 7%, configurable par circulaire | Circ. 2025-08 | ReferenceData/LegalLimits | P0 | J1 |
| FR-163 | Listes OFAC/UE (sanctions) | Synchronisation automatique hebdo, versioning, matching fuzzy | Circ. 2025-17 | ReferenceData/SanctionsList | P0 | J1 |

**Total BC19 : 7 FRs**

---

### 4.20 BC20 — Securities (Titres / Portefeuille / Dépositaire / Custody / BVMT) [NOUVEAU]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-164 | Portefeuille titres (positions) | Enregistrement actions/obligations : ISIN, quantité, valeur unitaire, devise, date achat | BVMT règles | Securities/Portfolio | P1 | J3 |
| FR-165 | Ordres bourse (achat/vente) | Ordre marché/limite, transmission courtier, exécution, règlement T+2/T+3, commission | BVMT règles | Securities/Order | P1 | J3 |
| FR-166 | Custody (dépositaire) | Safekeeping titres, versement divides/coupons, remboursement capital, reporting | BVMT règles | Securities/Custody | P1 | J3 |
| FR-167 | Calcul gains/losses (unrealized) | Évaluation position mark-to-market (quotidien), gain/perte non-réalisée, P&L impact | IFRS 9 | Securities/Valuation | P1 | J3 |
| FR-168 | Dividendes / coupons | Versement automatique revenus titres, allocations portefeuille, imposition (SRE) | BVMT règles | Securities/Income | P1 | J3 |
| FR-169 | Opérations titres avancées (splits, conversions) | Opérations d'émetteur (splits, consolidations, conversions), mise à jour positions | BVMT règles | Securities/Corporate | P2 | J3 |
| FR-170 | Reporting BVMT (statistiques) | Positions titres, volumes traded, clients actifs bourse, audit trail complet | BVMT règles | Securities/BVMTReport | P1 | J3 |

**Total BC20 : 7 FRs**

---

### 4.21 BC21 — Insurance (Assurances liées / Crédit / Décès / Risque / Sinistres) [NOUVEAU]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-171 | Assurance crédit (linked) | Couverture perte crédit classification 3/4, prime mensuelle, limite montant/âge emprunteur | Circ. 2025-17 | Insurance/CreditLife | P1 | J3 |
| FR-172 | Assurance décès (décès emprunteur) | Capital restant dû assuré si décès, bénéficiaire = banque, prime fonction montant/âge | Circ. 2025-17 | Insurance/DeathBenefit | P1 | J3 |
| FR-173 | Assurance risque (perte emploi) | Couverture revenus emprunteur, indemnité chômage, prime fonction montant | Circ. 2025-17 | Insurance/JobLoss | P1 | J3 |
| FR-174 | Polices liées (bundling) | Polices rattachées crédit/compte, primes auto-débitées compte, renouvellement automatique | Circ. 2025-17 | Insurance/Policy | P1 | J3 |
| FR-175 | Déclaration sinistre (claims) | Notification sinistre, dossier sinistre, documentation réclamation, versement indemnité | Circ. 2025-17 | Insurance/Claim | P1 | J3 |
| FR-176 | Tarification assurance (risk-based) | Prime fonction âge/montant/durée, commission assureur, marge banque | Circ. 2025-17 | Insurance/Pricing | P1 | J3 |

**Total BC21 : 6 FRs**

---

### 4.22 BC22 — Compliance (Étendu — goAML / Travel Rule / TuniCheque / Effectiveness) [NOUVEAU v4.0]

| ID | Exigence | Description | Référence BCT | Temenos Map | Priorité | Jalon |
|---|---|---|---|---|---|---|
| FR-177 | goAML intégrée (CTAF submission) | Déclarations soupçon électroniques CTAF via goAML API, XML conforme, numéro suivi | Circ. 2025-17 | Compliance/goAML | P0 | J2 |
| FR-178 | Travel rule (originator/beneficiary data) | Copie identifiant originator (CIN, nom) + compte → bénéficiaire pour transferts >250k TND | GAFI R.16 rev. | Compliance/TravelRule | P0 | J2 |
| FR-179 | TuniCheque API (verification temps réel) | Appel API TuniCheque avant encaissement chèque, vérification provision, blocage si négatif | Circ. 2025-03 | Compliance/TuniCheque | P1 | J2 |
| FR-180 | Effectiveness metrics (GAFI evaluation) | Reporting : nombre investigations, SAR soumis, taux clôture, montants gélés, impact | GAFI R.16 | Compliance/Effectiveness | P0 | J2 |
| FR-181 | Conformité Loi 2016-33 (banques islamiques) | Validation produits sharia board, double-booking NCT/Sharia, zakat calculation | Loi 2016-33 | Compliance/Islamic | P1 | J2 |
| FR-182 | Intégration BioDev (biométrie ANCS) | Tests intrusion biométrie e-KYC via ANCS (Circ. 2025-06), validation security, certification | Circ. 2025-06 | Compliance/BioDev | P0 | J0 |

**Total BC22 : 6 FRs**

---

## 5. Non-Functional Requirements (NFRs)

| NFR | Spécification | Mesure | Cible |
|---|---|---|---|
| **Performance P99** | Latence API interne (sans persistance) | P99 latency | <5ms |
| **Performance E2E** | Latence incluant persistance PostgreSQL | P95 latency | <200ms |
| **Disponibilité** | SLA uptime production | % uptime | 99.9% |
| **Sécurité** | Vulnérabilités critiques | 0 non-mitigées | Avant release |
| **Encryption** | Chiffrement données sensibles | AES-256-GCM | 100% PII |
| **HSM** | Signatures cryptographiques | Clés privées | Toutes dans HSM |
| **i18n** | Langues complètement supportées | AR RTL + FR + EN | 100% UI/messages |
| **Accessibilité** | WCAG 2.1 compliance | Niveau | AA minimum |
| **Auditabilité** | Couverture opérations audit trail | % opérations loggées | 100% immutable |
| **Scalabilité** | Architecture microservices-ready | Conteneurisation | Kubernetes-ready |
| **Isolation données** | Isolation logique/physique par client (multi-tenant) | Cryptage niveau tenant | 100% |
| **Retention** | Conformité INPDP conservation données | Archivage post-clôture | 10 ans |
| **Compliance** | Couverture ISO 27001:2022 | % contrôles Annexe A | 93/93 (100%) |
| **Tests** | Couverture domain layer | Tarpaulin % | 95%+ |
| **BDD** | Scénarios Gherkin | Count | ≥400 |

---

## 6. Glossaire métier (Ubiquitous Language DDD)

| Terme | Définition | Type Rust | BC | Référence |
|---|---|---|---|---|
| **Compte** | Instrument dépôt/crédit identifié RIB | `Account` struct | BC2 | Circ. 2025-17 |
| **Client** | PP/PM titulaire fiche KYC | `Customer` aggregate | BC1 | Circ. 2025-17 |
| **Fiche KYC** | Enregistrement identification Annexe 1 | `KycProfile` | BC1 | Circ. 2025-17 art. 5 |
| **Bénéficiaire effectif** | PP possédant ≥25% capital | `Beneficiary` struct | BC1 | Circ. 2025-17 art. 8 |
| **PEP** | Personne politiquement exposée | `PepCheck` aggregate | BC1 | Circ. 2025-17 art. 13 |
| **EDD** | Enhanced Due Diligence renforcée | `EddProfile` struct | BC1 | Circ. 2025-17 art. 14 |
| **Créance** | Engagement crédit classifiable 0-4 | `Loan` aggregate | BC3 | Circ. 91-24 |
| **Classe créance** | Classification : 0=courant→4=compromis | `AssetClass` enum | BC3 | Circ. 91-24 |
| **Provision** | Montant comptabilisé risque perte | `Provision` struct | BC3 | Circ. 91-24 |
| **ECL** | Expected Credit Loss IFRS 9 | `ExpectedCreditLoss` struct | BC3/BC7 | IFRS 9 |
| **Stage 1/2/3** | Étapes IFRS 9 risque crédit | `CreditStage` enum | BC3/BC7 | IFRS 9 |
| **Ratio solvabilité** | FPN/RWA ≥10% | `SolvencyRatio` type | BC6 | Circ. 2025-08 |
| **Tier 1** | CET1+AT1 ≥7% | `Tier1Ratio` type | BC6 | Circ. 2025-08 |
| **Ratio C/D** | Crédits/Dépôts ≤120% | `CDRatio` type | BC6 | Circ. 2025-08 |
| **Concentration** | Risque/client ≤25% FPN | `ConcentrationRisk` type | BC6 | Circ. 91-24 |
| **RWA** | Risk-Weighted Assets | `RiskWeightedAssets` struct | BC6 | Bâle III |
| **FPN** | Fonds Propres Nets | `RegulatoryCapital` struct | BC6 | Circ. 2025-08 |
| **Arrangement** | Contrat client (compte+crédit+DAT) | `Arrangement` aggregate | BC14 | Circ. 2025-17 |
| **Limite arrangement** | Plafonds crédit/débit/virement | `ArrangementLimit` struct | BC14 | Circ. 2025-17 |
| **Déclaration soupçon** | Signalement CTAF operation suspecte | `SuspicionReport` aggregate | BC4/BC22 | Circ. 2025-17 art. 125 |
| **Gel avoirs** | Blocage irrévocable sans CTAF | `AssetFreeze` aggregate | BC2/BC4 | Circ. 2025-17 |
| **Travel rule** | Copie originator/beneficiary données | `TravelRuleData` struct | BC22 | GAFI R.16 |
| **L/C** | Lettre de crédit | `LetterOfCredit` aggregate | BC15 | UCP 600 |
| **Murabaha** | Cost-plus Islamic finance | `Murabaha` aggregate | BC17 | Loi 2016-33 |
| **Ijara** | Islamic lease | `Ijara` aggregate | BC17 | Loi 2016-33 |
| **Waqf** | Charitable endowment Islam. | `Waqf` aggregate | BC17 | Loi 2016-33 |
| **Sweep** | Transfert liquidité automatique | `SweepAccount` aggregate | BC16 | Circ. 2025-17 |
| **Pooling** | Regroupement logique soldes | `NotionalPool` aggregate | BC16 | Circ. 2025-17 |
| **Sharia board** | Conseil validation produits Islam. | `ShariaBoard` entity | BC17 | Loi 2016-33 |
| **ODS** | Operational Data Store | `DataStore` enum | BC18 | Circ. 2025-17 |
| **ADS** | Analytical Data Store | `DataStore` enum | BC18 | Circ. 2025-17 |
| **MDM** | Master Data Management | `MasterData` aggregate | BC18 | Circ. 2025-17 |
| **Audit trail** | Enregistrement immutable opérations | `AuditEntry` aggregate | BC11 | Circ. 2006-19 |
| **Écriture comptable** | Enregistrement journal NCT | `JournalEntry` aggregate | BC7 | NCT art. 10 |
| **RIB** | Relevé Identité Bancaire | `RIB` value object | BC2 | Circ. 2025-17 |
| **DAT** | Dépôt À Terme | `Account` type=DAT | BC2 | Circ. 2025-17 |
| **RBAC** | Role-Based Access Control | `Role` enum | BC12 | BCT governance |
| **2FA** | Two-Factor Authentication | `AuthFactor` struct | BC12 | Circ. 2025-06 |
| **HSM** | Hardware Security Module | `HsmKey` opaque type | BC13 | PCI DSS v4.0.1 |
| **DPO** | Data Protection Officer | `User` with role=DPO | BC13 | Loi données 2025 |
| **DPIA** | Data Protection Impact Assessment | `DPIA` aggregate | BC13 | Loi données 2025 |
| **Consentement** | Accord traitement données PII | `Consent` aggregate | BC13 | Loi données 2025 |
| **Zakat** | Aumône Islam. 2.5% assets | `Zakat` struct | BC17 | Loi 2016-33 |
| **Sukuk** | Obligation Islam. | `Sukuk` aggregate | BC17 | Loi 2016-33 |
| **Portfolio** | Ensemble titres client | `Securities.Portfolio` struct | BC20 | BVMT |
| **Custody** | Safekeeping titres dépositaire | `Custody` aggregate | BC20 | BVMT |
| **Polices liées** | Assurances bundled crédit | `InsurancePolicy` aggregate | BC21 | Circ. 2025-17 |
| **Sinistre** | Incident assuré | `InsuranceClaim` aggregate | BC21 | Circ. 2025-17 |

---

## 7. Invariants métier (25+ invariants traçables)

| ID | Invariant | Description | Règle | Référence BCT | BC |
|---|---|---|---|---|---|
| INV-01 | KYC avant compte | Fiche KYC validée requise avant ouverture compte | Pas de compte sans KYC valide | Circ. 2025-17 art. 5 | BC1/BC2 |
| INV-02 | Compte unique par devise | Un client = 1 compte courant TND max | Pas duplication comptes | Circ. 2025-17 | BC2 |
| INV-03 | Solde invariant | Solde = Σ mouvements crédits - débits | Vérification quotidienne | Circ. 91-24 | BC2/BC7 |
| INV-04 | Créance classée unique | Chaque créance = exactement 1 classe (0-4) | Pas multi-classe | Circ. 91-24 | BC3 |
| INV-05 | Concentration limite | Risque/client ≤ 25% FPN | Alerte si >20%, blocage si >25% | Circ. 91-24 | BC3/BC6 |
| INV-06 | Solvabilité minimum | FPN/RWA ≥ 10% | Alerte si <12%, breach if <10% | Circ. 2025-08 | BC6 |
| INV-07 | Tier 1 minimum | (CET1+AT1)/RWA ≥ 7% | Alerte si <8%, breach if <7% | Circ. 2025-08 | BC6 |
| INV-08 | Ratio C/D plafond | Crédits/Dépôts ≤ 120% | Alerte si >110%, breach if >120% | Circ. 2025-08 | BC6 |
| INV-09 | Provision obligatoire | Classe X → provision Y% | Classe 0=0%, 1=20%, 2=50%, 3=75%, 4=100% | Circ. 91-24 | BC3/BC7 |
| INV-10 | Rétention données KYC | Fiches conservées 10 ans post-clôture | Archivage non-destroyable | Circ. 2025-17 art. 20 | BC1 |
| INV-11 | Audit trail immutable | Chaque opération signée cryptographiquement | Hash chain SHA256 | Circ. 2006-19 | BC11 |
| INV-12 | Segmentation 3LoD | 1LoD opérateurs, 2LoD compliance, 3LoD audit | Rôles distincts, pas multiples | BCT governance | BC11/BC12 |
| INV-13 | Consentement INPDP | Données PII traitées = consentement explicite | Checkbox obligatoire avant KYC | Loi données 2025 art. 7 | BC1/BC13 |
| INV-14 | Gestion PEP/EDD | Client PEP flaggé automatiquement → EDD lancée | Workflow obligatoire 10j | Circ. 2025-17 art. 13-14 | BC1 |
| INV-15 | DOS obligatoire | Transaction suspecte → SAR CTAF dans 24h | Pas suppressions manuelles SAR | Circ. 2025-17 art. 125 | BC4/BC22 |
| INV-16 | Gel avoirs absolu | Compte gelé ne peut plus débiter | Sauf virements entrants | Circ. 2025-17 | BC2/BC4 |
| INV-17 | Travel rule obligatoire | Transfert >250k TND = données originator | Copie complète vers bénéficiaire | GAFI R.16 rev. | BC22 |
| INV-18 | Taux immutable DAT | Taux DAT fixe pendant durée contrat | Pas modification retroactive | Circ. 2025-17 | BC2 |
| INV-19 | Double-booking Islamic | Produit Islamic = comptabilité NCT + Sharia | Deux écritures synchrones | Loi 2016-33 | BC17/BC7 |
| INV-20 | Sharia validation | Produit Islamic approuvé sharia board avant lancement | 3 érudits minimum | Loi 2016-33 | BC17 |
| INV-21 | ISO 27001 coverage | 93 contrôles Annexe A mappés BC | Chaque contrôle → BC responsable | ISO 27001:2022 | BC13 |
| INV-22 | PCI DSS tokenisation | PAN jamais stocké en clair | Token AES-256-GCM | PCI DSS v4.0.1 | BC13 |
| INV-23 | DPIA avant traitement risqué | Biométrie/géolocalisation = DPIA validation DPO | Approval DPO avant go-live | Loi données 2025 | BC13 |
| INV-24 | Notification breach 72h | Incident données → INPDP notifiée <72h | Logging obligatoire incident | Loi données 2025 art. 33 | BC13 |
| INV-25 | Portabilité données | Client demande → export données JSON <30j | Format standard, non-propriétaire | Loi données 2025 art. 20 | BC1/BC13 |

---

## 8. Data Model (Tables principales par BC)

| BC | Entités principales | Tables | Clés |
|---|---|---|---|
| **BC1-Customer** | `Customer`, `KycProfile`, `Beneficiary`, `PepCheck`, `EddProfile`, `RiskScore` | customers, kyc_profiles, beneficiaries, pep_checks, edd_profiles, risk_scores | customer_id (PK) |
| **BC2-Account** | `Account`, `Balance`, `Movement`, `InterestSchedule` | accounts, balances, movements, interest_schedules | account_id, rib (UNIQUE) |
| **BC3-Credit** | `Loan`, `LoanSchedule`, `AssetClass`, `Provision` | loans, loan_schedules, asset_classes, provisions | loan_id, customer_id (FK) |
| **BC4-AML** | `Alert`, `Investigation`, `SuspicionReport`, `AssetFreeze` | aml_alerts, investigations, suspicion_reports, asset_freezes | alert_id, investigation_id |
| **BC5-Sanctions** | `SanctionList`, `ScreeningResult`, `SanctionMatch` | sanction_lists, screening_results, sanction_matches | list_id, screening_id |
| **BC6-Prudential** | `PrudentialRatio`, `RiskWeightedAsset`, `RatioBreachAlert` | prudential_ratios, rwa_calculations, ratio_alerts | ratio_id, calculation_date |
| **BC7-Accounting** | `JournalEntry`, `Ledger`, `ChartOfAccounts`, `TrialBalance` | journal_entries, ledgers, chart_of_accounts, trial_balances | entry_id, account_code |
| **BC8-Reporting** | `RegulatoryReport`, `ReportTemplate`, `ReportSubmission` | regulatory_reports, report_templates, report_submissions | report_id, template_id |
| **BC9-Payment** | `PaymentOrder`, `Transfer`, `SwiftMessage`, `Clearing` | payment_orders, transfers, swift_messages, clearing_batches | payment_id, transfer_id |
| **BC10-ForeignExchange** | `FxOperation`, `FxPosition`, `ExchangeRate` | fx_operations, fx_positions, exchange_rates | operation_id, position_id |
| **BC11-Governance** | `AuditTrail`, `Committee`, `ControlCheck`, `ComplianceReport` | audit_trails, committees, control_checks, compliance_reports | audit_id, committee_id |
| **BC12-Identity** | `User`, `Role`, `Permission`, `Session`, `TwoFactorAuth` | users, roles, permissions, sessions, totp_seeds | user_id, role_id, session_id |
| **BC13-Compliance** | `ComplianceControl`, `DPIA`, `ConsentRecord`, `BreachReport`, `HsmKey` | compliance_controls, dpias, consents, breach_reports, hsm_keys | control_id, dpia_id, consent_id |
| **BC14-Arrangement** | `Arrangement`, `ArrangementTerm`, `ArrangementLimit`, `ArrangementProduct` | arrangements, arrangement_terms, arrangement_limits, arrangement_products | arrangement_id, customer_id |
| **BC15-TradeFinance** | `LetterOfCredit`, `BankGuarantee`, `DocumentaryCredit`, `TradeDocument` | letters_of_credit, bank_guarantees, documentary_credits, trade_documents | lc_id, bg_id |
| **BC16-CashManagement** | `SweepAccount`, `NotionalPool`, `Liquidity Forecast`, `FxForward` | sweep_accounts, notional_pools, liquidity_forecasts, fx_forwards | sweep_id, pool_id |
| **BC17-IslamicBanking** | `Murabaha`, `Ijara`, `Waqf`, `Wakala`, `Musharaka`, `Zakat`, `ShariaBoard` | murabaha_contracts, ijara_leases, waqf_funds, wakala_agreements, musharaka_partnerships, zakat_records, sharia_boards | contract_id, customer_id |
| **BC18-DataHub** | `DataLakeTable`, `MasterDataEntity`, `DataQualityReport` | data_lake_tables, master_data_entities, data_quality_reports, etl_runs | table_id, entity_id |
| **BC19-ReferenceData** | `ExchangeRate`, `Country`, `Currency`, `CalendarDay`, `SectorCode`, `RiskWeightTable` | reference_exchange_rates, reference_countries, reference_currencies, calendar_holidays, sector_codes, risk_weight_tables | rate_id, country_code |
| **BC20-Securities** | `SecurityPortfolio`, `Security`, `SecurityOrder`, `SecurityPosition`, `CustodyRecord` | security_portfolios, securities, security_orders, security_positions, custody_records | portfolio_id, security_id, order_id |
| **BC21-Insurance** | `InsurancePolicy`, `InsuranceClaim`, `InsurancePremium` | insurance_policies, insurance_claims, insurance_premiums | policy_id, claim_id |
| **BC22-Compliance** | `GoAMLSubmission`, `TravelRuleData`, `TuniChequeCheck`, `EffectivenessMetric` | goaml_submissions, travel_rule_data, tunicheque_checks, effectiveness_metrics | submission_id, metric_id |

---

## 9. ADRs (Architecture Decision Records)

### ADR-001: Hexagonal Architecture (Ports & Adapters)
**Decision** : Architecture 3-layer stricte (Domain → Application → Infrastructure).
**Rationale** : Isolation domaine métier, testabilité maximale, évolutivité réglementaire (chaque circulaire = module injectable).
**Status** : ACCEPTED (v3.0+)

### ADR-002: Domain-Driven Design (DDD)
**Decision** : 22 bounded contexts organisés par domaine métier, ubiquitous language, entités + value objects.
**Rationale** : Alignement parfait code ↔ métier bancaire tunisien, traçabilité directe vers exigences légales.
**Status** : ACCEPTED (v3.0+)

### ADR-003: BDD with Gherkin (Cucumber)
**Decision** : Tests scénario en Gherkin, exécution via Cucumber-rs, documentations vivantes.
**Rationale** : Spécifications légales = tests automatiques, vérifiabilité auditeurs BCT, 0 ambiguïté.
**Status** : ACCEPTED (v3.0+)

### ADR-004: Async/Await with Tokio
**Decision** : Runtime async Tokio, 1 thread-per-core, optimisation latence P99 <5ms.
**Rationale** : Performance bancaire critique, milliers transactions/sec, PostgreSQL async via sqlx.
**Status** : ACCEPTED (v3.0+)

### ADR-005: PostgreSQL ACID + Encryption
**Decision** : PostgreSQL 16 LUKS AES-XTS-512 (disk), PGCrypto (in-transit), chiffrement application AES-256-GCM fields sensibles.
**Rationale** : ACID garantis conformité transaction, chiffrement multicouches PCI DSS, conformité INPDP.
**Status** : ACCEPTED (v3.0+)

### ADR-006: Hash Chain Audit Trail (Immutable)
**Decision** : Chaque opération → SHA256 hashée chaîne précédente, stockage append-only, vérification quotidienne intégrité.
**Rationale** : Preuve immuabilité audit trail, détection tampering, conformité Circ. 2006-19, Loi données 2025.
**Status** : ACCEPTED (v3.0+)

### ADR-007: Multi-language i18n (AR RTL + FR + EN)
**Decision** : Arabe tunisien RTL natively, Français, English, base lexicon 500+ termes bancaires.
**Rationale** : Accessibilité clients tunisiens, régulateurs parlent arabe/français, conformité WCAG 2.1 AA.
**Status** : ACCEPTED (v3.0+)

### ADR-008: CI/CD GitHub Actions (Lint + Test + Audit)
**Decision** : Workflows GHA : rustfmt, clippy, cargo-tarpaulin, cargo-audit, E2E Playwright.
**Rationale** : Qualité code guarantee, vulnérabilités détectées pre-merge, performance regression testing.
**Status** : ACCEPTED (v3.0+)

### ADR-009: HSM for Cryptographic Keys
**Decision** : Clés privées signatures bancaires = HSM Thales Luna, accès MFA, audit trail HSM.
**Rationale** : PCI DSS v4.0.1 mandatory 2025, conformité banking-grade sécurité, NIST FIPS 140-2 L3.
**Status** : ACCEPTED (v3.0+)

### ADR-010: SMSI ISO 27001:2022 (93 Controls)
**Decision** : Couverture 100% contrôles Annexe A ISO 27001:2022, mapping chaque BC, certification avant production.
**Rationale** : Certification obligatoire tunisienne (Circ. 2025-17 implicite), baseline sécurité internationale reconnue.
**Status** : ACCEPTED (v3.0+)

### ADR-011: Arrangement as Central BC (v4.0 NEW)
**Decision** : Arrangement = aggregate root central liant Account + Loan + DAT + Products, source truth conditions/limits/pricing.
**Rationale** : Temenos Party/Holdings integration, parité CBS, négociation client flexible (pricing engines, simulations).
**Status** : ACCEPTED (v4.0)

### ADR-012: Islamic Finance Engine (v4.0 NEW)
**Decision** : BC17-IslamicBanking native, Sharia board validation, double-booking NCT/Islamic, waqf perpetual endowments.
**Rationale** : Loi 2016-33 régit banques tunisiennes islamiques, parité Temenos Islamic module, zakat automation.
**Status** : ACCEPTED (v4.0)

### ADR-013: Microservices Preparation (v4.0 NEW)
**Decision** : Application layer ready Kubernetes : service discovery (Consul), tracing (Jaeger), metrics (Prometheus), event bus (async channels).
**Rationale** : Jalon 4 target microservices orchestration, scalabilité horizontale crédit/AML/FX, isolation domaines.
**Status** : PROPOSED (v4.0 implementation J4)

### ADR-014: GAFI R.16 Travel Rule (v4.0 NEW)
**Decision** : Originator/beneficiary data copiée transferts >250k TND, goAML intégrée CTAF, audit effectiveness metrics.
**Rationale** : GAFI evaluation nov 2026, R.16 revised juin 2025, travel rule non-negotiable compliance.
**Status** : ACCEPTED (v4.0)

### ADR-015: Open Banking PSD3-ready (v4.0 NEW)
**Decision** : APIs conformes PSD3 spec (consent management, SCA, TPP access), prepared fintech sandbox (token scoping).
**Rationale** : PSD3/PSR accord provisoire nov 2025, anticipation open banking Tunisie/Afrique, fintech partnerships leverage.
**Status** : PROPOSED (v4.0 Jalon 4)

---

## 10. Temenos Mapping (22 BCs → 17 Temenos Categories)

| Temenos Category | Temenos Endpoints | BANKO BC | Coverage % | Notes |
|---|---|---|---|---|
| **Party** | 120+ | BC1 (Customer) | 90% | KYC, bénéficiaires, PEP, scoring |
| **Holdings** | 150+ | BC2 (Account), BC20 (Securities) | 85% | Comptes, DAT, portfolios titres |
| **Order** | 80+ | BC9 (Payment), BC15 (TradeFinance), BC20 (Securities) | 75% | Virements, L/C, ordres bourse |
| **Product** | 100+ | BC14 (Arrangement) | 80% | Bundles, conditions, pricing |
| **Credit** | 180+ | BC3 (Credit), BC17 (IslamicBanking) | 85% | Crédits, murabaha, ijara |
| **Collateral** | 60+ | BC14 (Arrangement) future | 50% | Nantissement (v4.1 planned) |
| **FX** | 90+ | BC10 (ForeignExchange) | 75% | Spot, forward, position, taux |
| **Risk** | 120+ | BC4 (AML), BC5 (Sanctions), BC6 (Prudential), BC22 (Compliance) | 85% | AML, sanctions, ratios, stress-test |
| **AML** | 70+ | BC4 (AML), BC22 (Compliance) | 90% | Surveillance, DOS, goAML, travel rule |
| **Enterprise** | 100+ | BC7 (Accounting), BC11 (Governance) | 80% | GL, audit trail, comités |
| **Accounting** | 140+ | BC7 (Accounting), BC8 (Reporting) | 85% | NCT, IFRS 9, balance, P&L |
| **Analytics** | 100+ | BC8 (Reporting), BC18 (DataHub) | 75% | Dashboards, ODS, ADS |
| **Islamic Banking** | 80+ | BC17 (IslamicBanking) | 90% | Murabaha, ijara, waqf, zakat |
| **Cash Management** | 100+ | BC16 (CashManagement) | 80% | Sweeps, pooling, forecast, liquidity |
| **Securities** | 110+ | BC20 (Securities) | 75% | Portfolios, custody, ordres |
| **System** | 150+ | BC11 (Governance), BC12 (Identity), BC13 (Compliance) | 80% | Auth, RBAC, audit, config |
| **Trade Finance** | 80+ | BC15 (TradeFinance) | 85% | L/C, bank guarantees, documentary |

**Total Temenos endpoints target: 450-500 (80-85% parité)**

---

## 11. Timeline Réaliste (Solo-dev 8h/sem, 12-16 mois)

| Phase | Jalon | Contextes | Semaines | Heures IA | Heures Solo | Fin prévue |
|---|---|---|---|---|---|---|
| **0** | Fondations | BC1, BC2, BC7, BC11, BC12, BC13 | 1-6 | 48h | 16h | 22 mai 2026 |
| **1** | Core Banking | BC3, BC4, BC5, BC6, BC8, BC9, BC10, BC14 | 7-14 | 120h | 40h | 16 juillet 2026 |
| **2** | Compliance+ Trade | BC15, BC16, BC17, BC19, BC22 extended | 15-20 | 110h | 37h | 21 septembre 2026 |
| **3** | Analytics+ Securities | BC18, BC20, BC21 | 21-26 | 80h | 27h | 2 novembre 2026 |
| **4** | Maturité | Microservices, Open Banking, hardening | 27-32 | 60h | 20h | 14 décembre 2026 |
| **Buffer** | Stabilité + fixes | Émergence, CI, polishing | 33-39 | 40h | 13h | 1er février 2027 |

**Total: 39 semaines ~ 9 mois calendaires (avril 2026 → février 2027)**
**Horizon conservatif (16 mois avec pauses): mai 2026 → août 2027**

---

## 12. Critères d'acceptation (Definition of Done)

Chaque Jalon clôturé = checklist:

- [ ] 100% FRs Jalon implémentés (code merged)
- [ ] 95%+ couverture domain tests (Tarpaulin)
- [ ] 100% scenarios BDD passe (Cucumber)
- [ ] Lint passe (rustfmt, clippy)
- [ ] Audit sécurité passe (cargo-audit)
- [ ] Performance P99 <5ms interne, P95 <200ms E2E
- [ ] i18n complete (AR/FR/EN)
- [ ] Documentation ADRs + glossaire à jour
- [ ] Audit trail exhaustif (100% opérations)
- [ ] Conformité ISO 27001 (% contrôles) ✓
- [ ] Conformité PCI DSS (tokenisation) ✓
- [ ] Conformité Loi données 2025 (DPIA, consentement) ✓

---

## 13. Success Metrics (Post-MVP)

| Métrique | Cible v4.0 | Mesure |
|---|---|---|
| Déploiements production (banques tunisiennes) | ≥2 | Annuaire GitHub releases |
| Nombre utilisateurs actifs | ≥50 (sandbox + prod) | Audit login logs |
| Forks GitHub + contributeurs | ≥10 | GitHub insights |
| Certification ISO 27001 | Oui | Audit certificateur externe |
| SAQ PCI DSS validée | Oui | Attestation ASV |
| Zéro trouvailles critiques audit GAFI | Oui | Rapport évaluateurs GAFI (nov 2026) |
| Documentation (% BC couverts) | 100% | Markdown pages |
| Temps moyen déploiement | <30min | CI/CD logs |

---

## 14. Risques et Mitigations

| Risque | Probabilité | Impact | Mitigation |
|---|---|---|---|
| Délai compliance réglementaire (circulaires) | Haute | Critique | Veille légale bi-hebdo, changements modulaires par BC |
| Burnout solo-dev | Moyenne | Élevé | Rythme 8h/sem réaliste, pauses programmées, code review IA |
| Microservices complexity J4 | Moyenne | Moyen | Prototype K8s déploiement J3, documentation anticipée |
| Performance P99 dégradation | Basse | Élevé | Benchmarking quotidien, profiling continu, alertes autom. |
| Discontinuité API Temenos | Basse | Moyen | Mapping versionné, abstraction couche application, tests regression |
| GAFI evaluation défaut | Très basse | Critique | Effectivité metrics v4.0, goAML depuis J2, travel rule conforme |

---

## Conclusion

BANKO v4.0 est l'ambitieux projet de parité Temenos pour le secteur bancaire tunisien. Avec 22 bounded contexts, 250+ FRs, 93 contrôles ISO 27001:2022, conformité GAFI R.16, Loi données 2025, et support finance islamique native, BANKO offre aux banques tunisiennes une alternative souveraine, auditable, gratuite en 12-16 mois calendaires (solo-dev 8h/sem avec accélération IA).

**Horizon cible** : Février 2027 (code feature-complete), avec déploiements production anticipés décembre 2026.

**Engagement** : Chaque ligne de code traçable vers texte légal. Aucune action bancaire illégale ne compile.

---

**Document généré** : 7 avril 2026
**Version** : 4.0.0
**Auteur** : GILMRY / Projet BANKO
**Licence** : AGPL-3.0 (Ce PRD est documentaire — logiciel sous AGPL-3.0)
