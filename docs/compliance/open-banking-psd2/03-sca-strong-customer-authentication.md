# Authentification Forte du Client (SCA) -- Open Banking BANKO

| Metadata | Valeur |
|---|---|
| **Version** | 1.0.0 |
| **Date** | 6 avril 2026 |
| **Statut** | En vigueur |
| **Classification** | Interne -- Diffusion restreinte |
| **Auteur** | Equipe Architecture BANKO |
| **Approbation** | Comite de conformite / RSSI |

---

## Table des matieres

1. [Principes de l'authentification forte](#1-principes-de-lauthentification-forte)
2. [Cadre reglementaire tunisien](#2-cadre-reglementaire-tunisien)
3. [Implementation SCA dans BANKO](#3-implementation-sca-dans-banko)
4. [Exemptions SCA](#4-exemptions-sca)
5. [Dynamic linking](#5-dynamic-linking)
6. [Delegation SCA](#6-delegation-sca)
7. [Protocoles et standards](#7-protocoles-et-standards)
8. [Securite de l'implementation](#8-securite-de-limplementation)
9. [Monitoring et detection de fraude (TRA)](#9-monitoring-et-detection-de-fraude-tra)

---

## 1. Principes de l'authentification forte

### 1.1 Definition

L'authentification forte du client (Strong Customer Authentication -- SCA) est un mecanisme de verification de l'identite reposant sur l'utilisation d'au moins **deux des trois categories de facteurs** suivantes :

| Categorie | Designation | Definition | Exemples |
|---|---|---|---|
| **Connaissance** | Quelque chose que le client sait | Information connue uniquement du client | Mot de passe, code PIN, question secrete |
| **Possession** | Quelque chose que le client possede | Objet physique ou numerique detenu par le client | Telephone mobile, carte a puce, cle de securite |
| **Inherence** | Quelque chose que le client est | Caracteristique biometrique du client | Empreinte digitale, reconnaissance faciale, voix |

### 1.2 Exigences fondamentales

| Exigence | Description | Justification |
|---|---|---|
| **Independance des facteurs** | La compromission d'un facteur ne doit pas compromettre les autres | Securite en profondeur |
| **Element dynamique** | Au moins un facteur doit generer un code a usage unique ou lie au contexte | Prevention du rejeu |
| **Confidentialite** | Les facteurs de connaissance ne doivent jamais etre stockes en clair | Protection des donnees |
| **Canal separe** | Les facteurs doivent idealement transiter par des canaux distincts | Resistance aux attaques MITM |
| **Limitation temporelle** | Le code/jeton genere doit avoir une duree de vie limitee | Limitation de la fenetre d'attaque |

### 1.3 Cas d'application obligatoire

| Scenario | SCA requise | Justification |
|---|---|---|
| Acces au compte en ligne | Oui | Verification de l'identite du titulaire |
| Initiation d'un paiement electronique | Oui | Protection contre la fraude |
| Action a distance presentant un risque de fraude | Oui | Principe de precaution |
| Octroi de consentement Open Banking | Oui | Protection des donnees |
| Modification des informations sensibles (adresse, telephone) | Oui | Prevention de l'usurpation d'identite |

---

## 2. Cadre reglementaire tunisien

### 2.1 Circulaire BCT 2025-06 -- e-KYC et biometrie

La circulaire BCT 2025-06 etablit le cadre de l'identification electronique et introduit des exigences directement applicables a l'authentification forte :

| Disposition | Description | Impact SCA BANKO |
|---|---|---|
| **e-KYC biometrique** | Autorisation de l'identification a distance par biometrie | Inherence comme facteur SCA |
| **Tests d'intrusion ANCS** | Obligation de tests de penetration par l'ANCS avant mise en production | Validation securitaire de l'implementation SCA |
| **Securite des canaux** | Exigences de chiffrement des canaux de communication | TLS 1.3 obligatoire |
| **Conservation des preuves** | Obligation de conserver les preuves d'authentification | Journalisation des evenements SCA |
| **Niveau de confiance** | Classification des niveaux de verification d'identite | Calibration des facteurs SCA |

### 2.2 Loi 2016-48 -- Securite des operations bancaires

| Disposition | Article | Pertinence SCA |
|---|---|---|
| Obligation de securite des systemes d'information | Art. 8 | Fondement legal de l'exigence SCA |
| Responsabilite en cas de fraude | Art. 12 | Partage de responsabilite si SCA absente |
| Protection des donnees clients | Art. 15 | Confidentialite des facteurs d'authentification |
| Obligation de notification | Art. 18 | Notification en cas de compromission des facteurs |

### 2.3 PSD3 SCA comme reference future

Le reglement PSR (Payment Services Regulation), composante directement applicable du paquet PSD3, modernise les exigences SCA :

| Evolution PSD3/PSR | Par rapport a PSD2/RTS | Pertinence BANKO |
|---|---|---|
| **Exemptions ameliorees** | Seuils revus, TRA elargie | Meilleure experience utilisateur |
| **SCA pour IBAN portability** | Nouveau cas d'usage | A preparer pour Phase 4 |
| **Delegation SCA encadree** | Regles claires pour la delegation TPP-banque | Implementation Phase 3 |
| **Biometrie comportementale** | Reconnaissance comme facteur d'inherence | Innovation possible |
| **Authentication outage** | Procedures de secours obligatoires | Plan de continuite SCA |

---

## 3. Implementation SCA dans BANKO

### 3.1 Cartographie des facteurs

| Facteur | Type | Categorie | Implementation | Module BANKO | Technologie | Niveau de securite |
|---|---|---|---|---|---|---|
| Mot de passe | Statique | Connaissance | Hash irreversible | Identity BC | Argon2id | Eleve |
| Code PIN | Statique | Connaissance | Hash irreversible, 6 chiffres min | Identity BC | Argon2id | Moyen |
| Question secrete | Statique | Connaissance | Hash irreversible (deprecie, fallback) | Identity BC | Argon2id | Faible |
| OTP SMS | Dynamique | Possession | Code 6 chiffres, validite 5 min | Identity BC | Via passerelle SMS | Moyen |
| TOTP (application) | Dynamique | Possession | Code 6 chiffres, rotation 30s | Identity BC | TOTP (RFC 6238) | Eleve |
| Push notification | Dynamique | Possession | Approbation sur appareil enregistre | Identity BC | Firebase/APNs + HMAC | Eleve |
| Cle de securite | Materiel | Possession | Challenge-response cryptographique | Identity BC | FIDO2/WebAuthn | Tres eleve |
| Empreinte digitale | Biometrique | Inherence | Verification locale sur l'appareil | Identity BC | WebAuthn/FIDO2 | Eleve |
| Reconnaissance faciale | Biometrique | Inherence | Verification locale ou serveur (e-KYC) | Identity BC | WebAuthn/FIDO2 | Eleve |

### 3.2 Combinaisons SCA recommandees

| Combinaison | Facteur 1 (Categorie) | Facteur 2 (Categorie) | Securite | UX | Recommandation |
|---|---|---|---|---|---|
| **Mot de passe + TOTP** | Connaissance | Possession | Elevee | Bonne | Standard pour web |
| **Mot de passe + Push** | Connaissance | Possession | Elevee | Excellente | Recommande pour mobile |
| **Mot de passe + Cle FIDO2** | Connaissance | Possession | Tres elevee | Bonne | Recommande pour admin |
| **PIN + Empreinte** | Connaissance | Inherence | Elevee | Excellente | Recommande pour mobile |
| **Empreinte + TOTP** | Inherence | Possession | Elevee | Bonne | Alternative sans mot de passe |
| **Reconnaissance faciale + Push** | Inherence | Possession | Elevee | Excellente | Premium mobile |
| **Cle FIDO2 + Empreinte** | Possession | Inherence | Tres elevee | Bonne | Maximum securite |

### 3.3 Architecture technique

```
[Client]
    |
    |-- (1) Initiation requete protegee
    v
[API Gateway / Traefik]
    |
    |-- (2) Routage vers Identity BC
    v
[Identity BC -- SCA Orchestrator]
    |
    +-- (3a) Evaluation risque (TRA Engine)
    |       |
    |       +-- Exemption applicable? --> Oui --> Acces accorde
    |       |
    |       +-- Non --> Continuer SCA
    |
    +-- (3b) Selection des facteurs requis
    |
    +-- (4) Challenge Facteur 1
    |       |
    |       +-- Connaissance: verification hash (Argon2id)
    |       +-- Possession: envoi OTP/TOTP/Push
    |       +-- Inherence: challenge WebAuthn
    |
    +-- (5) Verification Facteur 1
    |
    +-- (6) Challenge Facteur 2
    |
    +-- (7) Verification Facteur 2
    |
    +-- (8) Generation token SCA (JWT signe, duree limitee)
    |
    v
[Service cible] <-- Verification du token SCA
```

### 3.4 Specifications du token SCA

| Champ | Description | Exemple |
|---|---|---|
| `sub` | Identifiant du client | `uuid-client` |
| `sca_methods` | Facteurs utilises | `["password", "totp"]` |
| `sca_level` | Niveau d'authentification | `high` |
| `iat` | Date d'emission | `1712390400` |
| `exp` | Date d'expiration | `1712390700` (5 min) |
| `txn` | Identifiant de transaction (si paiement) | `uuid-payment` |
| `amt` | Montant (si dynamic linking) | `1500.00` |
| `ccy` | Devise (si dynamic linking) | `TND` |
| `ben` | Beneficiaire (si dynamic linking) | `hash-beneficiaire` |

---

## 4. Exemptions SCA

### 4.1 Catalogue des exemptions

Conformement aux meilleures pratiques PSD3/PSR et en anticipation d'un cadre tunisien, BANKO supporte les exemptions suivantes :

| Exemption | Condition | Seuil/Critere | Risque residuel | Implementation |
|---|---|---|---|---|
| **Paiements de faible valeur** | Montant inferieur au seuil | < 50 TND (unitaire), cumul < 200 TND ou 5 transactions | Faible | Compteur par client, reset apres SCA |
| **Beneficiaire de confiance** | Beneficiaire prealablement valide par SCA | Liste blanche du client | Faible | Table `trusted_beneficiaries` |
| **Virements recurrents** | Meme montant, meme beneficiaire, meme periodicite | Apres SCA initial | Faible | Flag `recurring_validated` sur l'ordre |
| **Paiements inities par le commercant** | Transaction initiee sans intervention du payeur | Mandat prealablement autorise par SCA | Moyen | Reference au mandat |
| **Analyse de risque (TRA)** | Score de risque en dessous du seuil | Taux de fraude reference < seuil | Moyen | Moteur TRA (section 9) |
| **Acces aux informations de compte** | Consultation sans action financiere | Acces en lecture seule, moins de 90 jours depuis derniere SCA | Faible | Compteur temporel |
| **Terminaux non surveilles** | Parking, peage, transport | Transaction automatisee | Faible | Identification du terminal |

### 4.2 Seuils TRA (Transaction Risk Analysis)

| Tranche de montant | Taux de fraude reference max | Type de transaction |
|---|---|---|
| < 100 TND | 0.13% | Virement |
| 100 - 500 TND | 0.06% | Virement |
| < 50 TND | 0.20% | Paiement carte a distance |
| 50 - 250 TND | 0.10% | Paiement carte a distance |
| 250 - 500 TND | 0.06% | Paiement carte a distance |

### 4.3 Compteurs de securite

| Compteur | Seuil de declenchement SCA | Reset |
|---|---|---|
| Nombre de transactions sans SCA | 5 transactions consecutives | Apres SCA reussie |
| Montant cumule sans SCA | 200 TND | Apres SCA reussie |
| Delai depuis derniere SCA (consultation) | 90 jours | Apres SCA reussie |
| Delai depuis derniere SCA (paiement) | 24 heures | Apres SCA reussie |

---

## 5. Dynamic linking

### 5.1 Principe

Le dynamic linking (lien dynamique) est un mecanisme de securite qui lie le code d'authentification au contexte specifique de la transaction : **montant** et **beneficiaire**. Toute modification de l'un de ces elements invalide l'authentification.

### 5.2 Implementation dans BANKO

| Etape | Action | Detail technique |
|---|---|---|
| 1. Initiation | Le client initie un paiement | Collecte montant + IBAN beneficiaire |
| 2. Generation du challenge | BANKO genere un challenge SCA lie au contexte | Hash(montant + beneficiaire + nonce + timestamp) |
| 3. Affichage | Le montant et le beneficiaire sont affiches au client | Sur le canal d'authentification (app, SMS) |
| 4. Validation | Le client valide sur son appareil | Verification du hash cote serveur |
| 5. Execution | Le paiement est execute avec le montant et beneficiaire valides | Toute divergence = rejet |

### 5.3 Cas particuliers

| Cas | Traitement | Justification |
|---|---|---|
| Paiement batch (plusieurs beneficiaires) | Affichage du nombre total et montant total | PSD3/PSR Article 97(2) |
| Modification du montant apres SCA | Nouvelle SCA obligatoire | Integrite du lien dynamique |
| Modification du beneficiaire apres SCA | Nouvelle SCA obligatoire | Integrite du lien dynamique |
| Paiement en devise etrangere | Affichage en devise d'origine + contre-valeur TND | Transparence |

---

## 6. Delegation SCA

### 6.1 Principe

Dans le contexte Open Banking, un TPP (PISP) peut avoir besoin d'initier un paiement pour le compte du client. La question se pose de savoir qui realise la SCA : la banque (ASPSP) ou le TPP.

### 6.2 Modeles de delegation

| Modele | Description | SCA realisee par | Avantages | Inconvenients |
|---|---|---|---|---|
| **Redirection** | Le client est redirige vers la banque pour la SCA | Banque (ASPSP) | Controle total de la banque | Friction UX (changement de contexte) |
| **Decoupled** | La banque envoie un challenge sur un canal separe (app mobile) | Banque (ASPSP) | Bonne UX, pas de redirection | Necessite l'app de la banque |
| **Embedded** | Le TPP collecte les credentials et les transmet a la banque | TPP (avec verification banque) | UX fluide cote TPP | Risque de phishing, deprecated PSD3 |

### 6.3 Approche BANKO

| Phase | Modele supporte | Justification |
|---|---|---|
| Phase 2-3 | **Redirection** (prioritaire) | Plus securise, conforme PSD3 |
| Phase 3 | **Decoupled** (optionnel) | Meilleure UX pour les clients mobile |
| -- | **Embedded** (non supporte) | Deprecie par PSD3, risque de securite |

### 6.4 Flux de redirection detaille

| Etape | Acteur | Action | Endpoint |
|---|---|---|---|
| 1 | TPP | Initie la demande de paiement | `POST /api/v1/payments/sepa-credit-transfers` |
| 2 | BANKO | Retourne l'URL d'autorisation | `authorization_url` dans la reponse |
| 3 | TPP | Redirige le client vers BANKO | URL d'autorisation |
| 4 | Client | Realise la SCA sur l'interface BANKO | Formulaire SCA BANKO |
| 5 | BANKO | Redirige vers le TPP avec un code d'autorisation | `redirect_uri` du TPP + `code` |
| 6 | TPP | Echange le code contre un token d'acces | `POST /oauth/token` |
| 7 | TPP | Confirme le paiement | `PUT /api/v1/payments/{id}/confirm` |

---

## 7. Protocoles et standards

### 7.1 FIDO2 / WebAuthn

| Aspect | Specification |
|---|---|
| **Standard** | W3C Web Authentication (WebAuthn) Level 2 + FIDO2 CTAP2 |
| **Facteurs couverts** | Possession (cle de securite) + Inherence (biometrie locale) |
| **Avantages** | Resistant au phishing, pas de secret partage, interoperable |
| **Implementation BANKO** | Librairie `webauthn-rs` (Rust) cote serveur, WebAuthn API cote navigateur |
| **Authenticators supportes** | Cles de securite USB (YubiKey), capteurs biometriques (Touch ID, Windows Hello), Android |

### 7.2 OAuth 2.0 + PKCE

| Aspect | Specification |
|---|---|
| **Standard** | RFC 6749 (OAuth 2.0) + RFC 7636 (PKCE) |
| **Usage BANKO** | Flux d'autorisation pour les TPP |
| **Grant type** | Authorization Code + PKCE (obligatoire) |
| **Tokens** | Access token (JWT, 5 min) + Refresh token (opaque, 24h) |
| **Scopes** | Mappes sur les permissions de consentement |
| **Best practices** | RFC 9700 (OAuth 2.0 Security BCP) |

### 7.3 OpenID Connect

| Aspect | Specification |
|---|---|
| **Standard** | OpenID Connect Core 1.0 |
| **Usage BANKO** | Verification d'identite, SSO |
| **Claims** | `sub`, `name`, `email`, `phone_number`, `address` (selon consentement) |
| **Discovery** | `/.well-known/openid-configuration` |
| **JWK Set** | `/.well-known/jwks.json` |

### 7.4 Tableau recapitulatif des protocoles

| Protocole | Version | Usage dans BANKO | Module | Phase |
|---|---|---|---|---|
| FIDO2/WebAuthn | Level 2 | Authentification biometrique + cle physique | Identity BC | Phase 1 |
| OAuth 2.0 + PKCE | RFC 6749 + 7636 | Autorisation TPP | Identity BC | Phase 2 |
| OpenID Connect | Core 1.0 | Verification identite, SSO | Identity BC | Phase 2 |
| TOTP | RFC 6238 | Second facteur (application) | Identity BC | Phase 1 |
| HOTP | RFC 4226 | Second facteur (materiel) | Identity BC | Phase 1 |
| mTLS | RFC 8705 | Authentification mutuelle TPP | Infrastructure | Phase 2 |
| DPoP | RFC 9449 | Proof-of-possession pour les tokens | Identity BC | Phase 3 |
| RAR | RFC 9396 | Rich Authorization Requests | Identity BC | Phase 3 |

---

## 8. Securite de l'implementation

### 8.1 Protection contre la force brute

| Mesure | Configuration | Implementation |
|---|---|---|
| **Verrouillage de compte** | 5 tentatives echouees --> verrouillage 30 min | Compteur Redis par `customer_id` |
| **Verrouillage progressif** | Delai exponentiel entre tentatives (1s, 2s, 4s, 8s, ...) | Middleware Actix-web |
| **CAPTCHA** | Apres 3 tentatives echouees | hCaptcha ou Turnstile |
| **Alerte securite** | Notification client apres 3 tentatives echouees | Email + push notification |
| **Blocage IP** | 20 tentatives echouees depuis une meme IP | Rate limiter par IP |

### 8.2 Rate limiting SCA

| Endpoint | Limite | Fenetre | Action en cas de depassement |
|---|---|---|---|
| `POST /auth/login` | 10 requetes | 1 minute | 429 + verrouillage temporaire |
| `POST /auth/verify-otp` | 5 requetes | 5 minutes | 429 + invalidation OTP |
| `POST /auth/webauthn/verify` | 10 requetes | 1 minute | 429 + alerte securite |
| `POST /auth/sca/challenge` | 3 requetes | 1 minute | 429 + delai force |
| `POST /oauth/token` | 20 requetes | 1 minute par TPP | 429 + notification |

### 8.3 Gestion des sessions

| Parametre | Valeur | Justification |
|---|---|---|
| **Duree session web** | 15 minutes d'inactivite | Equilibre securite/UX |
| **Duree session mobile** | 5 minutes d'inactivite (operations sensibles) | Risque de perte d'appareil |
| **Duree token SCA** | 5 minutes | Fenetre d'utilisation limitee |
| **Rotation des refresh tokens** | A chaque utilisation | Prevention du vol de token |
| **Binding session-IP** | Optionnel (configurable) | Detection de vol de session |
| **Single session** | Configurable par client | Prevention acces simultane suspect |

### 8.4 Stockage des secrets

| Secret | Algorithme | Parametres | Justification |
|---|---|---|---|
| Mot de passe | Argon2id | m=65536, t=3, p=4 | Resistance GPU/ASIC, recommandation OWASP |
| Code PIN | Argon2id | m=65536, t=3, p=4 | Meme niveau que mot de passe |
| Cle TOTP | AES-256-GCM | Cle de chiffrement en HSM | Protection de la seed TOTP |
| Credential WebAuthn | Stockage direct | Cle publique uniquement | Pas de secret cote serveur |
| Token de session | HMAC-SHA256 | Cle rotee quotidiennement | Integrite du token |

### 8.5 Protection contre les attaques specifiques

| Attaque | Mesure de protection | Implementation |
|---|---|---|
| Phishing | FIDO2/WebAuthn (origin binding) | Verification de l'origine dans le challenge |
| Man-in-the-Middle | TLS 1.3 + certificate pinning (mobile) | Configuration Traefik + SDK mobile |
| Replay | Nonce unique par challenge + timestamp | Stockage des nonces utilises (TTL 10 min) |
| SIM swapping | Depreciation OTP SMS, preference TOTP/WebAuthn | Score de risque eleve si seul SMS |
| Session hijacking | Binding session-device + rotation tokens | Fingerprint appareil |
| Credential stuffing | Detection de patterns + CAPTCHA adaptatif | Analyse comportementale |

---

## 9. Monitoring et detection de fraude (TRA)

### 9.1 Transaction Risk Analysis (TRA)

Le moteur TRA evalue le risque de chaque transaction en temps reel pour determiner si une exemption SCA est applicable.

### 9.2 Facteurs de risque evalues

| Facteur | Poids | Description | Source de donnees |
|---|---|---|---|
| **Montant** | Eleve | Montant de la transaction par rapport a l'historique | Transaction courante + historique |
| **Beneficiaire** | Eleve | Beneficiaire nouveau vs connu | Table `trusted_beneficiaries` |
| **Geolocalisation** | Moyen | Localisation inhabituelle | IP + GPS (mobile) |
| **Appareil** | Moyen | Appareil nouveau vs enregistre | Device fingerprint |
| **Horaire** | Faible | Transaction a une heure inhabituelle | Historique comportemental |
| **Frequence** | Moyen | Nombre de transactions dans un intervalle | Compteur temps reel |
| **Pays destination** | Moyen | Pays a risque (liste GAFI) | Base de reference AML BC |
| **Comportement** | Eleve | Deviation par rapport au profil habituel | Modele ML (Phase 3+) |

### 9.3 Matrice de decision TRA

| Score de risque | Niveau | Action | SCA |
|---|---|---|---|
| 0-20 | Tres faible | Autorisation directe | Exemption applicable |
| 21-40 | Faible | Autorisation avec monitoring renforce | Exemption applicable (si seuil TRA respecte) |
| 41-60 | Moyen | SCA standard (2 facteurs) | Obligatoire |
| 61-80 | Eleve | SCA renforcee + verification manuelle possible | Obligatoire + alerte |
| 81-100 | Critique | Blocage temporaire + verification manuelle | Obligatoire + blocage |

### 9.4 Metriques de monitoring SCA

| Metrique | Description | Seuil d'alerte | Endpoint Prometheus |
|---|---|---|---|
| `sca_success_rate` | Taux de reussite des authentifications SCA | < 95% | `/metrics` |
| `sca_failure_rate` | Taux d'echec des authentifications SCA | > 5% | `/metrics` |
| `sca_avg_duration_ms` | Duree moyenne du processus SCA | > 10 000 ms | `/metrics` |
| `sca_exemption_rate` | Pourcentage de transactions exemptees | > 80% (anomalie) | `/metrics` |
| `sca_brute_force_attempts` | Tentatives de force brute detectees | > 10 / heure / client | `/metrics` |
| `sca_lockout_count` | Nombre de comptes verrouilles | > 50 / heure | `/metrics` |
| `tra_score_distribution` | Distribution des scores TRA | Derive significative | `/metrics` |
| `fraud_rate_by_exemption` | Taux de fraude par type d'exemption | Au-dessus des seuils PSD3 | `/metrics` |

### 9.5 Alertes et escalade

| Evenement | Severite | Action automatique | Escalade |
|---|---|---|---|
| 5 echecs SCA consecutifs | Warning | Verrouillage 30 min + notification client | Equipe support |
| 10 echecs SCA en 1h (meme IP) | Critical | Blocage IP + alerte securite | Equipe RSSI |
| Score TRA > 80 | High | Blocage transaction + notification client | Equipe fraude |
| Taux de fraude depassant seuil TRA | Critical | Suspension des exemptions concernees | Direction risques |
| Tentative depuis pays sanctionne | Critical | Blocage + rapport Sanctions BC | Equipe conformite + AML BC |
| Appareil non reconnu + montant eleve | Warning | SCA obligatoire + notification | Equipe fraude |

### 9.6 Reporting reglementaire

| Rapport | Frequence | Destinataire | Contenu |
|---|---|---|---|
| Taux de fraude par type d'exemption | Trimestriel | BCT / Autorite de supervision | Taux par tranche de montant et type |
| Statistiques SCA | Mensuel | Direction des risques | Volumes, taux de reussite, duree |
| Incidents de securite SCA | Immediat (72h) | BCT + ANCS | Description, impact, mesures |
| Audit des exemptions | Annuel | Audit interne | Revue de la pertinence des seuils |

---

## Annexe A -- Matrice de conformite SCA

| Exigence | Source | Statut BANKO | Phase |
|---|---|---|---|
| Authentification a deux facteurs | BCT Circ. 2025-06 / PSD3 | Conforme | Phase 1 |
| Independance des facteurs | PSD3 RTS Art. 9 | Conforme | Phase 1 |
| Dynamic linking pour paiements | PSD3 PSR Art. 97 | Implemente | Phase 1 |
| Exemptions calibrees | PSD3 PSR Art. 98 | Implemente | Phase 2 |
| Delegation SCA (redirection) | PSD3 PSR Art. 97(5) | Planifie | Phase 3 |
| Tests d'intrusion ANCS | BCT Circ. 2025-06 | Planifie | Phase 1 |
| Conservation des preuves SCA | BCT Circ. 2025-06 | Conforme | Phase 1 |
| Monitoring temps reel | Best practice | Implemente | Phase 1 |
| Reporting taux de fraude | PSD3 PSR Art. 99 | Planifie | Phase 2 |

---

## Annexe B -- References

| Reference | Source |
|---|---|
| Circulaire BCT 2025-06 (e-KYC) | Banque Centrale de Tunisie, 2025 |
| Loi 2016-48 | JORT, 2016 |
| PSD3/PSR accord provisoire | Conseil de l'UE, 27 novembre 2025 |
| FIDO2/WebAuthn Level 2 | W3C, 2021 |
| RFC 6238 (TOTP) | IETF |
| RFC 7636 (PKCE) | IETF |
| RFC 9700 (OAuth 2.0 Security BCP) | IETF |
| RFC 9449 (DPoP) | IETF |
| OWASP Authentication Cheat Sheet | OWASP Foundation |
| Argon2 specification | Password Hashing Competition, 2015 |

---

*Document precedent : [02 -- Gestion du consentement](./02-consent-management.md)*
*Document suivant : [04 -- Specifications de securite API](./04-api-security-specifications.md)*
