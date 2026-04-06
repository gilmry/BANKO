# BANKO — Plan d'Implémentation ISO/IEC 27001:2022

> **Version** : 1.0.0 — 6 avril 2026
> **Statut** : Document initial
> **Classification** : Confidentiel — Usage interne et auditeurs
> **Licence** : AGPL-3.0
> **Auteur** : Projet BANKO
> **Norme de référence** : ISO/IEC 27001:2022 (seule édition en vigueur depuis octobre 2025)
> **Horizon** : 18 mois (avril 2026 — septembre 2027)

---

## Table des matières

1. [Vision et objectifs](#1-vision-et-objectifs)
2. [État des lieux en Tunisie](#2-état-des-lieux-en-tunisie)
3. [Roadmap 18 mois](#3-roadmap-18-mois)
4. [Ressources nécessaires](#4-ressources-nécessaires)
5. [KPIs de suivi](#5-kpis-de-suivi)
6. [Synergie avec les autres normes](#6-synergie-avec-les-autres-normes)
7. [Risques du projet de certification](#7-risques-du-projet-de-certification)

---

## 1. Vision et objectifs

### 1.1 Vision

BANKO ambitionne de devenir la première plateforme bancaire open source certifiée ISO/IEC 27001:2022 en Tunisie. Cette certification vise à :

- **Démontrer l'engagement** de BANKO envers la sécurité de l'information au niveau le plus élevé reconnu internationalement
- **Faciliter l'adoption** par les banques tunisiennes en offrant un cadre de sécurité préconfiguré et auditable
- **Renforcer la confiance** des régulateurs (BCT, CTAF, INPDP) et des partenaires internationaux
- **Contribuer à la conformité** des banques tunisiennes en vue de l'évaluation mutuelle du GAFI prévue en 2026-2027
- **Positionner la Tunisie** comme acteur crédible de l'innovation bancaire sécurisée en Afrique du Nord

### 1.2 Objectifs stratégiques

| N° | Objectif | Indicateur cible | Échéance |
|---|---|---|---|
| O-1 | Obtenir la certification ISO 27001:2022 pour le périmètre SMSI de BANKO | Certificat délivré par un organisme accrédité | M15 (juillet 2027) |
| O-2 | Implémenter 100 % des 92 contrôles applicables de l'Annexe A | Taux de conformité des contrôles = 100 % | M12 (avril 2027) |
| O-3 | Atteindre un niveau de risque résiduel acceptable (aucun risque critique non traité) | 0 risque critique, < 5 risques élevés | M9 (janvier 2027) |
| O-4 | Garantir la conformité à la loi tunisienne sur la protection des données personnelles 2025 | Registre des traitements complet, DPO désigné, DPIA réalisées | M3 (juillet 2026) |
| O-5 | Satisfaire les exigences de la circulaire BCT 2025-06 pour les tests d'intrusion e-KYC | Rapport de test d'intrusion ANCS validé | M9 (janvier 2027) |

### 1.3 Périmètre de la certification

Le périmètre de la certification couvre l'intégralité de la plateforme BANKO telle que décrite dans la [Déclaration d'Applicabilité](01-scope-and-statement-of-applicability.md) :

- Les 12 bounded contexts métier
- L'infrastructure technique (Rust/Actix-web, PostgreSQL 16, Traefik, MinIO, Docker/K8s)
- Les processus de développement, de déploiement et d'exploitation
- La documentation et les processus organisationnels de sécurité

---

## 2. État des lieux en Tunisie

### 2.1 Certifications ISO 27001 dans le secteur bancaire tunisien

À la date de rédaction de ce document, seuls **deux établissements bancaires** en Tunisie disposent d'une certification ISO 27001 :

| Établissement | Année de certification | Périmètre | Organisme certificateur | Statut |
|---|---|---|---|---|
| **Arab Tunisian Bank (ATB)** | 2017 (certification initiale) | Système d'information global de la banque | Bureau Veritas | Renouvelée (dernière audit de surveillance conforme) |
| **Union Internationale de Banques (UIB)** | 2022 | Périmètre SGSS (Société Générale Securities Services) — activités de conservation et d'administration de titres | Bureau Veritas | Active |

### 2.2 Analyse du contexte

| Facteur | Analyse |
|---|---|
| **Maturité du marché** | Le secteur bancaire tunisien compte environ 23 banques résidentes. Moins de 10 % sont certifiées ISO 27001. Le niveau de maturité en sécurité de l'information est hétérogène. |
| **Pression réglementaire** | La BCT renforce progressivement les exigences de sécurité (circulaires 2006-19, 2021-05, 2025-06). La certification ISO 27001 n'est pas obligatoire mais constitue un gage de conformité reconnu. |
| **Contexte GAFI** | L'évaluation mutuelle du GAFI/MENAFATF prévue en 2026-2027 pousse les banques à renforcer leurs dispositifs de contrôle interne et de sécurité. |
| **Loi données personnelles 2025** | L'entrée en application de la loi en juillet 2026 crée une urgence pour les banques de mettre en conformité leurs systèmes de traitement de données. ISO 27001 + ISO 27701 constituent le cadre de référence. |
| **Opportunité pour BANKO** | En tant que plateforme open source, BANKO peut offrir un système bancaire « certifiable par défaut » (Security by Design), réduisant considérablement le coût et le délai de certification pour les banques adoptantes. |

### 2.3 Avantages concurrentiels de BANKO

| Avantage | Détail |
|---|---|
| **Rust — Sécurité mémoire native** | Élimine structurellement les vulnérabilités de type buffer overflow, use-after-free et data races, couvrant une part significative des CVE critiques. |
| **SQLx — Requêtes typées** | Rend l'injection SQL structurellement impossible à la compilation. |
| **Architecture hexagonale** | L'isolation du domaine métier facilite l'implémentation des contrôles de sécurité sans impacter la logique métier. |
| **Open source (AGPL-3.0)** | La transparence du code permet un audit indépendant par quiconque, renforçant la confiance des régulateurs et des auditeurs. |
| **Conformité intégrée** | Le référentiel légal et normatif est intégré dès la conception, avec traçabilité norme-module. |

---

## 3. Roadmap 18 mois

### 3.1 Vue d'ensemble

```
M1   M2   M3   M4   M5   M6   M7   M8   M9   M10  M11  M12  M13  M14  M15  M16  M17  M18
├────────────┤ ├──────────────────────────────┤ ├────────────────┤ ├────────────────┤ ├────────────┤
   Phase 1            Phase 2                      Phase 3            Phase 4            Phase 5
   Gap Analysis       Implémentation              Audit interne      Certification      Surveillance
   Avr-Juin 2026      Jul-Déc 2026                Jan-Mar 2027       Avr-Jun 2027       Jul-Sep 2027
```

### 3.2 Phase 1 — Analyse d'écart et fondations (M1-M3 : avril — juin 2026)

| Activité | Description | Livrables | Responsable | Mois |
|---|---|---|---|---|
| 1.1 | **Gap analysis ISO 27001:2022** — Évaluation de l'écart entre l'état actuel de BANKO et les exigences de la norme | Rapport de gap analysis, liste des non-conformités | RSSI (à désigner) | M1 |
| 1.2 | **Définition du périmètre du SMSI** — Formalisation du périmètre, identification des actifs, des parties intéressées | SoA validée (cf. [01-scope-and-statement-of-applicability.md](01-scope-and-statement-of-applicability.md)) | RSSI | M1 |
| 1.3 | **Politique de sécurité de l'information** — Rédaction et approbation de la politique SMSI par la direction | Politique SMSI signée | Direction, RSSI | M1 |
| 1.4 | **Évaluation des risques initiale** — Identification et évaluation des risques, définition des plans de traitement | Registre des risques (cf. [02-risk-assessment-register.md](02-risk-assessment-register.md)) | RSSI | M2 |
| 1.5 | **Désignation du DPO** — Conformément à la loi données personnelles 2025 (application juillet 2026) | Lettre de mission du DPO, notification INPDP | Direction | M2 |
| 1.6 | **Formation de l'équipe** — Sensibilisation ISO 27001:2022 pour toute l'équipe, formation approfondie pour le RSSI et les auditeurs internes | Attestations de formation | RSSI | M2-M3 |
| 1.7 | **Registre des traitements de données personnelles** — Inventaire des traitements par bounded context | Registre des traitements conforme loi 2025 | DPO | M3 |
| 1.8 | **DPIA pour les traitements à haut risque** — Customer (KYC, biométrie), Identity (authentification) | Rapports DPIA validés | DPO | M3 |
| 1.9 | **Sélection de l'organisme de certification** — Choix d'un organisme accrédité (Bureau Veritas, BSI, SGS, AFNOR) | Contrat de certification signé | Direction | M3 |

### 3.3 Phase 2 — Implémentation des contrôles (M4-M9 : juillet — décembre 2026)

| Activité | Description | Livrables | Responsable | Mois |
|---|---|---|---|---|
| 2.1 | **Implémentation des contrôles organisationnels prioritaires** — A.5.1 à A.5.6 (politiques, rôles, séparation des tâches) | Politiques documentées, matrice RACI, procédures | RSSI | M4 |
| 2.2 | **Module Identity — Contrôles d'accès** — A.5.15 à A.5.18, A.8.2, A.8.5 (RBAC, MFA, gestion des accès privilégiés) | Module Identity avec RBAC, MFA, journalisation | Équipe développement | M4-M5 |
| 2.3 | **Module Governance — Piste d'audit** — A.8.15, A.8.16, A.8.17 (journalisation, surveillance, synchronisation) | Journaux d'audit immuables, monitoring Prometheus | Équipe développement | M5-M6 |
| 2.4 | **Chiffrement et protection des données** — A.8.24, A.8.11, A.8.10 (cryptographie, masquage, suppression) | Chiffrement AES-256 at rest, TLS 1.3, masquage des données de test | Équipe développement | M5-M6 |
| 2.5 | **Module AML et Sanctions** — A.5.7, A.5.31 (threat intelligence, conformité réglementaire) | Modules AML et Sanctions conformes BCT 2025-17 | Équipe développement | M6-M7 |
| 2.6 | **Infrastructure sécurisée** — A.8.20 à A.8.23 (sécurité réseau, mTLS, filtrage web) | K8s Network Policies, mTLS, WAF Traefik | Équipe infrastructure | M6-M7 |
| 2.7 | **Sauvegarde et continuité** — A.8.13, A.8.14, A.5.30 (sauvegardes, redondance, PCA/PRA) | Stratégie 3-2-1, PostgreSQL HA, PCA/PRA documentés | Équipe infrastructure | M7-M8 |
| 2.8 | **Gestion des fournisseurs** — A.5.19 à A.5.23 (sécurité fournisseurs, cloud) | Clauses contractuelles, politique cloud, audit fournisseurs | RSSI | M8 |
| 2.9 | **Gestion des incidents** — A.5.24 à A.5.28 (planification, réponse, enseignements) | Procédure de gestion des incidents, playbooks, template notification 72h | RSSI | M8-M9 |
| 2.10 | **Tests d'intrusion e-KYC (ANCS)** — A.8.29 (circulaire BCT 2025-06) | Rapport de test d'intrusion par prestataire accrédité ANCS | RSSI, prestataire ANCS | M9 |
| 2.11 | **Documentation complète du SMSI** — Ensemble des politiques, procédures et enregistrements | Corpus documentaire SMSI complet | RSSI | M9 |

### 3.4 Phase 3 — Audit interne et actions correctives (M10-M12 : janvier — mars 2027)

| Activité | Description | Livrables | Responsable | Mois |
|---|---|---|---|---|
| 3.1 | **Audit interne du SMSI** — Audit complet par un auditeur interne formé (ou prestataire externe) selon ISO 19011 | Rapport d'audit interne, liste des non-conformités | Auditeur interne | M10-M11 |
| 3.2 | **Revue de direction** — Présentation des résultats de l'audit interne à la direction, décisions sur les actions correctives | PV de revue de direction signé | Direction, RSSI | M11 |
| 3.3 | **Actions correctives** — Traitement de toutes les non-conformités majeures et mineures identifiées | Plan d'actions correctives, preuves de clôture | RSSI, équipes concernées | M11-M12 |
| 3.4 | **Revue des risques** — Mise à jour du registre des risques après implémentation des contrôles | Registre des risques mis à jour, risques résiduels évalués | RSSI | M12 |
| 3.5 | **Tests de continuité** — Exercice de PCA/PRA grandeur nature | Rapport d'exercice, temps de restauration mesurés | Équipe infrastructure | M12 |
| 3.6 | **Pré-audit de certification (optionnel)** — Audit blanc par l'organisme de certification ou un consultant externe | Rapport de pré-audit, recommandations | Consultant externe | M12 |

### 3.5 Phase 4 — Audit de certification (M13-M15 : avril — juin 2027)

| Activité | Description | Livrables | Responsable | Mois |
|---|---|---|---|---|
| 4.1 | **Audit de certification — Stage 1 (revue documentaire)** — L'organisme de certification vérifie la documentation du SMSI, la SoA, les politiques et procédures | Rapport Stage 1, confirmation de la date du Stage 2 | Organisme certificateur | M13 |
| 4.2 | **Traitement des observations du Stage 1** — Correction des écarts documentaires identifiés | Documents corrigés | RSSI | M13-M14 |
| 4.3 | **Audit de certification — Stage 2 (audit sur site)** — Vérification de l'implémentation effective des contrôles, entretiens, observation des pratiques | Rapport Stage 2, liste des non-conformités | Organisme certificateur | M14-M15 |
| 4.4 | **Traitement des non-conformités** — Correction des non-conformités dans le délai imparti (généralement 90 jours pour les majeures) | Preuves de correction | RSSI, équipes concernées | M15 |
| 4.5 | **Décision de certification** — L'organisme certificateur émet le certificat ISO 27001:2022 | **Certificat ISO 27001:2022** | Organisme certificateur | M15 |

### 3.6 Phase 5 — Surveillance et amélioration continue (M16-M18 : juillet — septembre 2027)

| Activité | Description | Livrables | Responsable | Mois |
|---|---|---|---|---|
| 5.1 | **Mise en place du cycle d'amélioration continue (PDCA)** — Processus formel Plan-Do-Check-Act | Procédure d'amélioration continue documentée | RSSI | M16 |
| 5.2 | **Audit de surveillance annuel (préparation)** — Préparation du premier audit de surveillance (12 mois après certification) | Dossier de surveillance | RSSI | M16-M17 |
| 5.3 | **Veille réglementaire continue** — Suivi des évolutions normatives et réglementaires | Bulletins de veille, mises à jour du référentiel légal | Responsable conformité | Continu |
| 5.4 | **Revue trimestrielle des risques** — Mise à jour du registre des risques | Registre des risques actualisé | RSSI | M18 |
| 5.5 | **Intégration ISO 27701:2025** — Évaluation de l'opportunité d'une certification complémentaire vie privée | Étude de faisabilité ISO 27701 | DPO | M17-M18 |
| 5.6 | **Bilan du projet de certification** — Retour d'expérience global, enseignements, ajustements | Rapport de bilan | Direction, RSSI | M18 |

---

## 4. Ressources nécessaires

### 4.1 Ressources humaines

| Rôle | Profil | Mode d'engagement | Période |
|---|---|---|---|
| **Responsable de la Sécurité des Systèmes d'Information (RSSI)** | Expérience en sécurité de l'information, connaissance ISO 27001, secteur bancaire souhaité | Temps plein | M1-M18 (permanent) |
| **Délégué à la Protection des Données (DPO)** | Expertise en protection des données, connaissance de la loi tunisienne 2025, idéalement certifié CIPP/E ou équivalent | Temps partiel (50 %) ou mutualisé | M2-M18 (permanent) |
| **Auditeur interne ISO 27001** | Formé ISO 27001 Lead Auditor, indépendant des activités auditées | Temps partiel ou prestation externe | M10-M12, puis annuel |
| **Équipe sécurité opérationnelle** | 2 ingénieurs sécurité (SOC, gestion des vulnérabilités, réponse aux incidents) | Temps plein | M4-M18 (permanent) |
| **Développeurs sécurité** | Développeurs Rust/Svelte avec compétences en sécurité applicative (OWASP) | Existants (formation complémentaire) | M1-M18 |
| **Ingénieur infrastructure sécurité** | Expertise K8s, réseau, chiffrement, haute disponibilité | Temps plein | M4-M18 (permanent) |
| **Consultant ISO 27001 (accompagnement)** | Consultant certifié ISO 27001 Lead Implementer, expérience secteur bancaire | Prestation externe (100 jours) | M1-M12 |

### 4.2 Budget estimatif

| Poste de dépenses | Estimation (TND) | Estimation (EUR) | Période |
|---|---|---|---|
| **Ressources humaines (RSSI, DPO, équipe sécurité)** | 360 000 — 540 000 | 105 000 — 160 000 | 18 mois |
| **Consultant ISO 27001 (accompagnement)** | 80 000 — 120 000 | 24 000 — 35 000 | 12 mois |
| **Formation et sensibilisation** | 25 000 — 40 000 | 7 500 — 12 000 | 18 mois |
| **Outils de sécurité (SIEM, scanner, DLP)** | 40 000 — 80 000 | 12 000 — 24 000 | Annuel |
| **Tests d'intrusion (prestataire ANCS)** | 30 000 — 50 000 | 9 000 — 15 000 | Par campagne |
| **Audit de certification (Stage 1 + Stage 2)** | 50 000 — 80 000 | 15 000 — 24 000 | Ponctuel |
| **Infrastructure sécurité (HSM, WAF, backup)** | 60 000 — 100 000 | 18 000 — 30 000 | Investissement + annuel |
| **Pré-audit / audit blanc (optionnel)** | 15 000 — 25 000 | 4 500 — 7 500 | Ponctuel |
| **Total estimé** | **660 000 — 1 035 000** | **195 000 — 307 500** | **18 mois** |

> **Note** : Ces estimations sont indicatives et varient selon la taille de l'équipe, le choix de l'organisme de certification et le niveau de maturité initial. Pour une banque déployant BANKO, le coût serait significativement réduit grâce aux contrôles de sécurité intégrés dans la plateforme.

### 4.3 Outils et technologies

| Catégorie | Outil recommandé | Usage | Priorité |
|---|---|---|---|
| SIEM | Wazuh (open source) ou ELK Stack | Corrélation des événements de sécurité, détection d'anomalies | Élevée |
| Scanner de vulnérabilités | Trivy (open source) | Scan des conteneurs Docker et des dépendances | Élevée |
| Gestion des secrets | HashiCorp Vault | Stockage et rotation des credentials, clés de chiffrement | Élevée |
| Monitoring | Prometheus + Grafana | Surveillance des métriques, alertes | Élevée |
| SAST | cargo clippy + cargo audit + Semgrep | Analyse statique du code Rust | Élevée |
| DAST | OWASP ZAP | Tests dynamiques de l'API et du frontend | Moyenne |
| DLP | Open source (à sélectionner) | Prévention de la fuite de données | Moyenne |
| Gestion documentaire | Dépôt Git (docs/) | Versionning de la documentation SMSI | Élevée |
| Gestion des risques | Tableur ou outil dédié | Registre des risques, suivi des traitements | Élevée |

---

## 5. KPIs de suivi

### 5.1 Indicateurs de progression

| ID | KPI | Formule | Cible | Fréquence de mesure |
|---|---|---|---|---|
| KPI-01 | **Taux de contrôles implémentés** | (Contrôles Done + In Progress) / 92 contrôles applicables x 100 | 100 % au M12 | Mensuelle |
| KPI-02 | **Taux de contrôles validés** | Contrôles Done / 92 contrôles applicables x 100 | > 95 % au M12 | Mensuelle |
| KPI-03 | **Nombre de risques critiques non traités** | Count(risques niveau Critique sans plan de traitement actif) | 0 au M9 | Trimestrielle |
| KPI-04 | **Nombre de risques élevés non traités** | Count(risques niveau Élevé sans plan de traitement actif) | < 5 au M12 | Trimestrielle |
| KPI-05 | **Couverture documentaire** | (Documents SMSI rédigés et validés) / (Documents SMSI requis) x 100 | 100 % au M9 | Mensuelle |

### 5.2 Indicateurs de sécurité opérationnelle

| ID | KPI | Formule | Cible | Fréquence de mesure |
|---|---|---|---|---|
| KPI-06 | **Nombre d'incidents de sécurité** | Count(incidents de sécurité confirmés) par période | Tendance décroissante | Mensuelle |
| KPI-07 | **Temps moyen de détection (MTTD)** | Moyenne(délai entre occurrence et détection de l'incident) | < 1 heure | Mensuelle |
| KPI-08 | **Temps moyen de réponse (MTTR)** | Moyenne(délai entre détection et résolution de l'incident) | < 4 heures | Mensuelle |
| KPI-09 | **Temps moyen de correction des vulnérabilités critiques** | Moyenne(délai entre identification et correction d'une vulnérabilité critique) | < 24 heures | Mensuelle |
| KPI-10 | **Taux de réussite des tests de restauration** | (Tests de restauration réussis) / (Tests de restauration effectués) x 100 | 100 % | Mensuelle |

### 5.3 Indicateurs de conformité réglementaire

| ID | KPI | Formule | Cible | Fréquence de mesure |
|---|---|---|---|---|
| KPI-11 | **Conformité à la loi données personnelles 2025** | (Exigences satisfaites) / (Exigences totales) x 100 | 100 % au M3 (juillet 2026) | Mensuelle |
| KPI-12 | **Conformité aux circulaires BCT** | (Exigences BCT satisfaites) / (Exigences BCT applicables) x 100 | 100 % au M9 | Trimestrielle |
| KPI-13 | **Couverture des recommandations GAFI** | (Recommandations GAFI couvertes par BANKO) / (Recommandations GAFI applicables) x 100 | > 90 % au M12 | Trimestrielle |
| KPI-14 | **Nombre de non-conformités ouvertes** | Count(non-conformités identifiées en audit et non clôturées) | 0 au M15 | Mensuelle |
| KPI-15 | **Taux de formation sécurité** | (Personnes formées) / (Personnes devant être formées) x 100 | 100 % | Annuelle |

### 5.4 Tableau de bord synthétique

| Phase | KPIs principaux | Seuil d'alerte |
|---|---|---|
| Phase 1 (M1-M3) | KPI-05 ≥ 30 %, KPI-11 ≥ 80 %, DPO désigné | KPI-05 < 20 % |
| Phase 2 (M4-M9) | KPI-01 ≥ 70 %, KPI-02 ≥ 40 %, KPI-12 ≥ 80 % | KPI-01 < 50 % |
| Phase 3 (M10-M12) | KPI-02 ≥ 95 %, KPI-03 = 0, KPI-14 tendance décroissante | KPI-03 > 0 |
| Phase 4 (M13-M15) | KPI-14 = 0, Certification obtenue | Non-conformité majeure Stage 2 |
| Phase 5 (M16-M18) | KPI-06 tendance décroissante, KPI-07 < 1h, KPI-08 < 4h | KPI-06 en hausse |

---

## 6. Synergie avec les autres normes

### 6.1 Vue d'ensemble des synergies

La certification ISO 27001:2022 de BANKO s'inscrit dans un écosystème normatif plus large. Les synergies entre les différentes normes et réglementations permettent de mutualiser les efforts et de maximiser la couverture de conformité.

| Norme / Réglementation | Lien avec ISO 27001:2022 | Synergie pour BANKO |
|---|---|---|
| **PCI DSS 4.0** | Sécurité des données de cartes de paiement | Les contrôles A.8.24 (cryptographie), A.8.20-22 (réseau), A.8.15 (journalisation) couvrent une part significative des exigences PCI DSS. Le module Payment bénéficie directement. |
| **Loi données personnelles 2025** | Protection des données personnelles | ISO 27001 clause A.5.34 + future certification ISO 27701:2025 couvrent les exigences de la loi (DPO, DPIA, notification 72h, chiffrement). |
| **ISO 27701:2025** | Gestion de la vie privée (autonome depuis 2025) | Extension naturelle d'ISO 27001 pour la vie privée. Couvre l'IA (scoring crédit), la biométrie (e-KYC) et l'IoT. Certification complémentaire envisageable en Phase 5. |
| **Circulaire BCT 2006-19** | Contrôle interne | Les contrôles organisationnels (A.5.1-A.5.6) et la piste d'audit (A.8.15) répondent directement aux exigences de la circulaire. |
| **Circulaire BCT 2021-05** | Gouvernance bancaire | Les trois lignes de défense (A.5.2, A.5.3, A.5.35) correspondent au modèle exigé par la circulaire. Le module Governance implémente les comités obligatoires. |
| **Circulaire BCT 2025-06** | Tests d'intrusion e-KYC | Le contrôle A.8.29 (tests de sécurité) intègre l'exigence de tests d'intrusion par un prestataire accrédité ANCS. |
| **Circulaire BCT 2025-17** | LBC/FT/FP | Les contrôles A.5.7 (threat intelligence), A.5.31 (conformité légale) et A.8.15 (journalisation) soutiennent la conformité LBC/FT. Les modules AML et Sanctions implémentent les exigences spécifiques. |
| **ISO 22301:2019** | Continuité d'activité | Le contrôle A.5.30 (continuité TIC) est directement aligné. Le PCA/PRA de BANKO peut servir de base à une certification ISO 22301. |
| **ISO 31000:2018** | Management du risque | La méthodologie d'évaluation des risques du SMSI (cf. [02-risk-assessment-register.md](02-risk-assessment-register.md)) est alignée sur ISO 31000. |
| **Recommandations GAFI** | 40 recommandations LBC/FT | ISO 27001 renforce la crédibilité du dispositif de sécurité de l'information dans le contexte de l'évaluation mutuelle du GAFI. |

### 6.2 Matrice de correspondance contrôles ISO 27001 / autres normes

| Contrôle ISO 27001 | PCI DSS 4.0 | Loi données 2025 | BCT 2006-19 | BCT 2021-05 | BCT 2025-17 |
|---|---|---|---|---|---|
| A.5.1 (Politiques) | Req. 12.1 | — | Art. 2 | Art. 5 | Art. 3 |
| A.5.3 (Séparation tâches) | Req. 7.1 | — | Art. 4 | Art. 12 | — |
| A.5.15 (Contrôle d'accès) | Req. 7, 8 | Art. 32 | Art. 6 | — | Art. 8 |
| A.5.34 (Vie privée) | — | Art. 5-42 | — | — | — |
| A.8.15 (Journalisation) | Req. 10 | Art. 35 | Art. 7 | Art. 15 | Art. 12 |
| A.8.24 (Cryptographie) | Req. 3, 4 | Art. 33 | — | — | Art. 9 |
| A.8.28 (Codage sécurisé) | Req. 6 | — | — | — | — |
| A.8.29 (Tests sécurité) | Req. 11 | — | — | — | Art. 14 |

---

## 7. Risques du projet de certification

### 7.1 Registre des risques du projet

| ID | Risque | Probabilité | Impact | Niveau | Mesure d'atténuation |
|---|---|---|---|---|---|
| RP-01 | **Manque de ressources humaines qualifiées** — Difficulté à recruter un RSSI et une équipe sécurité expérimentés dans le contexte tunisien | Élevée | Élevé | **Critique** | Recours à un consultant externe en parallèle du recrutement ; formation accélérée des développeurs existants ; partenariat avec des universités tunisiennes |
| RP-02 | **Dépassement du budget** — Les coûts réels dépassent les estimations initiales (outils, prestataires, tests d'intrusion) | Moyenne | Moyen | **Moyen** | Marge de contingence de 15 % ; priorisation des dépenses ; utilisation maximale d'outils open source (Wazuh, Trivy, Prometheus) |
| RP-03 | **Retard dans le calendrier** — Les phases s'enchaînent avec du retard, repoussant la certification au-delà de M15 | Élevée | Moyen | **Élevé** | Jalons intermédiaires avec revue Go/No-Go ; identification précoce des retards ; flexibilité de l'organisme de certification |
| RP-04 | **Non-conformités majeures lors du Stage 2** — L'auditeur identifie des non-conformités majeures nécessitant des corrections significatives | Moyenne | Élevé | **Élevé** | Pré-audit (audit blanc) en M12 ; audit interne rigoureux ; traitement préventif des non-conformités |
| RP-05 | **Évolution réglementaire en cours de projet** — Nouvelle circulaire BCT ou modification de la loi données personnelles pendant le projet | Moyenne | Moyen | **Moyen** | Veille réglementaire continue ; flexibilité du périmètre du SMSI ; marge dans la documentation pour intégrer des évolutions |
| RP-06 | **Résistance au changement** — L'équipe de développement perçoit les exigences ISO 27001 comme une contrainte excessive | Moyenne | Moyen | **Moyen** | Sensibilisation aux bénéfices de la certification ; intégration des contrôles de sécurité dans le workflow de développement existant (CI/CD) ; automatisation maximale |
| RP-07 | **Dépendance vis-à-vis d'un prestataire ANCS** — Indisponibilité d'un prestataire accrédité ANCS pour les tests d'intrusion e-KYC dans les délais | Moyenne | Élevé | **Élevé** | Identification précoce des prestataires accrédités ; contractualisation dès M3 ; planification des tests en M9 avec une marge de 2 mois |
| RP-08 | **Complexité de la loi données personnelles 2025** — Interprétation incertaine de certaines dispositions de la nouvelle loi avant l'entrée en application (juillet 2026) | Moyenne | Moyen | **Moyen** | Veille sur les décrets d'application ; consultation de l'INPDP ; adoption d'une approche prudente (conformité maximale) |

### 7.2 Stratégie de gestion des risques du projet

| Stratégie | Application |
|---|---|
| **Évitement** | Planification anticipée des activités critiques (recrutement RSSI dès M1, contractualisation ANCS dès M3) |
| **Réduction** | Pré-audit en M12 pour réduire le risque de non-conformité en Stage 2 ; formation continue ; automatisation des contrôles |
| **Transfert** | Externalisation partielle (consultant ISO 27001, tests d'intrusion) ; assurance cyber-risques |
| **Acceptation** | Risques résiduels faibles acceptés avec surveillance (évolution réglementaire, dépassement budgétaire modéré) |

### 7.3 Facteurs clés de succès

| N° | Facteur | Importance |
|---|---|---|
| 1 | **Engagement de la direction** — Soutien actif et visible de la direction générale, allocation des ressources | Critique |
| 2 | **Compétence du RSSI** — Désignation d'un RSSI expérimenté, capable de piloter le projet de bout en bout | Critique |
| 3 | **Intégration dans le développement** — Les contrôles de sécurité doivent être intégrés dans le workflow de développement, pas imposés en surcouche | Élevée |
| 4 | **Automatisation** — Maximiser l'automatisation des contrôles (CI/CD, monitoring, alertes) pour réduire la charge opérationnelle | Élevée |
| 5 | **Documentation continue** — Maintenir la documentation à jour en continu, pas en mode « rush » avant l'audit | Élevée |
| 6 | **Culture de sécurité** — Développer une culture de sécurité au sein de l'équipe, au-delà de la simple conformité | Moyenne |
| 7 | **Transparence open source** — Exploiter la transparence du code source comme un atout auprès des auditeurs et des régulateurs | Moyenne |

---

> **Documents associés** :
> - [01-scope-and-statement-of-applicability.md](01-scope-and-statement-of-applicability.md) — Périmètre du SMSI et Déclaration d'Applicabilité
> - [02-risk-assessment-register.md](02-risk-assessment-register.md) — Registre d'évaluation des risques
> - [03-controls-annex-a-mapping.md](03-controls-annex-a-mapping.md) — Correspondance des contrôles Annexe A
> - [Référentiel légal et normatif](../../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) — Cadre réglementaire tunisien
>
> **Prochaine revue prévue** : Juillet 2026 (fin de la Phase 1)
>
> **Approbation** : Ce plan doit être approuvé par la direction générale de l'organisme déployant BANKO avant lancement de la Phase 1.
