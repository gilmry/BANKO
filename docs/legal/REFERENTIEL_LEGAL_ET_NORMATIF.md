# BANKO — Référentiel Légal et Normatif

> **Version** : 0.3.0 — 6 avril 2026
> **Statut** : Document fondateur — Phase TOGAF Architecture Vision
> **Licence** : AGPL-3.0
> **Auteur** : GILMRY / Projet BANKO

---

## Table des matières

1. [Objectif de ce document](#1-objectif-de-ce-document)
2. [Cadre institutionnel tunisien](#2-cadre-institutionnel-tunisien)
3. [Lois fondamentales](#3-lois-fondamentales)
4. [Circulaires BCT — Réglementation prudentielle](#4-circulaires-bct--réglementation-prudentielle)
   - 4.6 [Réforme prudentielle et préparation IFRS 9](#46-réforme-prudentielle-et-préparation-ifrs-9)
   - 4.7 [Plateforme TuniCheque](#47-plateforme-tunicheque)
   - 4.8 [Enrôlement électronique (e-KYC)](#48-enrôlement-électronique-e-kyc)
   - 4.9 [Plateforme numérique de change](#49-plateforme-numérique-de-change)
5. [Lutte anti-blanchiment et financement du terrorisme (LBC/FT)](#5-lutte-anti-blanchiment-et-financement-du-terrorisme-lbcft)
   - 5.4 [Circulaire BCT 2026 — Bureaux de change](#54-circulaire-bct-2026--bureaux-de-change)
   - 5.5 [Stratégie CMF 2026](#55-stratégie-cmf-2026)
   - 5.6 [Évaluation GAFI 2026 — Risque critique](#56-évaluation-gafi-2026--risque-critique)
6. [Gouvernance bancaire](#6-gouvernance-bancaire)
7. [Normes comptables](#7-normes-comptables)
8. [Réglementation des changes](#8-réglementation-des-changes)
9. [Protection des données personnelles](#9-protection-des-données-personnelles)
   - 9.2 [Nouvelle loi organique sur la protection des données personnelles (2025)](#92-nouvelle-loi-organique-sur-la-protection-des-données-personnelles-2025)
10. [Marché financier et valeurs mobilières](#10-marché-financier-et-valeurs-mobilières)
11. [Normes internationales](#11-normes-internationales)
    - 11.5 [ISO/IEC 27001:2022](#115-isoiec-270012022--système-de-management-de-la-sécurité-de-linformation)
    - 11.6 [ISO/IEC 27701:2025](#116-isoiec-277012025--management-de-la-vie-privée)
    - 11.7 [PCI DSS v4.0.1](#117-pci-dss-v401)
    - 11.8 [Open Banking — PSD3/PSR et FIDA](#118-open-banking--psd3psr-et-fida)
12. [Matrice de traçabilité Norme → Module BANKO](#12-matrice-de-traçabilité-norme--module-banko)
13. [Sources centralisées](#13-sources-centralisées)

---

## 1. Objectif de ce document

Ce référentiel constitue le socle fondateur du projet BANKO. Il inventorie de manière exhaustive et sourcée l'ensemble des textes légaux, réglementaires et normatifs auxquels un système bancaire tunisien doit se conformer. Chaque module de BANKO sera conçu et développé en traçabilité directe avec les exigences listées ici.

L'approche est conforme au cadre TOGAF (The Open Group Architecture Framework), phase A — Architecture Vision, et à la méthodologie BMAD (Business Mission Architecture Design) : on part de la base légale pour dériver l'architecture métier, puis l'architecture applicative.

**Principe directeur** : aucune fonctionnalité de BANKO ne sera implémentée sans référence traçable à un texte légal ou normatif.

**Convention de citation** : chaque texte est identifié par un code `[REF-XX]` renvoyant à la [Section 13 — Sources centralisées](#13-sources-centralisées), où figurent l'URL officielle, le type de document et la date de vérification.

---

## 2. Cadre institutionnel tunisien

### 2.1 Autorités de régulation

| Institution | Sigle | Rôle | Réf. |
|---|---|---|---|
| Banque Centrale de Tunisie | BCT | Supervision bancaire, politique monétaire, réglementation prudentielle et des changes | [REF-01] |
| Commission Tunisienne des Analyses Financières | CTAF | Cellule de renseignement financier (FIU), réception et analyse des déclarations de soupçon | [REF-02] |
| Conseil du Marché Financier | CMF | Régulation du marché financier, protection des investisseurs | [REF-03] |
| Conseil Bancaire et Financier | CBF | Organe disciplinaire du secteur bancaire | [REF-04] |
| Instance Nationale de Protection des Données Personnelles | INPDP | Protection des données à caractère personnel | [REF-05] |
| Fonds de Garantie des Dépôts Bancaires | FGDB | Garantie des dépôts, résolution bancaire | [REF-06] |
| Ministère des Finances | MF | Tutelle, politique fiscale et réglementation comptable | [REF-07] |
| Ordre des Experts-Comptables de Tunisie | OECT | Normalisation comptable, audit | [REF-08] |

### 2.2 Associations professionnelles

| Association | Rôle |
|---|---|
| Association Professionnelle Tunisienne des Banques et Établissements Financiers (APTBEF) | Représentation des intérêts du secteur bancaire |
| Institut de Financement et de Banque de Tunisie (IFBT) | Formation professionnelle bancaire |

---

## 3. Lois fondamentales

### 3.1 Loi bancaire principale

**Loi n° 2016-48 du 11 juillet 2016** — Relative aux banques et aux établissements financiers [REF-09] [REF-10] [REF-11]

C'est la loi fondamentale qui régit toute l'activité bancaire en Tunisie. Elle définit :
- Les opérations bancaires (réception de dépôts, octroi de crédits, moyens de paiement)
- Les catégories d'établissements (banques, établissements financiers, établissements de paiement)
- Les conditions d'agrément et d'exercice
- Les règles de gouvernance (articles 49, 50, 51 : comités d'audit, de risques, de nomination/rémunération)
- Le dispositif de résolution et de liquidation
- Le rôle de supervision de la BCT

**Implications BANKO** : Toute l'architecture métier du système (modules comptes, crédits, paiements, reporting) dérive directement de cette loi.

---

### 3.2 Statuts de la Banque Centrale de Tunisie

**Loi n° 2016-35 du 25 avril 2016** — Portant fixation du statut de la Banque Centrale de Tunisie [REF-12]

Définit l'indépendance, les missions et les pouvoirs de supervision de la BCT.

---

### 3.3 Système comptable des entreprises

**Loi n° 96-112 du 30 décembre 1996** — Relative au système comptable des entreprises [REF-13]

Établit le cadre conceptuel comptable tunisien et les obligations de tenue comptable.

**Décret n° 96-2459 du 30 décembre 1996** — Portant approbation du cadre conceptuel de la comptabilité [REF-13]

---

### 3.4 Loi sur la sécurité des relations financières

**Loi n° 2005-96 du 18 octobre 2005** — Relative au renforcement de la sécurité des relations financières

Renforce les obligations de transparence et d'audit pour les sociétés faisant appel public à l'épargne.

---

## 4. Circulaires BCT — Réglementation prudentielle

### 4.1 Division et couverture des risques

**Circulaire BCT n° 91-24 du 17 décembre 1991** — Division, couverture des risques et suivi des engagements [REF-14] [REF-15] [REF-16]

C'est la circulaire fondatrice du dispositif prudentiel tunisien, entrée en vigueur le 2 janvier 1992. Elle fixe :
- **Ratio de concentration** : les risques sur un même bénéficiaire ne doivent pas excéder **25% des fonds propres nets**
- **Ratio de division** : total des grands risques (≥ 5% FPN) ≤ **3 fois les fonds propres nets**
- **Ratio de division renforcé** : total des risques ≥ 15% FPN ≤ **1,5 fois les fonds propres nets**
- Règles de classification des créances et de provisionnement
- Obligations de suivi des engagements

**Implications BANKO** : Module de gestion des engagements, calcul automatique des ratios de concentration, système d'alertes.

---

### 4.2 Ratio de solvabilité et adéquation des fonds propres

**Circulaire BCT n° 2016-03** — Ratio de solvabilité [REF-17] [REF-18]

Introduit des exigences renforcées en fonds propres pour convergence vers Bâle III :
- **Ratio de solvabilité global minimum** : **10%**
- **Ratio Tier 1 (fonds propres de base)** : **7%**
- Exigences en fonds propres pour risque opérationnel : **15% du PNB moyen** sur 3 ans

---

**Circulaire BCT n° 2018-06 du 5 juin 2018** — Normes d'adéquation des fonds propres [REF-19] [REF-20]

Remplace et complète les dispositions de la circulaire 91-24 relatives à la couverture des risques. Met à jour :
- Calcul des exigences en fonds propres pour risque de crédit
- Exigences pour risque de marché (taux d'intérêt, actions) — applicables au 31/12/2018
- Risque opérationnel : **15% du PNB moyen sur 3 exercices**
- Convergence vers les standards de Bâle III

**Implications BANKO** : Module de calcul prudentiel automatisé (ratio de solvabilité, Tier 1, RWA, exigences par type de risque).

---

### 4.3 Ratio Crédits/Dépôts (liquidité)

**Circulaire BCT n° 2018-10** — Ratio Crédits/Dépôts [REF-21] [REF-22] [REF-23]

Institue un ratio de transformation pour maîtriser le risque de liquidité :
- **Seuil maximum** : ratio Crédits/Dépôts ≤ **120%**
- Les banques dépassant 120% doivent prendre des mesures correctives à chaque trimestre
- Intégration des ressources spéciales en devises au dénominateur
- S'inscrit dans la convergence vers le LCR (Liquidity Coverage Ratio) de Bâle III

**Implications BANKO** : Calcul en temps réel du ratio C/D, reporting trimestriel, alertes de dépassement.

---

### 4.4 Classification des créances et provisionnement

**Circulaire BCT n° 91-24 (sections relatives)** + modifications successives [REF-14]

Établit les classes de créances :
- **Classe 0** : Actifs courants
- **Classe 1** : Actifs nécessitant un suivi particulier
- **Classe 2** : Actifs incertains (provisionnement 20%)
- **Classe 3** : Actifs préoccupants (provisionnement 50%)
- **Classe 4** : Actifs compromis (provisionnement 100%)

**Circulaire BCT n° 2023-02 du 24 février 2023** — Modifications au dispositif de provisionnement [REF-24] [REF-25]

**Implications BANKO** : Moteur de classification automatique des créances, calcul des provisions, reporting réglementaire.

---

### 4.5 Distribution des dividendes

**Circulaire BCT annuelle** — Conditions de distribution des dividendes [REF-26] [REF-27]

Plafonne la distribution à **35% du résultat net** pour les banques respectant les ratios de solvabilité et Tier 1.

---

### 4.6 Réforme prudentielle et préparation IFRS 9

**Circulaire BCT n° 2025-01 du 29 janvier 2025** — Modifications aux dispositions sur la division, couverture des risques et suivi des engagements [REF-71]

Remplace l'Annexe III de la circulaire historique n° 91-24.

---

**Circulaire BCT n° 2025-08 du 16 mai 2025** — Mesures préparatoires aux réformes prudentielles et normatives majeures [REF-74]

Circulaire structurante imposant aux banques un ultimatum de 3 mois pour soumettre leurs plans stratégiques et opérationnels. Trois piliers :
- **Nouvelles normes d'adéquation des fonds propres** — applicables à partir de **2026**
- **Nouvelles règles de classification et couverture des risques** — applicables à partir de **2027**
- **Adoption des IFRS 9** — modèle de pertes attendues (ECL, Expected Credit Loss)
- Mécanismes de gouvernance des données pour le calcul des ratios prudentiels
- Délai de conformité : **13 août 2025**

**Implications BANKO** : Module prudentiel à adapter pour les nouvelles normes de capital (2026), moteur ECL conforme IFRS 9, infrastructure de données prudentielles.

---

### 4.7 Plateforme TuniCheque

**Circulaire BCT n° 2025-03 du 31 janvier 2025** — Conditions de fonctionnement de la plateforme électronique unifiée des chèques (TuniCheque) [REF-72]

Lancée le 2 février 2025, développée avec SIBTEL et la fintech Kaoun :
- Vérification en temps réel de la couverture des chèques, du statut du compte et des interdictions
- Adhésion obligatoire pour toutes les banques et La Poste Tunisienne

**Circulaire BCT n° 2025-04** — Liste initiale des établissements adhérents à TuniCheque

**Implications BANKO** : Module de vérification chèques intégré, API TuniCheque, alertes interdictions.

---

### 4.8 Enrôlement électronique (e-KYC)

**Circulaire BCT n° 2025-06 du 28 février 2025** — Règles minimales régissant l'enrôlement électronique des clients bancaires [REF-73]

Encadre les processus automatisés/semi-automatisés d'identification numérique :
- Reconnaissance faciale et biométrie pour vérification d'identité équivalente à l'entretien en personne
- **Tests d'intrusion obligatoires** par des auditeurs accrédités par l'**ANCS** (Agence Nationale de la Cybersécurité)
- Cycle d'audit technologique de **2 ans**
- Délai de conformité : **6 mois** pour les banques déjà autorisées
- Les prestataires externalisés doivent satisfaire les mêmes exigences de sécurité

**Implications BANKO** : Module Identity avec enrôlement numérique, intégration biométrie (FIDO2/WebAuthn), cycle d'audit ANCS.

---

### 4.9 Plateforme numérique de change

**Circulaire BCT n° 2025-12 du 9 octobre 2025** — Système numérique de dépôt et traitement des demandes d'autorisation de change [REF-75]

**Implications BANKO** : Intégration API BCT pour les autorisations de change dématérialisées.

---

## 5. Lutte anti-blanchiment et financement du terrorisme (LBC/FT)

### 5.1 Cadre légal

**Loi organique n° 2015-26 du 7 août 2015** — Relative à la lutte contre le terrorisme et à la répression du blanchiment d'argent [REF-28] [REF-29] [REF-02]

Loi fondamentale LBC/FT qui :
- Criminalise le blanchiment d'argent et le financement du terrorisme
- Crée la CTAF (Commission Tunisienne des Analyses Financières) — article 118
- Impose l'obligation de déclaration de soupçon auprès de la CTAF — article 125
- Définit les obligations de vigilance (CDD) pour les institutions financières
- Impose la conservation des données pendant **10 ans minimum**

---

**Loi organique n° 2019-9 du 23 janvier 2019** — Modifiant et complétant la loi organique n° 2015-26 [REF-30]

Renforce le dispositif LBC/FT, élargit les obligations de vigilance.

---

### 5.2 Circulaires BCT LBC/FT

**Circulaire BCT n° 2017-08 du 19 septembre 2017** — Règles de contrôle interne pour la gestion du risque de blanchiment d'argent et de financement du terrorisme [REF-31] [REF-32]

Impose aux banques :
- L'élaboration d'une fiche KYC (Know Your Customer) conforme à l'Annexe 1
- La connaissance de l'identité, de la situation juridique, professionnelle, économique et financière du client
- La connaissance de l'actionnariat des clients personnes morales
- La mise en place d'un dispositif de contrôle interne LBC/FT

---

**Circulaire BCT n° 2025-17 du 22 décembre 2025** — Nouveau cadre LBC/FT/FP [REF-33]

Remplace et durcit la circulaire 2017-08. Apports majeurs :
- Recalibrage complet du dispositif LBC/FT
- Introduction explicite de la lutte contre le **financement de la prolifération d'armes de destruction massive (FP)**
- Renforcement des exigences KYC, filtrage, surveillance et gouvernance de la conformité
- Scénarios d'investigation renforcés
- Procédures de gel des avoirs
- **Applicable immédiatement** sans phase transitoire (depuis le 22/12/2025)

**Implications BANKO** : Module KYC/CDD complet, moteur de filtrage (listes sanctions), surveillance transactionnelle, déclarations de soupçon automatisées, audit trail intégral.

---

### 5.3 Guide CMF LBC/FT

**Guide CMF 2018** — Relatif à la lutte contre le blanchiment d'argent, le financement du terrorisme [REF-34]

---

### 5.4 Circulaire BCT 2026 — Bureaux de change

**Circulaire BCT n° 2026-02 du 23 janvier 2026** — Nouveau cadre LBC/FT pour les bureaux de change [REF-77]

Obligations renforcées :
- Identification systématique avec documents d'identité officiels pour **toute transaction**
- Screening obligatoire contre les listes de sanctions nationales et internationales (ONU, GAFI)
- Vigilance renforcée pour les PPE et clients de juridictions à haut risque GAFI
- **Inscription obligatoire sur la plateforme goAML** de la CTAF
- Conservation documentaire **10 ans**
- Sanctions : de l'avertissement à la fermeture définitive

---

### 5.5 Stratégie CMF 2026

**Stratégie CMF 2026 LBC/FT/FP** — Adoptée le 2 janvier 2026 [REF-81]

Trois axes stratégiques :
- Renforcement de l'adhésion des acteurs (approche basée sur les risques)
- Développement des compétences (formation de 500+ professionnels)
- Amélioration de la qualité du reporting (60+ missions de contrôle ciblées annuelles)

**Règlement CMF anti-blanchiment** — Publié au JORT n° 9 du 23 janvier 2026 [REF-82]

Extension du périmètre LBC/FT au **crowdfunding en valeurs mobilières** (première en Tunisie).

---

### 5.6 Évaluation GAFI 2026 — Risque critique

**Mission d'inspection GAFI sur site** : 26 janvier — 13 février 2026
**Plénière de discussion des résultats** : **1er novembre 2026** (5ème cycle, méthodologie 2022)

La Tunisie n'est **pas** actuellement sur la liste grise du GAFI mais fait face à un risque réel :
- 26,985 milliards TND de cash en circulation (+20% annuel)
- Suppression du plafond de 5 000 TND pour les paiements en espèces (Loi Finances 2026)
- La nouvelle méthodologie (2022) met l'accent sur l'**effectivité** et non la simple conformité technique

**Implications BANKO** : Les modules KYC, surveillance transactionnelle et déclaration de soupçon doivent démontrer une effectivité mesurable (statistiques, traçabilité, résultats).

---

## 6. Gouvernance bancaire

### 6.1 Contrôle interne

**Circulaire BCT n° 2006-19 du 28 novembre 2006** — Contrôle interne [REF-35] [REF-36]

Établit l'obligation pour les établissements de crédit de mettre en place :
- Un système de contrôle interne permanent
- Un comité d'audit interne
- Des processus assurant la sécurité, l'efficacité et l'efficience des opérations bancaires
- 9 articles traitant spécifiquement du risque de crédit

---

### 6.2 Cadre de gouvernance

**Circulaire BCT n° 2021-05 du 19 août 2021** — Cadre de gouvernance des banques et des établissements financiers [REF-37] [REF-38] [REF-39] [REF-40]

Circulaire majeure qui instaure une culture du risque renforcée :
- S'applique à toutes les banques et établissements financiers (loi 2016-48), sauf établissements de paiement
- Renforcement de l'indépendance des trois lignes de défense : **audit, risque, conformité**
- Comités obligatoires (articles 49, 50, 51 de la loi 2016-48) :
  - **Comité d'audit**
  - **Comité de risques**
  - **Comité de nomination et de rémunération**
- Responsabilité accrue de l'organe d'administration (pilotage stratégique et surveillance)
- Délai de conformité : 1 an après publication

**Implications BANKO** : Module de gouvernance avec workflows d'approbation multi-niveaux, séparation des pouvoirs, piste d'audit intégrale.

---

### 6.3 Supervision et reporting BCT

**Circulaire BCT n° 2018-09 du 18 octobre 2018** — Reporting réglementaire [REF-41]

---

## 7. Normes comptables

### 7.1 Cadre comptable tunisien

**Normes Comptables Tunisiennes (NCT)** — Corpus de 42+ normes [REF-42] [REF-43] [REF-44] [REF-45] [REF-46]

Le système comptable tunisien est structuré en :
- **NCT 01** : Norme comptable générale
- **NCT 21** : Présentation des états financiers des établissements bancaires
- **NCT 22** : Contrôle interne et organisation comptable dans les établissements bancaires
- **NCT 24** : Engagements et revenus y afférents dans les établissements bancaires
- **NCT 25** : Portefeuille-titres dans les établissements bancaires

---

### 7.2 Transition vers les IFRS

La BCT a érigé la transition vers les IFRS en projet stratégique (Plan BCT 2019-2021). Les normes clés pour le secteur bancaire [REF-47] [REF-48] :

- **IFRS 9** — Instruments financiers (classification, dépréciation ECL, couverture)
  - Impact majeur sur le provisionnement : passage du modèle « pertes avérées » (NCT) au modèle « pertes attendues » (ECL)
- **IFRS 7** — Informations à fournir sur les instruments financiers
- **IFRS 15** — Produits des activités ordinaires tirés de contrats
- **IFRS 16** — Contrats de location
- **IAS 1** — Présentation des états financiers

**Implications BANKO** : Double moteur comptable (NCT actuel + IFRS en préparation), module de provisionnement ECL.

---

## 8. Réglementation des changes

**Loi n° 76-18 du 21 janvier 1976** — Code des changes et du commerce extérieur [REF-49] [REF-50] [REF-51] [REF-52]

Loi fondamentale qui régit toutes les opérations de change entre la Tunisie et l'étranger :
- La BCT est chargée de l'application de la réglementation des changes
- Les opérations de change transitent obligatoirement par la BCT ou des intermédiaires agréés
- Contrôle des mouvements de capitaux

**Décret n° 77-608 du 27 juillet 1977** — Conditions d'application de la loi 76-18

**Circulaire BCT n° 2018-07 du 30 juillet 2018** — Activité de change manuel [REF-53]

**Implications BANKO** : Module de gestion des opérations en devises, contrôle de conformité change, reporting BCT.

---

## 9. Protection des données personnelles

**Loi organique n° 2004-63 du 27 juillet 2004** — Protection des données à caractère personnel [REF-54] [REF-55] [REF-56] [REF-57]

Loi fondamentale de la protection des données en Tunisie :
- Reconnaissance de la protection des données comme droit fondamental
- Obligations de légalité, transparence et proportionnalité
- Nécessité d'une base légale ou du consentement pour tout traitement
- Création de l'INPDP (Instance Nationale de Protection des Données Personnelles)
- Droits des personnes : accès, rectification, opposition
- Tunisie partie à la **Convention 108 du Conseil de l'Europe** depuis le 1er novembre 2017

**Implications BANKO** : Module privacy-by-design, gestion du consentement, droit d'accès/rectification/opposition, chiffrement, anonymisation, journalisation des accès.

---

### 9.2 Nouvelle loi organique sur la protection des données personnelles (2025)

**Loi organique adoptée le 20 juin 2025, ratifiée le 10 juillet 2025** — Remplace la loi n° 2004-63 [REF-79]

**Date d'application complète : 11 juillet 2026**

Alignement sur le RGPD européen et la Convention 108+ :
- **DPO obligatoire** (Délégué à la Protection des Données) pour les organismes traitant des données sensibles
- **Notification de violation sous 72 heures** auprès de l'INPDP
- **DPIA** (Data Protection Impact Assessment) obligatoire pour les traitements à haut risque
- **Amendes proportionnelles au chiffre d'affaires** (modèle RGPD)
- **Droits nouveaux** : portabilité des données, droit à l'effacement
- **Mesures techniques obligatoires** : chiffrement et pseudonymisation
- Cycle de révision législative de **5 ans**
- L'INPDP transformée en autorité quasi-judiciaire avec pouvoir d'exécution et indépendance administrative

**Implications BANKO** : Module privacy-by-design complet, gestion du consentement granulaire, APIs de portabilité/effacement, DPO dashboard, processus DPIA intégré, chiffrement obligatoire de toutes les données personnelles, notification automatisée de violations.

---

## 10. Marché financier et valeurs mobilières

**Loi n° 94-117 du 14 novembre 1994** — Portant réorganisation du marché financier [REF-58] [REF-59] [REF-60]

Crée le CMF et établit le cadre pour :
- Les émissions et admissions de valeurs mobilières
- Les obligations d'information financière
- Les déclarations de franchissement de seuil

Modifiée par la **Loi n° 2005-96** et la **Loi n° 2009-64**.

---

## 11. Normes internationales

### 11.1 Comité de Bâle — Normes prudentielles

| Norme | Objet | Seuils tunisiens | Statut en Tunisie | Réf. |
|---|---|---|---|---|
| **Bâle III — Pilier 1** | Ratio de solvabilité (CET1, Tier 1, Total) | 10% total, 7% Tier 1 | En cours d'adoption (Circ. 2016-03, 2018-06) | [REF-61] |
| **Bâle III — LCR** | Ratio de couverture de liquidité | Convergence progressive | En cours (Circ. 2018-10 comme étape) | [REF-61] |
| **Bâle III — NSFR** | Ratio de financement stable net | Non encore adopté | Prévu | [REF-61] |
| **Bâle III — Pilier 2** | Processus de surveillance prudentielle (ICAAP) | — | En cours | [REF-61] |
| **Bâle III — Pilier 3** | Discipline de marché (publication) | — | En cours | [REF-61] |
| **Bâle IV (2023)** | Output floor, standardisation risque crédit | — | Non encore adopté | [REF-62] |
| **Principes fondamentaux de Bâle (2024)** | 29 principes pour une supervision efficace | — | Référence | [REF-63] |

---

### 11.2 GAFI/FATF — Lutte anti-blanchiment

| Standard | Objet | Statut | Réf. |
|---|---|---|---|
| **40 Recommandations GAFI** | Cadre complet LBC/FT/FP | Transposées (Loi 2015-26, Circ. 2017-08, 2025-17) | [REF-64] |
| **Recommandation 1** (mise à jour fév. 2025) | Approche basée sur les risques + inclusion financière | À intégrer | [REF-65] |
| **Recommandation 16** (mise à jour juin 2025) | Transparence des paiements transfrontaliers | À intégrer | [REF-66] |
| **5ème cycle d'évaluations mutuelles** (2024+) | Méthodologie d'évaluation renforcée | En cours | [REF-67] |

---

### 11.3 Normes IFRS (cf. section 7.2)

### 11.4 Normes ISO applicables

| Norme ISO | Objet | Application BANKO |
|---|---|---|
| **ISO 20022** | Messagerie financière universelle | Formats de paiement, virements, SWIFT |
| **ISO 27001** | Système de management de la sécurité de l'information | Sécurité SI bancaire |
| **ISO 27701** | Extension vie privée de l'ISO 27001 | Protection des données clients |
| **ISO 22301** | Continuité d'activité | PCA/PRA bancaire |
| **ISO 31000** | Management du risque | Cadre de gestion des risques |
| **ISO 8583** | Messages de transactions financières (cartes) | Interopérabilité monétique |

---

### 11.5 ISO/IEC 27001:2022 — Système de management de la sécurité de l'information

**Norme publiée le 25 octobre 2022** — Seule édition valide depuis le 31 octobre 2025 [REF-86]

Restructuration majeure par rapport à l'édition 2013 :
- **93 contrôles** organisés en **4 thèmes** (vs 114 contrôles / 14 domaines) :
  - Organisationnels (37), Personnes (8), Physiques (14), Technologiques (34)
- **11 nouveaux contrôles** : Threat intelligence, Sécurité cloud, Gestion de configuration, Suppression d'information, Masquage de données, Prévention des fuites de données, Activités de surveillance, Filtrage web, Codage sécurisé, Préparation TIC pour la continuité d'activité, Surveillance de la sécurité physique

**Amendement 1:2024** (ISO/IEC 27001:2022/Amd 1:2024) [REF-87] : Intégration de l'évaluation des risques liés au changement climatique (clauses 4.1 et 4.2).

**Adoption en Tunisie** : Seules l'ATB (certifiée depuis 2017, périmètre e-banking) et l'UIB (certifiée 2022, périmètre SGSS) disposent de la certification ISO 27001.

**Voir** : [Documentation ISO 27001 BANKO](../compliance/iso-27001/01-scope-and-statement-of-applicability.md)

---

### 11.6 ISO/IEC 27701:2025 — Management de la vie privée

**Norme publiée en octobre 2025** — Standard désormais **autonome** (ne requiert plus ISO 27001) [REF-89]

Couverture étendue : profilage par IA, décisions automatisées, données biométriques, IoT, transferts transfrontaliers.

**Implications BANKO** : Pertinent pour les modules Identity (biométrie e-KYC) et Customer (données personnelles).

---

### 11.7 PCI DSS v4.0.1

**Version publiée en juin 2024** — Errata de la v4.0 (mars 2022) [REF-90]

Depuis le **31 mars 2025**, toutes les exigences précédemment « future-dated » sont devenues **obligatoires** :
- **Req 3.5.1.2** : Chiffrement au niveau disque seul **plus suffisant** — chiffrement au niveau champ obligatoire
- **Req 6.4.3** : Gestion des scripts sur les pages de paiement
- **Req 8.4.2** : MFA pour tout accès au CDE (pas seulement accès distant)
- **Req 11.6.1** : Détection de modification/altération sur les pages de paiement

**Voir** : [Documentation PCI DSS BANKO](../compliance/pci-dss/01-cde-scope-definition.md)

---

### 11.8 Open Banking — PSD3/PSR et FIDA

**PSD3/PSR** : Accord politique provisoire le 27 novembre 2025 [REF-91]. Abroge la PSD2 et la Directive Monnaie Électronique. Publication prévue ~Q2 2026, transposition nationale 18 mois après.

Apports : APIs standardisées, règles anti-obstruction, dashboards client, vérification IBAN/nom (VoP), SCA améliorée.

**FIDA** (Financial Data Access Regulation) : Proposition Commission juin 2023, adoption prévue H1 2026 [REF-92]. Étend l'open banking à tout le secteur financier (crédit, assurance, investissements, pensions, crypto-actifs).

**Situation Tunisie** : Aucune réglementation formelle d'open banking. Concept de « DSP2 tunisifiée » discuté depuis 2018 mais non concrétisé. BCT Sandbox active pour l'innovation fintech.

**Voir** : [Documentation Open Banking BANKO](../compliance/open-banking-psd2/01-readiness-roadmap.md)

---

## 12. Matrice de traçabilité Norme → Module BANKO

Cette matrice assure que chaque module de BANKO est directement traçable à ses obligations légales.

| Module BANKO | Textes légaux applicables | Réf. sources | Priorité |
|---|---|---|---|
| **Core Banking (comptes, dépôts)** | Loi 2016-48, NCT 21/24/25 | [REF-09] [REF-45] [REF-46] | P0 — Critique |
| **Crédits et engagements** | Loi 2016-48, Circ. 91-24, Circ. 2018-06, Circ. 2023-02 | [REF-09] [REF-14] [REF-19] [REF-24] | P0 — Critique |
| **Calcul prudentiel** | Circ. 91-24, 2016-03, 2018-06, 2018-10 | [REF-14] [REF-17] [REF-19] [REF-21] | P0 — Critique |
| **KYC / CDD / EDD** | Loi 2015-26, Loi 2019-9, Circ. 2017-08, Circ. 2025-17 | [REF-28] [REF-30] [REF-31] [REF-33] | P0 — Critique |
| **Surveillance transactionnelle (AML)** | Loi 2015-26, Circ. 2025-17, GAFI R.1/R.16 | [REF-28] [REF-33] [REF-65] [REF-66] | P0 — Critique |
| **Déclarations de soupçon** | Loi 2015-26 art. 125, Circ. 2025-17 | [REF-28] [REF-33] | P0 — Critique |
| **Gouvernance et contrôle interne** | Circ. 2006-19, Circ. 2021-05, Loi 2016-48 art. 49-51 | [REF-35] [REF-37] [REF-09] | P0 — Critique |
| **Comptabilité bancaire** | NCT 01/21/22/24/25, Loi 96-112 | [REF-42] [REF-44] [REF-45] [REF-46] [REF-13] | P0 — Critique |
| **Reporting réglementaire BCT** | Circ. 2018-09, toutes circulaires prudentielles | [REF-41] | P1 — Élevée |
| **Opérations de change** | Loi 76-18, Décret 77-608, Circ. 2018-07 | [REF-49] [REF-50] [REF-53] | P1 — Élevée |
| **Protection des données** | Loi 2004-63, Convention 108+, RGPD (référence) | [REF-54] [REF-57] | P1 — Élevée |
| **Moyens de paiement** | Loi 2016-48, ISO 20022, ISO 8583 | [REF-09] | P1 — Élevée |
| **Provisionnement IFRS 9** | IFRS 9, NCT 24 (transition) | [REF-47] [REF-48] | P2 — Moyen terme |
| **Marché financier** | Loi 94-117, Règlements CMF | [REF-58] [REF-59] | P2 — Moyen terme |
| **Sécurité SI** | ISO 27001, ISO 22301 | — | P1 — Élevée |
| **Enrôlement électronique (e-KYC)** | Circ. 2025-06, Loi 2016-48 | [REF-73] | P0 — Critique |
| **Plateforme TuniCheque** | Circ. 2025-03/04/09 | [REF-72] | P1 — Élevée |
| **Conformité GAFI (effectivité)** | GAFI 40 Recommandations, R.1, R.16, Circ. 2025-17 | [REF-65] [REF-66] [REF-76] | P0 — Critique |
| **Protection données (nouvelle loi)** | Loi données 2025, Convention 108+ | [REF-79] [REF-57] | P0 — Critique |
| **Sécurité cartes (PCI DSS)** | PCI DSS v4.0.1 | [REF-90] | P1 — Élevée |
| **Open Banking (anticipation)** | PSD3/PSR, FIDA (références) | [REF-91] [REF-92] | P2 — Moyen terme |
| **Réforme prudentielle IFRS 9** | Circ. 2025-08, IFRS 9 | [REF-74] [REF-47] | P0 — Critique |

---

## 13. Sources centralisées

> Toutes les références utilisées dans ce document sont listées ci-dessous. Le préfixe indique le type :
> - **🏛️ OFF** = Source officielle (institution, JORT, texte de loi)
> - **📋 REG** = Texte réglementaire (circulaire, note, guide)
> - **🌍 INT** = Norme ou organisme international
> - **📰 ANA** = Analyse, commentaire ou article spécialisé
>
> **Dernière vérification** : 6 avril 2026

### 13.1 Portails institutionnels

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-01]** | 🏛️ OFF | BCT — Portail officiel de la Banque Centrale de Tunisie | [bct.gov.tn](https://www.bct.gov.tn) |
| **[REF-02]** | 🏛️ OFF | CTAF — Commission Tunisienne des Analyses Financières | [ctaf.gov.tn](https://www.ctaf.gov.tn) |
| **[REF-03]** | 🏛️ OFF | CMF — Conseil du Marché Financier | [cmf.tn](https://www.cmf.tn) |
| **[REF-04]** | 🏛️ OFF | CBF — Conseil Bancaire et Financier | [cbf.org.tn](https://www.cbf.org.tn) |
| **[REF-05]** | 🏛️ OFF | INPDP — Instance Nationale de Protection des Données Personnelles | [inpdp.tn](https://www.inpdp.tn) |
| **[REF-06]** | 🏛️ OFF | FGDB — Fonds de Garantie des Dépôts Bancaires | [fgdb.gov.tn](https://www.fgdb.gov.tn) |
| **[REF-07]** | 🏛️ OFF | Ministère des Finances — Portail officiel | [finances.gov.tn](https://www.finances.gov.tn) |
| **[REF-08]** | 🏛️ OFF | OECT — Ordre des Experts-Comptables de Tunisie | [oect.org.tn](https://oect.org.tn) |

### 13.2 Lois fondamentales

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-09]** | 🏛️ OFF | Loi n° 2016-48 du 11/07/2016 — Relative aux banques et aux établissements financiers (BCT) | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/Loi_2016_48_fr.pdf) |
| **[REF-10]** | 🏛️ OFF | Loi n° 2016-48 (miroir CMF) | [PDF — cmf.tn](https://www.cmf.tn/sites/default/files/pdfs/reglementation/textes-reference/loi2016_48_fr.pdf) |
| **[REF-11]** | 🏛️ OFF | Loi n° 2016-48 (miroir FGDB) | [PDF — fgdb.gov.tn](https://www.fgdb.gov.tn/storage/79/Loi-n%C2%B0-2016-48-du-11-juillet-2016.pdf) |
| **[REF-12]** | 🏛️ OFF | Loi n° 2016-35 du 25/04/2016 — Statut de la BCT | [bct.gov.tn — Supervision](https://www.bct.gov.tn/bct/siteprod/page.jsp?id=59) |
| **[REF-13]** | 🏛️ OFF | Loi n° 96-112 du 30/12/1996 — Système comptable des entreprises + Décret 96-2459 | [finances.gov.tn — Cadre réglementaire](https://www.finances.gov.tn/fr/cadre-reglementaire-5) |

### 13.3 Circulaires BCT — Réglementation prudentielle

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-14]** | 📋 REG | Circulaire BCT n° 91-24 du 17/12/1991 — Division, couverture des risques et suivi des engagements (texte original) | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/cir91_24.pdf) |
| **[REF-15]** | 📋 REG | Circulaire BCT n° 91-24 — Version modifiée et consolidée | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/Cir_91_24_M_fr.pdf) |
| **[REF-16]** | 📋 REG | Circulaire BCT n° 91-24 (miroir BNA) | [PDF — bna.tn](http://www.bna.tn/documents/cir_91_24.pdf) |
| **[REF-17]** | 📰 ANA | Circulaire BCT n° 2016-03 — Ratio de solvabilité (analyse IlBoursa) | [ilboursa.com](https://www.ilboursa.com/marches/bct--nouveau-dispositif-prudentiel-pour-les-banques-et-les-etablissements-financiers_14288) |
| **[REF-18]** | 📰 ANA | Renforcement de la supervision bancaire en Tunisie (RSBP) | [rsbp-tn.org](https://s4.rsbp-tn.org/library/msme-finance/ksep-lib-le-renforcement-de-la-supervision-bancaire.html) |
| **[REF-19]** | 📋 REG | Circulaire BCT n° 2018-06 du 05/06/2018 — Normes d'adéquation des fonds propres | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/Cir_2018_06_fr.pdf) |
| **[REF-20]** | 📋 REG | Circulaire BCT n° 2018-06 (miroir CBF) | [PDF — cbf.org.tn](https://www.cbf.org.tn/wp-content/uploads/2023/01/06-Circulaire-de-la-BCT-aux-banques-et-aux-etablissements-financiers-n%C2%B02018-06-du-5-Juin-2018.pdf) |
| **[REF-21]** | 📋 REG | Circulaire BCT n° 2018-10 — Ratio Crédits/Dépôts | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/Cir_2018_10_fr.pdf) |
| **[REF-22]** | 📰 ANA | Ratio Crédits/Dépôts : enjeux et implications (IlBoursa) | [ilboursa.com](https://www.ilboursa.com/marches/banques--nouveau-ratio-creditsdepots-les-enjeux-et-les-implications_15242) |
| **[REF-23]** | 📰 ANA | Ratio Crédits/Dépôts : enjeux (Tustex) | [tustex.com](https://www.tustex.com/economie-actualites-des-societes/secteur-bancaire-tunisie-valeurs-revient-sur-le-nouveau-ratio-creditsdepots-ses-enjeux-et-ses) |
| **[REF-24]** | 📋 REG | Circulaire BCT n° 2023-02 du 24/02/2023 — Modifications provisionnement (CBF) | [PDF — cbf.org.tn](https://www.cbf.org.tn/wp-content/uploads/2023/03/02-Circulaire-aux-banques-et-aux-etablissements-financiers-n%C2%B02023-02-du-24-Fevrier-2023.pdf) |
| **[REF-25]** | 📰 ANA | Implications de la nouvelle circulaire BCT sur les provisions bancaires | [ilboursa.com](https://www.ilboursa.com/analyses/chronique-implications_de_la_nouvelle_circulaire_de_la_bct_sur_les_provisions_bancaires-12) |
| **[REF-26]** | 📰 ANA | BCT — Circulaire distribution dividendes 2023 (TAP) | [tap.info.tn](https://www.tap.info.tn/en/Portal-Economy/17106395-bct-issues-circular) |
| **[REF-27]** | 📰 ANA | BCT — Nouvelles règles distribution dividendes 2025 (La Presse) | [lapresse.tn](https://www.lapresse.tn/2026/01/30/bct-nouvelles-regles-pour-la-distribution-des-dividendes-2025/) |

### 13.4 LBC/FT — Lutte anti-blanchiment

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-28]** | 🏛️ OFF | Loi organique n° 2015-26 du 07/08/2015 — Lutte contre le terrorisme et blanchiment | [legislation-securite.tn](https://legislation-securite.tn/latest-laws/loi-organique-n-2015-26-du-7-aout-2015-relative-a-la-lutte-contre-le-terrorisme-et-a-la-repression-du-blanchiment-dargent/) |
| **[REF-29]** | 🏛️ OFF | Loi organique n° 2015-26 — Texte complet (OHCHR) | [PDF — ohchr.org](https://www.ohchr.org/sites/default/files/lib-docs/HRBodies/UPR/Documents/Session27/TN/26Annexe16Loi2015_26fr.pdf) |
| **[REF-30]** | 🏛️ OFF | Loi organique n° 2019-9 du 23/01/2019 — Modification de la loi 2015-26 | [legislation-securite.tn](https://legislation-securite.tn/latest-laws/loi-organique-n-2019-9-du-23-janvier-2019-modifiant-et-completant-la-loi-organique-n-2015-26-du-7-aout-2015-relative-a-la-lutte-contre-le-terrorisme-et-a-la-repression-du-blanchiment-d/) |
| **[REF-31]** | 📋 REG | Circulaire BCT n° 2017-08 du 19/09/2017 — Contrôle interne LBC/FT | [legislation-securite.tn](https://legislation-securite.tn/latest-laws/circulaire-aux-banques-et-aux-etablissements-financiers-n2017-08-du-19-septembre-2017-portant-sur-les-regles-de-controle-interne-pour-la-gestion-du-risque-de-blanchiment-dargent-et-de/) |
| **[REF-32]** | 📋 REG | Circulaire BCT n° 2017-08 (miroir CBF) | [PDF — cbf.org.tn](https://www.cbf.org.tn/wp-content/uploads/2023/01/08-Circulaire-de-la-BCT-aux-banques-et-aux-etablissements-financiers-n%C2%B02017-08-du-19-septembre-2017.pdf) |
| **[REF-33]** | 📰 ANA | Circulaire BCT n° 2025-17 du 22/12/2025 — Nouveau cadre LBC/FT/FP (Challenges.tn) | [challenges.tn](https://www.challenges.tn/economie/banques-tunisiennes-la-bct-renforce-le-controle-interne-lba-ft-avec-de-nouvelles-obligations-des-decembre-2025/) |
| **[REF-34]** | 📋 REG | Guide CMF 2018 — Lutte contre le blanchiment d'argent | [PDF — cmf.tn](https://www.cmf.tn/sites/default/files/pdfs/documentation/guides/guide_blanchiment_v2018.pdf) |

### 13.5 Gouvernance bancaire

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-35]** | 📋 REG | Circulaire BCT n° 2006-19 du 28/11/2006 — Contrôle interne | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/arabe/documents/Cir_2006_19_fr.pdf) |
| **[REF-36]** | 📋 REG | Circulaire BCT n° 2006-19 (miroir BNA) | [PDF — bna.tn](http://www.bna.tn/documents/cir_2006_19.pdf) |
| **[REF-37]** | 📋 REG | Circulaire BCT n° 2021-05 du 19/08/2021 — Cadre de gouvernance bancaire | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/Cir_2021_05_fr.pdf) |
| **[REF-38]** | 📋 REG | Circulaire BCT n° 2021-05 (miroir CBF) | [PDF — cbf.org.tn](https://www.cbf.org.tn/wp-content/uploads/2023/01/05-Circulaire-aux-banques-et-aux-etablissements-financiers-n%C2%B02021-05-du-19-Aout-2021.pdf) |
| **[REF-39]** | 📰 ANA | BCT — Nouveau cadre de gouvernance bancaire (IlBoursa) | [ilboursa.com](https://www.ilboursa.com/marches/la-bct-annonce-un-nouveau-cadre-de-gouvernance-des-banques-et-etablissements-financiers_29834) |
| **[REF-40]** | 📰 ANA | Formation gouvernance — Apports circulaire 2021-05 (IFBT) | [ifbt.tn](https://www.ifbt.tn/formations/gouvernance-bancaire-apports-de-la-circulaire-de-la-bct-n-2021-05-role-des-fonctions-de-controle/) |
| **[REF-41]** | 📋 REG | Circulaire BCT n° 2018-09 du 18/10/2018 — Reporting réglementaire | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/Cir_2018_09_fr.pdf) |

### 13.6 Normes comptables

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-42]** | 🏛️ OFF | Tunisian IFRS Group — Corpus des textes NCT | [tunisianifrsgroup.wordpress.com](https://tunisianifrsgroup.wordpress.com/ifrs-dans-le-monde/textes-des-nct/) |
| **[REF-43]** | 🏛️ OFF | ProComptable — Index des normes comptables tunisiennes | [procomptable.com](http://www.procomptable.com/normes/indexp.htm) |
| **[REF-44]** | 🏛️ OFF | NCT 01 — Norme comptable générale (OECT) | [PDF — oect.org.tn](https://oect.org.tn/wp-content/uploads/2023/01/NC_01.pdf) |
| **[REF-45]** | 🏛️ OFF | NCT 22 — Contrôle interne et organisation comptable bancaire (MF) | [PDF — finances.gov.tn](https://www.finances.gov.tn/sites/default/files/NC22.pdf) |
| **[REF-46]** | 🏛️ OFF | NCT 21 — Présentation des états financiers bancaires (MF) | [PDF — finances.gov.tn](https://www.finances.gov.tn/sites/default/files/NC21.pdf) |
| **[REF-47]** | 📰 ANA | Impact de la norme IFRS 9 sur le secteur bancaire tunisien (Leaders) | [leaders.com.tn](https://m.leaders.com.tn/article/31935-impact-de-la-norme-ifrs-9-sur-le-secteur-bancaire-tunisien) |
| **[REF-48]** | 📰 ANA | IFRS — Intégration du secteur financier tunisien (IlBoursa) | [ilboursa.com](https://www.ilboursa.com/marches/ifrs-la-boucle-finale-de-lintegration-du-secteur-financier-tunisien-dans-leconomie-mondiale_17039) |

### 13.7 Réglementation des changes

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-49]** | 🏛️ OFF | BCT — Réglementation des changes (recueil consolidé) | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/Reg_des_Chges_ao13.pdf) |
| **[REF-50]** | 🏛️ OFF | Loi n° 76-18 du 21/01/1976 — Code des changes (BCT) | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/loi7618.pdf) |
| **[REF-51]** | 🏛️ OFF | Code des changes et du commerce extérieur — Tunisie (Droit-Afrique) | [PDF — droit-afrique.com](https://www.droit-afrique.com/upload/doc/tunisie/Tunisie-Code-2010-changes-et-commerce-exterieur.pdf) |
| **[REF-52]** | 🏛️ OFF | Loi n° 76-18 — Code des changes (WIPO) | [wipolex-res.wipo.int](https://wipolex-res.wipo.int/edocs/lexdocs/laws/fr/tn/tn037fr.html) |
| **[REF-53]** | 📋 REG | Circulaire BCT n° 2018-07 du 30/07/2018 — Activité de change manuel | [legislation-securite.tn](https://legislation-securite.tn/latest-laws/circulaire-de-la-banque-centrale-de-tunisie-n-2018-07-du-30-juillet-2018-relative-a-lexercice-de-lactivite-de-change-manuel-par-les-personnes-physiques-par-louverture-de-bur/) |

### 13.8 Protection des données personnelles

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-54]** | 🏛️ OFF | Loi organique n° 2004-63 du 27/07/2004 — Protection des données personnelles (INS) | [PDF — ins.tn](https://www.ins.tn/sites/default/files/2020-04/Loi%2063-2004%20Fr.pdf) |
| **[REF-55]** | 🏛️ OFF | Loi organique n° 2004-63 (legislation-securite.tn) | [legislation-securite.tn](https://legislation-securite.tn/latest-laws/loi-organique-n-2004-63-du-27-juillet-2004-portant-sur-la-protection-des-donnees-a-caractere-personnel/) |
| **[REF-56]** | 🏛️ OFF | INPDP — Textes législatifs | [inpdp.tn/textes](https://www.inpdp.tn/textes.xhtml) |
| **[REF-57]** | 🌍 INT | Conseil de l'Europe — Fiche INPDP Tunisie (Convention 108) | [coe.int](https://www.coe.int/fr/web/tunis/inpdp) |

### 13.9 Marché financier

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-58]** | 🏛️ OFF | Loi n° 94-117 du 14/11/1994 — Réorganisation du marché financier (CMF) | [PDF — cmf.tn](https://www.cmf.tn/sites/default/files/pdfs/reglementation/textes-reference/loi_94117_141194_fr.pdf) |
| **[REF-59]** | 🏛️ OFF | CMF — Textes de référence (index) | [cmf.tn](https://www.cmf.tn/?q=textes-de-r%C3%A9f%C3%A9rence) |
| **[REF-60]** | 🏛️ OFF | Bourse de Tunis — Réglementation | [bvmt.com.tn](https://www.bvmt.com.tn/fr/content/r%C3%A9glementation) |

### 13.10 Normes internationales

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-61]** | 🌍 INT | Comité de Bâle — Bank for International Settlements (BIS) | [bis.org/bcbs](https://www.bis.org/bcbs/) |
| **[REF-62]** | 🌍 INT | GAO — Participation des agences US au Comité de Bâle (rapport 2025) | [PDF — gao.gov](https://files.gao.gov/reports/GAO-25-107995/index.html) |
| **[REF-63]** | 🌍 INT | IMF — Revised Basel Core Principles for Effective Banking Supervision (2024) | [PDF — imf.org](https://www.imf.org/-/media/files/publications/pp/2024/english/ppea2024037.pdf) |
| **[REF-64]** | 🌍 INT | FATF/GAFI — Approche basée sur les risques pour le secteur bancaire (guide 2014) | [PDF — fatf-gafi.org](https://www.fatf-gafi.org/content/dam/fatf-gafi/guidance/Risk-Based-Approach-Banking-Sector.pdf.coredownload.pdf) |
| **[REF-65]** | 🌍 INT | FATF — Mise à jour Recommandation 1 (inclusion financière, fév. 2025) | [fatf-gafi.org](https://www.fatf-gafi.org/en/publications/Fatfrecommendations/update-standards-promote-financial-conclusion-feb-2025.html) |
| **[REF-66]** | 🌍 INT | FATF — Mise à jour Recommandation 16 (transparence paiements, juin 2025) | [fatf-gafi.org](https://www.fatf-gafi.org/en/publications/Fatfrecommendations/update-Recommendation-16-payment-transparency-june-2025.html) |
| **[REF-67]** | 🌍 INT | FATF — Rapport annuel 2024-2025 | [fatf-gafi.org](https://www.fatf-gafi.org/en/publications/Fatfgeneral/FATF-Annual-report-2024-2025.html) |

### 13.11 Recueils et compilations

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-68]** | 🏛️ OFF | BCT — Réglementation bancaire : recueil complet de textes | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/reg_bancaire.pdf) |
| **[REF-69]** | 🏛️ OFF | BCT — Index des circulaires | [bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/page.jsp?id=226) |
| **[REF-70]** | 🏛️ OFF | CBF — Circulaires et notes (index complet) | [cbf.org.tn](https://www.cbf.org.tn/circulaires-notes-7/) |

### 13.12 Nouvelles références 2025-2026

| Réf. | Type | Description | URL |
|---|---|---|---|
| **[REF-71]** | 📋 REG | Circulaire BCT n° 2025-01 du 29/01/2025 — Modification Annexe III risques (Circ. 91-24) | [amef-consulting.com](https://www.amef-consulting.com/2026/01/secteur-bancaire-tunisien-retrospective-reglementaire-2025/) |
| **[REF-72]** | 📋 REG | Circulaire BCT n° 2025-03 du 31/01/2025 — TuniCheque | [PDF — bct.gov.tn](https://www.bct.gov.tn/bct/siteprod/documents/Cir_2025_03_fr.pdf) |
| **[REF-73]** | 📋 REG | Circulaire BCT n° 2025-06 du 28/02/2025 — Enrôlement électronique e-KYC | [managers.tn](https://managers.tn/2025/02/28/la-banque-centrale-de-tunisie-impose-des-regles-pour-lenrolement-electronique-des-clients-details/) |
| **[REF-74]** | 📋 REG | Circulaire BCT n° 2025-08 du 16/05/2025 — Réforme prudentielle et IFRS 9 | [challenges.tn](https://www.challenges.tn/economie/circulaire-2025-08-la-bct-fixe-un-ultimatum-de-3-mois-aux-banques-tunisiennes/) |
| **[REF-75]** | 📋 REG | Circulaire BCT n° 2025-12 du 09/10/2025 — Plateforme numérique change | [managers.tn](https://managers.tn/2025/10/10/la-bct-lance-une-plateforme-numerique-pour-le-depot-en-ligne-des-autorisations-de-change/) |
| **[REF-76]** | 📋 REG | Circulaire BCT n° 2025-17 du 22/12/2025 — Refonte LBC/FT/FP | [challenges.tn](https://www.challenges.tn/economie/banques-tunisiennes-la-bct-renforce-le-controle-interne-lba-ft-avec-de-nouvelles-obligations-des-decembre-2025/) |
| **[REF-77]** | 📋 REG | Circulaire BCT n° 2026-02 du 23/01/2026 — LBC/FT bureaux de change | [managers.tn](https://managers.tn/2026/01/26/nouvelle-circulaire-de-la-bct-ce-qui-change-pour-les-bureaux-de-change-en-tunisie/) |
| **[REF-78]** | 📋 REG | Circulaire BCT n° 2026-04 du 26/03/2026 — Autofinancement importations non-prioritaires | [ilboursa.com](https://www.ilboursa.com/marches/la-bct-serre-la-vis-sur-l-importation-des-produits-non-prioritaires-_60693) |
| **[REF-79]** | 🏛️ OFF | Loi organique données personnelles 2025 (adoptée 20/06/2025, application 11/07/2026) | [thd.tn](https://www.thd.tn/le-projet-de-loi-sur-la-protection-des-donnees-personnelles-approuve-par-le-conseil-ministeriel/) |
| **[REF-80]** | 🏛️ OFF | Loi de Finances 2026 (n° 17-2025) — Comptes devises, suppression plafond cash | [chaexpert.com](https://chaexpert.com/tunisie-loi-de-finances-2026/) |
| **[REF-81]** | 📋 REG | CMF — Stratégie 2026 LBC/FT/FP | [cmf.tn](https://www.cmf.tn/?q=le-cmf-adopte-sa-strat%C3%A9gie-2026-en-mati%C3%A8re-de-lutte-contre-le-blanchiment-d%E2%80%99argent-le-financement-du) |
| **[REF-82]** | 📋 REG | CMF — Règlement anti-blanchiment + crowdfunding (JORT n° 9, 23/01/2026) | [challenges.tn](https://www.challenges.tn/economie/tunisie-le-cmf-publie-un-nouveau-reglement-anti-blanchiment-et-etend-la-surveillance-au-crowdfunding/) |
| **[REF-83]** | 🌍 INT | FATF — Recommandation 16 révisée (transparence paiements, juin 2025) | [fatf-gafi.org](https://www.fatf-gafi.org/en/publications/Fatfrecommendations/update-Recommendation-16-payment-transparency-june-2025.html) |
| **[REF-84]** | 🌍 INT | FATF — Rapport ciblé stablecoins et wallets non hébergés (mars 2026) | [fatf-gafi.org](https://www.fatf-gafi.org/en/publications/Virtualassets/targeted-report-stablecoins-unhosted-wallets.html) |
| **[REF-85]** | 🌍 INT | FATF — Résultats plénière février 2026 | [fatf-gafi.org](https://www.fatf-gafi.org/en/publications/Fatfgeneral/outcomes-FATF-plenary-february-2026.html) |
| **[REF-86]** | 🌍 INT | ISO/IEC 27001:2022 — Systèmes de management de la sécurité de l'information | [iso.org](https://www.iso.org/standard/27001.html) |
| **[REF-87]** | 🌍 INT | ISO/IEC 27001:2022/Amd 1:2024 — Amendement changement climatique | [iso.org](https://www.iso.org/standard/88435.html) |
| **[REF-88]** | 🌍 INT | ISO/IEC 27002:2022 — Contrôles de sécurité de l'information | [iso.org](https://www.iso.org/standard/75652.html) |
| **[REF-89]** | 🌍 INT | ISO/IEC 27701:2025 — Management de la vie privée (standard autonome) | [iso.org](https://www.iso.org/standard/27701) |
| **[REF-90]** | 🌍 INT | PCI DSS v4.0.1 (juin 2024) | [pcisecuritystandards.org](https://www.pcisecuritystandards.org/) |
| **[REF-91]** | 🌍 INT | PSD3/PSR — Accord politique provisoire (27/11/2025) | [nortonrosefulbright.com](https://www.nortonrosefulbright.com/en/knowledge/publications/cedd39c6/psd3-and-psr-from-provisional-agreement-to-2026-readiness) |
| **[REF-92]** | 🌍 INT | FIDA — Financial Data Access Regulation (proposition juin 2023) | [financial-data-access.com](https://financial-data-access.com/) |
| **[REF-93]** | 📋 REG | CTAF — Rapport d'activité annuel 2024 | [lapresse.tn](https://www.lapresse.tn/2025/10/21/rapport-dactivite-annuel-de-la-ctaf-une-hausse-de-45-des-declarations-de-soupcons-en-2024/) |
| **[REF-94]** | 🏛️ OFF | BCT — Portail Fintech | [fintech.bct.gov.tn](https://fintech.bct.gov.tn/fr/node) |
| **[REF-95]** | 📰 ANA | AMEF Consulting — Rétrospective réglementaire bancaire 2025 | [amef-consulting.com](https://www.amef-consulting.com/2026/01/secteur-bancaire-tunisien-retrospective-reglementaire-2025/) |

---

> **Note** : Ce document est vivant et sera mis à jour à chaque nouvelle circulaire ou modification législative. L'équipe BANKO assure une veille réglementaire continue via les sites de la BCT [REF-69] et du CBF [REF-70].
>
> **Légende des types** : 🏛️ OFF = Source officielle | 📋 REG = Texte réglementaire | 🌍 INT = Norme internationale | 📰 ANA = Analyse spécialisée
