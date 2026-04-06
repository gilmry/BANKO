# Mapping des Exigences PCI DSS v4.0.1 -- Plateforme BANKO

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

1. [Vue d'ensemble PCI DSS v4.0.1](#1-vue-densemble-pci-dss-v401)
2. [Exigence 1 -- Contrôles de sécurité réseau](#2-exigence-1--contrôles-de-sécurité-réseau)
3. [Exigence 2 -- Configurations sécurisées](#3-exigence-2--configurations-sécurisées)
4. [Exigence 3 -- Protection des données stockées](#4-exigence-3--protection-des-données-stockées)
5. [Exigence 4 -- Chiffrement en transit](#5-exigence-4--chiffrement-en-transit)
6. [Exigence 5 -- Protection contre les logiciels malveillants](#6-exigence-5--protection-contre-les-logiciels-malveillants)
7. [Exigence 6 -- Systèmes et logiciels sécurisés](#7-exigence-6--systèmes-et-logiciels-sécurisés)
8. [Exigence 7 -- Restriction d'accès](#8-exigence-7--restriction-daccès)
9. [Exigence 8 -- Identification et authentification](#9-exigence-8--identification-et-authentification)
10. [Exigence 9 -- Accès physique](#10-exigence-9--accès-physique)
11. [Exigence 10 -- Journalisation et surveillance](#11-exigence-10--journalisation-et-surveillance)
12. [Exigence 11 -- Tests de sécurité](#12-exigence-11--tests-de-sécurité)
13. [Exigence 12 -- Politiques de sécurité](#13-exigence-12--politiques-de-sécurité)
14. [Synthèse des exigences anciennement future-dated](#14-synthèse-des-exigences-anciennement-future-dated)
15. [Écarts identifiés et plan de remédiation](#15-écarts-identifiés-et-plan-de-remédiation)

---

## 1. Vue d'ensemble PCI DSS v4.0.1

### 1.1 Structure du référentiel

PCI DSS v4.0.1 est organisé en **6 objectifs** couvrant **12 exigences** :

| Objectif | Exigences | Description |
|----------|-----------|-------------|
| **Construire et maintenir un réseau sécurisé** | 1, 2 | Contrôles réseau et configurations sécurisées |
| **Protéger les données des titulaires de cartes** | 3, 4 | Protection au repos et en transit |
| **Maintenir un programme de gestion des vulnérabilités** | 5, 6 | Anti-malware et développement sécurisé |
| **Mettre en oeuvre des mesures de contrôle d'accès strictes** | 7, 8, 9 | Accès logique et physique |
| **Surveiller et tester régulièrement les réseaux** | 10, 11 | Journalisation et tests de sécurité |
| **Maintenir une politique de sécurité de l'information** | 12 | Gouvernance et gestion des risques |

### 1.2 Convention de statut

| Statut | Signification |
|--------|---------------|
| **Conforme** | Implémenté et vérifié |
| **En cours** | Implémentation démarrée, non finalisée |
| **Planifié** | Prévu dans la feuille de route |
| **N/A** | Non applicable au contexte BANKO |
| **Responsabilité banque** | Relève de la banque déployant BANKO |

### 1.3 Convention de priorité

| Priorité | Signification |
|----------|---------------|
| **P0 -- Critique** | Bloquant pour la conformité, à traiter immédiatement |
| **P1 -- Haute** | Important, à traiter dans le trimestre |
| **P2 -- Moyenne** | Nécessaire, à traiter dans le semestre |
| **P3 -- Basse** | Amélioration continue |

---

## 2. Exigence 1 -- Contrôles de sécurité réseau

**Objectif** : Installer et maintenir des contrôles de sécurité réseau pour protéger le CDE.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 1.1.1 | Processus documentés pour les contrôles réseau | Documentation des Network Policies K8s et règles Docker | Infrastructure | En cours | P1 |
| 1.1.2 | Revue des règles réseau tous les 6 mois | Script d'audit automatisé des Network Policies | Governance BC | Planifié | P2 |
| 1.2.1 | Trafic entrant/sortant restreint au nécessaire | Traefik : seul point d'entrée HTTPS (443) ; règles deny-all par défaut | Traefik / K8s | Conforme | -- |
| 1.2.5 | Contrôle des services/protocoles/ports autorisés | Network Policies K8s par namespace ; docker-compose networks isolés | Infrastructure | Conforme | -- |
| 1.2.6 | Contrôles de sécurité appliqués à tous les trafics | Traefik middleware chain : rate-limiting, headers sécurité, CORS | Traefik | Conforme | -- |
| 1.2.8 | Fichiers de configuration des contrôles réseau sécurisés | Configurations versionées dans Git, accès restreint | Infrastructure | Conforme | -- |
| 1.3.1 | Trafic entrant vers le CDE restreint | Network Policy `banko-payment` : allow uniquement depuis Traefik | K8s / Docker | Conforme | -- |
| 1.3.2 | Trafic sortant du CDE restreint | Egress policy : uniquement vers PostgreSQL, Token Vault, PSP | K8s / Docker | Conforme | -- |
| 1.4.1 | NSC entre réseaux de confiance et non fiables | Traefik comme DMZ ; séparation frontend-net / payment-net | Traefik / K8s | Conforme | -- |
| ⚠️ 1.4.2 | Trafic entrant depuis réseaux non fiables vers le CDE contrôlé | WAF intégré Traefik + rate limiting par IP ; mécanisme anti-DDoS | Traefik | En cours | P1 |
| 1.5.1 | Contrôles de sécurité sur réseaux sans fil | Non applicable -- infrastructure conteneurisée, pas de Wi-Fi dans le CDE | -- | N/A | -- |

---

## 3. Exigence 2 -- Configurations sécurisées

**Objectif** : Appliquer des configurations sécurisées à tous les composants du système.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 2.1.1 | Processus documentés pour les configurations sécurisées | Guide de durcissement (hardening guide) pour chaque composant | Documentation | En cours | P1 |
| 2.2.1 | Normes de configuration développées pour chaque type de composant | Dockerfiles durcis, `postgresql.conf` sécurisé, `traefik.yml` audité | Infrastructure | Conforme | -- |
| 2.2.2 | Comptes par défaut gérés | PostgreSQL : mot de passe aléatoire généré au déploiement ; MinIO : credentials rotés | Infrastructure | Conforme | -- |
| 2.2.3 | Fonctions principales isolées par composant | Un conteneur = une fonction ; bounded contexts séparés | Docker / K8s | Conforme | -- |
| 2.2.4 | Services inutiles désactivés | Images Docker basées sur `alpine` / `distroless` ; ports minimaux exposés | Infrastructure | Conforme | -- |
| 2.2.5 | Services non sécurisés configurés de manière sécurisée | TLS obligatoire pour toute communication ; HTTP redirigé vers HTTPS | Traefik | Conforme | -- |
| 2.2.6 | Paramètres de sécurité système configurés | Headers sécurité HTTP (CSP, HSTS, X-Frame-Options) via Traefik middleware | Traefik | Conforme | -- |
| 2.2.7 | Accès administrateur chiffré (non-console) | SSH désactivé sur les conteneurs ; accès kubectl via VPN + certificat | Infrastructure | Conforme | -- |
| 2.3.1 | Fonctions sans fil non applicables | Infrastructure conteneurisée uniquement | -- | N/A | -- |

---

## 4. Exigence 3 -- Protection des données stockées

**Objectif** : Protéger les données de comptes stockées.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 3.1.1 | Processus documentés pour la protection des données stockées | Politique de classification des données + procédure de chiffrement documentées | Documentation | En cours | P1 |
| 3.2.1 | Rétention des données minimisée | SAD jamais stocké ; PAN tokenisé ; rétention des transactions configurable | Payment BC | Conforme | -- |
| 3.3.1 | SAD non conservé après autorisation | Architecture BANKO : SAD transmis au PSP en transit uniquement, jamais persisté | Payment BC | Conforme | -- |
| 3.3.2 | SAD non stocké par l'émetteur après autorisation | Non applicable (BANKO n'est pas émetteur) ; politique documentée | -- | N/A | -- |
| 3.4.1 | PAN masqué à l'affichage (max 6 premiers + 4 derniers) | Masquage systématique dans les DTOs : `mask_pan()` dans Application layer | Payment BC | Conforme | -- |
| 3.4.2 | PAN non récupérable par accès distant (copie, déplacement) | Tokens uniquement dans les réponses API ; dé-tokenisation interdite côté client | Payment BC | Conforme | -- |
| 3.5.1 | PAN rendu illisible partout où il est stocké | Tokenisation par défaut ; PAN clair uniquement dans le vault chiffré AES-256-GCM | Payment BC / Vault | Conforme | -- |
| ⚠️ 3.5.1.2 | Chiffrement au niveau champ (column-level), pas uniquement disque | Chiffrement AES-256-GCM au niveau colonne PostgreSQL pour `payment_tokens.pan_encrypted` | Payment BC / PostgreSQL | **En cours** | **P0** |
| 3.6.1 | Procédures de gestion des clés cryptographiques | Rotation trimestrielle des clés ; split knowledge ; double contrôle | Governance BC | En cours | P1 |
| 3.7.1-3.7.9 | Gestion complète du cycle de vie des clés | Key ceremony documentée ; HSM recommandé ; HashiCorp Vault comme alternative | Infrastructure | Planifié | P1 |

Voir [03-tokenization-and-encryption-guide.md](./03-tokenization-and-encryption-guide.md) pour les détails techniques.

---

## 5. Exigence 4 -- Chiffrement en transit

**Objectif** : Protéger les données de titulaires de cartes par une cryptographie forte lors de la transmission sur des réseaux ouverts et publics.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 4.1.1 | Processus documentés pour le chiffrement en transit | Architecture TLS documentée ; configuration Traefik versionée | Documentation | Conforme | -- |
| 4.2.1 | Cryptographie forte lors de la transmission de CHD | TLS 1.3 obligatoire (Traefik) ; cipher suites modernes uniquement | Traefik | Conforme | -- |
| 4.2.1.1 | Certificats de confiance utilisés | Certificats Let's Encrypt (production) ; auto-signés (dev uniquement) | Traefik | Conforme | -- |
| 4.2.1.2 | Protocoles et versions autorisés | TLS 1.3 uniquement ; TLS 1.2 / SSL désactivés dans la configuration Traefik | Traefik | Conforme | -- |
| 4.2.2 | PAN sécurisé lors d'envoi par messagerie (end-user) | Non applicable -- BANKO n'envoie jamais de PAN par messagerie | -- | N/A | -- |

---

## 6. Exigence 5 -- Protection contre les logiciels malveillants

**Objectif** : Protéger tous les systèmes et réseaux contre les logiciels malveillants.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 5.1.1 | Processus documentés pour la protection anti-malware | Politique anti-malware documentée ; images conteneurs scannées | Documentation | En cours | P2 |
| 5.2.1 | Solution anti-malware déployée sur les systèmes sensibles | Conteneurs : images `distroless` / `alpine` minimales ; scans Trivy dans CI/CD | Infrastructure | Conforme | -- |
| 5.2.2 | Anti-malware avec détection en temps réel ou analyses périodiques | Scans d'images conteneurs à chaque build + hebdomadaire en production | CI/CD | Conforme | -- |
| 5.2.3 | Systèmes non sensibles évalués périodiquement | Évaluation trimestrielle des composants non couverts par l'anti-malware | Governance BC | Planifié | P2 |
| 5.3.1 | Anti-malware maintenu à jour | Images de base mises à jour mensuellement ; `cargo audit` dans CI/CD | Infrastructure | Conforme | -- |
| 5.3.2 | Anti-malware avec mises à jour automatiques | Dépendances Rust : Dependabot + `cargo audit` ; npm : `npm audit` | CI/CD | Conforme | -- |
| ⚠️ 5.4.1 | Mécanismes anti-hameçonnage (anti-phishing) | Filtrage e-mail (SPF, DKIM, DMARC) ; formation sécurité trimestrielle ; FIDO2/WebAuthn pour les opérateurs | Identity BC / Gouvernance | **En cours** | **P0** |

---

## 7. Exigence 6 -- Systèmes et logiciels sécurisés

**Objectif** : Développer et maintenir des systèmes et logiciels sécurisés.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 6.1.1 | Processus documentés pour le développement sécurisé | CONTRIBUTING.md + guides Claude Code + revue de code obligatoire | Documentation | Conforme | -- |
| 6.2.1 | Logiciel développé de manière sécurisée | Rust (memory safety native) ; OWASP Top 10 intégré ; architecture hexagonale | Backend | Conforme | -- |
| 6.2.2 | Personnel de développement formé au développement sécurisé | Formation annuelle OWASP + spécifique Rust ; sessions mensuelles de sensibilisation | Governance BC | En cours | P2 |
| 6.2.3 | Revues de code avant mise en production | Pull requests obligatoires ; revue par au moins 2 développeurs ; CI automatisée | CI/CD | Conforme | -- |
| 6.2.4 | Prévention des vulnérabilités courantes | Rust élimine buffer overflow, use-after-free ; SQLx prévient l'injection SQL ; validation au Domain layer | Backend | Conforme | -- |
| 6.3.1 | Vulnérabilités identifiées et corrigées | `cargo audit` + `npm audit` dans CI/CD ; veille CVE continue | CI/CD | Conforme | -- |
| 6.3.2 | Inventaire des logiciels et composants | `Cargo.lock` et `package-lock.json` versionés ; SBOM généré à chaque release | Backend / Frontend | Conforme | -- |
| 6.3.3 | Correctifs de sécurité installés dans les délais | SLA : correctifs critiques sous 72h, hauts sous 30j, moyens sous 90j | Infrastructure | En cours | P1 |
| ⚠️ 6.4.1 | Applications web protégées contre les attaques connues | WAF Traefik middleware + validation stricte côté Actix-web ; headers sécurité (CSP, etc.) | Traefik / Backend | Conforme | -- |
| ⚠️ 6.4.2 | Applications web orientées public protégées contre les attaques | WAF + rate limiting + CAPTCHA sur endpoints sensibles | Traefik | Conforme | -- |
| ⚠️ 6.4.3 | **Gestion des scripts sur les pages de paiement** | Inventaire de tous les scripts JS exécutés sur les pages de paiement ; Content-Security-Policy stricte ; intégrité des scripts (SRI - Subresource Integrity) ; monitoring des modifications | Frontend (Astro/Svelte) | **En cours** | **P0** |
| 6.5.1 | Environnements de dev/test séparés de la production | Environnements Docker isolés (dev/staging/prod) ; données synthétiques en dev | Infrastructure | Conforme | -- |
| 6.5.2 | Données de production non utilisées en dev/test | Générateur de données synthétiques (`make seed`) ; politique documentée | Infrastructure | Conforme | -- |
| 6.5.3 | Comptes de test supprimés avant production | Pipeline CI/CD : nettoyage automatique des données de test | CI/CD | Conforme | -- |
| 6.5.4 | Code et comptes personnalisés supprimés avant production | Revue pre-release ; checklist de mise en production | CI/CD | Conforme | -- |
| 6.5.5 | Procédures de gestion des changements | Workflow Git : branches feature → PR → revue → merge → déploiement | CI/CD | Conforme | -- |
| 6.5.6 | Gestion des changements applicatifs incluant la documentation | Changelog automatisé ; documentation API (OpenAPI) mise à jour | Documentation | En cours | P2 |

---

## 8. Exigence 7 -- Restriction d'accès

**Objectif** : Restreindre l'accès aux composants du système et aux données des titulaires de cartes au besoin d'en connaître.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 7.1.1 | Processus documentés pour le contrôle d'accès | Politique de contrôle d'accès documentée ; matrice RBAC | Governance BC | En cours | P1 |
| 7.1.2 | Modèle de contrôle d'accès défini | RBAC (Role-Based Access Control) implémenté dans le Governance BC | Governance BC | Conforme | -- |
| 7.2.1 | Accès basé sur le besoin d'en connaître | Rôles BANKO : `payment_operator`, `payment_admin`, `auditor` ; moindre privilège | Governance BC | Conforme | -- |
| 7.2.2 | Accès attribué en fonction du rôle | Middleware Actix-web vérifiant les permissions JWT par endpoint | Backend | Conforme | -- |
| 7.2.3 | Privilèges requis approuvés par le management | Workflow d'approbation pour l'attribution de rôles sensibles | Governance BC | Planifié | P2 |
| 7.2.4 | Revue des comptes utilisateurs périodique | Revue trimestrielle des accès au CDE ; rapport automatisé | Governance BC | Planifié | P1 |
| 7.2.5 | Comptes applicatifs et système gérés | Service accounts K8s avec RBAC ; pas de compte partagé | Infrastructure | Conforme | -- |
| ⚠️ 7.2.5.1 | Revue des comptes applicatifs et système | Audit semestriel automatisé des service accounts et de leurs permissions | Governance BC | Planifié | P1 |
| 7.2.6 | Accès aux données CHD restreint au minimum | Seuls les rôles `payment_operator` et `payment_admin` accèdent aux endpoints Payment | Payment BC | Conforme | -- |
| 7.3.1-7.3.3 | Contrôle d'accès appliqué en temps réel | Middleware JWT vérifié à chaque requête ; invalidation immédiate en cas de révocation | Identity BC | Conforme | -- |

---

## 9. Exigence 8 -- Identification et authentification

**Objectif** : Identifier les utilisateurs et authentifier l'accès aux composants du système.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 8.1.1 | Processus documentés pour l'identification et l'authentification | Politique d'authentification documentée ; guide d'intégration Identity BC | Documentation | En cours | P1 |
| 8.2.1 | Identifiant unique pour chaque utilisateur | UUID généré pour chaque compte ; pas de comptes partagés | Identity BC | Conforme | -- |
| 8.2.2 | Comptes partagés interdits | Politique technique : un JWT = un utilisateur unique | Identity BC | Conforme | -- |
| 8.2.3 | Gestion des comptes de service | Service accounts nommés et tracés dans les logs | Identity BC / K8s | Conforme | -- |
| 8.3.1 | Authentification forte pour tous les utilisateurs | JWT + refresh tokens ; mots de passe conformes NIST SP 800-63B | Identity BC | Conforme | -- |
| 8.3.4 | Verrouillage après tentatives échouées | Verrouillage après 5 tentatives ; déblocage après 30 min ou intervention admin | Identity BC | Conforme | -- |
| 8.3.6 | Complexité minimale des mots de passe | Minimum 12 caractères, mixte (majuscules, minuscules, chiffres, spéciaux) | Identity BC | Conforme | -- |
| 8.3.7 | Nouveaux mots de passe différents des 4 précédents | Historique des hachages conservé ; vérification à la modification | Identity BC | Conforme | -- |
| 8.3.9 | Mots de passe modifiés au moins tous les 90 jours (si pas de MFA) | Politique configurable ; 90 jours par défaut ; désactivable si MFA actif | Identity BC | Conforme | -- |
| ⚠️ 8.4.2 | **MFA pour tout accès au CDE** | TOTP (RFC 6238) + FIDO2/WebAuthn pour tous les utilisateurs accédant aux fonctions Payment ; MFA obligatoire pour les accès administratifs | Identity BC | **En cours** | **P0** |
| ⚠️ 8.4.3 | MFA pour tout accès distant au réseau | VPN + certificat client + MFA pour accès distant à l'infrastructure | Infrastructure | Responsabilité banque | P0 |
| 8.5.1 | MFA correctement implémenté | Facteurs indépendants ; canal de transmission séparé ; résistance au replay | Identity BC | En cours | P0 |
| 8.6.1 | Authentification des comptes système/applicatifs | Service accounts K8s ; mTLS entre services ; rotation des secrets | Infrastructure | Conforme | -- |
| 8.6.2 | Mots de passe/passphrases des comptes système non codés en dur | Variables d'environnement + HashiCorp Vault ; aucun secret dans le code source | Infrastructure | Conforme | -- |
| 8.6.3 | Mots de passe des comptes système protégés | Secrets K8s chiffrés (sealed-secrets) ; rotation trimestrielle | Infrastructure | Conforme | -- |

---

## 10. Exigence 9 -- Accès physique

**Objectif** : Restreindre l'accès physique aux données des titulaires de cartes.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 9.1.1 | Processus documentés pour l'accès physique | Documentation fournie ; implémentation relève de la banque et de l'hébergeur | Documentation | Responsabilité banque | P2 |
| 9.2.1-9.2.4 | Contrôles d'accès physique aux installations | Responsabilité de la banque déployant BANKO et de son hébergeur (datacenter) | -- | Responsabilité banque | -- |
| 9.3.1-9.3.4 | Accès physique aux médias contenant CHD contrôlé | Guide de destruction sécurisée des médias fourni dans la documentation BANKO | Documentation | Responsabilité banque | -- |
| 9.4.1-9.4.7 | Protection des médias contenant CHD | Classification des médias documentée ; procédures de destruction | Documentation | Responsabilité banque | -- |
| 9.5.1 | Protection des terminaux de paiement (POI) | Non applicable -- BANKO est un logiciel, pas un terminal physique ; guide d'intégration terminal fourni | Documentation | N/A | -- |

**Note** : L'exigence 9 relève principalement de la **responsabilité de la banque** et de son **hébergeur**. BANKO fournit la documentation et les recommandations nécessaires. Voir [04-responsibility-matrix.md](./04-responsibility-matrix.md).

---

## 11. Exigence 10 -- Journalisation et surveillance

**Objectif** : Journaliser et surveiller tous les accès aux composants du système et aux données des titulaires de cartes.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 10.1.1 | Processus documentés pour la journalisation | Politique de logging documentée ; niveaux de log définis par BC | Documentation | En cours | P1 |
| 10.2.1 | Logs activés sur tous les composants du CDE | Actix-web : logging structuré (JSON) ; PostgreSQL : `pgaudit` activé ; Traefik : access logs | Tous les composants CDE | Conforme | -- |
| 10.2.1.1 | Logs capturent tous les accès individuels aux CHD | Audit trail dans le Payment BC : chaque accès aux tokens journalisé avec user, timestamp, action | Payment BC | Conforme | -- |
| 10.2.1.2 | Logs capturent les actions administratives | Toutes les actions admin journalisées via middleware Actix-web | Governance BC | Conforme | -- |
| 10.2.1.3 | Logs capturent les accès aux pistes d'audit | Accès aux logs protégé par RBAC ; logs d'accès aux logs eux-mêmes journalisés | Monitoring | Conforme | -- |
| 10.2.1.4 | Logs capturent les tentatives d'accès logique invalides | Logging des erreurs 401/403 avec détails de la requête (sans CHD) | Identity BC | Conforme | -- |
| 10.2.1.5 | Logs capturent les modifications des identifications | Création, modification, suppression de comptes journalisées | Identity BC | Conforme | -- |
| 10.2.1.6 | Logs capturent l'arrêt/démarrage des logs d'audit | Surveillance de l'état du système de logging ; alertes en cas d'arrêt | Monitoring | Conforme | -- |
| 10.2.1.7 | Logs capturent la création/suppression d'objets système | Audit des changements de schéma DB ; modifications de configuration | Infrastructure | Conforme | -- |
| 10.2.2 | Logs contiennent les éléments nécessaires à l'investigation | Format structuré : `{timestamp, user_id, source_ip, action, resource, result, component}` | Tous | Conforme | -- |
| 10.3.1 | Logs protégés contre la modification | Logs en append-only ; stockage sur volume séparé ; intégrité vérifiée | Monitoring | Conforme | -- |
| 10.3.2 | Logs sauvegardés rapidement sur un système centralisé | ELK Stack / Prometheus + Loki : collecte en temps réel | Monitoring | Conforme | -- |
| 10.3.3 | Logs protégés sur les systèmes centralisés | Accès RBAC au cluster de logging ; chiffrement au repos | Monitoring | Conforme | -- |
| 10.3.4 | Mécanismes de détection d'intégrité des logs | Hachage SHA-256 des blocs de logs ; vérification périodique | Monitoring | Planifié | P1 |
| 10.4.1 | Logs revus au moins quotidiennement | Dashboard Kibana/Grafana avec alertes automatiques sur anomalies | Monitoring | En cours | P1 |
| 10.4.2 | Revue des logs de tous les composants CDE | Tableaux de bord consolidés ; alertes par type d'événement | Monitoring | En cours | P1 |
| ⚠️ 10.4.2.1 | Revue automatisée des logs par mécanismes ciblés | Règles de corrélation et d'alerte automatisées (ElastAlert / alerting Prometheus) ; seuils configurables | Monitoring | **En cours** | **P0** |
| 10.5.1 | Historique des logs conservé au moins 12 mois | Rétention configurable : 12 mois en ligne, archivage au-delà ; politique documentée | Monitoring | Conforme | -- |
| 10.6.1 | Synchronisation temporelle (NTP) | Tous les conteneurs synchronisés sur le serveur NTP de l'hôte ; format UTC | Infrastructure | Conforme | -- |
| 10.6.2 | Données temporelles fiables et précises | NTP avec sources multiples ; dérive maximale tolérée : 1 seconde | Infrastructure | Conforme | -- |
| 10.7.1 | Défaillances des systèmes critiques détectées et traitées | Alertes sur indisponibilité des composants CDE ; procédure d'escalade documentée | Monitoring | En cours | P1 |

---

## 12. Exigence 11 -- Tests de sécurité

**Objectif** : Tester régulièrement la sécurité des systèmes et des réseaux.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 11.1.1 | Processus documentés pour les tests de sécurité | Programme de tests de sécurité documenté ; calendrier annuel | Documentation | En cours | P1 |
| 11.2.1 | Points d'accès sans fil détectés et identifiés | Non applicable -- infrastructure conteneurisée | -- | N/A | -- |
| 11.3.1 | Scans de vulnérabilités internes trimestriels | `cargo audit`, `npm audit`, scans Trivy sur les images conteneurs | CI/CD | Conforme | -- |
| 11.3.1.1 | Vulnérabilités hautes/critiques résolues et re-scannées | SLA : critiques 72h, hautes 30j ; re-scan automatique après correction | CI/CD | En cours | P1 |
| 11.3.2 | Scans de vulnérabilités externes trimestriels (ASV) | Responsabilité de la banque (engagement d'un ASV certifié PCI) | -- | Responsabilité banque | P1 |
| 11.4.1 | Tests de pénétration externes annuels | Responsabilité de la banque ; BANKO fournit le plan de test recommandé | Documentation | Responsabilité banque | P1 |
| 11.4.2 | Tests de pénétration internes annuels | Méthodologie documentée ; couverture de tous les composants CDE | -- | Responsabilité banque | P1 |
| 11.4.3 | Vulnérabilités exploitables corrigées et re-testées | Procédure de remédiation documentée ; re-test obligatoire | -- | Responsabilité banque | P1 |
| 11.4.5 | Tests de segmentation au moins annuels | Vérification automatisée des Network Policies + tests manuels | Infrastructure | Planifié | P1 |
| 11.5.1 | Mécanismes de détection d'intrusion (IDS/IPS) | IDS/IPS au niveau réseau K8s (Falco / Calico Enterprise) | Infrastructure | Planifié | P1 |
| 11.5.2 | Détection de modification de fichiers (FIM) | Surveillance d'intégrité des fichiers critiques du conteneur Payment | Infrastructure | Planifié | P1 |
| ⚠️ 11.6.1 | **Détection de modification/altération des pages de paiement** | Content-Security-Policy stricte ; SRI (Subresource Integrity) sur tous les scripts ; monitoring de l'intégrité DOM des pages de paiement ; alertes en temps réel sur toute modification non autorisée | Frontend / Monitoring | **En cours** | **P0** |

---

## 13. Exigence 12 -- Politiques de sécurité

**Objectif** : Soutenir la sécurité de l'information par des politiques et des programmes organisationnels.

| Sous-exigence | Description | Implémentation BANKO | Module concerné | Statut | Priorité |
|---------------|-------------|----------------------|-----------------|--------|----------|
| 12.1.1 | Politique de sécurité de l'information établie et publiée | SECURITY.md publié ; politique interne documentée | Documentation | Conforme | -- |
| 12.1.2 | Politique revue annuellement | Revue planifiée annuellement ; déclenchée aussi par changements majeurs | Governance BC | Conforme | -- |
| 12.1.3 | Rôles et responsabilités de sécurité définis | Matrice RACI documentée (voir [04-responsibility-matrix.md](./04-responsibility-matrix.md)) | Governance BC | Conforme | -- |
| 12.2.1 | Politiques d'utilisation acceptable documentées | Guide de bonnes pratiques pour les opérateurs BANKO | Documentation | En cours | P2 |
| ⚠️ 12.3.1 | **Analyses de risques ciblées pour chaque exigence PCI DSS** | Méthodologie d'analyse de risques documentée ; analyse ciblée par exigence avec identification des menaces, probabilité, impact et contrôles compensatoires | Governance BC | **En cours** | **P0** |
| 12.3.2 | Analyse de risques ciblée pour les technologies personnalisées | Évaluation de risque pour : Rust/Actix-web, PostgreSQL 16, Traefik, Docker/K8s | Governance BC | En cours | P1 |
| ⚠️ 12.3.3 | Protocoles cryptographiques revus au moins annuellement | Inventaire des protocoles et algorithmes ; revue annuelle de leur adéquation | Governance BC | Planifié | P1 |
| 12.3.4 | Matériel et logiciel revu au moins annuellement | Inventaire des composants CDE mis à jour ; vérification de la fin de support | Governance BC | Planifié | P2 |
| 12.4.1 | Conformité PCI DSS assignée à un responsable | RSSI désigné comme responsable de la conformité PCI DSS | Governance BC | Responsabilité banque | P1 |
| 12.5.1 | Périmètre CDE documenté et maintenu | Présent document ([01-cde-scope-definition.md](./01-cde-scope-definition.md)) | Documentation | Conforme | -- |
| 12.5.2 | Périmètre CDE revalidé annuellement | Procédure de revue annuelle documentée (section 8 du document CDE) | Governance BC | Conforme | -- |
| 12.6.1 | Programme de sensibilisation à la sécurité | Formation annuelle obligatoire ; sessions trimestrielles de rappel | Governance BC | Responsabilité banque | P1 |
| 12.6.2 | Sensibilisation au moins annuelle | Programme incluant PCI DSS, ingénierie sociale, phishing | Governance BC | Responsabilité banque | P1 |
| ⚠️ 12.6.3.1 | Sensibilisation incluant menaces et vulnérabilités spécifiques | Formation adaptée au contexte tunisien : SMT, menaces locales, mobile money | Governance BC | Responsabilité banque | P1 |
| 12.8.1-12.8.5 | Gestion des prestataires de services | Clauses PCI DSS dans les contrats PSP ; attestation de conformité annuelle | Governance BC | Responsabilité banque | P1 |
| 12.9.1 | Prestataires reconnaissent leur responsabilité | Template de contrat avec clauses PCI DSS obligatoires fourni par BANKO | Documentation | Conforme | -- |
| 12.10.1 | Plan de réponse aux incidents documenté | Modèle de plan de réponse aux incidents fourni ; adaptation par la banque | Documentation | Conforme | -- |
| 12.10.2 | Plan testé annuellement | Exercice de simulation recommandé ; guide de test fourni | Documentation | Responsabilité banque | P1 |

---

## 14. Synthèse des exigences anciennement future-dated

Les exigences suivantes, initialement classées « future-dated » dans PCI DSS v4.0, sont devenues **obligatoires le 31 mars 2025**. Elles requièrent une attention particulière :

| Exigence | Titre | Description | Statut BANKO | Priorité | Échéance interne |
|----------|-------|-------------|:------------:|----------|------------------|
| ⚠️ 3.5.1.2 | Chiffrement au niveau champ | Le chiffrement au niveau disque seul n'est plus suffisant ; chiffrement au niveau colonne obligatoire pour les données CHD stockées | En cours | P0 | T2 2026 |
| ⚠️ 5.4.1 | Mécanismes anti-hameçonnage | Mécanismes techniques et organisationnels de lutte contre le phishing | En cours | P0 | T2 2026 |
| ⚠️ 6.4.3 | Gestion des scripts pages de paiement | Inventaire, autorisation et surveillance de tous les scripts exécutés sur les pages de paiement | En cours | P0 | T2 2026 |
| ⚠️ 8.4.2 | MFA pour accès au CDE | Authentification multi-facteur obligatoire pour tout accès au CDE (pas uniquement les accès distants) | En cours | P0 | T2 2026 |
| ⚠️ 10.4.2.1 | Revue automatisée des logs | Mécanismes automatisés de revue des logs avec corrélation et alertes | En cours | P0 | T2 2026 |
| ⚠️ 11.6.1 | Détection de modification des pages de paiement | Mécanismes de détection de changement/altération sur les pages de paiement (protection contre le web skimming) | En cours | P0 | T2 2026 |
| ⚠️ 12.3.1 | Analyses de risques ciblées | Analyse de risques ciblée documentée pour chaque exigence PCI DSS permettant une approche personnalisée | En cours | P0 | T3 2026 |
| ⚠️ 7.2.5.1 | Revue des comptes applicatifs/système | Revue périodique de tous les comptes applicatifs et de service et de leurs privilèges | Planifié | P1 | T3 2026 |
| ⚠️ 12.3.3 | Revue des protocoles cryptographiques | Revue annuelle des suites et protocoles cryptographiques utilisés | Planifié | P1 | T3 2026 |

### Analyse d'impact

| Impact | Nombre d'exigences | Modules BANKO concernés |
|--------|:-------------------:|------------------------|
| **P0 -- Critique** | 7 | Payment BC, Identity BC, Frontend, Monitoring, Governance BC |
| **P1 -- Haute** | 2 | Governance BC, Infrastructure |
| **Total** | 9 | -- |

---

## 15. Écarts identifiés et plan de remédiation

### 15.1 Écarts critiques (P0)

| # | Exigence | Écart identifié | Action corrective | Responsable | Échéance | Effort estimé |
|---|----------|-----------------|-------------------|-------------|----------|---------------|
| 1 | 3.5.1.2 | Chiffrement au niveau colonne non encore implémenté dans PostgreSQL | Implémenter le chiffrement AES-256-GCM au niveau champ dans le module Payment ; utiliser l'extension `pgcrypto` ou chiffrement applicatif via Rust | Équipe Backend | T2 2026 | 3 semaines |
| 2 | 6.4.3 | Inventaire des scripts sur les pages de paiement incomplet | Implémenter CSP stricte (`script-src` avec nonces) ; inventorier tous les scripts ; mettre en place SRI sur tous les scripts tiers | Équipe Frontend | T2 2026 | 2 semaines |
| 3 | 8.4.2 | MFA implémenté uniquement pour les accès admin, pas pour tous les accès CDE | Étendre le MFA (TOTP + FIDO2) à tous les utilisateurs accédant aux fonctions du Payment BC | Équipe Identity | T2 2026 | 3 semaines |
| 4 | 11.6.1 | Pas de mécanisme de détection de modification des pages de paiement | Implémenter un monitoring d'intégrité des pages de paiement (hash comparatif, CSP reporting, surveillance DOM) | Équipe Frontend + Monitoring | T2 2026 | 2 semaines |
| 5 | 5.4.1 | Mécanismes anti-phishing partiellement déployés | Compléter le déploiement SPF/DKIM/DMARC ; implémenter FIDO2 résistant au phishing ; programme de formation trimestriel | Équipe Sécurité | T2 2026 | 2 semaines |
| 6 | 12.3.1 | Analyses de risques ciblées non encore réalisées | Réaliser une analyse de risques ciblée pour chaque exigence PCI DSS selon la méthodologie définie | RSSI | T3 2026 | 4 semaines |
| 7 | 10.4.2.1 | Revue automatisée des logs en cours de configuration | Finaliser les règles de corrélation ElastAlert ; configurer les seuils d'alerte ; valider les tableaux de bord | Équipe Monitoring | T2 2026 | 2 semaines |

### 15.2 Écarts importants (P1)

| # | Exigence | Écart identifié | Action corrective | Responsable | Échéance | Effort estimé |
|---|----------|-----------------|-------------------|-------------|----------|---------------|
| 8 | 1.1.1 | Documentation des contrôles réseau incomplète | Documenter exhaustivement les Network Policies K8s et les règles Docker | Équipe Infrastructure | T3 2026 | 1 semaine |
| 9 | 3.6.1 / 3.7.x | Procédures de gestion des clés à finaliser | Documenter les key ceremonies ; implémenter la rotation automatique | Équipe Sécurité | T3 2026 | 2 semaines |
| 10 | 7.2.5.1 | Revue des comptes applicatifs non automatisée | Script d'audit automatisé des service accounts K8s | Équipe Infrastructure | T3 2026 | 1 semaine |
| 11 | 12.3.3 | Inventaire cryptographique à formaliser | Inventaire de tous les algorithmes et protocoles ; processus de revue annuelle | Équipe Sécurité | T3 2026 | 1 semaine |

### 15.3 Chronogramme de remédiation

```
T2 2026 (Avril - Juin)
  ├── Sem 1-3 : Chiffrement au niveau champ (3.5.1.2)
  ├── Sem 1-2 : Gestion scripts pages paiement (6.4.3)
  ├── Sem 1-3 : Extension MFA au CDE (8.4.2)
  ├── Sem 2-3 : Détection modification pages (11.6.1)
  ├── Sem 2-3 : Anti-phishing complet (5.4.1)
  └── Sem 3-4 : Revue automatisée logs (10.4.2.1)

T3 2026 (Juillet - Septembre)
  ├── Sem 1-4 : Analyses de risques ciblées (12.3.1)
  ├── Sem 1-2 : Documentation réseau (1.1.1)
  ├── Sem 2-3 : Gestion des clés (3.6.1)
  ├── Sem 3   : Revue comptes applicatifs (7.2.5.1)
  └── Sem 3   : Inventaire cryptographique (12.3.3)
```

---

## Références

| Document | Lien |
|----------|------|
| PCI DSS v4.0.1 (juin 2024) | [PCI SSC Document Library](https://www.pcisecuritystandards.org/document_library/) |
| Définition du périmètre CDE | [01-cde-scope-definition.md](./01-cde-scope-definition.md) |
| Guide tokenisation et chiffrement | [03-tokenization-and-encryption-guide.md](./03-tokenization-and-encryption-guide.md) |
| Matrice des responsabilités | [04-responsibility-matrix.md](./04-responsibility-matrix.md) |
| Référentiel légal et normatif | [REFERENTIEL_LEGAL_ET_NORMATIF.md](../../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) |

---

*Document généré dans le cadre du programme de conformité PCI DSS de la plateforme BANKO. Toute modification doit suivre le processus de revue documentaire.*
