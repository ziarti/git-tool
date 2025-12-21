# Git-Tool

Outil CLI en Rust pour identifier les branches Git en retard sur la branche principale.

## 📋 Description

`git-tool` analyse un dépôt Git et identifie les branches distantes qui ont accumulé un retard significatif (en nombre de commits) par rapport à une branche de référence.

**Cas d'usage :**
- Identifier les branches à nettoyer
- Détecter les branches abandonnées
- Suivre l'état de santé des branches de développement

## 🚀 Installation

### Prérequis
- Rust 1.70+ (vérifier avec `rustc --version`)
- Git installé et configuré

### Installation depuis les sources

```bash
# Cloner le dépôt
git clone git@github.com:ziarti/git-tool.git
cd git-tool

# Option 1 : Compiler en mode release
cargo build --release
# Le binaire sera dans target/release/git-tool

# Option 2 : Installer globalement
cargo install --path .
# git-tool sera disponible dans votre PATH
```

## 📖 Utilisation

### Syntaxe de base

```bash
git-tool stale-branches [OPTIONS]
```

### Options

| Option | Court | Description | Défaut |
|--------|-------|-------------|--------|
| `--threshold` | `-t` | Nombre minimum de commits de retard | 100 |
| `--origin-branch` | `-o` | Branche de référence | origin/production |
| `--help` | `-h` | Afficher l'aide | - |

### Exemples

```bash
# Utilisation par défaut (>100 commits derrière origin/production)
git-tool stale-branches

# Branches avec >50 commits de retard
git-tool stale-branches -t 50

# Comparer avec origin/master au lieu de origin/production
git-tool stale-branches -o origin/master

# Combiner les deux options
git-tool stale-branches -t 30 -o origin/main

# Depuis le code source (sans installation)
cargo run -- stale-branches -t 50
```

### Sortie exemple

```
🔍 Recherche des branches obsolètes à partir de la branch origin/production (>100 commits)...

Auteur: Jane Smith
┌─────────────────────────┬────────────┬─────────────┬───────────────────────────┐
│ Branch                  │ Date       │ Auteur      │ Nombre de commits en plus │
├─────────────────────────┼────────────┼─────────────┼───────────────────────────┤
│ origin/bugfix-legacy    │ 2025-09-20 │ Jane Smith  │ 203                       │ (rouge)
│ origin/feature-old      │ 2025-10-15 │ Jane Smith  │ 145                       │ (jaune)
└─────────────────────────┴────────────┴─────────────┴───────────────────────────┘
2 branches analysées pour Jane Smith

Auteur: John Doe
┌─────────────────────────┬────────────┬─────────────┬───────────────────────────┐
│ Branch                  │ Date       │ Auteur      │ Nombre de commits en plus │
├─────────────────────────┼────────────┼─────────────┼───────────────────────────┤
│ origin/api-refactor     │ 2025-11-01 │ John Doe    │ 112                       │ (blanc)
└─────────────────────────┴────────────┴─────────────┴───────────────────────────┘
1 branches analysées pour John Doe

Total branch: 3
```

**Code couleur** :
- 🔴 Rouge : > 2x le seuil (très obsolète)
- 🟡 Jaune : > 1.5x le seuil (attention)
- ⚪ Blanc : > seuil (à surveiller)

## 🛠️ Développement

### Structure du projet

```
git-tool/
├── Cargo.toml              # Dépendances et configuration
├── src/
│   ├── lib.rs              # Bibliothèque publique (API)
│   ├── main.rs             # Point d'entrée binaire
│   ├── cli.rs              # Définition CLI avec clap
│   ├── errors.rs           # Types d'erreurs personnalisés (GitError)
│   ├── git_cmd.rs          # Trait GitCommandTrait et implémentation
│   ├── git_repository.rs   # Logique métier Git (BranchInfo, validation)
│   └── renderer.rs         # Affichage avec tableaux colorés
├── tests/
│   └── git_tests.rs        # Tests d'intégration avec MockGitCommand
└── README.md               # Ce fichier
```

**Architecture :**
- **lib.rs + main.rs** : Séparation bibliothèque/binaire pour réutilisabilité et tests
- **Traits** : `GitCommandTrait` permet l'injection de dépendances et le mocking
- **Erreurs typées** : `GitError` enum avec variants spécifiques (thiserror)
- **Tests** : Tests unitaires dans `#[cfg(test)]` + tests d'intégration dans `tests/`

### Commandes de développement

```bash
# Compiler en mode développement
cargo build

# Lancer tous les tests (unitaires + intégration)
cargo test

# Lancer uniquement les tests unitaires
cargo test --lib

# Lancer uniquement les tests d'intégration
cargo test --test git_tests

# Revoir les snapshot tests
cargo insta review

# Accepter tous les snapshots
cargo insta accept

# Vérifier le code sans compiler
cargo check

# Formater le code
cargo fmt

# Linter
cargo clippy
```

## 🧰 Technologies utilisées

### Dépendances principales
- **Rust 1.70+** - Langage de programmation système
- **clap 4.x** - Parsing des arguments CLI avec macros derive
- **comfy_table** - Affichage de tableaux formatés et colorés
- **thiserror 2.x** - Génération ergonomique de types d'erreurs
- **anyhow 1.x** - Gestion d'erreurs simplifiée pour applications

### Dépendances de développement
- **insta** - Snapshot testing pour les sorties formatées

### Bibliothèque standard
- **std::process::Command** - Exécution des commandes Git
- **std::collections::HashMap** - Groupement des branches par auteur
- **std::env** - Récupération du dossier de travail actuel

## 📝 Fonctionnement technique

### Workflow principal

1. **Validation du repo Git** : Exécute `git rev-parse --git-dir` pour vérifier qu'on est dans un dépôt Git valide
   - Fonctionne avec repos normaux, submodules, worktrees, et sous-dossiers
   - Fail-fast : arrêt immédiat si pas un repo Git

2. **Liste les branches** : Exécute `git for-each-ref` pour obtenir toutes les branches distantes avec métadonnées (date, auteur)

3. **Calcule le retard** : Pour chaque branche, exécute `git rev-list --count <branch>..<origin>` pour compter les commits de retard

4. **Filtre les résultats** : Ne garde que les branches dépassant le seuil configuré

5. **Affichage coloré** : Groupe par auteur et affiche avec code couleur selon la sévérité

### Gestion d'erreurs

L'application utilise des **erreurs typées** avec des messages clairs :

**`GitError::NotARepository`**
```
Error: Not a valid git repository

Caused by:
    Not a git repository: /tmp/my-folder
```

**`GitError::CommandFailed`**
```
Error: Failed list branches from origin/production

Caused by:
    La commande git for-each-ref ... a échoué: fatal: ...
```

**`GitError::ExecutionError`**
```
Caused by:
    Une erreur lors de l'exécution est survenue: ...
```

**`GitError::ParsingError`**
```
Caused by:
    Erreur de parsing du nombre de commits: ...
```

### Architecture testable

- **Trait `GitCommandTrait`** : Abstraction pour les commandes Git
- **`GitCommand`** : Implémentation réelle pour production
- **`MockGitCommand`** : Implémentation de test sans dépendance Git
- **Tests d'intégration** : Validation du comportement sans exécuter Git réellement


## 📄 Licence

Projet éducatif - MIT ou Apache-2.0

## 👤 Auteur

Arthur Farrugia
