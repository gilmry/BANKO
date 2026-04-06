# BANKO — Correspondance des Contrôles de l'Annexe A ISO/IEC 27001:2022

> **Version** : 1.0.0 — 6 avril 2026
> **Statut** : Document initial
> **Classification** : Confidentiel — Usage interne et auditeurs
> **Licence** : AGPL-3.0
> **Auteur** : Projet BANKO
> **Norme de référence** : ISO/IEC 27001:2022, Annexe A (alignée sur ISO/IEC 27002:2022)

---

## Table des matières

1. [Introduction](#1-introduction)
2. [Contrôles organisationnels (A.5.1 — A.5.37)](#2-contrôles-organisationnels-a51--a537)
3. [Contrôles relatifs aux personnes (A.6.1 — A.6.8)](#3-contrôles-relatifs-aux-personnes-a61--a68)
4. [Contrôles physiques (A.7.1 — A.7.14)](#4-contrôles-physiques-a71--a714)
5. [Contrôles technologiques (A.8.1 — A.8.34)](#5-contrôles-technologiques-a81--a834)
6. [Focus — 11 nouveaux contrôles 2022](#6-focus--11-nouveaux-contrôles-2022)
7. [Attributs des contrôles](#7-attributs-des-contrôles)

---

## 1. Introduction

### 1.1 Restructuration de l'Annexe A dans l'édition 2022

L'édition ISO/IEC 27001:2022, seule version en vigueur depuis la fin de la période de transition en octobre 2025, introduit une restructuration majeure de l'Annexe A par rapport à l'édition 2013 :

| Aspect | ISO 27001:2013 | ISO 27001:2022 |
|---|---|---|
| Nombre de contrôles | 114 | **93** |
| Organisation | 14 domaines | **4 thèmes** |
| Nouveaux contrôles | — | **11 contrôles** |
| Contrôles fusionnés | — | 24 contrôles fusionnés en 11 |
| Système d'attributs | Inexistant | **5 dimensions d'attributs** |

### 1.2 Les 4 thèmes de l'Annexe A 2022

| Thème | Identifiant | Nombre de contrôles |
|---|---|---|
| Contrôles organisationnels | A.5 | 37 |
| Contrôles relatifs aux personnes | A.6 | 8 |
| Contrôles physiques | A.7 | 14 |
| Contrôles technologiques | A.8 | 34 |
| **Total** | | **93** |

### 1.3 Objectif du présent document

Ce document établit la correspondance détaillée entre chacun des 93 contrôles de l'Annexe A et leur implémentation technique et organisationnelle au sein de la plateforme BANKO. Pour chaque contrôle, les éléments suivants sont documentés :

- **Module BANKO** : Bounded context(s) ou composant(s) technique(s) concerné(s)
- **Implémentation technique** : Mécanismes concrets dans le code, l'infrastructure ou les processus
- **Statut** : Planned / In Progress / Done

Ce mapping constitue un outil de pilotage pour l'équipe de développement et une preuve d'audit pour les certificateurs.

---

## 2. Contrôles organisationnels (A.5.1 — A.5.37)

| ID | Contrôle | Description | Module BANKO | Implémentation technique | Statut |
|---|---|---|---|---|---|
| A.5.1 | Politiques de sécurité de l'information | Définir, approuver, publier et communiquer des politiques de sécurité | Governance | Document `SECURITY.md` publié dans le dépôt ; politique SMSI formelle à rédiger ; revue annuelle par la direction | Planned |
| A.5.2 | Fonctions et responsabilités liées à la sécurité de l'information | Définir et attribuer les rôles de sécurité | Governance | Matrice RACI dans le module Governance ; rôles RSSI, DPO, responsable conformité documentés | Planned |
| A.5.3 | Séparation des tâches | Séparer les tâches et domaines de responsabilité conflictuels | Governance, Identity | Matrice de séparation des pouvoirs dans le module Governance ; contraintes RBAC empêchant un même utilisateur de créer et valider une opération | Planned |
| A.5.4 | Responsabilités de la direction | La direction doit exiger la conformité de tous les employés | Governance | Charte de sécurité signée par la direction ; engagement formalisé dans le PV du comité de direction | Planned |
| A.5.5 | Relations avec les autorités | Maintenir des contacts avec les autorités compétentes | Governance, Reporting | Module Reporting : templates de rapports BCT, CTAF ; registre des contacts régulateurs ; procédure de notification d'incident | Planned |
| A.5.6 | Relations avec les groupes d'intérêt spécialisés | Maintenir des contacts avec des groupes spécialisés en sécurité | Governance | Abonnement aux CERT tunisien et internationaux (CERT-TCC), participation OWASP, veille sécurité automatisée | Planned |
| A.5.7 | Renseignement sur les menaces | Collecter et analyser le renseignement sur les menaces | AML, Sanctions, Identity | Intégration de flux de threat intelligence (indicateurs de compromission), corrélation avec les alertes AML, veille sur les TTPs du secteur bancaire | Planned |
| A.5.8 | Sécurité de l'information dans la gestion de projet | Intégrer la sécurité dans la gestion de projet | Tous modules | Security by Design : chaque user story inclut des critères de sécurité ; revue de sécurité à chaque milestone ; DPIA pour les fonctionnalités à risque | Planned |
| A.5.9 | Inventaire des informations et autres actifs associés | Identifier et maintenir un inventaire des actifs | Tous modules | Registre des actifs informationnels par bounded context ; classification automatique des données dans PostgreSQL ; inventaire des composants (SBOM) | Planned |
| A.5.10 | Utilisation correcte des informations et autres actifs associés | Définir les règles d'utilisation acceptable | Tous modules | Politique d'utilisation acceptable documentée ; règles d'accès aux données de production ; interdiction de copie de données prod vers dev | Planned |
| A.5.11 | Restitution des actifs | Restituer les actifs à la fin du contrat/emploi | Governance, Identity | Procédure de offboarding : révocation des accès GitHub, K8s, DB ; récupération des clés matérielles ; checklist de départ | Planned |
| A.5.12 | Classification des informations | Classifier les informations selon leur valeur et sensibilité | Tous modules | 4 niveaux de classification : Public, Interne, Confidentiel, Secret bancaire ; étiquetage dans les métadonnées PostgreSQL ; tag des champs dans le schéma | Planned |
| A.5.13 | Étiquetage des informations | Appliquer un marquage conforme à la classification | Tous modules | Headers HTTP de classification, labels sur les documents générés, watermarking des rapports réglementaires | Planned |
| A.5.14 | Transfert des informations | Protéger les informations lors des transferts | Payment, ForeignExchange | TLS 1.3 pour toutes les API ; chiffrement des messages SWIFT ; signature des virements SEPA ; chiffrement des fichiers échangés avec la BCT | Planned |
| A.5.15 | Contrôle d'accès | Définir et mettre en oeuvre des règles de contrôle d'accès | Identity, Governance | RBAC (Role-Based Access Control) avec permissions granulaires par bounded context ; principe du moindre privilège ; revue trimestrielle des accès | Planned |
| A.5.16 | Gestion des identités | Gérer le cycle de vie complet des identités | Identity | Provisionnement/dé-provisionnement automatique ; identités uniques ; corrélation identité-personne ; annuaire centralisé | Planned |
| A.5.17 | Informations d'authentification | Protéger les secrets d'authentification | Identity | Hachage bcrypt/argon2 des mots de passe ; stockage sécurisé des tokens JWT ; rotation automatique des secrets ; pas de credentials en clair dans le code | Planned |
| A.5.18 | Droits d'accès | Provisionner, réviser et révoquer les droits d'accès | Identity, Governance | Workflow d'approbation pour l'attribution de droits ; revue trimestrielle automatisée ; révocation immédiate en cas d'incident ; journalisation des changements | Planned |
| A.5.19 | Sécurité de l'information dans les relations avec les fournisseurs | Gérer les risques liés aux fournisseurs | Infrastructure | Clauses de sécurité dans les contrats d'hébergement ; exigence de certification ISO 27001 ou SOC 2 pour les hébergeurs ; audit annuel des fournisseurs | Planned |
| A.5.20 | Prise en compte de la sécurité dans les accords fournisseurs | Inclure des exigences de sécurité dans les contrats | Infrastructure | Templates de clauses contractuelles de sécurité ; SLA de disponibilité, de notification d'incident, de localisation des données | Planned |
| A.5.21 | Gestion de la sécurité dans la chaîne d'approvisionnement TIC | Surveiller la sécurité de la chaîne d'approvisionnement | Tous modules | `cargo audit` et `npm audit` dans le pipeline CI ; SBOM (Software Bill of Materials) généré à chaque build ; verrouillage des versions (Cargo.lock) | In Progress |
| A.5.22 | Surveillance, revue et gestion des changements des services fournisseurs | Surveiller les changements des services des fournisseurs | Infrastructure | Monitoring des SLA hébergeur ; notifications de changements ; revue annuelle des contrats ; procédure de changement de fournisseur | Planned |
| A.5.23 | Sécurité de l'information pour l'utilisation de services en nuage | Gérer les risques spécifiques au cloud | Infrastructure | Politique de sécurité cloud ; chiffrement des données avant transfert vers le cloud ; gestion des clés indépendante du fournisseur cloud ; localisation des données en Tunisie ou UE | Planned |
| A.5.24 | Planification et préparation de la gestion des incidents | Planifier la gestion des incidents de sécurité | Governance | Procédure de gestion des incidents documentée ; rôles et responsabilités définis ; playbooks par type d'incident ; outils de communication de crise | Planned |
| A.5.25 | Évaluation et décision concernant les événements de sécurité | Qualifier les événements de sécurité | Governance, AML | Critères de qualification (événement/incident/crise) ; matrice de sévérité ; escalade automatique selon le niveau | Planned |
| A.5.26 | Réponse aux incidents de sécurité de l'information | Répondre aux incidents conformément aux procédures | Governance | Playbooks de réponse par type d'incident ; notification INPDP sous 72h (loi données 2025) ; notification BCT ; communication clients si nécessaire | Planned |
| A.5.27 | Enseignements tirés des incidents | Tirer les leçons des incidents pour s'améliorer | Governance | Post-mortem systématique ; mise à jour du registre des risques ; amélioration des contrôles ; partage des enseignements | Planned |
| A.5.28 | Collecte de preuves | Collecter et préserver les preuves | Governance, AML | Journaux d'audit horodatés et signés ; chaîne de custody documentée ; snapshots d'environnement ; exports de logs forensiques | Planned |
| A.5.29 | Sécurité de l'information durant une perturbation | Maintenir la sécurité pendant les crises | Tous modules | Politique de sécurité applicable en mode dégradé ; priorité des contrôles de sécurité même en situation de crise ; procédure de retour à la normale | Planned |
| A.5.30 | Préparation des TIC pour la continuité d'activité | Assurer la résilience des systèmes TIC | Tous modules | PCA/PRA documentés ; RTO < 15 min pour PostgreSQL ; RPO < 1 min ; tests de basculement trimestriels ; réplication géographique | Planned |
| A.5.31 | Exigences légales, statutaires, réglementaires et contractuelles | Identifier et respecter les exigences légales | Tous modules | Référentiel légal et normatif (`REFERENTIEL_LEGAL_ET_NORMATIF.md`) ; matrice de traçabilité norme-module ; veille réglementaire continue | In Progress |
| A.5.32 | Droits de propriété intellectuelle | Respecter la propriété intellectuelle | Tous modules | Licence AGPL-3.0 ; vérification des licences des dépendances (`cargo license`, `license-checker`) ; DCO sign-off obligatoire | In Progress |
| A.5.33 | Protection des enregistrements | Protéger les enregistrements conformément aux obligations | Tous modules | Conservation des données LBC/FT pendant 10 ans ; archivage sécurisé avec chiffrement ; politique de rétention par type de données | Planned |
| A.5.34 | Vie privée et protection des données à caractère personnel | Protéger les données personnelles | Customer, Identity | Conformité loi données 2025 ; registre des traitements ; DPIA ; droit d'accès, de rectification, d'effacement ; chiffrement AES-256 | Planned |
| A.5.35 | Revue indépendante de la sécurité de l'information | Faire auditer le SMSI par des tiers indépendants | Governance | Audit interne annuel ; tests d'intrusion par prestataire accrédité ANCS (circulaire BCT 2025-06) ; audit de certification ISO 27001 | Planned |
| A.5.36 | Conformité aux politiques et normes de sécurité | Vérifier la conformité continue | Governance | Tableau de bord de conformité ; contrôles automatisés dans le pipeline CI ; revue de conformité trimestrielle | Planned |
| A.5.37 | Procédures d'exploitation documentées | Documenter les procédures opérationnelles | Tous modules | Runbooks d'exploitation dans le dépôt ; procédures de déploiement, de rollback, de sauvegarde, de restauration ; Makefile comme point d'entrée | In Progress |

---

## 3. Contrôles relatifs aux personnes (A.6.1 — A.6.8)

| ID | Contrôle | Description | Module BANKO | Implémentation technique | Statut |
|---|---|---|---|---|---|
| A.6.1 | Présélection | Vérifier les antécédents des candidats avant l'embauche | Governance | Vérification des antécédents pour les contributeurs ayant accès aux données de production ; procédure documentée dans le guide de contribution | Planned |
| A.6.2 | Termes et conditions d'emploi | Inclure les obligations de sécurité dans les contrats | Governance | Clauses de confidentialité et de sécurité dans les contrats ; engagement de conformité à la politique de sécurité ; clause de non-divulgation | Planned |
| A.6.3 | Sensibilisation, enseignement et formation à la sécurité | Former le personnel à la sécurité de l'information | Governance | Programme de formation annuel : sécurité des développements (OWASP Top 10), anti-phishing, protection des données, gestion des incidents ; certification requise | Planned |
| A.6.4 | Processus disciplinaire | Sanctionner les violations de la politique de sécurité | Governance | Processus disciplinaire graduel documenté ; sanctions proportionnées ; procédure d'enquête ; protection des lanceurs d'alerte | Planned |
| A.6.5 | Responsabilités après la fin ou le changement d'emploi | Gérer la sécurité lors des départs | Identity, Governance | Procédure de offboarding automatisée : désactivation des comptes sous 24h, révocation des clés SSH et tokens, retrait des accès K8s et DB, restitution du matériel | Planned |
| A.6.6 | Accords de confidentialité ou de non-divulgation | Formaliser les engagements de confidentialité | Governance | NDA obligatoire pour tout contributeur accédant aux données bancaires ; CLA (Contributor License Agreement) pour les contributions open source ; DCO sign-off | Planned |
| A.6.7 | Travail à distance | Sécuriser le travail à distance | Identity, Infrastructure | Accès VPN obligatoire pour les environnements de production ; MFA sur tous les accès distants ; chiffrement du disque des postes de développement ; politique BYOD documentée | Planned |
| A.6.8 | Signalement des événements de sécurité de l'information | Fournir un canal de signalement des incidents | Governance | Canal de signalement dans `SECURITY.md` ; adresse e-mail dédiée ; possibilité de signalement anonyme ; délai de réponse garanti (< 48h) ; programme de divulgation responsable | In Progress |

---

## 4. Contrôles physiques (A.7.1 — A.7.14)

> **Note** : BANKO étant une plateforme logicielle déployée en environnement cloud ou hébergé, la majorité des contrôles physiques sont **délégués aux hébergeurs**. La conformité est assurée contractuellement et vérifiée lors des audits de fournisseurs (cf. A.5.19 et A.5.20).

| ID | Contrôle | Description | Module BANKO | Implémentation technique | Statut |
|---|---|---|---|---|---|
| A.7.1 | Périmètres de sécurité physique | Définir des périmètres de sécurité pour protéger les zones sensibles | Infrastructure (hébergeur) | Exigence contractuelle : centres de données avec périmètre de sécurité physique (clôtures, murs, contrôle d'accès à l'entrée) ; vérification lors des audits fournisseurs | Planned |
| A.7.2 | Contrôles physiques des accès | Protéger les zones sécurisées par des contrôles d'accès | Infrastructure (hébergeur) | Exigence contractuelle : accès par badge, biométrie ou code ; journalisation des accès ; escorte des visiteurs ; revue trimestrielle des droits d'accès physique | Planned |
| A.7.3 | Sécurisation des bureaux, des salles et des équipements | Protéger les bureaux et les salles informatiques | Infrastructure (hébergeur) | Exigence contractuelle : salles serveurs sécurisées avec contrôle d'accès renforcé, système anti-incendie, climatisation redondante | Planned |
| A.7.4 | Surveillance de la sécurité physique | Surveiller en continu les locaux | **Non applicable** | Contrôle exclu — déploiement cloud uniquement, pas de locaux propres. Délégué intégralement à l'hébergeur certifié (cf. [SoA section 6](01-scope-and-statement-of-applicability.md#6-exclusions-justifiées)) | N/A |
| A.7.5 | Protection contre les menaces physiques et environnementales | Protéger contre les menaces naturelles et environnementales | Infrastructure (hébergeur) | Exigence contractuelle intégrant les risques climatiques tunisiens : protection contre les inondations côtières, système de refroidissement dimensionné pour canicules (> 45 °C), alimentation électrique de secours | Planned |
| A.7.6 | Travail dans les zones sécurisées | Définir les règles de travail dans les zones sécurisées | Infrastructure (hébergeur) | Exigence contractuelle : procédures de travail en salle serveur, interdiction de supports non autorisés, supervision des interventions | Planned |
| A.7.7 | Bureau propre et écran verrouillé | Politique de bureau propre et écran verrouillé | Governance | Politique de bureau propre documentée ; verrouillage automatique de l'écran après 5 minutes d'inactivité ; sensibilisation de l'équipe | Planned |
| A.7.8 | Emplacement et protection du matériel | Protéger le matériel contre les risques environnementaux | Infrastructure (hébergeur) | Exigence contractuelle : serveurs installés sur racks sécurisés, protection contre les surtensions, surveillance de la température et de l'humidité | Planned |
| A.7.9 | Sécurité des actifs hors des locaux | Protéger les équipements utilisés hors des locaux | Governance | Chiffrement obligatoire du disque des postes de développement (BitLocker/LUKS) ; localisation à distance (MDM) ; procédure en cas de vol ou perte | Planned |
| A.7.10 | Supports de stockage | Gérer les supports de stockage tout au long de leur cycle de vie | Infrastructure, Identity | Chiffrement de tous les supports de stockage ; destruction sécurisée (NIST SP 800-88) ; registre des supports ; traçabilité des transferts | Planned |
| A.7.11 | Services généraux | Protéger les installations contre les coupures de services | Infrastructure (hébergeur) | Exigence contractuelle : alimentation électrique redondante (UPS + générateurs), climatisation dimensionnée pour les conditions climatiques tunisiennes, connectivité réseau multi-opérateurs | Planned |
| A.7.12 | Sécurité du câblage | Protéger le câblage contre les interceptions et dommages | Infrastructure (hébergeur) | Exigence contractuelle : câblage structuré, fibres optiques protégées, chemins de câbles sécurisés, séparation réseau électrique/données | Planned |
| A.7.13 | Maintenance du matériel | Maintenir le matériel pour assurer sa disponibilité et son intégrité | Infrastructure (hébergeur) | Exigence contractuelle : maintenance préventive planifiée, pièces de rechange sur site, contrats de support constructeur, journalisation des interventions | Planned |
| A.7.14 | Mise au rebut ou réutilisation sécurisée du matériel | Effacer les données avant mise au rebut ou réutilisation | Infrastructure (hébergeur) | Exigence contractuelle : effacement sécurisé conforme NIST SP 800-88, certificat de destruction, destruction physique des disques pour les données classifiées Secret | Planned |

---

## 5. Contrôles technologiques (A.8.1 — A.8.34)

| ID | Contrôle | Description | Module BANKO | Implémentation technique | Statut |
|---|---|---|---|---|---|
| A.8.1 | Terminaux utilisateurs | Protéger les informations stockées sur ou accessibles via les terminaux | Identity, Infrastructure | Politique BYOD ; chiffrement du disque obligatoire ; antivirus/EDR ; verrouillage automatique ; MDM pour les postes ayant accès à la production | Planned |
| A.8.2 | Droits d'accès privilégiés | Restreindre et gérer les droits d'accès privilégiés | Identity, Governance | Comptes administrateurs séparés des comptes utilisateurs ; MFA obligatoire pour les accès privilégiés (DB admin, K8s admin) ; journalisation de toutes les actions privilégiées ; rotation des secrets | Planned |
| A.8.3 | Restriction d'accès aux informations | Restreindre l'accès aux informations selon la politique d'accès | Identity, tous modules | RBAC par bounded context : un utilisateur du module Payment n'accède pas aux données du module Credit ; Row-Level Security dans PostgreSQL ; API gateway avec vérification des permissions | Planned |
| A.8.4 | Accès au code source | Protéger l'accès en lecture et écriture au code source | Governance | Branch protection rules sur GitHub ; revue de code obligatoire (2 approbations) ; accès en écriture restreint aux mainteneurs ; journalisation des accès au dépôt | In Progress |
| A.8.5 | Authentification sécurisée | Mettre en oeuvre des mécanismes d'authentification sécurisés | Identity | JWT avec refresh tokens (durée de vie courte : access 15 min, refresh 7 jours) ; MFA (TOTP, FIDO2) ; protection contre le brute force (rate limiting) ; verrouillage de compte après 5 tentatives | Planned |
| A.8.6 | Dimensionnement des capacités | Surveiller et ajuster les capacités des ressources | Infrastructure | Monitoring Prometheus des ressources (CPU, mémoire, disque, connexions DB) ; alertes sur seuils ; Horizontal Pod Autoscaler K8s ; capacity planning trimestriel | Planned |
| A.8.7 | Protection contre les programmes malveillants | Protéger contre les logiciels malveillants | Tous modules | Analyse statique du code (SAST) dans le pipeline CI ; `cargo audit` pour les vulnérabilités connues dans les dépendances Rust ; `npm audit` pour le frontend ; signature des images Docker | Planned |
| A.8.8 | Gestion des vulnérabilités techniques | Identifier, évaluer et traiter les vulnérabilités | Tous modules | Veille CVE sur les composants utilisés (Actix-web, PostgreSQL, Traefik, MinIO) ; `cargo audit` et `npm audit` dans le CI ; scans de vulnérabilités réguliers ; patch d'urgence < 24h pour les critiques | In Progress |
| A.8.9 | Gestion de la configuration | Documenter et contrôler les configurations | Infrastructure | Infrastructure as Code (Docker Compose, K8s manifests) ; configurations versionnées dans Git ; revue des changements de configuration ; baseline de sécurité pour chaque composant | Planned |
| A.8.10 | Suppression d'informations | Supprimer les informations devenues inutiles | Customer, tous modules | Implémentation du droit à l'effacement (loi données 2025) ; mécanisme de suppression logique puis physique ; politique de rétention par type de données ; vérification de l'effacement effectif | Planned |
| A.8.11 | Masquage des données | Masquer les données conformément aux politiques | Customer, Account | Anonymisation des données pour les environnements de test ; pseudonymisation pour le reporting statistique ; masquage des numéros de compte et données sensibles dans les logs ; `make seed` avec données fictives | Planned |
| A.8.12 | Prévention de la fuite de données | Prévenir la divulgation non autorisée d'informations | Tous modules | Contrôle des exports de données (API de téléchargement avec autorisation) ; DLP sur les communications sortantes ; détection d'exfiltration par analyse des volumes de données ; watermarking des rapports | Planned |
| A.8.13 | Sauvegarde des informations | Maintenir des copies de sauvegarde des données et logiciels | Infrastructure, tous modules | Stratégie 3-2-1 : 3 copies, 2 supports différents, 1 hors site ; sauvegardes PostgreSQL (WAL archiving continu + snapshots quotidiens) ; sauvegardes MinIO ; chiffrement AES-256 des sauvegardes ; tests de restauration mensuels | Planned |
| A.8.14 | Redondance des moyens de traitement de l'information | Assurer la redondance des composants critiques | Infrastructure | PostgreSQL streaming replication avec failover automatique ; déploiement K8s multi-replicas ; réplication MinIO ; équilibrage de charge Traefik ; sites géographiquement distribués | Planned |
| A.8.15 | Journalisation | Enregistrer les événements et générer des preuves | Governance, tous modules | Journaux d'audit structurés (JSON) pour chaque opération métier ; horodatage précis (NTP) ; journalisation des accès, modifications, suppressions ; logs immuables (append-only) ; rétention conforme BCT 2006-19 | Planned |
| A.8.16 | Activités de surveillance | Surveiller les systèmes pour détecter les anomalies | Infrastructure, tous modules | Monitoring Prometheus + Grafana ; endpoint `/metrics` sur chaque service ; alertes sur les anomalies (temps de réponse, erreurs, tentatives d'accès) ; corrélation avec les événements de sécurité | Planned |
| A.8.17 | Synchronisation des horloges | Synchroniser les horloges de tous les systèmes | Infrastructure | NTP obligatoire sur tous les serveurs et conteneurs ; vérification de la synchronisation dans le health check ; horodatage UTC pour les journaux d'audit ; précision < 1 seconde | Planned |
| A.8.18 | Utilisation de programmes utilitaires privilégiés | Restreindre l'utilisation des programmes utilitaires système | Infrastructure | Accès restreint aux outils d'administration (psql, kubectl, docker exec) ; authentification renforcée ; journalisation de toutes les commandes exécutées ; alertes sur les usages inhabituels | Planned |
| A.8.19 | Installation de logiciels sur les systèmes en exploitation | Contrôler l'installation de logiciels en production | Infrastructure | Déploiement exclusivement via le pipeline CI/CD ; images Docker construites et signées dans le CI ; interdiction d'installation manuelle de paquets en production ; K8s admission controllers | Planned |
| A.8.20 | Sécurité des réseaux | Protéger les réseaux et les services réseau | Infrastructure | Network Policies K8s pour la segmentation ; pare-feu au niveau de l'hébergeur ; Traefik comme point d'entrée unique ; filtrage des ports ; détection d'intrusion réseau (IDS) | Planned |
| A.8.21 | Sécurité des services réseau | Sécuriser les services réseau | Infrastructure | TLS 1.3 pour toutes les communications externes ; mTLS entre services K8s ; HSTS avec preload ; OCSP stapling ; certificats Let's Encrypt avec renouvellement automatique | Planned |
| A.8.22 | Séparation des réseaux | Séparer les réseaux en fonction des besoins de sécurité | Infrastructure | Namespaces K8s séparés (dev, staging, production) ; Network Policies interdisant la communication inter-environnements ; VLAN séparés pour la gestion et les données ; DMZ pour les services exposés | Planned |
| A.8.23 | Filtrage web | Filtrer les accès aux ressources web externes | Infrastructure | Filtrage des accès web sortants des conteneurs de production (whitelist) ; WAF (Web Application Firewall) pour les requêtes entrantes ; blocage des domaines malveillants connus ; analyse du trafic HTTPS | Planned |
| A.8.24 | Utilisation de la cryptographie | Définir et appliquer une politique de cryptographie | Identity, tous modules | Chiffrement au repos : AES-256-GCM pour PostgreSQL (TDE) et MinIO ; chiffrement en transit : TLS 1.3 ; hachage des mots de passe : argon2id ; signature des tokens : ECDSA ; gestion des clés : rotation annuelle | Planned |
| A.8.25 | Cycle de vie du développement sécurisé | Intégrer la sécurité dans le cycle de développement | Tous modules | SDLC sécurisé : analyse de menaces (threat modeling), revue de code sécurité, tests SAST/DAST, tests de sécurité dans le CI ; `make audit` ; conformité OWASP SAMM | In Progress |
| A.8.26 | Exigences de sécurité des applications | Définir et respecter les exigences de sécurité applicatives | Tous modules | Validation des entrées au niveau Domain Layer (constructeurs d'entités) ; sanitisation des sorties ; protection CSRF ; headers de sécurité HTTP ; Content Security Policy | In Progress |
| A.8.27 | Architecture de systèmes sécurisés et principes d'ingénierie | Appliquer les principes de sécurité dans l'architecture | Tous modules | Architecture hexagonale (isolation du domaine) ; DDD (bounded contexts) ; principe du moindre privilège ; defense in depth ; fail securely ; zero trust entre services | In Progress |
| A.8.28 | Codage sécurisé | Appliquer les principes de codage sécurisé | Tous modules | **Rust** : sécurité mémoire native (ownership, borrowing, borrow checker) — absence de buffer overflow, use-after-free, data races ; **SQLx** : requêtes typées à la compilation (injection SQL impossible structurellement) ; revue de code obligatoire ; guidelines de codage sécurisé | **In Progress** |
| A.8.29 | Tests de sécurité dans le développement et l'acceptation | Tester la sécurité à toutes les étapes | Tous modules | Tests unitaires de sécurité (`#[cfg(test)]`) ; tests BDD (Cucumber) incluant des scénarios de sécurité ; tests E2E (Playwright) ; tests d'intrusion par prestataire accrédité ANCS (circulaire BCT 2025-06) ; `make test` | Planned |
| A.8.30 | Développement externalisé | Contrôler la sécurité du développement externalisé | Tous modules | Contributions open source soumises à revue de code (2 approbations) ; CI/CD avec tests de sécurité automatisés ; DCO sign-off obligatoire ; CONTRIBUTING.md documenté | In Progress |
| A.8.31 | Séparation des environnements | Séparer les environnements de dev, test et production | Infrastructure | Docker Compose pour le développement local ; environnement de staging isolé ; production sur K8s avec namespaces séparés ; credentials et données différents par environnement ; pas de données de production en dev/staging | Planned |
| A.8.32 | Gestion des changements | Contrôler les changements affectant la sécurité | Tous modules | Git flow : branches feature, PR obligatoires, revue de code, tests CI bloquants ; changelog documenté ; rollback possible à tout moment ; notifications de déploiement | In Progress |
| A.8.33 | Informations de test | Protéger les données utilisées pour les tests | Tous modules | Données de test exclusivement fictives (`make seed`) ; interdiction d'utiliser des données de production ; anonymisation automatique si extraction de données pour benchmarks ; suppression des données de test après usage | Planned |
| A.8.34 | Protection des systèmes d'information durant les tests d'audit | Protéger les systèmes pendant les audits et tests | Infrastructure | Environnement dédié pour les tests d'audit et d'intrusion ; isolation du périmètre de test ; supervision en temps réel durant les tests ; restauration automatique après les tests destructifs | Planned |

---

## 6. Focus — 11 nouveaux contrôles 2022

L'édition 2022 de la norme ISO/IEC 27001 introduit 11 contrôles entièrement nouveaux, qui n'existaient pas dans l'édition 2013. Cette section détaille leur implémentation spécifique dans le contexte de BANKO.

### 6.1 A.5.7 — Renseignement sur les menaces (*Threat Intelligence*)

| Élément | Détail |
|---|---|
| **Objectif** | Collecter, analyser et utiliser le renseignement sur les menaces pour anticiper et prévenir les attaques. |
| **Contexte BANKO** | Le secteur bancaire tunisien est une cible privilégiée pour les cyberattaques (fraude, espionnage économique, hacktivisme). L'évaluation GAFI prévue en 2026-2027 renforce l'urgence d'un dispositif de veille. |
| **Implémentation prévue** | Abonnement aux flux de threat intelligence (CERT-TCC, MISP, AlienVault OTX) ; intégration des indicateurs de compromission (IoC) dans le module AML et Sanctions ; corrélation automatique avec les logs d'accès ; rapports de menaces trimestriels. |
| **Modules concernés** | AML, Sanctions, Identity, Governance |
| **Statut** | Planned |

### 6.2 A.5.23 — Sécurité de l'information pour l'utilisation de services en nuage (*Cloud Security*)

| Élément | Détail |
|---|---|
| **Objectif** | Gérer les risques spécifiques liés à l'utilisation de services cloud. |
| **Contexte BANKO** | Le déploiement de production utilise Kubernetes, potentiellement hébergé chez un fournisseur cloud. Les données bancaires sont soumises à des exigences de localisation et de souveraineté. |
| **Implémentation prévue** | Politique de sécurité cloud formalisée ; chiffrement côté client avant transfert vers le cloud (client-side encryption) ; gestion des clés indépendante du fournisseur (HSM ou KMS propre) ; exigence de localisation des données en Tunisie ou dans une juridiction disposant d'un accord d'adéquation ; audit de la configuration cloud (CIS Benchmarks). |
| **Modules concernés** | Infrastructure, tous modules |
| **Statut** | Planned |

### 6.3 A.5.30 — Préparation des TIC pour la continuité d'activité (*ICT Readiness for Business Continuity*)

| Élément | Détail |
|---|---|
| **Objectif** | Assurer que les systèmes TIC sont prêts à maintenir les opérations en cas de perturbation. |
| **Contexte BANKO** | L'indisponibilité d'un système bancaire a un impact immédiat sur les clients et l'économie. La réglementation BCT exige des plans de continuité. Les risques climatiques tunisiens (cf. [SoA section 4](01-scope-and-statement-of-applicability.md#4-évaluation-du-changement-climatique)) renforcent cette exigence. |
| **Implémentation prévue** | PCA (Plan de Continuité d'Activité) et PRA (Plan de Reprise d'Activité) formalisés ; RTO < 15 min pour les services critiques (Payment, Account) ; RPO < 1 min pour PostgreSQL ; réplication géographique vers un site hors zone à risque climatique ; tests de basculement trimestriels ; intégration des scénarios climatiques. |
| **Modules concernés** | Tous modules (priorité : Payment, Account, Accounting) |
| **Statut** | Planned |

### 6.4 A.7.4 — Surveillance de la sécurité physique (*Physical Security Monitoring*)

| Élément | Détail |
|---|---|
| **Objectif** | Surveiller en continu les locaux pour détecter les accès physiques non autorisés. |
| **Contexte BANKO** | Contrôle exclu du périmètre — voir [SoA section 6](01-scope-and-statement-of-applicability.md#6-exclusions-justifiées). |
| **Justification d'exclusion** | BANKO est une plateforme logicielle sans locaux physiques propres. La surveillance physique est intégralement déléguée aux hébergeurs certifiés via les contrôles A.5.19 et A.5.20. |
| **Statut** | Non applicable |

### 6.5 A.8.9 — Gestion de la configuration (*Configuration Management*)

| Élément | Détail |
|---|---|
| **Objectif** | S'assurer que les configurations matérielles, logicielles, de services et de réseaux sont établies, documentées, mises en oeuvre, surveillées et revues. |
| **Contexte BANKO** | L'infrastructure BANKO repose sur Docker Compose (dev) et Kubernetes (production), avec de multiples composants configurables (PostgreSQL, Traefik, MinIO, Actix-web). |
| **Implémentation prévue** | Infrastructure as Code : `docker-compose.yml`, K8s manifests, Helm charts versionnés dans Git ; baseline de sécurité pour chaque composant (configuration hardening) ; revue des changements de configuration via PR ; outil de détection de dérive de configuration (drift detection) ; inventaire des configurations dans le registre des actifs. |
| **Modules concernés** | Infrastructure |
| **Statut** | Planned |

### 6.6 A.8.10 — Suppression d'informations (*Information Deletion*)

| Élément | Détail |
|---|---|
| **Objectif** | Supprimer les informations stockées dans les systèmes d'information, les dispositifs ou tout autre support de stockage lorsqu'elles ne sont plus nécessaires. |
| **Contexte BANKO** | La loi tunisienne sur la protection des données personnelles (2025) consacre le droit à l'effacement. Parallèlement, la réglementation LBC/FT impose la conservation des données de vigilance pendant 10 ans. Il faut concilier ces deux exigences. |
| **Implémentation prévue** | Politique de rétention par type de données et par bounded context ; mécanisme de suppression en deux étapes (soft delete puis hard delete après confirmation) ; procédure de conciliation rétention LBC/FT vs droit à l'effacement ; vérification de l'effacement effectif (y compris dans les sauvegardes) ; journalisation des suppressions. |
| **Modules concernés** | Customer, Account, tous modules |
| **Statut** | Planned |

### 6.7 A.8.11 — Masquage des données (*Data Masking*)

| Élément | Détail |
|---|---|
| **Objectif** | Réduire l'exposition des données sensibles en appliquant des techniques de masquage. |
| **Contexte BANKO** | Les environnements de développement et de test ne doivent pas contenir de données réelles. Le reporting statistique doit pouvoir exploiter des données sans exposer les informations personnelles. |
| **Implémentation prévue** | Anonymisation des données pour les environnements de dev/test (remplacement des noms, numéros de compte, adresses) ; pseudonymisation réversible pour les besoins de rapprochement analytique ; masquage des données sensibles dans les logs (numéros de carte, mots de passe) ; `make seed` génère exclusivement des données fictives ; conformité avec la norme ISO 27701:2025 pour la vie privée. |
| **Modules concernés** | Customer, Account, Credit, Payment |
| **Statut** | Planned |

### 6.8 A.8.12 — Prévention de la fuite de données (*Data Leakage Prevention*)

| Élément | Détail |
|---|---|
| **Objectif** | Appliquer des mesures de prévention de la fuite de données aux systèmes, réseaux et autres dispositifs. |
| **Contexte BANKO** | Les données bancaires (soldes, transactions, données personnelles) sont des cibles de valeur pour l'exfiltration. La loi données 2025 impose la notification de violation sous 72h. |
| **Implémentation prévue** | Contrôle des API d'export (autorisation granulaire pour les téléchargements) ; détection d'anomalies dans les volumes de données consultées ou exportées ; alertes sur les exports massifs ; watermarking des rapports (traçabilité du document exporté) ; restriction des accès réseau sortants des conteneurs de production ; classification des données intégrée dans les contrôles DLP. |
| **Modules concernés** | Tous modules |
| **Statut** | Planned |

### 6.9 A.8.16 — Activités de surveillance (*Monitoring Activities*)

| Élément | Détail |
|---|---|
| **Objectif** | Surveiller les réseaux, systèmes et applications pour détecter les comportements anormaux et les incidents de sécurité. |
| **Contexte BANKO** | La circulaire BCT 2006-19 exige un système de contrôle interne permanent. La surveillance continue est essentielle pour détecter les fraudes, les intrusions et les anomalies opérationnelles dans un contexte bancaire. |
| **Implémentation prévue** | Prometheus pour la collecte de métriques (endpoint `/metrics` sur chaque service Actix-web) ; Grafana pour la visualisation et les tableaux de bord ; alertes configurées sur les anomalies (temps de réponse > P99 5ms, taux d'erreur, tentatives d'authentification échouées) ; corrélation avec les journaux d'audit du module Governance ; SIEM pour la corrélation des événements de sécurité. |
| **Modules concernés** | Infrastructure, Governance, tous modules |
| **Statut** | Planned |

### 6.10 A.8.23 — Filtrage web (*Web Filtering*)

| Élément | Détail |
|---|---|
| **Objectif** | Gérer les accès aux sites web externes pour réduire l'exposition aux contenus malveillants. |
| **Contexte BANKO** | Les conteneurs de production ne doivent accéder qu'aux ressources réseau strictement nécessaires. Le filtrage web protège également les postes de développement contre les sites de phishing. |
| **Implémentation prévue** | Whitelist des domaines autorisés pour les accès sortants des conteneurs de production (registres de paquets, API partenaires, services de mise à jour) ; WAF (Web Application Firewall) Traefik pour les requêtes entrantes (protection OWASP Top 10) ; blocage des domaines malveillants connus (intégration avec les flux de threat intelligence A.5.7) ; filtrage DNS. |
| **Modules concernés** | Infrastructure |
| **Statut** | Planned |

### 6.11 A.8.28 — Codage sécurisé (*Secure Coding*)

| Élément | Détail |
|---|---|
| **Objectif** | Appliquer les principes de codage sécurisé au développement de logiciels. |
| **Contexte BANKO** | Le choix de Rust comme langage backend offre des garanties structurelles de sécurité mémoire (ownership, borrowing, borrow checker) qui éliminent des classes entières de vulnérabilités (buffer overflow, use-after-free, data races). SQLx fournit des requêtes typées à la compilation, rendant l'injection SQL structurellement impossible. |
| **Implémentation prévue** | Guidelines de codage sécurisé Rust documentées ; validation des entrées au niveau du Domain Layer (constructeurs d'entités avec règles métier) ; utilisation exclusive de SQLx (requêtes typées, pas de format strings pour le SQL) ; architecture hexagonale isolant le domaine de l'infrastructure ; `cargo clippy` avec lints de sécurité activés ; revue de code obligatoire avec focus sécurité ; conformité OWASP Secure Coding Practices. |
| **Modules concernés** | Tous modules |
| **Statut** | **In Progress** — Les garanties structurelles de Rust et SQLx sont déjà effectives. Les guidelines formelles et l'intégration de `clippy` dans le CI sont en cours. |

---

## 7. Attributs des contrôles

### 7.1 Présentation

L'édition 2022 de la norme ISO/IEC 27002 (qui détaille les contrôles de l'Annexe A de l'ISO 27001) introduit un système de **5 dimensions d'attributs** permettant de catégoriser et de filtrer les contrôles selon différentes perspectives. Ce système facilite la sélection des contrôles et leur mise en correspondance avec d'autres référentiels.

### 7.2 Les 5 dimensions d'attributs

| Dimension | Description | Valeurs possibles |
|---|---|---|
| **Type de contrôle** | Nature de l'action de sécurité | Préventif, Détectif, Correctif |
| **Propriétés de sécurité de l'information** | Objectif de sécurité visé (triade CIA) | Confidentialité, Intégrité, Disponibilité |
| **Concepts de cybersécurité** | Phase du cadre NIST CSF correspondante | Identifier, Protéger, Détecter, Répondre, Rétablir |
| **Capacités opérationnelles** | Domaine de compétence opérationnelle | Gouvernance, Gestion des actifs, Protection de l'information, Sécurité des ressources humaines, Sécurité physique, Sécurité des systèmes et réseaux, Sécurité des applications, Configuration sécurisée, Gestion des identités et des accès, Gestion des menaces et des vulnérabilités, Continuité, Sécurité des relations avec les fournisseurs, Conformité, Gestion des événements de sécurité, Assurance de la sécurité de l'information |
| **Domaines de sécurité** | Périmètre organisationnel du contrôle | Gouvernance et écosystème, Protection, Défense, Résilience |

### 7.3 Application aux contrôles BANKO — Exemples

| ID | Contrôle | Type | Propriétés CIA | Concept NIST | Capacité opérationnelle | Domaine |
|---|---|---|---|---|---|---|
| A.5.7 | Renseignement sur les menaces | Préventif | C, I, A | Identifier, Détecter | Gestion des menaces et des vulnérabilités | Défense |
| A.5.23 | Sécurité cloud | Préventif | C, I, A | Protéger | Sécurité des systèmes et réseaux | Gouvernance, Protection |
| A.5.30 | Continuité TIC | Correctif | A | Rétablir | Continuité | Résilience |
| A.8.9 | Gestion de la configuration | Préventif | C, I, A | Protéger | Configuration sécurisée | Protection |
| A.8.10 | Suppression d'informations | Préventif | C | Protéger | Protection de l'information | Protection |
| A.8.11 | Masquage des données | Préventif | C | Protéger | Protection de l'information | Protection |
| A.8.12 | Prévention de la fuite de données | Préventif, Détectif | C | Protéger, Détecter | Protection de l'information | Protection, Défense |
| A.8.16 | Activités de surveillance | Détectif | C, I, A | Détecter | Gestion des événements de sécurité | Défense |
| A.8.23 | Filtrage web | Préventif | C, I, A | Protéger | Sécurité des systèmes et réseaux | Protection |
| A.8.28 | Codage sécurisé | Préventif | C, I, A | Protéger | Sécurité des applications | Protection |

### 7.4 Utilité des attributs pour BANKO

Le système d'attributs permet de :

1. **Prioriser les contrôles** : En phase initiale, privilégier les contrôles préventifs pour construire une base solide, puis ajouter les contrôles détectifs et correctifs.

2. **Aligner avec le NIST CSF** : Vérifier la couverture de chaque fonction du NIST Cybersecurity Framework (Identify, Protect, Detect, Respond, Recover).

3. **Répondre aux exigences BCT** : La circulaire BCT 2006-19 met l'accent sur le contrôle interne (préventif) et la surveillance (détectif). Les attributs permettent de filtrer les contrôles pertinents.

4. **Faciliter les audits** : Les auditeurs peuvent utiliser les attributs pour vérifier la couverture par domaine de sécurité ou par capacité opérationnelle.

5. **Compléter avec ISO 27701:2025** : La norme ISO 27701:2025, désormais autonome, couvre la vie privée y compris pour l'IA, la biométrie et l'IoT. Les attributs facilitent l'identification des contrôles ayant un impact sur la protection des données personnelles (propriété : Confidentialité).

---

> **Documents associés** :
> - [01-scope-and-statement-of-applicability.md](01-scope-and-statement-of-applicability.md) — Périmètre du SMSI et SoA
> - [02-risk-assessment-register.md](02-risk-assessment-register.md) — Registre des risques
> - [04-implementation-plan.md](04-implementation-plan.md) — Plan d'implémentation ISO 27001
> - [Référentiel légal et normatif](../../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) — Cadre réglementaire tunisien
>
> **Prochaine revue prévue** : Juillet 2026
>
> **Approbation** : Ce document doit être validé par le RSSI avant utilisation dans le cadre d'un audit de certification.
