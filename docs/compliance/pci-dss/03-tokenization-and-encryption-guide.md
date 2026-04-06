# Guide de Tokenisation et de Chiffrement -- Plateforme BANKO

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

1. [Stratégie de protection des données cartes](#1-stratégie-de-protection-des-données-cartes)
2. [Tokenisation](#2-tokenisation)
3. [Chiffrement au repos](#3-chiffrement-au-repos)
4. [Chiffrement en transit](#4-chiffrement-en-transit)
5. [HSM et gestion des clés](#5-hsm-et-gestion-des-clés)
6. [Intégration avec Rust](#6-intégration-avec-rust)
7. [Conformité avec la réglementation tunisienne](#7-conformité-avec-la-réglementation-tunisienne)

---

## 1. Stratégie de protection des données cartes

### 1.1 Principes fondamentaux

La stratégie de protection des données cartes de BANKO repose sur le principe de **défense en profondeur**, combinant plusieurs mécanismes complémentaires pour minimiser le risque de compromission :

| Couche de protection | Mécanisme | Objectif principal |
|----------------------|-----------|-------------------|
| **Couche 1 -- Minimisation** | Ne jamais stocker ce qui n'est pas nécessaire | Réduire la surface d'attaque |
| **Couche 2 -- Tokenisation** | Remplacer le PAN par un token irréversible | Réduire le périmètre PCI DSS |
| **Couche 3 -- Chiffrement au repos** | AES-256-GCM au niveau champ | Protéger les données persistées |
| **Couche 4 -- Chiffrement en transit** | TLS 1.3 / mTLS | Protéger les données en mouvement |
| **Couche 5 -- Contrôle d'accès** | RBAC + MFA | Restreindre l'accès aux données |
| **Couche 6 -- Surveillance** | Logging + alertes temps réel | Détecter les accès anormaux |

### 1.2 Données concernées et traitement

| Type de donnée | Stockée ? | Mécanisme de protection | Emplacement |
|----------------|:---------:|------------------------|-------------|
| PAN (complet) | **Non** (tokenisé) | Tokenisation avant persistance | Token Vault uniquement |
| PAN (tronqué) | Oui (6+4 ou moins) | Troncature irréversible | PostgreSQL -- tables `payment_transactions` |
| Nom du titulaire | Non | Délégué au PSP | -- |
| Date d'expiration | Non | Délégué au PSP | -- |
| CVV/CVC2 | **Jamais** | Transit uniquement vers PSP | Mémoire Actix-web (transitoire) |
| PIN | **Jamais** | Jamais présent dans BANKO | -- |
| Token de paiement | Oui | Aléatoire, sans relation mathématique avec le PAN | PostgreSQL -- tables `payment_tokens` |

### 1.3 Architecture de protection

```
    Navigateur client                     PSP Externe
         │                                     ▲
         │ TLS 1.3                              │ HTTPS/mTLS
         ▼                                      │
    ┌──────────┐                          ┌─────┴──────┐
    │ Traefik  │──── HTTP interne ───────►│ Payment    │
    │ (TLS)    │                          │ Handler    │
    └──────────┘                          └─────┬──────┘
                                                │
                              ┌─────────────────┼─────────────────┐
                              │                 │                 │
                              ▼                 ▼                 ▼
                     ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
                     │ PostgreSQL   │  │ Token Vault  │  │ Key Store    │
                     │ (tokens +   │  │ (PAN chiffré │  │ (clés de     │
                     │  PAN tronqué)│  │  AES-256-GCM)│  │  chiffrement)│
                     └──────────────┘  └──────────────┘  └──────────────┘
                     Chiffrement champ  Chiffrement champ  HSM / Vault
```

---

## 2. Tokenisation

### 2.1 Principe de la tokenisation

La tokenisation consiste à remplacer une donnée sensible (le PAN) par une valeur de substitution (le token) qui n'a **aucune relation mathématique** avec la donnée originale. Contrairement au chiffrement, un token ne peut pas être « déchiffré » -- seul le vault de tokens conserve la correspondance.

| Caractéristique | Chiffrement | Tokenisation |
|-----------------|:-----------:|:------------:|
| Réversibilité | Oui (avec la clé) | Non (lookup dans le vault) |
| Relation mathématique avec l'original | Oui | Non |
| Compromission de la clé = compromission des données | Oui | Non applicable |
| Réduction du périmètre PCI DSS | Limitée | **Significative** |
| Performance | Variable (calcul crypto) | Rapide (lookup en table) |

### 2.2 Architecture de tokenisation BANKO

#### 2.2.1 Composants

| Composant | Rôle | Technologie | In-scope PCI DSS ? |
|-----------|------|-------------|:-------------------:|
| **Payment Handler** | Réception du PAN, demande de tokenisation | Actix-web (Rust) | **Oui** |
| **Token Vault** | Génération du token, stockage du mapping | Service dédié (Rust) ou HashiCorp Vault | **Oui** |
| **Token Database** | Persistance du mapping token ↔ PAN chiffré | PostgreSQL (schéma dédié `token_vault`) | **Oui** |
| **Key Store** | Stockage des clés de chiffrement du vault | HSM ou HashiCorp Vault | **Oui** |
| **Autres modules BANKO** | Utilisent uniquement les tokens | Actix-web (Rust) | **Non** (si segmentés) |

#### 2.2.2 Flux de tokenisation

```
    1. Le Payment Handler reçoit le PAN du client (via TLS 1.3)
    2. Le PAN est immédiatement transmis au Token Vault (mTLS interne)
    3. Le Token Vault :
       a. Vérifie si un token existe déjà pour ce PAN (dédoublonnage)
       b. Si non : génère un token aléatoire (UUID v4 ou format personnalisé)
       c. Chiffre le PAN avec AES-256-GCM (clé du Key Store)
       d. Stocke le mapping : token → PAN chiffré
       e. Retourne le token au Payment Handler
    4. Le Payment Handler utilise le token pour toutes les opérations
    5. Le PAN clair est effacé de la mémoire (zéroisation)
    6. Pour la communication avec le PSP : dé-tokenisation contrôlée
```

#### 2.2.3 Schéma de la base Token Vault

```sql
-- Schéma dédié, accès restreint
CREATE SCHEMA token_vault;

CREATE TABLE token_vault.pan_tokens (
    token_id        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_value     VARCHAR(64) NOT NULL UNIQUE,  -- Token aléatoire
    pan_encrypted   BYTEA NOT NULL,               -- PAN chiffré AES-256-GCM
    pan_hash        BYTEA NOT NULL,               -- SHA-256(PAN) pour dédoublonnage
    nonce           BYTEA NOT NULL,               -- Nonce AES-GCM (12 bytes)
    key_version     INTEGER NOT NULL,             -- Version de la clé utilisée
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_accessed   TIMESTAMPTZ,
    access_count    INTEGER DEFAULT 0
);

-- Index pour recherche par hash (dédoublonnage)
CREATE UNIQUE INDEX idx_pan_hash ON token_vault.pan_tokens(pan_hash);

-- Pas d'index sur pan_encrypted (jamais recherché directement)
```

### 2.3 Format Preserving Tokenization vs tokens aléatoires

| Critère | Format Preserving (FPE) | Token aléatoire (UUID) |
|---------|:-----------------------:|:----------------------:|
| Format identique au PAN (16 chiffres) | Oui | Non |
| Compatibilité systèmes legacy | Excellente | Nécessite adaptation |
| Sécurité cryptographique | Bonne (FF1/FF3-1) | Excellente (pas de relation) |
| Risque de confusion token/PAN | **Élevé** | Nul |
| Complexité d'implémentation | Élevée | Faible |
| **Recommandation BANKO** | Non recommandé | **Recommandé** |

**Décision BANKO** : Utilisation de **tokens aléatoires** (UUID v4) pour éviter toute confusion entre tokens et PAN réels. Les systèmes internes BANKO sont conçus pour accepter des tokens de format différent du PAN.

### 2.4 Avantages pour la réduction du périmètre PCI DSS

La tokenisation permet de **réduire significativement** le nombre de composants in-scope :

| Sans tokenisation | Avec tokenisation |
|:-----------------:|:-----------------:|
| Tous les modules manipulant des transactions sont in-scope | Seuls le Payment Handler et le Token Vault sont in-scope |
| ~8 bounded contexts in-scope | 1 bounded context + Token Vault in-scope |
| SAQ D / ROC complet | Périmètre réduit possible (selon modèle PSP) |

---

## 3. Chiffrement au repos

### 3.1 Exigence PCI DSS v4.0.1

L'exigence **3.5.1** impose que le PAN soit rendu illisible partout où il est stocké. L'exigence **3.5.1.2**, devenue **obligatoire le 31 mars 2025**, précise que le chiffrement au niveau disque seul n'est **plus suffisant** :

| Méthode | Conforme avant mars 2025 | Conforme après mars 2025 |
|---------|:------------------------:|:------------------------:|
| Chiffrement disque complet (FDE) uniquement | Oui (avec contrôle d'accès) | **Non** |
| Chiffrement au niveau base de données (TDE) | Oui | **Partiellement** (selon implémentation) |
| Chiffrement au niveau colonne/champ | Oui | **Oui** |
| Tokenisation | Oui | **Oui** |
| Hachage unidirectionnel fort | Oui (avec sel) | **Oui** |
| Troncature | Oui (max 6+4) | **Oui** |

### 3.2 Pourquoi le chiffrement disque seul n'est plus suffisant

Le chiffrement au niveau disque (FDE -- Full Disk Encryption) présente les limitations suivantes, désormais reconnues comme insuffisantes par le PCI SSC :

| Limitation | Explication | Risque |
|------------|-------------|--------|
| **Transparence pour l'application** | Les données sont déchiffrées automatiquement lorsque le système est démarré | Un accès à la base de données en fonctionnement expose les données en clair |
| **Pas de contrôle d'accès granulaire** | Tout utilisateur ayant accès au volume déchiffré voit les données | Un DBA ou un attaquant avec accès SQL voit le PAN en clair |
| **Protection uniquement hors tension** | Protège contre le vol physique du disque, pas contre l'accès logique | Scénario de vol physique peu probable en environnement cloud/conteneurisé |
| **Pas de séparation des rôles** | La même clé protège toutes les données du volume | Impossible de restreindre l'accès à certaines colonnes |

### 3.3 Chiffrement au niveau champ dans BANKO

#### 3.3.1 Algorithme et paramètres

| Paramètre | Valeur | Justification |
|-----------|--------|---------------|
| **Algorithme** | AES-256-GCM | Standard NIST ; authentifié (intégrité + confidentialité) |
| **Taille de clé** | 256 bits | Niveau de sécurité maximal AES |
| **Nonce / IV** | 96 bits (12 octets), unique par chiffrement | Recommandation NIST SP 800-38D |
| **Tag d'authentification** | 128 bits | Intégrité garantie |
| **Encodage stockage** | BYTEA (PostgreSQL) | Stockage binaire natif |
| **Dérivation de clé** | HKDF-SHA256 à partir de la clé maître | Isolation des clés par usage |

#### 3.3.2 Hiérarchie des clés

```
    ┌──────────────────────────────────┐
    │       Clé Maître (KEK)           │
    │  Stockée dans HSM / Vault        │
    │  Jamais exposée en clair         │
    └──────────┬───────────────────────┘
               │ HKDF-SHA256
               ▼
    ┌──────────────────────────────────┐
    │    Clé de Chiffrement Données    │
    │         (DEK) versionnée         │
    │  Version actuelle + N-1          │
    └──────────┬───────────────────────┘
               │ AES-256-GCM
               ▼
    ┌──────────────────────────────────┐
    │     Données chiffrées            │
    │  pan_encrypted = AES-GCM(DEK,   │
    │                   nonce, PAN)    │
    └──────────────────────────────────┘
```

#### 3.3.3 Colonnes chiffrées dans PostgreSQL

| Table | Colonne | Type de donnée | Méthode de protection |
|-------|---------|----------------|----------------------|
| `token_vault.pan_tokens` | `pan_encrypted` | PAN complet | AES-256-GCM (colonne chiffrée) |
| `token_vault.pan_tokens` | `pan_hash` | Empreinte du PAN | SHA-256 avec sel (recherche dédoublonnage) |
| `payment.transactions` | `card_token` | Token de paiement | Token (pas de PAN) -- pas de chiffrement nécessaire |
| `payment.transactions` | `pan_truncated` | PAN tronqué (6+4) | Troncature irréversible -- pas de chiffrement nécessaire |
| `payment.transactions` | `amount_encrypted` | Montant (optionnel) | AES-256-GCM si requis par la politique locale |

### 3.4 Gestion des clés

#### 3.4.1 Cycle de vie des clés

| Phase | Action | Fréquence | Responsable |
|-------|--------|-----------|-------------|
| **Génération** | Génération dans le HSM / Vault avec entropie certifiée | À l'initialisation puis à chaque rotation | Administrateur sécurité (double contrôle) |
| **Distribution** | Transmission sécurisée aux composants autorisés | À chaque démarrage du service | Automatique (Vault Agent / sidecar K8s) |
| **Utilisation** | Chiffrement/déchiffrement des données | Continue | Payment Handler |
| **Rotation** | Nouvelle version de la DEK ; re-chiffrement progressif des données existantes | **Trimestrielle** (90 jours) | Administrateur sécurité |
| **Révocation** | Invalidation d'une clé compromise | Sur incident | RSSI |
| **Destruction** | Suppression cryptographique irréversible | Après expiration de la rétention | Administrateur sécurité (double contrôle) |

#### 3.4.2 Rotation des clés

La rotation des clés s'effectue sans interruption de service selon le processus suivant :

1. **Génération** d'une nouvelle DEK (version N+1) dans le Key Store
2. **Activation** de la nouvelle DEK pour tous les nouveaux chiffrements
3. **Re-chiffrement progressif** des données existantes (batch asynchrone)
4. **Vérification** de l'intégrité après re-chiffrement
5. **Archivage** de l'ancienne DEK (version N) pendant la période de rétention
6. **Destruction** de l'ancienne DEK après confirmation du re-chiffrement complet

#### 3.4.3 DUKPT (Derived Unique Key Per Transaction)

Pour les intégrations avec des terminaux de paiement physiques, BANKO supporte le protocole **DUKPT** (ANSI X9.24) :

| Caractéristique | Description |
|-----------------|-------------|
| **Principe** | Chaque transaction utilise une clé unique dérivée d'une clé initiale (IPEK) |
| **Avantage** | Compromission d'une clé de transaction ne compromet pas les autres transactions |
| **Usage BANKO** | Communication avec les terminaux de paiement via le PSP |
| **Implémentation** | Déléguée au PSP ; BANKO valide la conformité du PSP |

---

## 4. Chiffrement en transit

### 4.1 TLS 1.3 obligatoire

Toute communication transportant des données CHD doit être protégée par **TLS 1.3** (minimum). La configuration de Traefik dans BANKO applique cette exigence :

| Paramètre | Configuration | Justification |
|-----------|---------------|---------------|
| **Version minimale** | TLS 1.3 | Seule version recommandée par PCI DSS v4.0.1 |
| **Cipher suites** | TLS_AES_256_GCM_SHA384, TLS_CHACHA20_POLY1305_SHA256 | Suites AEAD uniquement |
| **Certificats** | RSA 4096 ou ECDSA P-384 | Taille de clé conforme aux recommandations ANSSI |
| **HSTS** | Activé (max-age 31536000, includeSubDomains) | Protection contre le downgrade HTTP |
| **OCSP Stapling** | Activé | Vérification de révocation performante |

### 4.2 Configuration Traefik

```yaml
# Extrait de la configuration TLS Traefik pour BANKO
tls:
  options:
    default:
      minVersion: VersionTLS13
      cipherSuites:
        - TLS_AES_256_GCM_SHA384
        - TLS_CHACHA20_POLY1305_SHA256
      sniStrict: true
    secure:
      minVersion: VersionTLS13
      clientAuth:
        clientAuthType: RequireAndVerifyClientCert
        caFiles:
          - /certs/ca.pem
```

### 4.3 mTLS pour communications inter-services

Les communications entre les composants du CDE utilisent le **mutual TLS (mTLS)** pour garantir l'authentification bidirectionnelle :

| Communication | Protocole | Authentification |
|---------------|-----------|------------------|
| Traefik → Payment Handler | HTTP interne (réseau isolé) | Network Policy K8s (isolation réseau) |
| Payment Handler → Token Vault | **mTLS** | Certificats client/serveur dédiés |
| Payment Handler → PostgreSQL | **TLS** | Certificat serveur + authentification PostgreSQL |
| Payment Handler → PSP externe | **HTTPS / mTLS** | Certificats échangés avec le PSP |
| Token Vault → Key Store (HSM/Vault) | **mTLS** | Certificats client dédiés |

### 4.4 Certificate pinning pour applications mobiles

Pour les applications mobiles bancaires intégrant les API BANKO, le **certificate pinning** est obligatoire :

| Aspect | Recommandation |
|--------|---------------|
| **Type de pinning** | Pin du certificat intermédiaire (pas du leaf) |
| **Backup pins** | Au moins 2 pins de backup (rotation sans interruption) |
| **Durée de validité** | Max 90 jours avant rotation obligatoire |
| **Mécanisme de secours** | Endpoint de mise à jour des pins signé par clé de confiance |
| **Reporting** | Envoi de rapports en cas d'échec de validation (HPKP reporting) |

---

## 5. HSM et gestion des clés

### 5.1 Hardware Security Module (HSM)

Un HSM est **fortement recommandé** pour les déploiements de production de BANKO dans les environnements suivants :

| Contexte | Recommandation HSM |
|----------|:-----------------:|
| Banque avec licence SMT (Système Monétique Tunisien) | **Obligatoire** |
| Banque traitant plus de 6 millions de transactions/an (Niveau 1 PCI) | **Obligatoire** |
| Banque traitant moins de 6 millions de transactions/an | **Fortement recommandé** |
| Environnement de développement / test | Non requis (HashiCorp Vault suffit) |

#### 5.1.1 Spécifications HSM minimales

| Critère | Exigence minimale |
|---------|-------------------|
| **Certification** | FIPS 140-2 Level 3 (minimum) ou FIPS 140-3 Level 3 |
| **Algorithmes** | AES-256, RSA-4096, ECDSA P-384, SHA-256/384/512 |
| **Performance** | ≥ 1000 opérations AES/seconde (adapté au volume transactionnel) |
| **Haute disponibilité** | Cluster actif-actif ou actif-passif |
| **Sauvegarde** | Sauvegarde chiffrée des clés sur carte à puce (split knowledge) |
| **Audit** | Journalisation interne tamper-proof de toutes les opérations |

### 5.2 HashiCorp Vault comme alternative logicielle

Pour les déploiements où un HSM physique n'est pas disponible, **HashiCorp Vault** constitue une alternative acceptable :

| Fonctionnalité | Configuration BANKO |
|----------------|---------------------|
| **Backend de stockage** | Consul (HA) ou PostgreSQL dédié |
| **Unseal mechanism** | Shamir's Secret Sharing (3/5 minimum) ou Auto-unseal via cloud KMS |
| **Secret engine** | Transit (chiffrement/déchiffrement), KV v2 (stockage de secrets) |
| **Authentification** | AppRole pour les services, OIDC pour les opérateurs |
| **Politiques d'accès** | Least privilege par chemin ; Payment Handler : transit uniquement |
| **Audit** | Audit log activé, envoyé vers le SIEM centralisé |
| **Haute disponibilité** | Cluster Vault en mode HA (3 noeuds minimum) |

### 5.3 Key ceremonies

Les opérations critiques sur les clés cryptographiques nécessitent des cérémonies formalisées :

| Cérémonie | Participants | Contrôle | Documentation |
|-----------|-------------|----------|---------------|
| **Génération de la KEK** | 2 administrateurs sécurité + 1 témoin | Double contrôle (dual control) | PV signé par tous les participants |
| **Rotation de la DEK** | 1 administrateur sécurité + 1 opérateur Payment | Split knowledge | Ticket de changement + log d'audit |
| **Révocation d'urgence** | RSSI + 1 administrateur sécurité | Procédure d'urgence documentée | Rapport d'incident |
| **Destruction de clé** | 2 administrateurs sécurité | Vérification que toutes les données ont été re-chiffrées | PV de destruction |

### 5.4 Split knowledge et dual control

| Principe | Implémentation BANKO |
|----------|---------------------|
| **Split knowledge** | La clé maître (KEK) est divisée en parts (Shamir's Secret Sharing) ; aucun individu ne connaît la clé complète |
| **Dual control** | Toute opération sur la KEK nécessite la présence de 2 personnes minimum avec des rôles distincts |
| **Seuil de reconstruction** | 3 parts sur 5 nécessaires pour reconstruire la KEK (schéma 3/5) |
| **Gardiens des parts** | 5 personnes distinctes, identifiées par nom dans le registre des clés |

---

## 6. Intégration avec Rust

### 6.1 Avantages de Rust pour la sécurité cryptographique

Le choix de Rust pour le backend BANKO apporte des garanties de sécurité particulièrement précieuses dans le contexte PCI DSS :

| Avantage Rust | Impact PCI DSS |
|---------------|---------------|
| **Memory safety** (pas de buffer overflow) | Élimine une classe entière de vulnérabilités (Req. 6.2.4) |
| **Ownership system** (pas de use-after-free) | Protège les données sensibles en mémoire |
| **No garbage collector** | Contrôle précis du moment de zéroisation des secrets |
| **Type system fort** | Prévient les confusions entre PAN, tokens et données masquées |
| **Zero-cost abstractions** | Sécurité sans compromis sur la performance (P99 < 5ms) |

### 6.2 Crates recommandés

| Crate | Version | Usage | Audit de sécurité |
|-------|---------|-------|:------------------:|
| `ring` | 0.17+ | Primitives cryptographiques bas niveau (AES-GCM, SHA-256, HKDF) | Audité (Google) |
| `rustls` | 0.23+ | TLS 1.3 natif Rust (remplacement OpenSSL) | Audité (ISRG) |
| `aes-gcm` | 0.10+ | Chiffrement AES-256-GCM haut niveau | Audité (RustCrypto) |
| `zeroize` | 1.8+ | Zéroisation sécurisée de la mémoire | Audité (RustCrypto) |
| `secrecy` | 0.8+ | Wrapper pour valeurs secrètes (empêche le logging accidentel) | Audité |
| `argon2` | 0.5+ | Hachage de mots de passe (Identity BC) | Standard (RFC 9106) |
| `rand` | 0.8+ | Génération de nombres aléatoires cryptographiquement sûrs | Audité |
| `x509-parser` | 0.16+ | Parsing et validation de certificats | Communautaire |

### 6.3 Patterns de chiffrement dans le code BANKO

#### 6.3.1 Type newtype pour le PAN

```rust
use secrecy::{ExposeSecret, Secret};
use zeroize::Zeroize;

/// PAN encapsulé -- ne peut pas être affiché ou logué accidentellement.
/// Le trait Display affiche uniquement la version tronquée.
#[derive(Clone, Zeroize)]
pub struct Pan(Secret<String>);

impl Pan {
    /// Crée un PAN validé (Luhn check).
    pub fn new(value: String) -> Result<Self, PanError> {
        if !luhn_check(&value) {
            return Err(PanError::InvalidChecksum);
        }
        if value.len() < 13 || value.len() > 19 {
            return Err(PanError::InvalidLength);
        }
        Ok(Pan(Secret::new(value)))
    }

    /// Accès contrôlé au PAN clair (uniquement pour chiffrement/tokenisation).
    pub fn expose(&self) -> &str {
        self.0.expose_secret()
    }

    /// Version tronquée pour affichage (6 premiers + 4 derniers).
    pub fn truncated(&self) -> String {
        let pan = self.0.expose_secret();
        let len = pan.len();
        format!("{}****{}", &pan[..6], &pan[len-4..])
    }
}

/// Le PAN ne peut JAMAIS être affiché en clair par fmt::Display.
impl std::fmt::Display for Pan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.truncated())
    }
}

/// Le PAN ne peut JAMAIS apparaître dans les logs via fmt::Debug.
impl std::fmt::Debug for Pan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pan([REDACTED])")
    }
}
```

#### 6.3.2 Service de chiffrement

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use zeroize::Zeroize;

pub struct EncryptionService {
    cipher: Aes256Gcm,
    key_version: u32,
}

impl EncryptionService {
    pub fn new(key_bytes: &[u8; 32], key_version: u32) -> Self {
        let key = Key::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);
        Self { cipher, key_version }
    }

    /// Chiffre une donnée sensible. Le nonce est généré aléatoirement.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData, CryptoError> {
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self.cipher.encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)?;
        Ok(EncryptedData {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
            key_version: self.key_version,
        })
    }

    /// Déchiffre une donnée. La mémoire du résultat doit être zéroïsée après usage.
    pub fn decrypt(&self, data: &EncryptedData) -> Result<Vec<u8>, CryptoError> {
        let nonce = Nonce::from_slice(&data.nonce);
        self.cipher.decrypt(nonce, data.ciphertext.as_ref())
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}
```

### 6.4 Zéroisation mémoire

La zéroisation mémoire est **critique** pour éviter que des données sensibles persistent en mémoire après usage :

| Situation | Risque sans zéroisation | Solution BANKO |
|-----------|------------------------|----------------|
| PAN en mémoire après tokenisation | Extraction via dump mémoire | `zeroize` crate sur le type `Pan` |
| Clé de chiffrement après rotation | Ancienne clé récupérable | `Zeroize` sur `EncryptionService::drop()` |
| Buffer de réponse PSP | Données carte temporaires | `Vec<u8>` avec `zeroize` avant libération |
| Logs en mémoire | PAN dans les buffers de log | `secrecy::Secret` empêche le logging |

```rust
impl Drop for EncryptionService {
    fn drop(&mut self) {
        // La clé est zéroïsée automatiquement grâce au trait Zeroize
        // implémenté par aes_gcm::Key
    }
}
```

---

## 7. Conformité avec la réglementation tunisienne

### 7.1 Cadre réglementaire applicable

En complément de PCI DSS v4.0.1, les déploiements de BANKO en Tunisie doivent se conformer aux réglementations suivantes en matière de protection des données et de chiffrement :

| Réglementation | Référence | Exigences cryptographiques |
|----------------|-----------|---------------------------|
| **Loi organique n 2004-63** relative à la protection des données à caractère personnel | JORT n 61 du 30 juillet 2004 | Mesures techniques appropriées pour protéger les données personnelles |
| **Décret n 2007-3003** relatif aux conditions et procédures de déclaration/autorisation de traitement | JORT du 4 décembre 2007 | Sécurité et confidentialité des traitements automatisés |
| **Circulaires BCT** relatives à la sécurité des systèmes d'information bancaires | Banque Centrale de Tunisie | Chiffrement des données sensibles, contrôle d'accès, journalisation |
| **Cadre réglementaire SMT** | Système Monétique Tunisien | Conformité PCI DSS pour les opérateurs monétiques |

### 7.2 Exigences spécifiques

| Exigence | Implémentation BANKO | Conformité |
|----------|---------------------|:----------:|
| **Chiffrement des données personnelles** au repos | AES-256-GCM au niveau champ pour les données bancaires | Conforme |
| **Pseudonymisation** des données pour les traitements analytiques | Tokenisation du PAN ; anonymisation pour les rapports statistiques | Conforme |
| **Localisation des données** (selon réglementation BCT) | Déploiement on-premise ou cloud souverain configurable | Conforme (selon hébergeur) |
| **Droit d'accès et de rectification** des titulaires | API dédiée dans le Customer BC pour l'exercice des droits | Conforme |
| **Notification en cas de violation** de données | Procédure de notification documentée (72h max) ; intégrée au plan de réponse aux incidents | Conforme |
| **Registre des traitements** | Documenté dans le Governance BC ; mis à jour automatiquement | En cours |

### 7.3 Contexte du paiement mobile en Tunisie

L'essor des solutions de paiement mobile en Tunisie (OFT, Walletii, Kashy) introduit des considérations supplémentaires :

| Aspect | Recommandation BANKO |
|--------|---------------------|
| **Intégration avec les wallets mobiles** | API sécurisée (OAuth 2.0 + mTLS) pour les fournisseurs de paiement mobile |
| **Tokenisation des cartes dans les wallets** | Support des standards EMVCo pour la tokenisation de paiement |
| **Authentification forte (SCA)** | 3D Secure 2.x intégré pour les paiements en ligne et mobiles |
| **Protection des données sur mobile** | Certificate pinning obligatoire ; stockage sécurisé (Keychain/Keystore) |
| **Conformité OFT** | Documentation d'intégration fournie ; conformité aux exigences de l'OFT |

---

## Références

| Document | Lien |
|----------|------|
| PCI DSS v4.0.1 -- Exigence 3 (Protection des données stockées) | [PCI SSC](https://www.pcisecuritystandards.org/) |
| NIST SP 800-38D (AES-GCM) | [NIST](https://csrc.nist.gov/publications/detail/sp/800-38d/final) |
| NIST SP 800-57 (Gestion des clés) | [NIST](https://csrc.nist.gov/publications/detail/sp/800-57-part-1/rev-5/final) |
| PCI SSC Tokenization Guidelines | [PCI SSC Information Supplements](https://www.pcisecuritystandards.org/document_library/) |
| RustCrypto -- aes-gcm | [GitHub](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm) |
| RustCrypto -- zeroize | [GitHub](https://github.com/RustCrypto/utils/tree/master/zeroize) |
| Définition du périmètre CDE | [01-cde-scope-definition.md](./01-cde-scope-definition.md) |
| Mapping des exigences PCI DSS | [02-requirements-mapping.md](./02-requirements-mapping.md) |
| Matrice des responsabilités | [04-responsibility-matrix.md](./04-responsibility-matrix.md) |
| Référentiel légal BANKO | [REFERENTIEL_LEGAL_ET_NORMATIF.md](../../legal/REFERENTIEL_LEGAL_ET_NORMATIF.md) |

---

*Document généré dans le cadre du programme de conformité PCI DSS de la plateforme BANKO. Toute modification doit suivre le processus de revue documentaire.*
