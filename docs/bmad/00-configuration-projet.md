# Configuration Projet — BANKO

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
LICENCE           : AGPL-3.0 (copyleft fort — modifications doivent rester
                    open source même en déploiement SaaS)
LANGUES (i18n)    : AR (arabe tunisien — RTL), FR (français), EN (anglais)
DOMAINE LÉGAL     : Droit bancaire tunisien (Loi 2016-48, Circulaires BCT,
                    Loi LBC/FT 2015-26/2019-9, Loi INPDP 2004-63)
                    + Normes internationales (Bâle III, GAFI/FATF 40 Recommandations,
                    IFRS 9, ISO 20022/27001/22301/31000/8583)
RÉFÉRENTIEL LÉGAL : docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md (70 références)
RÉFÉRENCE         : https://github.com/gilmry/koprogo (même recette technique)

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
1. Domain (entités, value objects, invariants — droit bancaire tunisien)
2. Application (use cases, DTOs, ports)
3. Infrastructure backend (repositories, handlers, routes, middleware)
4. Frontend (pages, composants, stores, API clients, validators, i18n AR/FR/EN)
5. Infrastructure as Code (Terraform, Ansible, Helm, secrets, HSM)
6. CI/CD (GitHub Actions workflows : lint, test, security audit, docker build)
7. Monitoring & Sécurité (Prometheus, Grafana, Loki, Suricata, CrowdSec)
8. Tests (unitaires, intégration, BDD, E2E API, E2E Playwright, security audit)

SPÉCIFICITÉS BANKO (vs KoproGo) :
- Domaine = secteur bancaire réglementé (pas copropriété)
- Auditabilité BCT : chaque opération horodatée, signée, immutable
- Piste d'audit intégrale (audit trail) — obligation légale Circ. 2006-19
- Double moteur comptable : NCT actuel + pré-IFRS 9
- Module KYC/AML conforme Circ. 2025-17 (applicable immédiatement)
- Module prudentiel : ratios solvabilité (10%), Tier 1 (7%), C/D (120%)
- Classification créances automatique (classes 0-4, provisionnement)
- Support RTL (arabe) natif dans le frontend
- HSM obligatoire pour signatures cryptographiques bancaires
- Conformité INPDP (Loi 2004-63) — privacy-by-design
- Interopérabilité : ISO 20022 (SWIFT), ISO 8583 (monétique)
══════════════════════════════════════════════════════════════
```

> **Note** : Les invariants de qualité (SOLID, DDD, BDD, TDD, Hexagonale, YAGNI, DRY) ne sont PAS des paramètres — ils sont **non négociables** quel que soit le projet.
