# Mapping Open Banking Tunisien -- Analyse Comparative et Recommandations

| Metadata | Valeur |
|---|---|
| **Version** | 1.0.0 |
| **Date** | 6 avril 2026 |
| **Statut** | En vigueur |
| **Classification** | Interne -- Diffusion restreinte |
| **Auteur** | Equipe Architecture BANKO |
| **Approbation** | Comite de conformite |

---

## Table des matieres

1. [Analyse comparative internationale](#1-analyse-comparative-internationale)
2. [Circulaires BCT existantes utilisables pour l'Open Banking](#2-circulaires-bct-existantes-utilisables-pour-lopen-banking)
3. [Gaps reglementaires identifies](#3-gaps-reglementaires-identifies)
4. [Recommandations pour BANKO](#4-recommandations-pour-banko)
5. [Opportunites pour la Tunisie](#5-opportunites-pour-la-tunisie)
6. [Feuille de route reglementaire suggeree pour la BCT](#6-feuille-de-route-reglementaire-suggeree-pour-la-bct)

---

## 1. Analyse comparative internationale

### 1.1 Tableau comparatif detaille

| Dimension | Tunisie (actuel, avril 2026) | Tunisie (prevu / en discussion) | Nigeria (CBN) | Arabie Saoudite (SAMA) | UE (PSD3/PSR/FIDA) | Royaume-Uni (FCA/OBL) |
|---|---|---|---|---|---|---|
| **Cadre legal** | Aucun texte specifique Open Banking | "DSP2 tunisifiee" discutee depuis 2018, pas de texte | Regulatory Guidelines on Open Banking (2021) | Saudi Open Banking Framework (2023) | PSD3 Directive + PSR Reglement (accord nov. 2025) | CMA Order (2017) + Data Protection Act |
| **Statut juridique des TPP** | Non defini | A definir | AISP, PISP, CISP definis | TPP licencies par SAMA | AISP, PISP, CISP, FISP (FIDA) | AISP, PISP licencies par FCA |
| **Standard API** | Aucun standard impose | A definir | API Specifications publiees | Base sur UK OB Standard | Berlin Group NextGenPSD2 (reference) | UK Open Banking Standard v3.1 |
| **Consentement** | Loi donnees personnelles 2025 (base generale) | Cadre specifique Open Banking a creer | Consent framework dans les guidelines | Consent management centralise | Customer dashboards obligatoires (PSD3) + FIDA consent | Explicit consent, dashboard |
| **SCA** | Circ. BCT 2025-06 (e-KYC, biometrie) | Exigences specifiques Open Banking a creer | Requirements dans les guidelines | SCA via framework SAMA | SCA amelioree, exemptions calibrees (PSR) | SCA PSD2 via FCA RTS |
| **Licensing TPP** | Pas de processus | A definir | Enregistrement CBN | Premiere licence mars 2026 | Passeport europeen (via directive) | Enregistrement/Autorisation FCA |
| **Sandbox reglementaire** | BCT Sandbox active (depuis janv. 2020) | Extension au Open Banking envisagee | CBN Regulatory Sandbox | SAMA Sandbox operationnelle | Regulatory sandboxes nationaux | FCA Regulatory Sandbox |
| **Delais de mise en conformite** | Non applicable | A definir | 12-18 mois apres publication | En cours de deploiement | ~18 mois apres publication (PSR) | Deja en vigueur |
| **Protection des donnees** | Loi 2025 (sanctions juillet 2026) | Enforcement en cours | NDPR (Nigeria Data Protection Regulation) | PDPL (Personal Data Protection Law) | RGPD | UK GDPR + DPA 2018 |
| **Autorite de supervision** | BCT (Banque Centrale de Tunisie) | BCT + ANCS | CBN | SAMA | Autorites nationales + EBA | FCA + PSR |
| **Formats de donnees** | Migration ISO 20022 en cours | ISO 20022 comme standard | Formats propres | ISO 20022 partiel | ISO 20022 obligatoire (SEPA) | ISO 20022 + formats OB |
| **Paiements mobiles** | Forte croissance (+81% vol. 2025) | Acceleration attendue | NIBSS, Interswitch ecosysteme mature | SADAD, STC Pay | SEPA Instant Credit Transfer | Faster Payments, Open Banking Payments |
| **Maturite ecosysteme fintech** | 16 etablissements agrees, BCT-Lab | Expansion progressive | Ecosysteme vibrant (Paystack, Flutterwave) | Ecosysteme emergent (Tasheel, HyperPay) | Ecosysteme mature (2000+ TPP) | Ecosysteme mature (300+ TPP) |

### 1.2 Analyse des forces et faiblesses tunisiennes

| Categorie | Forces | Faiblesses |
|---|---|---|
| **Reglementaire** | BCT Sandbox active depuis 2020, loi donnees 2025 moderne | Absence de cadre Open Banking, pas de statut TPP |
| **Technologique** | Adoption ISO 20022, e-KYC biometrique autorise | Infrastructure telecom inegale, taux de penetration internet |
| **Marche** | Croissance mobile payments +81%, 16 EP agrees | Taux de bancarisation faible (37%), marche de taille limitee |
| **Talent** | Bassin d'ingenieurs qualifies, diaspora technologique | Fuite des cerveaux, manque de specialistes Open Banking |
| **Institutionnel** | BCT-Lab innovant, portail fintech.bct.gov.tn | Lenteur legislative, fragmentation des responsabilites |

---

## 2. Circulaires BCT existantes utilisables pour l'Open Banking

### 2.1 Mapping des circulaires aux concepts Open Banking

| Circulaire BCT | Objet | Date | Concept Open Banking associe | Pertinence | Detail du mapping |
|---|---|---|---|---|---|
| **Circ. 2025-03** | TuniCheque -- Dematerialisation du cheque | 2025 | Account information sharing | Elevee | Le partage d'informations sur les comptes pour la verification de cheque prefigure le partage de donnees AIS. Les mecanismes d'echange de donnees inter-bancaires peuvent etre etendus aux TPP. |
| **Circ. 2025-06** | e-KYC -- Identification electronique | 2025 | Digital identity verification for TPPs | Tres elevee | L'identification electronique avec biometrie et tests ANCS constitue le socle de l'onboarding TPP et de la SCA. Les exigences de securite sont directement transposables. |
| **Circ. 2025-12** | Change numerique -- Plateforme de change en ligne | 2025 | Digital FX authorization | Moyenne | L'autorisation de transactions de change en ligne ouvre la voie au partage de donnees FX via APIs et aux services de change inities par des tiers. |
| **Circ. 2025-17** | LBC/FT -- Lutte contre le blanchiment et financement du terrorisme | 2025 | TPP due diligence | Elevee | Les exigences de due diligence et de connaissance du client s'appliquent aux TPP comme a tout acteur financier. Le cadre LBC/FT impose la tracabilite des operations, applicable au monitoring Open Banking. |
| **Circ. 2026-02** | Bureau de change -- Reglementation des services de change | 2026 | FX service provider compliance | Moyenne | Les regles de conformite pour les bureaux de change definissent un modele reglementaire pour les prestataires de services FX, transposable aux TPP offrant des services de change. |

### 2.2 Circulaires complementaires pertinentes

| Circulaire / Texte | Objet | Pertinence Open Banking | Application |
|---|---|---|---|
| **Loi 2016-48** | Banques et etablissements financiers | Cadre general bancaire | Base legale pour les obligations des banques envers les TPP |
| **Loi 2005-51** | Transfert electronique de fonds | Paiements electroniques | Fondement legal pour les services de paiement initie par tiers |
| **Loi organique 2025-xx** | Protection des donnees personnelles | Consentement, portabilite | Fondement du partage de donnees consenti (voir [02-consent-management](./02-consent-management.md)) |
| **Circ. BCT 2020-xx** | Sandbox reglementaire fintech | Innovation encadree | Cadre de test pour les services Open Banking |
| **Reglementation ANCS** | Cybersecurite et tests d'intrusion | Securite des APIs | Exigences de tests pour les composants Open Banking |

### 2.3 Analyse des gaps par circulaire

| Circulaire | Couvre | Ne couvre pas | Gap a combler |
|---|---|---|---|
| **Circ. 2025-03 (TuniCheque)** | Echange d'info inter-bancaire | Acces par des tiers non bancaires | Extension du perimetre aux TPP agrees |
| **Circ. 2025-06 (e-KYC)** | Identification biometrique, tests ANCS | Authentification delegue (SCA pour TPP) | Ajout de dispositions pour la delegation SCA |
| **Circ. 2025-12 (Change numerique)** | Operations de change en ligne | APIs standardisees pour le change | Standardisation des interfaces d'acces |
| **Circ. 2025-17 (LBC/FT)** | Due diligence, reporting STR | Categorisation specifique des TPP | Classification des TPP selon le risque LBC/FT |
| **Circ. 2026-02 (Bureau change)** | Conformite bureaux de change | Integration avec les plateformes Open Banking | Interoperabilite avec l'ecosysteme TPP |

---

## 3. Gaps reglementaires identifies

### 3.1 Gaps critiques

| Gap | Description | Impact | Urgence | Reference internationale |
|---|---|---|---|---|
| **Loi sur les services de paiement** | Absence d'equivalent de PSD/PSD2/PSD3 en droit tunisien | Critique -- Pas de base legale pour les TPP | Tres haute | PSD3 (UE), PSA (UK), CBN Guidelines (Nigeria) |
| **Statut juridique des TPP** | Pas de definition legale des AISP, PISP, CISP | Critique -- Les tiers ne peuvent pas operer legalement | Tres haute | PSD3 Art. 1-4 (categories de prestataires) |
| **Standard API obligatoire** | Pas de norme technique imposee aux banques | Haut -- Fragmentation des interfaces | Haute | NextGenPSD2 (Berlin Group), UK OB Standard |
| **Mecanisme de resolution des litiges** | Pas de procedure dediee aux litiges Open Banking | Haut -- Insecurite juridique pour les acteurs | Haute | PSD3 Art. 101-103, FCA complaint procedures |
| **Protection du consommateur specifique** | Protection generique, pas adaptee a l'Open Banking | Moyen -- Risques pour les clients finaux | Haute | PSD3 Titre IV (droits des utilisateurs) |

### 3.2 Gaps importants

| Gap | Description | Impact | Urgence | Reference internationale |
|---|---|---|---|---|
| **Regime de responsabilite** | Pas de partage de responsabilite clair entre banque et TPP | Haut -- Litige impossible a trancher | Moyenne | PSD3 Art. 71-73 (liability framework) |
| **Registre central des TPP** | Pas de registre officiel des tiers autorises | Moyen -- Verification des TPP difficile | Moyenne | EBA Register (UE), FCA Register (UK) |
| **Obligation d'interface dediee** | Banques non obligees de fournir des APIs | Critique -- Pas d'acces technique pour les TPP | Tres haute | PSD3/PSR (interface dediee obligatoire) |
| **Portabilite des donnees financieres** | Droit general dans la loi 2025, pas d'application specifique | Moyen -- Transfert de donnees non standardise | Moyenne | FIDA (UE), Section 1033 (US) |
| **Supervision des APIs** | Pas de monitoring de la qualite des APIs par le regulateur | Moyen -- Pas de garantie de performance | Basse | EBA monitoring (UE), OBIE/OBL (UK) |
| **Acces equitable** | Pas de regles anti-obstruction | Haut -- Banques peuvent bloquer les TPP | Haute | PSD3 Art. 36 (anti-obstruction measures) |

### 3.3 Gaps emergents (anticipation)

| Gap | Description | Horizon | Reference |
|---|---|---|---|
| **Open Finance** | Extension au-dela des comptes de paiement (credit, assurance, investissement) | 2028-2030 | FIDA (UE) |
| **Identite numerique souveraine** | Systeme d'identite numerique interoperable | 2027-2029 | eIDAS 2.0 (UE), National ID systems |
| **Finance embarquee (Embedded Finance)** | APIs pour l'integration de services financiers dans des plateformes non-financieres | 2028+ | BaaS frameworks |
| **Monnaie numerique de banque centrale (MNBC)** | Integration avec une eventuelle MNBC tunisienne | 2029+ | Euro numerique, e-Naira (Nigeria) |
| **IA et scoring** | Encadrement de l'utilisation de l'IA dans les decisions financieres basees sur les donnees partagees | 2027+ | AI Act (UE) |

---

## 4. Recommandations pour BANKO

### 4.1 Recommandations strategiques

| # | Recommandation | Priorite | Phase | Justification |
|---|---|---|---|---|
| R1 | **Concevoir les APIs PSD3-compatible des maintenant** | Critique | Phase 1 | Anticiper la reglementation evite une refonte couteuse. Le modele PSD3/NextGenPSD2 est le standard de facto mondial. |
| R2 | **Implementer le consent management conforme a la loi donnees 2025** | Critique | Phase 1-2 | La loi est deja adoptee (sanctions juillet 2026). Le consentement est le fondement de l'Open Banking. |
| R3 | **Supporter les formats ISO 20022 pour l'interoperabilite** | Haute | Phase 1 | La Tunisie est en cours de migration ISO 20022. C'est le standard mondial des messages financiers. |
| R4 | **Prevoir un portail developpeur (Developer Portal)** | Haute | Phase 1-2 | L'adoption par les TPP depend de la qualite de la documentation et des outils de test. |
| R5 | **S'aligner sur les patterns africains (Nigeria CBN) pour l'interoperabilite regionale** | Moyenne | Phase 2-3 | La Tunisie appartient au continent africain ; l'interoperabilite regionale est strategique. |

### 4.2 Recommandations techniques detaillees

| # | Recommandation | Detail | Composant BANKO | Effort |
|---|---|---|---|---|
| T1 | **Adopter le modele Berlin Group NextGenPSD2 pour les APIs AIS/PIS** | Endpoints, modeles de donnees, flux d'autorisation conformes | Account BC, Payment BC | Eleve |
| T2 | **Implementer OAuth 2.0 + PKCE avec profil FAPI 2.0** | Flux d'autorisation securise de niveau financier | Identity BC | Eleve |
| T3 | **Deployer mTLS pour l'authentification des TPP** | Certificats clients obligatoires pour tous les TPP | Infrastructure (Traefik) | Moyen |
| T4 | **Construire le Consent Service comme bounded context transversal** | Modele de donnees, lifecycle, audit trail | Governance BC | Eleve |
| T5 | **Implementer le rate limiting multi-niveaux** | Par TPP, par client, par endpoint, burst/sustained | Infrastructure (Redis + middleware) | Moyen |
| T6 | **Mettre en place la journalisation immutable (append-only log)** | Hash chaine, timestamp, non-repudiation | Infrastructure (PostgreSQL) | Moyen |
| T7 | **Supporter ISO 20022 pour les messages de paiement** | pacs.008 (Credit Transfer), camt.053 (Statement) | Payment BC, Accounting BC | Eleve |
| T8 | **Implementer la Verification of Payee (VoP)** | Verification IBAN-nom avant execution | Payment BC | Moyen |
| T9 | **Construire un moteur TRA (Transaction Risk Analysis)** | Scoring de risque en temps reel | Identity BC, AML BC | Eleve |
| T10 | **Preparer les APIs FX pour le partage de donnees** | Taux de change, operations de change via API | ForeignExchange BC | Moyen |

### 4.3 Recommandations organisationnelles

| # | Recommandation | Responsable | Delai |
|---|---|---|---|
| O1 | **Creer une equipe Open Banking dediee** (chef de projet, architecte API, juriste) | Direction generale | T2 2026 |
| O2 | **Etablir un dialogue regulier avec la BCT** sur les evolutions reglementaires Open Banking | Equipe conformite | Continu |
| O3 | **Identifier et onboarder des TPP pilotes** pour tester les APIs en sandbox | Business development | T3 2026 |
| O4 | **Former les equipes de developpement** aux standards Open Banking (NextGenPSD2, FAPI) | RH + Architecture | T2-T3 2026 |
| O5 | **Planifier les tests de penetration ANCS** pour les composants Open Banking | RSSI | T4 2026 |
| O6 | **Participer aux groupes de travail BCT** sur la reglementation fintech et Open Banking | Direction + Conformite | Continu |
| O7 | **Etablir des partenariats avec les ecosystemes africains** (Nigeria, Kenya, Afrique du Sud) | Business development | 2027 |

### 4.4 Matrice effort-impact

| Recommandation | Effort | Impact | Priorite resultante |
|---|---|---|---|
| R1 -- APIs PSD3-compatible | Eleve | Tres eleve | Critique |
| R2 -- Consent management | Eleve | Tres eleve | Critique |
| R3 -- ISO 20022 | Moyen | Eleve | Haute |
| R4 -- Developer Portal | Moyen | Eleve | Haute |
| R5 -- Patterns africains | Faible | Moyen | Moyenne |
| T1 -- NextGenPSD2 | Eleve | Tres eleve | Critique |
| T2 -- OAuth 2.0 FAPI | Eleve | Tres eleve | Critique |
| T3 -- mTLS | Moyen | Eleve | Haute |
| T4 -- Consent Service | Eleve | Tres eleve | Critique |
| T5 -- Rate limiting | Moyen | Eleve | Haute |
| T6 -- Append-only log | Moyen | Eleve | Haute |
| T7 -- ISO 20022 | Eleve | Eleve | Haute |
| T8 -- VoP | Moyen | Moyen | Moyenne |
| T9 -- TRA Engine | Eleve | Eleve | Haute |
| T10 -- APIs FX | Moyen | Moyen | Moyenne |

---

## 5. Opportunites pour la Tunisie

### 5.1 Inclusion financiere

| Opportunite | Description | Impact potentiel | Indicateur cle |
|---|---|---|---|
| **Acces bancaire elargi** | Les TPP peuvent proposer des services financiers accessibles via smartphone, sans agence physique | Augmentation du taux de bancarisation de 37% a 60%+ | Nombre de comptes actifs |
| **Micro-credit digital** | Scoring de credit alternatif base sur les donnees de transaction partagees | Acces au credit pour les populations non bancarisees | Volume de micro-credits digitaux |
| **Transferts de fonds diaspora** | APIs optimisees pour les transferts internationaux a faible cout | Reduction du cout des remittances (actuellement 7-10% du montant) | Cout moyen par transfert |
| **Paiement marchand digitalise** | Initiation de paiement sans carte physique | Reduction de l'economie informelle | Volume de paiements digitaux |
| **Aggregation budgetaire** | Applications de gestion budgetaire pour les menages | Amelioration de la litteracie financiere | Nombre d'utilisateurs d'applications budgetaires |

### 5.2 Competitivite economique

| Opportunite | Description | Beneficiaires | Horizon |
|---|---|---|---|
| **Attraction d'investissements fintech** | Un cadre Open Banking clair attire les investisseurs et les startups | Economie tunisienne, ecosysteme startup | 2027-2028 |
| **Positionnement regional** | Premiere plateforme core banking open source africaine avec APIs ouvertes | BANKO, banques tunisiennes | 2026-2027 |
| **Hub fintech maghrebin** | La Tunisie comme plaque tournante fintech pour le Maghreb et l'Afrique francophone | Secteur financier tunisien | 2028-2030 |
| **Export de services** | Les banques tunisiennes equipees de BANKO peuvent proposer du BaaS regional | Banques utilisatrices de BANKO | 2029-2030 |
| **Innovation de produits** | Les TPP creent des produits innovants impossibles sans Open Banking | Consommateurs, entreprises | 2027+ |

### 5.3 Attractivite fintech

| Facteur d'attractivite | Etat actuel | Avec Open Banking | Gain |
|---|---|---|---|
| **Nombre de fintechs actives** | ~40-50 | 150+ (projection 2030) | x3 |
| **Investissement fintech annuel** | ~15M TND | 80M+ TND (projection 2030) | x5 |
| **Emplois directs fintech** | ~500 | 3000+ (projection 2030) | x6 |
| **APIs bancaires disponibles** | 0 (standard) | 50+ endpoints | Depuis zero |
| **TPP enregistres** | 0 | 30+ (projection 2030) | Depuis zero |
| **Transactions Open Banking** | 0 | 10M+ / an (projection 2030) | Depuis zero |

### 5.4 Benefices pour les consommateurs

| Benefice | Description | Exemple concret |
|---|---|---|
| **Transparence** | Comparaison facilitee des offres bancaires | Application comparant les frais de tenue de compte de toutes les banques |
| **Controle** | Gestion centralisee de tous ses comptes | Agregateur montrant le solde total sur toutes les banques |
| **Cout reduit** | Competition accrue entre les prestataires | Frais de virement reduits grace aux PISP |
| **Innovation** | Nouveaux services impossibles auparavant | Scoring de credit alternatif pour les jeunes sans historique bancaire |
| **Securite** | Standards de securite eleves (SCA, consentement) | Fin du partage de credentials, consentement granulaire |

---

## 6. Feuille de route reglementaire suggeree pour la BCT

### 6.1 Proposition de calendrier

| Phase | Periode | Jalons | Responsable suggere |
|---|---|---|---|
| **Phase 0 -- Consultation** | T3-T4 2026 | Publication d'un document de consultation sur l'Open Banking en Tunisie | BCT -- Direction de la Supervision |
| | | Recueil des commentaires des banques, fintechs, associations de consommateurs | BCT + parties prenantes |
| | | Etude d'impact economique et reglementaire | BCT + Ministere des Finances |
| **Phase 1 -- Cadre legal** | T1-T2 2027 | Projet de loi sur les services de paiement (equivalent PSD) | BCT + Ministere des Finances |
| | | Definition du statut juridique des TPP (AISP, PISP) | BCT |
| | | Publication des conditions d'agrement TPP | BCT |
| **Phase 2 -- Standards techniques** | T3-T4 2027 | Publication du standard API tunisien (base NextGenPSD2) | BCT + Comite technique |
| | | Exigences de securite specifiques Open Banking (SCA, mTLS) | BCT + ANCS |
| | | Lancement du registre TPP | BCT |
| **Phase 3 -- Implementation** | T1-T4 2028 | Obligation pour les banques de publier des APIs AIS | BCT |
| | | Ouverture des APIs PIS (phase pilote) | BCT |
| | | Premiers TPP licencies | BCT |
| **Phase 4 -- Maturite** | 2029-2030 | Extension au Open Finance (credit, change, assurance) | BCT + CMF + CGA |
| | | Evaluation et ajustement du cadre | BCT |
| | | Interoperabilite regionale (Maghreb, Afrique) | BCT + partenaires |

### 6.2 Axes reglementaires prioritaires

| Axe | Description | Modele de reference | Priorite |
|---|---|---|---|
| **Loi sur les services de paiement** | Texte fondateur definissant les droits et obligations de tous les acteurs | PSD3 (UE), PSA (UK) | Critique |
| **Statut TPP** | Categories (AISP, PISP, CISP), conditions d'agrement, obligations | PSD3 Titre II | Critique |
| **Standard API** | Specification technique obligatoire pour les interfaces bancaires | Berlin Group NextGenPSD2 | Haute |
| **Regime de responsabilite** | Partage de responsabilite entre la banque et le TPP en cas de fraude ou d'erreur | PSD3 Art. 71-73 | Haute |
| **Protection du consommateur** | Droits specifiques des utilisateurs de services Open Banking | PSD3 Titre IV | Haute |
| **Anti-obstruction** | Regles empechant les banques de degrader l'acces API | PSD3 Art. 36 | Haute |
| **Supervision et reporting** | Mecanismes de supervision de la qualite et securite des APIs | EBA Guidelines | Moyenne |
| **Resolution des litiges** | Procedure de mediation et d'arbitrage pour les litiges Open Banking | PSD3 Art. 101-103 | Moyenne |
| **Interoperabilite regionale** | Accords de reconnaissance mutuelle avec d'autres pays | Passporting (UE), accords bilateraux | Basse (court terme) |

### 6.3 Structure institutionnelle suggeree

| Entite | Role | Modele | Justification |
|---|---|---|---|
| **BCT -- Direction Open Banking** | Supervision, agrement TPP, registre | EBA (UE), FCA (UK) | Autorite existante, competence bancaire |
| **Comite technique Open Banking** | Definition des standards API, certification | Berlin Group, OBIE/OBL | Expertise technique, representation industrie |
| **ANCS** | Tests de securite, certification securitaire | Tests d'intrusion (Circ. 2025-06) | Competence cybersecurite existante |
| **INPDP** | Protection des donnees, conformite consentement | CNIL (France), ICO (UK) | Autorite de protection des donnees |
| **Mediateur bancaire** | Resolution des litiges consommateurs | FOS (UK), Mediateur bancaire (France) | Protection du consommateur |
| **Association fintech tunisienne** | Representation des TPP, dialogue avec le regulateur | Fintech associations (UK, France) | Voix de l'industrie |

### 6.4 KPIs de suivi de la feuille de route

| KPI | Cible 2027 | Cible 2028 | Cible 2030 | Source de mesure |
|---|---|---|---|---|
| Nombre de TPP agrees | 5 | 15 | 30+ | Registre BCT |
| Nombre de banques avec APIs conformes | 3 | 10 | Toutes les banques de detail | Supervision BCT |
| Volume de transactions Open Banking / mois | 10 000 | 500 000 | 5 000 000+ | Reporting bancaire |
| Nombre d'utilisateurs de services Open Banking | 5 000 | 100 000 | 1 000 000+ | Enquete BCT |
| Taux de fraude Open Banking | < 0.01% | < 0.01% | < 0.005% | Reporting BCT |
| Disponibilite moyenne des APIs | 99.5% | 99.9% | 99.95% | Monitoring BCT |
| Temps de reponse moyen des APIs (P95) | < 1 000 ms | < 500 ms | < 200 ms | Monitoring BCT |
| Satisfaction des TPP | -- | > 70% | > 85% | Enquete annuelle |
| Emplois fintech directs | 800 | 1 500 | 3 000+ | Statistiques emploi |

---

## Annexe A -- Glossaire des termes reglementaires

| Terme | Definition | Contexte |
|---|---|---|
| **AISP** | Account Information Service Provider -- Prestataire de services d'information sur les comptes | PSD2/PSD3, acces en lecture seule aux donnees de compte |
| **PISP** | Payment Initiation Service Provider -- Prestataire de services d'initiation de paiement | PSD2/PSD3, initiation de virements pour le compte du client |
| **CISP** | Card Issuer Service Provider -- Prestataire de services d'emission de carte | PSD2, verification de provision pour paiement par carte |
| **FISP** | Financial Information Service Provider -- Prestataire de services d'information financiere | FIDA, acces aux donnees financieres elargies |
| **ASPSP** | Account Servicing Payment Service Provider -- Prestataire de services de paiement gestionnaire de comptes | PSD2/PSD3, la banque du client |
| **TPP** | Third Party Provider -- Prestataire tiers | Terme generique pour AISP, PISP, CISP, FISP |
| **PSU** | Payment Service User -- Utilisateur de services de paiement | Le client final |
| **SCA** | Strong Customer Authentication -- Authentification forte du client | Mecanisme de securite a deux facteurs minimum |
| **TRA** | Transaction Risk Analysis -- Analyse de risque transactionnelle | Methode d'evaluation du risque pour les exemptions SCA |
| **VoP** | Verification of Payee -- Verification du beneficiaire | Controle IBAN-nom avant execution d'un virement |
| **BCT** | Banque Centrale de Tunisie | Autorite de supervision bancaire tunisienne |
| **ANCS** | Agence Nationale de la Cybersecurite | Autorite tunisienne de cybersecurite |
| **INPDP** | Instance Nationale de Protection des Donnees Personnelles | Autorite tunisienne de protection des donnees |
| **EP** | Etablissement de paiement | Entite agreee pour fournir des services de paiement |

---

## Annexe B -- Chronologie des textes de reference

| Date | Texte / Evenement | Juridiction | Pertinence |
|---|---|---|---|
| 2005 | Loi 2005-51 (Transfert electronique de fonds) | Tunisie | Base legale paiements electroniques |
| 2016 | Loi 2016-48 (Banques et etablissements financiers) | Tunisie | Cadre bancaire general |
| 2018 | PSD2 entre en vigueur | UE | Reference internationale Open Banking |
| 2018 | UK Open Banking operationnel (CMA Order) | UK | Pionnier mondial |
| 2018 | Debut des discussions "DSP2 tunisifiee" | Tunisie | Premiere reflexion locale |
| 2020 (janv.) | BCT Sandbox active | Tunisie | Innovation encadree |
| 2021 | CBN Open Banking Guidelines | Nigeria | Leader africain |
| 2023 | Saudi Arabia Open Banking Framework | Arabie Saoudite | Reference MENA |
| 2024 (oct.) | CFPB Section 1033 Final Rule | Etats-Unis | Acces aux donnees financieres |
| 2025 (mars) | Circ. BCT 2025-03 (TuniCheque) | Tunisie | Dematerialisation du cheque |
| 2025 (juin) | Loi organique protection des donnees | Tunisie | Consentement, portabilite |
| 2025 (juin) | Circ. BCT 2025-06 (e-KYC) | Tunisie | Identification electronique |
| 2025 (nov.) | Accord provisoire PSD3/PSR | UE | Nouveau cadre Open Banking |
| 2025 (dec.) | Circ. BCT 2025-12 (Change numerique) | Tunisie | FX digital |
| 2025 | Circ. BCT 2025-17 (LBC/FT) | Tunisie | Tracabilite, due diligence |
| 2026 (mars) | Premiere licence Open Banking Arabie Saoudite | Arabie Saoudite | Reference MENA |
| 2026 (H1) | Publication PSD3/PSR au JOUE (prevue) | UE | Nouveau cadre applicable |
| 2026 (H1) | Adoption FIDA (prevue) | UE | Open Finance |
| 2026 (juil.) | Entree en vigueur sanctions loi donnees | Tunisie | Enforcement protection donnees |

---

## Annexe C -- References

| Reference | Source |
|---|---|
| PSD3/PSR accord provisoire | Conseil de l'UE, 27 novembre 2025 |
| FIDA proposition de reglement | Commission europeenne, COM(2023) 360 |
| CBN Open Banking Guidelines | Central Bank of Nigeria, 2021 |
| Saudi Arabia Open Banking Framework | Saudi Arabian Monetary Authority (SAMA), 2023 |
| UK CMA Open Banking Order | Competition and Markets Authority, 2017 |
| CFPB Section 1033 Final Rule | Consumer Financial Protection Bureau, octobre 2024 |
| Berlin Group NextGenPSD2 v1.3.12 | Berlin Group |
| BCT Portail Fintech | fintech.bct.gov.tn |
| Loi organique 2025-xx (donnees personnelles) | JORT, juin 2025 |
| Circulaire BCT 2025-03 (TuniCheque) | BCT, 2025 |
| Circulaire BCT 2025-06 (e-KYC) | BCT, 2025 |
| Circulaire BCT 2025-12 (Change numerique) | BCT, 2025 |
| Circulaire BCT 2025-17 (LBC/FT) | BCT, 2025 |
| Circulaire BCT 2026-02 (Bureau change) | BCT, 2026 |
| Observatoire Fintech Tunisie | Rapports annuels |
| ISO 20022 Financial Messaging | ISO |

---

*Document precedent : [04 -- Specifications de securite API](./04-api-security-specifications.md)*

*Fin de la serie documentaire Open Banking BANKO*
