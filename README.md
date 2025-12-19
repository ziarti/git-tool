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
├── Cargo.toml          # Dépendances (clap, comfy_table)
├── src/
│   ├── main.rs         # Point d'entrée et orchestration
│   ├── cli.rs          # Définition CLI avec clap
│   ├── git.rs          # Logique Git et struct BranchInfo
│   └── renderer.rs     # Affichage avec tableaux colorés
├── CLAUDE.md           # Documentation pour Claude Code
└── README.md           # Ce fichier
```

### Commandes de développement

```bash
# Compiler en mode développement
cargo build

# Lancer les tests
cargo test

# Vérifier le code sans compiler
cargo check

# Formater le code
cargo fmt

# Linter
cargo clippy
```

## 🧰 Technologies utilisées

- **Rust** - Langage de programmation
- **clap** - Parsing des arguments CLI avec macros derive
- **comfy_table** - Affichage de tableaux formatés et colorés
- **std::process::Command** - Exécution des commandes git
- **std::collections::HashMap** - Groupement des branches par auteur

## 📝 Fonctionnement technique

1. **Liste les branches** : Exécute `git for-each-ref` pour obtenir toutes les branches distantes avec métadonnées (date, auteur)
2. **Calcule le retard** : Pour chaque branche, exécute `git rev-list --count <branch>..<origin>` pour compter les commits
3. **Filtre et affiche** : Ne garde que les branches dépassant le seuil configuré


## 📄 Licence

Projet éducatif - MIT ou Apache-2.0

## 👤 Auteur

Arthur Farrugia
