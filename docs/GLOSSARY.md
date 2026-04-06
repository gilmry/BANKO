# Glossaire BANKO (Ubiquitous Language)

## Termes Metier

| Terme | Definition |
|-------|-----------|
| **Account** | Compte bancaire (courant, epargne, DAT) |
| **AML** | Anti-Money Laundering - Lutte contre le blanchiment |
| **Asset Class** | Classification de creance (0-4 selon circulaire BCT) |
| **Audit Trail** | Piste d'audit immutable de toutes les operations |
| **BCT** | Banque Centrale de Tunisie |
| **BIC** | Bank Identifier Code (code SWIFT) |
| **Bounded Context** | Contexte delimite DDD - module autonome |
| **C/D Ratio** | Ratio Credits/Depots (seuil: 120%) |
| **Customer** | Client de la banque (personne physique ou morale) |
| **DAT** | Depot A Terme |
| **DOS** | Declaration de Operation Suspecte |
| **DDD** | Domain-Driven Design |
| **FX** | Foreign Exchange - Operations de change |
| **GAFI/FATF** | Groupe d'Action Financiere Internationale |
| **HSM** | Hardware Security Module |
| **IFRS 9** | Norme comptable internationale (instruments financiers) |
| **INPDP** | Instance Nationale de Protection des Donnees Personnelles |
| **KYC** | Know Your Customer - Identification client |
| **LBC/FT** | Lutte contre le Blanchiment et le Financement du Terrorisme |
| **Money** | Value Object representant un montant + devise |
| **NCT** | Normes Comptables Tunisiennes |
| **PEP** | Personne Exposee Politiquement |
| **Provision** | Montant mis de cote pour couvrir un risque de credit |
| **RIB** | Releve d'Identite Bancaire (20 chiffres en Tunisie) |
| **Solvency Ratio** | Ratio de solvabilite (seuil: 10%) |
| **SWIFT** | Society for Worldwide Interbank Financial Telecommunication |
| **Tier 1** | Fonds propres de base (seuil: 7%) |
| **TND** | Dinar Tunisien |
| **Value Object** | Objet immutable defini par ses attributs (pas d'identite) |

## Termes Techniques

| Terme | Definition |
|-------|-----------|
| **Aggregate** | Cluster d'entites avec racine et invariants |
| **Port** | Interface (trait Rust) definie par la couche Application |
| **Adapter** | Implementation concrete d'un port |
| **Use Case** | Logique d'orchestration dans la couche Application |
| **DTO** | Data Transfer Object pour les contrats API |
| **Entity** | Objet avec identite unique |
| **Repository** | Abstraction d'acces aux donnees |
