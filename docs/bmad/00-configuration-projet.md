# Configuration Projet — BANKO

> **Version** : 4.0.1 — 7 avril 2026 (itération post-validation Phase F)

## Méthode Maury — Étape 0 (Pré-requis)

> Ce document est injecté dans chaque prompt agent BMAD.

---

```
══════════════════════════════════════════════════════════════
CONFIGURATION PROJET — BANKO
══════════════════════════════════════════════════════════════

NOM DU PROJET     : BANKO
DESCRIPTION       : Système bancaire open source AGPL-3.0 pour les banques
                    tunisiennes — irréfutable, transparent, légal, auditable
STACK BACKEND     : Rust + Actix-web 4.9 + SQLx (async PostgreSQL)
STACK FRONTEND    : Astro 6+ (SSG) + Svelte 5 (Islands) + Tailwind CSS
BASE DE DONNÉES   : PostgreSQL 16 (ACID, chiffrement, partitionnement)
INFRASTRUCTURE    : Docker, Traefik, OVH Cloud / Hébergement souverain Tunisie
IaC               : Terraform + Ansible
CI/CD             : GitHub Actions
MONITORING        : Prometheus, Grafana, Loki, Alertmanager
SÉCURITÉ          : LUKS AES-XTS-512, HSM (Hardware Security Module),
                    fail2ban, Suricata IDS, CrowdSec WAF,
                    GPG backups S3 off-site
                    SMSI ISO/IEC 27001:2022 (93 contrôles, 4 thèmes)
                    PCI DSS v4.0.1 (chiffrement niveau champ, MFA CDE, tokenisation)
                    ANCS : tests intrusion obligatoires (Circ. 2025-06), cycle 2 ans
                    ISO/IEC 27701:2025 : management vie privée (IA, biométrie, IoT)
LICENCE           : AGPL-3.0 (copyleft fort — modifications doivent rester
                    open source même en déploiement SaaS)
LANGUES (i18n)    : AR (arabe tunisien — RTL), FR (français), EN (anglais)
DOMAINE LÉGAL     : Droit bancaire tunisien (Loi 2016-48, Loi 2016-33 banques islamiques,
                    Circulaires BCT, Loi LBC/FT 2015-26/2019-9, Loi données personnelles 2025)
                    + 17 circulaires BCT 2025 + 4 circulaires BCT 2026
                    + Évaluation GAFI 5ème cycle (plénière 1er novembre 2026)
                    + Normes internationales (Bâle III, GAFI/FATF 40 Recommandations,
                    IFRS 9, ISO 20022/27001:2022/27701:2025/22301/31000/8583)
                    + PCI DSS v4.0.1 (sécurité données cartes, obligatoire mars 2025)
                    + PSD3/PSR (Open Banking, accord provisoire nov. 2025)
                    + FIDA (Open Finance, adoption prévue H1 2026)
RÉFÉRENTIEL LÉGAL : docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md (95 références)
RÉFÉRENCE         : https://github.com/gilmry/koprogo (même recette technique)
BENCHMARK TEMENOS : https://developer.temenos.com (550-700+ endpoints, 17 catégories)

PROFIL DÉVELOPPEUR :
- [x] Solo-dev side-project (6-15h/sem, emploi salarié / étudiante)
- [ ] Solo-dev temps plein (35-40h/sem)
- [ ] Duo (2 devs)
- [ ] Équipe (3+ devs)

RYTHME SOLO-DEV (calibré sur KoproGo — 258h sur 31 semaines) :
- Mode burst   : 15h/sem (vacances, motivation forte)
- Mode normal  : 10h/sem (soirées + 1 jour week-end)
- Mode light   : 6h/sem  (emploi/études chargés)
- Mode pause   : 0h/sem  (vie, fatigue, blocage)
- Moyenne lissée réaliste : 8h/semaine
- Taux d'activité : ~45% des semaines (55% à 0 commit)

VÉLOCITÉ IA (coefficients d'ajustement — heures/story) :
- Backend (domain + API)        : S=1.5h, M=3h, L=5h (÷3 vs humain)
- Frontend (composants + pages) : S=3h, M=5h, L=10h (÷1.5 vs humain)
- Infrastructure (IaC + CI/CD)  : S=4h, M=8h, L=20h (×1, pas de gain IA)
- Tests E2E (multi-rôles)       : S=4h, M=8h, L=16h (×1, pas de gain IA)
- Tests BDD (Gherkin + steps)   : S=1h, M=2h, L=4h (÷2 vs humain)
- i18n / Polish / Docs          : S=2h, M=4h, L=8h (÷1.5 vs humain)
- Réserve émergence             : +20% de capacité
- Réserve CI stabilisation      : +10% (corrections fmt, clippy, audit)

FORMULE DURÉE CALENDAIRE :
  Heures = Σ (stories × heures_par_taille) + 20% émergence + 10% CI
  Semaines = Heures ÷ rythme_hebdo_moyen (8h solo-dev étudiante)
  Mois = Semaines ÷ 4.3

COUCHES DU PROJET FULL-STACK ISO 27001 :
Le pipeline BMAD doit couvrir TOUTES ces couches :
1. Domain (entités, value objects, invariants — droit bancaire tunisien, 95 réf. légales)
2. Application (use cases, DTOs, ports)
3. Infrastructure backend (repositories, handlers, routes, middleware)
4. Frontend (pages, composants, stores, API clients, validators, i18n AR/FR/EN)
5. Infrastructure as Code (Terraform, Ansible, Helm, secrets, HSM)
6. CI/CD (GitHub Actions workflows : lint, test, security audit, docker build)
7. Monitoring & Sécurité (Prometheus, Grafana, Loki, Suricata, CrowdSec,
   SMSI ISO 27001:2022, conformité PCI DSS, dashboard conformité)
8. Tests (unitaires, intégration, BDD, E2E API, E2E Playwright, security audit)

SPÉCIFICITÉS BANKO (vs KoproGo) :
- Domaine = secteur bancaire réglementé (pas copropriété)
- Parité fonctionnelle Temenos Transact out-of-the-box ciblée (550-700+ endpoints)
- Auditabilité BCT : chaque opération horodatée, signée, immutable
- Piste d'audit intégrale (audit trail) — obligation légale Circ. 2006-19
- Double moteur comptable : NCT actuel + pré-IFRS 9
- Module KYC/AML conforme Circ. 2025-17 (applicable immédiatement)
- Module prudentiel : ratios solvabilité (10%), Tier 1 (7%), C/D (120%)
- Classification créances automatique (classes 0-4, provisionnement)
- Support RTL (arabe) natif dans le frontend
- HSM obligatoire pour signatures cryptographiques bancaires
- Conformité INPDP (Loi données personnelles 2025, remplace 2004-63) — privacy-by-design
- Support banques islamiques (Loi 2016-33) : produits sharia, waqf, murabaha
- Interopérabilité : ISO 20022 (SWIFT), ISO 8583 (monétique)
- Conformité ISO 27001:2022 : 93 contrôles Annexe A mappés aux 22 bounded contexts
- PCI DSS v4.0.1 : tokenisation PAN, chiffrement AES-256-GCM niveau champ, MFA pour accès CDE
- Préparation Open Banking : APIs PSD3-ready, consent management, SCA, portail développeur
- Nouvelle loi données personnelles 2025 : DPO obligatoire, DPIA, notification 72h, portabilité, effacement
- GAFI R.16 révisée (juin 2025) : travel rule élargie, données originator/beneficiary
- Intégration goAML : plateforme CTAF pour déclarations de soupçon électroniques
- TuniCheque : API vérification chèques temps réel (Circ. 2025-03)
- e-KYC biométrique : enrôlement électronique Circ. 2025-06, FIDO2/WebAuthn, tests ANCS
- Réforme prudentielle Circ. 2025-08 : nouvelles normes capital (2026), risques (2027), IFRS 9 ECL

OBJECTIF STRATÉGIQUE v4.0 — PARITÉ TEMENOS (PHASED) :
BANKO v4.0 cible la **parité fonctionnelle progressive avec Temenos Transact**.
Temenos propose : Party, Holdings, Order, Product, Credit, Collateral, FX, Risk, AML,
Enterprise, Accounting, Analytics, Islamic Banking, Cash Management, Securities,
Microservices, System. BANKO en intègre 22 bounded contexts (13 v3.0 + 9 nouveaux v4.0).
Alignement de référence : developer.temenos.com.

SCOPE MVP v4.0 (13 BCs P0 — "Core Banking Ready") :
  Customer, Account, Credit, AML, Sanctions, Prudential, Accounting,
  Payment, Governance, Identity, Reporting, ForeignExchange, ReferenceData
  → ~121 stories, ~363h, ~300-350 endpoints = **50% Temenos**

SCOPE v4.1 (+ 5 BCs P1 — "Extended Banking") :
  + Arrangement, Collateral, IslamicBanking, Insurance, Compliance
  → +50 stories, +150h, ~450 endpoints = **70% Temenos**

SCOPE v4.2 (+ 4 BCs P2 — "Full Temenos Parity") :
  + TradeFinance, CashManagement, Securities, DataHub
  → +30 stories, +90h, ~550-600 endpoints = **85%+ Temenos**

HORIZON RÉALISTE (post-validation Phase F) :
  v4.0 MVP (conservateur)  : 18-22 mois (avril 2026 → oct-déc 2027)
  v4.0 MVP (agressif, IA÷3 validé) : 12-16 mois (avril 2026 → août 2027)
  v4.1 Extended   : +6-8 mois après v4.0
  v4.2 Full parity : +8-12 mois après v4.1
  Total parité complète : 32-36+ mois

DÉCISION SCOPE : Scénario B retenu (MVP 13 BCs + P1 roadmap).
  Priorité : contextes critiques avant optionnels
  (Customer → Account → Credit → Payment → Accounting obligatoires).
  Arrangement (BC central) = premier BC P1, sprint immédiat après MVP.

CONDITION GO/NO-GO Sprint 1 :
  - IA velocity validée post-Sprint 0 (coefficient ÷3 confirmé ou ajusté)
  - Si coefficient réel < ÷2 : réduire scope MVP à 10 BCs (drop ReferenceData, FX, Reporting)
  - Revue trimestrielle : recalibrage vélocité + scope
══════════════════════════════════════════════════════════════
```

> **Note** : Les invariants de qualité (SOLID, DDD, BDD, TDD, Hexagonale, YAGNI, DRY) ne sont PAS des paramètres — ils sont **non négociables** quel que soit le projet.

---

## 22 Bounded Contexts (v3.0 → v4.0)

### Contextes existants (13 v3.0)

1. **Customer** : Gestion des clients (onboarding, KYC, profil)
2. **Account** : Gestion des comptes (ouverture, solde, type de compte)
3. **Credit** : Octroi de crédit et gestion des prêts
4. **AML** : Anti-blanchiment d'argent (alertes, seuils)
5. **Sanctions** : Contrôle des sanctions internationales
6. **Prudential** : Exigences prudentielles et capital réglementaire
7. **Accounting** : Comptabilité générale (journaux, écritures)
8. **Reporting** : Rapports réglementaires et statistiques
9. **Payment** : Virements et paiements (SEPA, SWIFT)
10. **ForeignExchange** : Changes et taux de change
11. **Governance** : Gouvernance (rôles, permissions, audit)
12. **Identity** : Gestion des identités (authentification, biométrie)

### Contextes nouveaux (9 v4.0 → parité Temenos)

13. **Arrangement** : Contrats, accords, limites, produits associés (central !) — **P1**
14. **Collateral** : Garanties, nantissements, évaluations collatérales — **P1**
15. **TradeFinance** : Lettres de crédit, garanties bancaires, documentaire — **P2**
16. **CashManagement** : Trésorerie, liquidity management, sweep accounts — **P2**
17. **IslamicBanking** : Produits sharia, murabaha, ijara, waqf (Loi 2016-33) — **P1**
18. **DataHub** : Data lake, data warehouse, MDM (Master Data Management) — **P2**
19. **ReferenceData** : Données de référence centralisées (codes, taux, tables) — **P0** (promu MVP)
20. **Securities** : Valeurs mobilières, portefeuille titres, dépositaire — **P2**
21. **Insurance** : Assurances liées (crédit, décès, risque), courtage intégré — **P1**
22. **Compliance** : Cross-cutting (ISO 27001 SoA, PCI DSS, INPDP, GAFI) — **P0** (promu MVP)

### Glossaire v4.0 (termes nouveaux BCs)

| Terme | Définition | BC |
|-------|------------|-----|
| Arrangement | Contrat cadre liant un client à un ou plusieurs produits, comptes et limites | Arrangement |
| Facility | Ligne de crédit ou engagement dans un Arrangement | Arrangement |
| Collateral | Garantie physique ou financière adossée à un crédit ou Arrangement | Collateral |
| LTV (Loan-to-Value) | Ratio valeur du prêt / valeur du collatéral — seuil réglementaire | Collateral |
| Murabaha | Vente à marge bénéficiaire convenue d'avance (finance islamique) | IslamicBanking |
| Ijara | Location-vente avec option d'achat (leasing sharia-compliant) | IslamicBanking |
| Waqf | Dotation immobilière à vocation charitable (fondation islamique) | IslamicBanking |
| Sukuk | Obligations conformes à la charia (certificats d'investissement) | IslamicBanking |
| Sweep Account | Compte avec transfert automatique de solde excédentaire | CashManagement |
| Cash Pooling | Centralisation des soldes de plusieurs comptes (trésorerie groupe) | CashManagement |
| Lettre de Crédit (LC) | Engagement irrévocable de paiement d'une banque pour le compte d'un importateur | TradeFinance |
| Garantie Bancaire | Engagement d'une banque de payer en cas de défaillance du débiteur | TradeFinance |
| MDM (Master Data Management) | Gestion centralisée des données de référence (codes, taux, entités) | DataHub |
| BVMT | Bourse des Valeurs Mobilières de Tunis — marché des titres | Securities |
| Dépositaire | Entité conservant les titres pour le compte d'investisseurs | Securities |
| DPO | Data Protection Officer — obligatoire Loi 2025 (INPDP) | Compliance |
| DPIA | Data Protection Impact Assessment — obligatoire données sensibles | Compliance |
| SoA | Statement of Applicability — ISO 27001:2022 contrôles sélectionnés | Compliance |
| ACL | Anti-Corruption Layer — couche isolant le domaine des systèmes externes | Infrastructure |

---

## Références documentation conformité

| Document | Chemin |
|----------|--------|
| Référentiel légal et normatif (95 réf.) | `docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md` |
| Index des références légales | `docs/legal/legal-references-index.md` |
| ISO 27001:2022 — Périmètre et SoA | `docs/compliance/iso-27001/` |
| PCI DSS v4.0.1 — Scope et exigences | `docs/compliance/pci-dss/` |
| Open Banking / PSD3 — Préparation | `docs/compliance/open-banking-psd2/` |
| Matrice de conformité globale | `docs/compliance/overall-compliance-matrix.md` |
| Tableau de bord conformité | `docs/compliance-dashboard.md` |
