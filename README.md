# 🏦 BANKO

> Système bancaire open source, conforme, auditable et transparent pour les banques tunisiennes.

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](LICENSE)
[![CI](https://github.com/gilmry/BANKO/actions/workflows/ci.yml/badge.svg)](https://github.com/gilmry/BANKO/actions/workflows/ci.yml)
[![Security](https://github.com/gilmry/BANKO/actions/workflows/security.yml/badge.svg)](https://github.com/gilmry/BANKO/actions/workflows/security.yml)

---

## Vision

BANKO est un système bancaire open source sous licence **AGPL-3.0**, conçu pour les banques tunisiennes. Son objectif : fournir un socle technologique **irréfutable, transparent et légal** qui implémente l'ensemble des normes réglementaires tunisiennes et internationales.

BANKO est auditable par la **Banque Centrale de Tunisie (BCT)**, le **gouvernement** et tous les acteurs de la sécurité du monde bancaire. Chaque fonctionnalité est traçable vers un texte légal ou normatif (voir [Référentiel Légal](docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md)).

## Principes fondateurs

1. **Conformité d'abord** — Aucune fonctionnalité sans référence légale traçable
2. **Transparence totale** — Code source ouvert, piste d'audit intégrale, reporting automatisé
3. **Auditabilité** — Chaque opération est journalisée, horodatée et signée cryptographiquement
4. **Sécurité par conception** — Chiffrement, contrôle d'accès strict, tests de sécurité continus
5. **Souveraineté** — Hébergement local, données en Tunisie, conformité INPDP

## Conformité réglementaire

BANKO implémente les exigences de :

- **Loi n° 2016-48** — Loi bancaire tunisienne
- **Circulaire BCT 91-24** — Division et couverture des risques
- **Circulaire BCT 2018-06** — Adéquation des fonds propres (Bâle III)
- **Circulaire BCT 2018-10** — Ratio Crédits/Dépôts
- **Circulaire BCT 2021-05** — Gouvernance bancaire
- **Circulaire BCT 2025-17** — LBC/FT/FP (KYC, surveillance, gel des avoirs)
- **Loi organique 2015-26 / 2019-9** — Lutte anti-blanchiment
- **Loi 2004-63** — Protection des données personnelles
- **NCT 21/22/24/25** — Normes comptables bancaires tunisiennes
- **Standards Bâle III**, **40 Recommandations GAFI/FATF**, **ISO 20022/27001**

Voir le [Référentiel Légal et Normatif complet](docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md).

## Architecture

Architecture **hexagonale** (Ports & Adapters) + **Domain-Driven Design** (DDD) :

```
┌─────────────────────────────────────────────────────┐
│         Infrastructure (Adapters)                   │
│  API REST │ PostgreSQL │ HSM │ SWIFT │ BCT Gateway  │
└───────────────┬─────────────────────────────────────┘
                │ implements
┌───────────────▼─────────────────────────────────────┐
│      Application (Use Cases + Ports)                │
│  Use Cases │ DTOs │ Repository Traits │ Commands    │
└───────────┬─────────────────────────────────────────┘
            │ uses
┌───────────▼─────────────────────────────────────────┐
│         Domain (Core Logic)                         │
│  Entités bancaires │ Règles métier │ Calculs régl.  │
└─────────────────────────────────────────────────────┘
```

## Stack technique

| Couche | Technologie | Justification |
|---|---|---|
| Backend | **Rust** + Actix-web | Performance, sécurité mémoire, fiabilité |
| Frontend | **Astro** + **Svelte** | Islands architecture, performance |
| Base de données | **PostgreSQL** | ACID, audit, extensions crypto |
| Infrastructure | Docker, Traefik, Prometheus/Grafana | Observabilité, déploiement reproductible |
| Sécurité | LUKS, HSM, CrowdSec, Suricata | Chiffrement, détection d'intrusion |

## Modules

| Module | Description | Priorité |
|---|---|---|
| Core Banking | Comptes, dépôts, soldes | P0 |
| Crédits | Octroi, suivi, classification créances | P0 |
| Calcul prudentiel | Ratios réglementaires (solvabilité, Tier 1, C/D) | P0 |
| KYC/AML | Identification clients, filtrage, surveillance | P0 |
| Gouvernance | Contrôle interne, comités, 3 lignes de défense | P0 |
| Comptabilité | Plan comptable bancaire, NCT, pré-IFRS 9 | P0 |
| Reporting BCT | États réglementaires, reporting automatisé | P1 |
| Change | Opérations devises, conformité change | P1 |
| Paiements | Virements, SWIFT, ISO 20022 | P1 |
| Protection données | Privacy-by-design, INPDP compliance | P1 |

## Roadmap (jalons capacitaires)

Comme Koprogo, BANKO utilise une **roadmap capacitaire** : on livre quand c'est prêt, pas selon des dates arbitraires.

- **Jalon 0** 🏗️ : Fondations — Architecture, référentiel légal, domain model, CI/CD
- **Jalon 1** 🔒 : Core banking + sécurité — Comptes, dépôts, KYC de base, chiffrement
- **Jalon 2** 📋 : Conformité prudentielle — Ratios BCT, classification créances, provisionnement
- **Jalon 3** 🎯 : LBC/FT complet — Surveillance transactionnelle, déclarations de soupçon
- **Jalon 4** 📊 : Reporting et comptabilité — États BCT, plan comptable, pré-IFRS 9
- **Jalon 5** 💱 : Paiements et change — SWIFT, virements, ISO 20022, conformité change
- **Jalon 6** 🔍 : Audit et transparence — Portail d'audit BCT, dashboards superviseurs

## Démarrage rapide

```bash
# Cloner le repo
git clone https://github.com/gilmry/banko.git
cd banko

# Setup
make setup

# Lancer l'environnement de développement
make dev

# Tests
make test
```

## Contribuer

Voir [CONTRIBUTING.md](CONTRIBUTING.md). Toutes les contributions doivent être signées (DCO).

## Gouvernance

Voir [GOVERNANCE.md](GOVERNANCE.md).

## Sécurité

Voir [SECURITY.md](SECURITY.md). Divulgation responsable : abuse@koprogo.com

## Licence

BANKO est distribué sous licence **AGPL-3.0** — les modifications doivent rester open source, même en déploiement SaaS. Voir [LICENSE](LICENSE).
