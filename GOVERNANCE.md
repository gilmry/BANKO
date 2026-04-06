# Gouvernance BANKO Open Source

> **Version** : 1.0
> **Dernière mise à jour** : 2026-04-04
> **Statut** : Projet Open Source Communautaire

---

## 📖 Table des Matières

1. [Vision & Principes](#vision--principes)
2. [Structure de Gouvernance](#structure-de-gouvernance)
3. [Membres de la Communauté](#membres-de-la-communauté)
4. [Mainteneurs Techniques](#mainteneurs-techniques)
5. [Contributeurs Externes](#contributeurs-externes)
6. [Processus de Décision](#processus-de-décision)
7. [Code of Conduct & Modération](#code-of-conduct--modération)
8. [Propriété Intellectuelle](#propriété-intellectuelle)

---

## Vision & Principes

**BANKO** est une plateforme bancaire open source conçue pour promouvoir:

- 🔓 **Transparence**: Code ouvert, décisions publiques, gouvernance collaborative
- 🔒 **Sécurité**: Architecture robuste, audit régulier, conformité réglementaire
- ⚖️ **Équité**: Accès égal pour tous, pas de discrimination, inclusivité
- 🎓 **Pédagogie**: Documentation exhaustive, architecture exemplaire (hexagonale/DDD)
- 🌍 **Ouverture**: Bienvenue à tous, indépendamment du contexte

### Objectif

Fournir une implémentation bancaire de référence open source, démontrant les meilleures pratiques en architecture logicielle et sécurité financière.

---

## Structure de Gouvernance

### Organes de Gouvernance

1. **Équipe de Mainteneurs**: Décisions techniques et code review
2. **Commité de Gouvernance**: Décisions stratégiques et modération
3. **Communauté**: Toutes les suggestions et discussions publiques

### Principes de Prise de Décision

| Domaine | Autorité | Processus |
|---------|----------|----------|
| **Architecture technique** | Lead Maintainer | Discussion publique + consensus technique |
| **Nouvelles features** | Communauté + Mainteneurs | GitHub Issues/Discussions + vote si controverse |
| **Sécurité/Bugs critiques** | Mainteneurs | Décision rapide, rapportés confidentiellement |
| **Changements de gouvernance** | Communauté | Consensus large (75%+) |
| **Modération/Code of Conduct** | Comité de Modération | Investigation + sanction graduée |

---

## Membres de la Communauté

### Qui Peut Participer ?

BANKO accueille:

1. **Contributeurs de code**: Rust, Astro/Svelte, documentation
2. **Rapporteurs de bugs**: Identification et triage
3. **Testeurs**: QA, tests de sécurité, E2E
4. **Rédacteurs**: Documentation, traductions, guides
5. **Sympathisants**: Toute personne adhérant aux valeurs du projet

### Participation

- ✅ Proposer des features via GitHub Issues
- ✅ Voter sur des decisions communautaires
- ✅ Soumettre des Pull Requests
- ✅ Participer aux discussions GitHub
- ✅ Signaler les bugs et vulnérabilités

---

## Mainteneurs Techniques

### Rôle des Mainteneurs

Les **mainteneurs** ont les **droits d'écriture** (commit access) sur le dépôt GitHub principal. Ils assurent:

- Review et merge des Pull Requests
- Gestion des releases et versioning
- Supervision de l'architecture hexagonale/DDD
- Résolution des bugs critiques
- Mentorat des nouveaux contributeurs

### Comment Devenir Mainteneur ?

- Nomination par le Lead Maintainer basée sur:
  - Contributions régulières et de qualité (code, docs, tests)
  - Maîtrise de l'architecture hexagonale et Rust
  - Respect du Code of Conduct
  - Disponibilité pour code review

### Mainteneurs Actuels

| Mainteneur | GitHub | Rôle |
|------------|--------|------|
| [À définir] | [@handle] | Lead Maintainer |

### Révocation de Mainteneur

Un mainteneur peut être révoqué en cas de:
- Inactivité prolongée (6+ mois sans contribution)
- Manquement grave au Code of Conduct
- Abus de privilèges (commit access mal utilisé)

---

## Contributeurs Externes

### Contributions Ouvertes

BANKO accueille:

- Code (backend Rust, frontend Astro/Svelte)
- Documentation (guides, traductions, API docs)
- Tests (unitaires, intégration, BDD, E2E)
- Design (UI/UX, icons, assets)
- Rapports de bugs et suggestions

### Developer Certificate of Origin (DCO)

**Tous les contributeurs** doivent signer leurs commits avec le **DCO**:

```bash
git commit -s -m "feat: add customer onboarding"
```

Le flag `-s` ajoute:
```
Signed-off-by: Votre Nom <votre.email@example.com>
```

En signant, vous certifiez que:
1. Vous avez écrit ce code OU avez le droit de le soumettre
2. Vous acceptez qu'il soit publié sous licence AGPL-3.0
3. Vous comprenez que la contribution est publique et permanente

---

## Processus de Décision

### Features Majeures

1. **Proposition**: GitHub Issue ou Discussion
2. **Feedback communauté**: 2 semaines de commentaires
3. **Analyse technique**: Lead Maintainer évalue faisabilité
4. **Vote communauté** (si controverse): Consensus requis
5. **Décision finale**: Mainteneurs
6. **Implémentation**: Ajout à la roadmap

### Features Mineures

1. **GitHub Issue/Discussion**: Proposition ouverte
2. **Triage**: Labeling et assignation
3. **Implémentation**: Contributeur ou mainteneur
4. **Review + Merge**: Validation technique

### Décisions Techniques (Architecture, Stack)

- **Lead Maintainer** décide sur architecture
- En cas de désaccord: **Consensus communauté** requis
- Changements majeurs (migration Rust → autre langage): **Discussion publique étendue**

---

## Code of Conduct & Modération

### Code of Conduct

BANKO adopte le **Contributor Covenant v2.1** (voir `CODE_OF_CONDUCT.md`).

**Principes**:
- Respect, bienveillance, inclusivité
- Zéro tolérance pour harcèlement, discrimination, toxicité
- Sanctions graduées: avertissement → ban temporaire → ban permanent

### Signalement

- **Email**: abuse@banko.tn (réponse sous 48h)
- **Anonymat**: Possible si demandé
- **Confidentialité**: Garantie, pas de publication sans consentement

### Équipe de Modération

| Rôle | Responsabilités |
|------|----------------|
| **Modérateurs** | Première ligne (issues/discussions) |
| **Mainteneurs** | Escalade et violations graves |
| **Lead Maintainer** | Décision finale |

---

## Propriété Intellectuelle

### Droits d'Auteur

- **Code source**: Propriété du projet BANKO
- **Contributions externes**: Restent propriété des auteurs sous licence AGPL-3.0 via DCO
- **Marques**: "BANKO" et logo sous protection du projet

### Licence Open Source

- **Licence**: **AGPL-3.0** (GNU Affero General Public License v3.0)
- **Pourquoi AGPL?**: Copyleft fort pour SaaS (modifications restent open source)
- **Modifiabilité**: Uniquement par vote communauté (consensus 75%+)

### Conformité Légale & Réglementaire

BANKO doit respecter:

- **GDPR**: Directive européenne sur protection des données
- **Conformité bancaire**: Règles prudentielles, AML/KYC, sanctions
- **Locales**: Réglementations nationales du contexte de déploiement

Toute feature affectant ces domaines doit passer par:
1. Review de sécurité (`SECURITY.md`)
2. Audit légal (si applicable)
3. Approbation mainteneurs

---

## Transparence & Communication

### Canaux de Communication

- **GitHub Issues**: Tracking bugs et features
- **GitHub Discussions**: Débats stratégiques, RFCs
- **GitHub Projects**: Roadmap et planning public
- **Email**: abuse@banko.tn pour modération

### Rapports Publics

- Roadmap mise à jour en temps réel
- Décisions documentées sur GitHub
- Sécurité communiquée via `SECURITY.md`

---

## Résolution de Conflits

### Conflits Techniques

1. **Discussion ouverte**: GitHub (arguments publics)
2. **Lead Maintainer décide**: Basé sur architecture hexagonale
3. **Escalade CA**: Vote si désaccord persistant
4. **Respect décision**: Binding, ou fork possible (AGPL le permet)

### Conflits Interpersonnels

1. **Médiation informelle**: Mainteneurs tentent résolution
2. **Médiation formelle**: Comité de Modération
3. **Sanctions**: Selon Code of Conduct

### Conflits d'Intérêt

- **Déclaration obligatoire**: Si impliqué dans décision
- **Abstention**: Ne pas voter sur sujets impliquant conflit

---

## Évolution de la Gouvernance

### Révision Régulière

- **Annuelle**: Revue mineure des statuts
- **Tous les 3-5 ans**: Révision majeure
- **À la croissance**: Adaptation si 10+ contributeurs actifs

### Processus de Modification

1. **Proposition**: GitHub Discussion
2. **Feedback**: 30 jours de commentaires
3. **Révision**: Intégration suggestions
4. **Vote**: Consensus communauté (75%+)
5. **Publication**: Mise à jour GOVERNANCE.md

---

## 📞 Contacts

- **Email général**: contact@banko.tn
- **Sécurité**: security@banko.tn
- **Modération**: abuse@banko.tn
- **GitHub**: [github.com/banko/banko](https://github.com/banko/banko)

---

**BANKO - Gouvernance transparente pour un projet bancaire open source fiable 🏦**
