# Matrice des Responsabilités PCI DSS -- Plateforme BANKO

| Propriété        | Valeur                                           |
|------------------|--------------------------------------------------|
| **Version**      | 1.0.0                                            |
| **Date**         | 6 avril 2026                                     |
| **Référentiel**  | PCI DSS v4.0.1 (juin 2024)                       |
| **Classification**| Confidentiel -- Usage interne et auditeurs       |
| **Auteur**       | Équipe Sécurité BANKO                            |
| **Approbateur**  | RSSI / Comité Sécurité                           |
| **Prochaine revue** | 6 avril 2027                                  |

---

## Table des matières

1. [Modèle de responsabilité partagée](#1-modèle-de-responsabilité-partagée)
2. [Matrice RACI complète](#2-matrice-raci-complète)
3. [Responsabilités spécifiques BANKO](#3-responsabilités-spécifiques-banko)
4. [Responsabilités de la banque déployant BANKO](#4-responsabilités-de-la-banque-déployant-banko)
5. [Contrat de sous-traitance](#5-contrat-de-sous-traitance)
6. [Validation de conformité](#6-validation-de-conformité)

---

## 1. Modèle de responsabilité partagée

### 1.1 Principe fondamental

La conformité PCI DSS est une **responsabilité partagée** entre les différentes parties prenantes. BANKO étant un logiciel open source de core banking, la conformité finale dépend de l'ensemble de la chaîne : l'éditeur du logiciel (projet BANKO), la banque qui déploie le logiciel, le processeur de paiement (PSP) et l'hébergeur de l'infrastructure.

**Aucune partie ne peut, à elle seule, assurer la conformité PCI DSS complète.**

### 1.2 Parties prenantes

| Partie prenante | Rôle | Périmètre de responsabilité |
|-----------------|------|----------------------------|
| **BANKO (éditeur)** | Développement et maintenance du logiciel open source | Sécurité du code source, configurations par défaut sécurisées, documentation, correctifs de vulnérabilités |
| **Banque cliente** | Déploiement, exploitation et administration de BANKO en production | Configuration réseau, gestion des accès, monitoring, formation, tests de pénétration, validation de conformité |
| **PSP tiers** | Traitement des transactions par carte (acquisition, autorisation, compensation) | Sécurité du traitement carte, tokenisation côté PSP, conformité PCI DSS Level 1 du PSP |
| **Hébergeur cloud / datacenter** | Infrastructure physique et/ou virtuelle | Sécurité physique, réseau de base, hyperviseur, stockage physique |

### 1.3 Modèle de responsabilité par couche

```
    ┌─────────────────────────────────────────────────────────┐
    │                    APPLICATION                           │
    │  Code source, logique métier, configurations applicatives│
    │  Responsabilité : BANKO (éditeur) + Banque cliente      │
    ├─────────────────────────────────────────────────────────┤
    │                    DONNÉES                               │
    │  Chiffrement, tokenisation, gestion des clés, rétention │
    │  Responsabilité : BANKO (éditeur) + Banque cliente      │
    ├─────────────────────────────────────────────────────────┤
    │                    PLATEFORME                             │
    │  OS, conteneurs, K8s, bases de données, middleware      │
    │  Responsabilité : Banque cliente + Hébergeur            │
    ├─────────────────────────────────────────────────────────┤
    │                    RÉSEAU                                │
    │  Pare-feu, segmentation, TLS, VPN                       │
    │  Responsabilité : Banque cliente + Hébergeur            │
    ├─────────────────────────────────────────────────────────┤
    │                    INFRASTRUCTURE PHYSIQUE                │
    │  Datacenter, alimentation, climatisation, accès physique│
    │  Responsabilité : Hébergeur                             │
    └─────────────────────────────────────────────────────────┘
```

### 1.4 Légende RACI

| Code | Signification | Définition |
|------|---------------|------------|
| **R** | Responsible (Réalisateur) | Exécute l'activité, réalise le travail |
| **A** | Accountable (Approbateur) | Responsable final, rend des comptes sur le résultat |
| **C** | Consulted (Consulté) | Fournit une expertise ou des informations nécessaires |
| **I** | Informed (Informé) | Tenu informé de l'avancement et des résultats |

**Règle** : Pour chaque activité, il y a **exactement un A** (Accountable). Il peut y avoir plusieurs R, C et I.

---

## 2. Matrice RACI complète

### 2.1 Exigence 1 -- Contrôles de sécurité réseau

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 1.1.1 | Documentation des contrôles réseau | C | **A/R** | I | C |
| 1.1.2 | Revue des règles réseau (semestrielle) | I | **A/R** | I | C |
| 1.2.1 | Restriction trafic entrant/sortant | C | **A/R** | I | R |
| 1.2.5 | Contrôle services/protocoles/ports | **R** (config par défaut) | **A/R** | I | R |
| 1.2.8 | Sécurisation des fichiers de configuration réseau | **R** (templates) | **A/R** | I | C |
| 1.3.1 | Restriction trafic entrant vers le CDE | **R** (Network Policies) | **A/R** | I | R |
| 1.3.2 | Restriction trafic sortant du CDE | **R** (Network Policies) | **A/R** | I | R |
| 1.4.1 | NSC entre réseaux fiables/non fiables | **R** (architecture) | **A/R** | I | R |
| ⚠️ 1.4.2 | Contrôle trafic entrant depuis réseaux non fiables | **R** (WAF config) | **A/R** | I | R |

### 2.2 Exigence 2 -- Configurations sécurisées

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 2.1.1 | Documentation des configurations sécurisées | **R** | **A** | I | C |
| 2.2.1 | Normes de configuration par type de composant | **R** (defaults) | **A/R** (adaptation) | I | R |
| 2.2.2 | Gestion des comptes par défaut | **R** (pas de defaults) | **A/R** | I | R |
| 2.2.3 | Isolation des fonctions principales | **R** (architecture BC) | **A/R** | I | C |
| 2.2.4 | Désactivation services inutiles | **R** (images minimales) | **A/R** | I | R |
| 2.2.5 | Sécurisation des services non sécurisés | **R** (TLS par défaut) | **A/R** | I | R |
| 2.2.6 | Configuration des paramètres de sécurité | **R** (headers HTTP) | **A/R** | I | C |
| 2.2.7 | Chiffrement des accès administrateur | C | **A/R** | I | R |

### 2.3 Exigence 3 -- Protection des données stockées

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 3.1.1 | Documentation de la protection des données | **R** | **A** | C | I |
| 3.2.1 | Minimisation de la rétention des données | **R** (politiques) | **A/R** (paramétrage) | C | I |
| 3.3.1 | SAD non conservé après autorisation | **A/R** | R (vérification) | **R** | I |
| 3.4.1 | Masquage du PAN à l'affichage | **A/R** | R (vérification) | C | I |
| 3.5.1 | PAN rendu illisible (tokenisation/chiffrement) | **A/R** | R (déploiement) | C | I |
| ⚠️ 3.5.1.2 | Chiffrement au niveau champ (obligatoire) | **A/R** | R (déploiement clés) | I | C |
| 3.6.1 | Procédures de gestion des clés | **R** (mécanismes) | **A/R** (opérations) | I | C |
| 3.7.1-9 | Cycle de vie complet des clés | C | **A/R** | I | C |

### 2.4 Exigence 4 -- Chiffrement en transit

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 4.1.1 | Documentation du chiffrement en transit | **R** | **A** | C | C |
| 4.2.1 | Cryptographie forte pour la transmission CHD | **R** (TLS 1.3 config) | **A/R** (certificats) | **R** | R |
| 4.2.1.1 | Certificats de confiance utilisés | C | **A/R** | R | R |
| 4.2.1.2 | Versions de protocole autorisées | **R** (config Traefik) | **A/R** | R | R |

### 2.5 Exigence 5 -- Protection contre les logiciels malveillants

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 5.1.1 | Documentation anti-malware | C | **A/R** | I | C |
| 5.2.1 | Déploiement anti-malware sur systèmes sensibles | **R** (images scannées) | **A/R** | I | R |
| 5.2.2 | Détection temps réel ou analyses périodiques | **R** (CI/CD scans) | **A/R** | I | R |
| 5.3.1-2 | Mises à jour anti-malware | **R** (dépendances) | **A/R** | I | R |
| ⚠️ 5.4.1 | Mécanismes anti-hameçonnage | C | **A/R** | I | C |

### 2.6 Exigence 6 -- Systèmes et logiciels sécurisés

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 6.1.1 | Documentation du développement sécurisé | **A/R** | I | I | I |
| 6.2.1 | Développement sécurisé | **A/R** | I | I | I |
| 6.2.2 | Formation développeurs | **A/R** | I | I | I |
| 6.2.3 | Revues de code | **A/R** | C | I | I |
| 6.2.4 | Prévention des vulnérabilités courantes | **A/R** | R (tests) | I | I |
| 6.3.1 | Identification et correction des vulnérabilités | **A/R** | R (déploiement correctifs) | I | I |
| 6.3.2 | Inventaire des composants logiciels | **A/R** (SBOM) | R (vérification) | I | I |
| 6.3.3 | Correctifs de sécurité dans les délais | **R** (publication) | **A/R** (application) | R | R |
| ⚠️ 6.4.3 | Gestion des scripts pages de paiement | **A/R** (CSP, SRI) | R (déploiement, monitoring) | C | I |
| 6.5.1 | Séparation environnements dev/prod | **R** (config) | **A/R** | I | R |
| 6.5.2 | Données production non utilisées en dev | **R** (données synthétiques) | **A/R** | I | I |

### 2.7 Exigence 7 -- Restriction d'accès

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 7.1.1 | Documentation contrôle d'accès | **R** (modèle RBAC) | **A/R** (politiques) | I | I |
| 7.1.2 | Modèle de contrôle d'accès | **A/R** (implémentation) | R (configuration) | I | I |
| 7.2.1 | Accès basé sur le besoin d'en connaître | **R** (rôles par défaut) | **A/R** (attribution) | I | I |
| 7.2.2 | Accès attribué par rôle | **A/R** (middleware) | R (gestion des rôles) | I | I |
| 7.2.4 | Revue périodique des comptes | C | **A/R** | I | I |
| ⚠️ 7.2.5.1 | Revue des comptes applicatifs/système | **R** (outils d'audit) | **A/R** | I | C |
| 7.2.6 | Accès CHD restreint au minimum | **A/R** | R (vérification) | I | I |

### 2.8 Exigence 8 -- Identification et authentification

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 8.1.1 | Documentation identification/authentification | **R** | **A** | I | I |
| 8.2.1 | Identifiant unique par utilisateur | **A/R** | R (application) | I | I |
| 8.3.1 | Authentification forte | **A/R** (mécanismes) | R (déploiement) | I | I |
| 8.3.4 | Verrouillage après tentatives échouées | **A/R** | R (paramétrage) | I | I |
| 8.3.6 | Complexité des mots de passe | **A/R** (politique) | R (application) | I | I |
| ⚠️ 8.4.2 | MFA pour tout accès au CDE | **A/R** (implémentation) | R (déploiement, enrôlement) | I | I |
| 8.4.3 | MFA pour accès distant | C | **A/R** | I | R |
| 8.6.2 | Pas de secrets codés en dur | **A/R** | R (gestion secrets) | I | I |

### 2.9 Exigence 9 -- Accès physique

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 9.1.1 | Documentation accès physique | I | **A/R** | I | **R** |
| 9.2.1-4 | Contrôles d'accès physique | I | **A** | I | **R** |
| 9.3.1-4 | Accès physique aux médias | I | **A/R** | I | R |
| 9.4.1-7 | Protection des médias | I | **A/R** | I | R |
| 9.5.1 | Protection des terminaux POI | I | **A/R** | C | I |

### 2.10 Exigence 10 -- Journalisation et surveillance

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 10.1.1 | Documentation de la journalisation | **R** | **A** | I | C |
| 10.2.1 | Logs activés sur tous les composants CDE | **A/R** (code applicatif) | R (infrastructure) | I | R |
| 10.2.1.1 | Logs d'accès aux CHD | **A/R** | R (monitoring) | I | I |
| 10.2.2 | Contenu des logs (éléments nécessaires) | **A/R** (format) | R (vérification) | I | I |
| 10.3.1 | Logs protégés contre la modification | C | **A/R** | I | R |
| 10.3.2 | Centralisation rapide des logs | C | **A/R** | I | R |
| 10.4.1 | Revue quotidienne des logs | I | **A/R** | I | C |
| ⚠️ 10.4.2.1 | Revue automatisée des logs | **R** (règles par défaut) | **A/R** (exploitation) | I | C |
| 10.5.1 | Rétention 12 mois minimum | C | **A/R** | I | R |
| 10.6.1-2 | Synchronisation temporelle (NTP) | C | **A/R** | I | R |

### 2.11 Exigence 11 -- Tests de sécurité

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 11.1.1 | Documentation tests de sécurité | C | **A/R** | I | C |
| 11.3.1 | Scans de vulnérabilités internes (trimestriels) | **R** (CI/CD) | **A/R** (production) | I | C |
| 11.3.2 | Scans de vulnérabilités externes (ASV) | I | **A/R** | I | C |
| 11.4.1 | Tests de pénétration externes (annuels) | C | **A/R** | I | C |
| 11.4.2 | Tests de pénétration internes (annuels) | C | **A/R** | I | C |
| 11.4.5 | Tests de segmentation (annuels) | C | **A/R** | I | R |
| 11.5.1 | IDS/IPS déployé | C | **A/R** | I | R |
| ⚠️ 11.6.1 | Détection modification pages de paiement | **A/R** (mécanismes) | R (monitoring, alertes) | I | I |

### 2.12 Exigence 12 -- Politiques de sécurité

| Sous-exigence | Description | BANKO (éditeur) | Banque cliente | PSP tiers | Hébergeur |
|---------------|-------------|:---------------:|:--------------:|:---------:|:---------:|
| 12.1.1 | Politique de sécurité établie et publiée | **R** (SECURITY.md) | **A/R** (politique interne) | I | I |
| 12.1.2 | Revue annuelle de la politique | C | **A/R** | I | I |
| 12.1.3 | Rôles et responsabilités définis | **R** (ce document) | **A/R** | C | C |
| ⚠️ 12.3.1 | Analyses de risques ciblées | C | **A/R** | C | C |
| 12.3.2 | Analyse de risques technologies personnalisées | **R** (évaluation stack) | **A/R** | I | I |
| ⚠️ 12.3.3 | Revue annuelle des protocoles cryptographiques | **R** (inventaire) | **A/R** | C | C |
| 12.5.1 | Périmètre CDE documenté | **R** (doc initiale) | **A/R** (maintenance) | C | C |
| 12.6.1-2 | Programme de sensibilisation | C | **A/R** | I | I |
| 12.8.1-5 | Gestion des prestataires de services | I | **A/R** | **R** | R |
| 12.9.1 | Reconnaissance de responsabilité par les prestataires | I | **A** | **R** | R |
| 12.10.1 | Plan de réponse aux incidents | **R** (modèle) | **A/R** (adaptation) | C | R |
| 12.10.2 | Test annuel du plan de réponse | I | **A/R** | C | R |

---

## 3. Responsabilités spécifiques BANKO

### 3.1 Développement sécurisé (Secure Coding)

En tant qu'éditeur du logiciel, le projet BANKO assume les responsabilités suivantes :

| Responsabilité | Description | Mécanisme | Fréquence |
|----------------|-------------|-----------|-----------|
| **Memory safety** | Élimination des vulnérabilités mémoire grâce à Rust | Compilateur Rust (borrow checker) | Continue (à chaque compilation) |
| **Prévention injection SQL** | Utilisation exclusive de requêtes typées SQLx | Revue de code + CI/CD | Continue |
| **Validation des entrées** | Validation au niveau Domain layer (constructeurs d'entités) | Tests unitaires + revue de code | Continue |
| **OWASP Top 10** | Protection contre les 10 vulnérabilités web les plus critiques | Tests de sécurité automatisés | Trimestrielle |
| **Gestion des dépendances** | Audit des dépendances Rust (cargo audit) et JS (npm audit) | CI/CD automatisé (Dependabot) | Continue |
| **SBOM** | Génération d'un Software Bill of Materials à chaque release | Pipeline CI/CD | À chaque release |
| **Correctifs de sécurité** | Publication de correctifs dans les délais définis | SLA : critiques 72h, hauts 30j | Sur événement |

### 3.2 Configurations par défaut sécurisées

BANKO fournit des configurations par défaut sécurisées (« secure by default ») :

| Configuration | Valeur par défaut | Justification PCI DSS |
|---------------|-------------------|----------------------|
| TLS version minimale | 1.3 | Req. 4.2.1.2 |
| Headers HTTP sécurité | CSP, HSTS, X-Frame-Options, X-Content-Type-Options | Req. 6.4.1 |
| Mots de passe par défaut | Aucun -- générés aléatoirement au déploiement | Req. 2.2.2 |
| Logging | Activé par défaut en mode structuré (JSON) | Req. 10.2.1 |
| Masquage PAN | Activé, non désactivable | Req. 3.4.1 |
| Verrouillage de compte | 5 tentatives, déblocage 30 min | Req. 8.3.4 |
| Complexité mots de passe | 12 caractères minimum, mixte | Req. 8.3.6 |
| Tokenisation | Activée par défaut pour tout PAN | Req. 3.5.1 |
| Session timeout | 15 minutes d'inactivité | Req. 8.2.8 |
| SAD post-autorisation | Suppression automatique irréversible | Req. 3.3.1 |

### 3.3 Documentation sécurité

BANKO fournit la documentation suivante en support de la conformité PCI DSS :

| Document | Contenu | Audience |
|----------|---------|----------|
| [01-cde-scope-definition.md](./01-cde-scope-definition.md) | Définition du périmètre CDE | RSSI, auditeurs |
| [02-requirements-mapping.md](./02-requirements-mapping.md) | Mapping des 12 exigences PCI DSS | RSSI, auditeurs, équipe technique |
| [03-tokenization-and-encryption-guide.md](./03-tokenization-and-encryption-guide.md) | Guide de tokenisation et chiffrement | Architectes, développeurs |
| Ce document | Matrice des responsabilités | Direction, RSSI, prestataires |
| `SECURITY.md` | Politique de sécurité du projet | Tous les contributeurs |
| `CONTRIBUTING.md` | Guide de contribution sécurisée | Développeurs |
| Hardening Guide (à produire) | Guide de durcissement par composant | Administrateurs système |

### 3.4 Gestion des correctifs logiciels (Patch Management)

| Sévérité | Délai de publication | Délai d'application (banque) | Communication |
|----------|---------------------|------------------------------|---------------|
| **Critique** (CVSS ≥ 9.0) | 72 heures | 72 heures après publication | Notification immédiate par e-mail sécurisé |
| **Haute** (CVSS 7.0-8.9) | 30 jours | 30 jours après publication | Notification dans les 24h |
| **Moyenne** (CVSS 4.0-6.9) | 90 jours | 90 jours après publication | Release notes standard |
| **Basse** (CVSS < 4.0) | Prochaine release planifiée | Prochaine fenêtre de maintenance | Release notes standard |

---

## 4. Responsabilités de la banque déployant BANKO

### 4.1 Configuration réseau et segmentation

| Activité | Description | Fréquence | Livrable attendu |
|----------|-------------|-----------|------------------|
| **Déploiement Network Policies** | Appliquer les Network Policies K8s fournies par BANKO et les adapter à l'infrastructure locale | Au déploiement + à chaque modification | Network Policies appliquées et testées |
| **Configuration pare-feu** | Configurer les règles de pare-feu conformément au [guide CDE](./01-cde-scope-definition.md) section 6 | Au déploiement | Matrice de flux validée |
| **Test de segmentation** | Vérifier l'efficacité de la segmentation par des tests de pénétration | **Annuelle** (minimum) | Rapport de test |
| **Revue des règles** | Revérifier la pertinence de toutes les règles réseau | **Semestrielle** | Rapport de revue signé |
| **Surveillance réseau** | Monitorer le trafic réseau pour détecter les anomalies | **Continue** | Alertes et tableaux de bord |

### 4.2 Gestion des accès et authentification

| Activité | Description | Fréquence | Livrable attendu |
|----------|-------------|-----------|------------------|
| **Enrôlement MFA** | Enrôler tous les utilisateurs accédant au CDE en MFA (Req. 8.4.2) | Au recrutement + continue | Registre d'enrôlement |
| **Attribution des rôles** | Attribuer les rôles BANKO selon le principe du moindre privilège | Sur demande validée | Ticket d'attribution approuvé |
| **Revue des accès** | Revérifier tous les accès au CDE | **Trimestrielle** | Rapport de revue des accès |
| **Gestion des départs** | Révoquer immédiatement les accès des personnes quittant l'organisation | Immédiate | Ticket de révocation |
| **Gestion des comptes de service** | Inventorier et auditer les comptes de service K8s | **Semestrielle** | Inventaire mis à jour |

### 4.3 Monitoring et logging

| Activité | Description | Fréquence | Livrable attendu |
|----------|-------------|-----------|------------------|
| **Déploiement SIEM** | Installer et configurer le système de gestion des logs (ELK, Splunk, etc.) | Au déploiement | SIEM opérationnel |
| **Revue quotidienne des logs** | Analyser les alertes et les événements de sécurité | **Quotidienne** | Rapport d'analyse |
| **Tuning des alertes** | Affiner les seuils d'alerte pour minimiser les faux positifs | **Mensuelle** | Seuils mis à jour |
| **Rétention des logs** | Conserver les logs au moins 12 mois (3 mois en ligne minimum) | **Continue** | Politique de rétention appliquée |
| **Intégrité des logs** | Vérifier l'intégrité des logs archivés | **Mensuelle** | Rapport de vérification |

### 4.4 Tests de pénétration

| Type de test | Description | Fréquence | Exécutant |
|-------------|-------------|-----------|-----------|
| **Pentest externe** | Test de pénétration depuis Internet sur les interfaces exposées | **Annuel** | Prestataire certifié (CREST, OSCP, CEH) |
| **Pentest interne** | Test de pénétration depuis le réseau interne | **Annuel** | Prestataire certifié ou équipe interne qualifiée |
| **Test de segmentation** | Vérification de l'isolation du CDE | **Annuel** (semestriel si changements) | Prestataire certifié |
| **Scan ASV** | Scan de vulnérabilités par un ASV approuvé PCI SSC | **Trimestriel** | ASV approuvé |
| **Scan interne** | Scan de vulnérabilités depuis le réseau interne | **Trimestriel** | Équipe sécurité interne ou prestataire |

### 4.5 Formation du personnel

| Formation | Public cible | Fréquence | Contenu |
|-----------|-------------|-----------|---------|
| **Sensibilisation sécurité générale** | Tous les employés ayant accès au CDE | **Annuelle** | PCI DSS, ingénierie sociale, phishing, mots de passe |
| **Formation PCI DSS approfondie** | Équipe IT et sécurité | **Annuelle** | Exigences PCI DSS, changements v4.0.1, procédures |
| **Formation développement sécurisé** | Développeurs contribuant à BANKO | **Annuelle** | OWASP, Rust security, revue de code sécurité |
| **Exercice de réponse aux incidents** | Équipe de réponse aux incidents | **Annuelle** | Simulation d'incident, communication, remédiation |
| **Sensibilisation anti-phishing** | Tous les employés | **Trimestrielle** | Reconnaissance des tentatives de phishing, signalement |
| **Formation contexte tunisien** | Équipe conformité | **Annuelle** | Réglementations BCT, SMT, paiement mobile (OFT, Walletii, Kashy) |

---

## 5. Contrat de sous-traitance

### 5.1 Clauses PCI DSS obligatoires

Tout contrat entre la banque déployant BANKO et ses prestataires (PSP, hébergeur, intégrateur) doit inclure les clauses suivantes, conformément à l'exigence **12.8** :

| # | Clause obligatoire | Référence PCI DSS | Contenu requis |
|---|-------------------|-------------------|----------------|
| 1 | **Reconnaissance de responsabilité** | 12.9.1 | Le prestataire reconnaît par écrit être responsable de la sécurité des données CHD qu'il possède, stocke, traite ou transmet pour le compte de la banque |
| 2 | **Attestation de conformité** | 12.8.2 | Le prestataire fournit une attestation annuelle de conformité PCI DSS (AOC) correspondant à son niveau |
| 3 | **Périmètre de responsabilité** | 12.8.5 | Identification claire des exigences PCI DSS dont le prestataire est responsable et de celles qui incombent à la banque |
| 4 | **Droit d'audit** | 12.8.4 | La banque se réserve le droit d'auditer la conformité PCI DSS du prestataire ou de demander une preuve de conformité |
| 5 | **Notification d'incident** | 12.8.3 | Le prestataire s'engage à notifier la banque dans un délai maximal de **24 heures** en cas d'incident de sécurité affectant les données CHD |
| 6 | **Inventaire des prestataires** | 12.8.1 | La banque maintient un inventaire de tous les prestataires ayant accès au CDE |
| 7 | **Clause de résiliation** | -- (bonne pratique) | Possibilité de résiliation en cas de non-conformité PCI DSS avérée |
| 8 | **Restitution/destruction des données** | -- (bonne pratique) | Obligation de restituer ou détruire de manière sécurisée les données CHD en fin de contrat |

### 5.2 Modèle de matrice contractuelle PSP

| Exigence | Responsabilité PSP | Responsabilité banque | Preuve requise |
|----------|:------------------:|:--------------------:|----------------|
| Traitement des transactions carte | **R/A** | I | AOC Level 1 |
| Tokenisation côté PSP | **R/A** | C | Documentation technique |
| Sécurité de la page de paiement hébergée | **R/A** | C | Rapport de scan |
| Notification des compromissions | **R** | **A** | Clause contractuelle |
| Conformité PCI DSS du PSP | **R/A** | C (vérification annuelle) | AOC + ROC |
| Communication sécurisée (mTLS) | **R** | **R** | Configuration validée bilatéralement |

### 5.3 Modèle de matrice contractuelle hébergeur

| Exigence | Responsabilité hébergeur | Responsabilité banque | Preuve requise |
|----------|:------------------------:|:--------------------:|----------------|
| Sécurité physique du datacenter | **R/A** | I | AOC / rapport d'audit |
| Alimentation et climatisation | **R/A** | I | SLA |
| Réseau de base (connectivité) | **R/A** | C | SLA + rapport de disponibilité |
| Pare-feu périmétrique (si managed) | **R** | **A** | Configuration validée |
| Sécurité de l'hyperviseur | **R/A** | I | AOC / rapport d'audit |
| Chiffrement du stockage physique | **R** | **A** (vérification) | Attestation technique |
| Notification des incidents physiques | **R** | **A** | Clause contractuelle |

---

## 6. Validation de conformité

### 6.1 Niveaux de marchands et type de validation

Le type de validation PCI DSS dépend du **volume annuel de transactions** par carte traitées par la banque :

| Niveau | Volume annuel de transactions | Type de validation | Évaluateur |
|--------|-------------------------------|-------------------|------------|
| **Niveau 1** | Plus de 6 millions | **ROC** (Report on Compliance) | QSA (Qualified Security Assessor) externe |
| **Niveau 2** | 1 à 6 millions | **SAQ** (Self-Assessment Questionnaire) type D + scan ASV | Interne (ou QSA recommandé) |
| **Niveau 3** | 20 000 à 1 million (e-commerce) | **SAQ** type A, A-EP ou D selon le modèle | Interne |
| **Niveau 4** | Moins de 20 000 (e-commerce) ou moins de 1 million (autre) | **SAQ** selon le modèle | Interne |

### 6.2 SAQ vs ROC -- Choix pour les banques déployant BANKO

| Modèle d'intégration PSP | SAQ applicable | Périmètre | Recommandation BANKO |
|--------------------------|----------------|-----------|---------------------|
| **Redirection complète** vers le PSP | SAQ A | Minimal (pas de CHD sur le serveur) | Recommandé pour les petites banques |
| **iFrame / hosted fields** du PSP | SAQ A-EP | Réduit (pages de paiement in-scope, pas de CHD côté serveur) | **Recommandé** comme compromis |
| **API directe** (PAN reçu par BANKO) | SAQ D / ROC | Complet (tout le CDE in-scope) | Pour les banques Niveau 1 avec équipe sécurité dédiée |

### 6.3 Processus de validation annuelle

| Étape | Description | Responsable | Échéance |
|-------|-------------|-------------|----------|
| 1. **Préparation** | Revue du périmètre CDE, mise à jour de la documentation | RSSI + Équipe sécurité | T1 (janvier-mars) |
| 2. **Auto-évaluation** | Remplissage du SAQ ou préparation pour le ROC | Équipe conformité | T1 |
| 3. **Tests techniques** | Scans ASV, tests de pénétration, scans internes | Prestataires certifiés | T1-T2 |
| 4. **Remédiation** | Correction des écarts identifiés | Équipes techniques | T2-T3 |
| 5. **Évaluation formelle** | Audit QSA (Niveau 1) ou finalisation SAQ | QSA / Équipe conformité | T3 |
| 6. **Soumission** | Envoi de l'AOC et des rapports à la marque de carte et à l'acquéreur | Direction + RSSI | T3-T4 |
| 7. **Suivi** | Suivi des actions correctives résiduelles | Équipe sécurité | Continue |

### 6.4 Attestation de conformité (AOC)

L'Attestation de Conformité (AOC) est le document officiel attestant de la conformité PCI DSS. Elle est requise par :

| Destinataire | Raison | Fréquence |
|-------------|--------|-----------|
| **Marques de cartes** (Visa, Mastercard, etc.) | Obligation contractuelle | Annuelle |
| **Acquéreur / banque acquéreuse** | Obligation contractuelle | Annuelle |
| **Banque Centrale de Tunisie (BCT)** | Exigence réglementaire (selon circulaires applicables) | Annuelle |
| **SMT (Système Monétique Tunisien)** | Participation au réseau monétique | Annuelle |
| **Clients de la banque** (sur demande) | Transparence et confiance | Sur demande |

### 6.5 Contexte tunisien -- Validation SMT

Pour les banques opérant au sein du **Système Monétique Tunisien (SMT)**, des exigences supplémentaires s'appliquent :

| Exigence SMT | Description | Impact BANKO |
|-------------|-------------|--------------|
| **Certification monétique** | Conformité aux standards du réseau monétique tunisien | BANKO doit supporter les protocoles SMT |
| **Interopérabilité** | Compatibilité avec les systèmes des autres banques tunisiennes | API d'interconnexion documentées |
| **Reporting BCT** | Rapports périodiques à la Banque Centrale de Tunisie | Module Reporting BC configuré pour les formats BCT |
| **Audit BCT** | Possibilité d'audit par la BCT | Documentation et logs accessibles aux auditeurs BCT |
| **Conformité paiement mobile** | Intégration avec les opérateurs mobiles (OFT, Walletii, Kashy) | API sécurisées pour les partenaires mobile money |

---

## Références

| Document | Lien |
|----------|------|
| PCI DSS v4.0.1 (juin 2024) | [PCI SSC Document Library](https://www.pcisecuritystandards.org/document_library/) |
| PCI SSC -- List of QSAs | [PCI SSC QSA Directory](https://www.pcisecuritystandards.org/assessors_and_solutions/qualified_security_assessors) |
| PCI SSC -- List of ASVs | [PCI SSC ASV Directory](https://www.pcisecuritystandards.org/assessors_and_solutions/approved_scanning_vendors) |
| Définition du périmètre CDE | [01-cde-scope-definition.md](./01-cde-scope-definition.md) |
| Mapping des exigences PCI DSS | [02-requirements-mapping.md](./02-requirements-mapping.md) |
| Guide tokenisation et chiffrement | [03-tokenization-and-encryption-guide.md](./03-tokenization-and-encryption-guide.md) |
| Référentiel légal et normatif | [REFERENTIEL_LEGAL_ET_NORMATIF.md](../../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) |

---

*Document généré dans le cadre du programme de conformité PCI DSS de la plateforme BANKO. Toute modification doit suivre le processus de revue documentaire.*
