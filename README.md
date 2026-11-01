# phone_trie

Stockage et accès efficace de numéros de téléphone à l'aide d'un **trie** (prefix tree) implémenté from scratch en Rust.

Le programme lit un fichier JSON de contacts, construit le trie en mémoire, puis génère un fichier PlantUML (format MindMap) permettant de visualiser la structure.

> Projet en cours d'initialisation : la lecture du JSON, le trie et l'export PlantUML ne sont pas encore implémentés.

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) 1.85 ou plus récent (édition 2024)
- [Docker](https://docs.docker.com/desktop/) (optionnel, uniquement pour visualiser les graphes)

## Lancer le projet

```bash
cargo run --release
```

## Lancer les tests

```bash
cargo test
```

## Structure du projet

```text
.
├── src/
│   ├── lib.rs     # bibliothèque (#![forbid(unsafe_code)])
│   └── main.rs    # point d'entrée du programme
├── data/          # fichiers JSON de test
└── graph/         # fichiers PlantUML (.puml) générés
```

## Format des données

Les fichiers d'entrée sont des tableaux JSON de contacts. Les numéros n'ont pas de préfixe international.

```json
[
    { "nb": "112", "name": "Urgences" },
    { "nb": "15", "name": "SAMU" }
]
```

## Visualiser un graphe PlantUML

PlantUML n'est pas appelé par le programme : la visualisation se fait manuellement.

1. Récupérer l'image : `docker pull plantuml/plantuml-server:jetty`
2. Lancer le serveur : `docker run -d -p 8080:8080 plantuml/plantuml-server:jetty`
3. Ouvrir <http://localhost:8080/>
4. Copier-coller le contenu d'un fichier du dossier `graph/`

Syntaxe des diagrammes MindMap : <https://plantuml.com/fr/mindmap-diagram>

## Conventions

L'historique git suit les [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (`feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `chore:`…).

## Équipe

- Gabin
- Maxime
- Erwin
