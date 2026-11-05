# phone_trie

Stockage et accès efficace de numéros de téléphone à l'aide d'un **trie** (prefix tree) implémenté from scratch en Rust.

Le programme lit un fichier JSON de contacts, construit le trie en mémoire, puis génère un fichier PlantUML (format MindMap) permettant de visualiser la structure.

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) 1.85 ou plus récent (édition 2024)
- [Docker](https://docs.docker.com/desktop/) (optionnel, uniquement pour visualiser les graphes)

## Utilisation

Le programme prend un fichier JSON de contacts et écrit le diagramme dans `graph/`, avec le même nom que le fichier d'entrée :

```bash
cargo run --release -- data/04_common_parts.json
```

```text
Diagramme écrit dans graph/04_common_parts.puml
```

Le dossier `graph/` est créé s'il n'existe pas, et un fichier déjà présent est remplacé.

Si le fichier est absent, si le JSON est mal formé ou si un numéro contient autre chose que des chiffres (`+33`, espaces, lettres, numéro vide), le programme n'écrit rien. Il affiche l'erreur sur la sortie d'erreur et se termine avec le code 1. Un seul contact invalide suffit pour refuser tout le fichier :

```text
Numéro invalide pour Alice : "+33612345678" (attendu : chiffres de 0 à 9, non vide)
```

Pour régénérer les diagrammes de tous les jeux de données valides :

```bash
for f in data/*.json; do cargo run --release -q -- "$f"; done
```

## Lancer les tests

```bash
cargo test
```

## Développement

Avant chaque commit, vérifier que le code est formaté, sans warning et que les tests passent :

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

Chaque fonctionnalité est développée sur sa propre branche puis intégrée à `main` par pull request. La CI GitHub Actions (`.github/workflows/ci.yml`) relance ces vérifications sur chaque pull request et sur `main`.

## Structure du projet

```text
.
├── src/
│   ├── lib.rs        # arbre des modules et API publique (#![forbid(unsafe_code)])
│   ├── main.rs       # point d'entrée minimal, appelle la bibliothèque
│   ├── contact.rs    # lecture et validation du fichier JSON
│   ├── error.rs      # types d'erreurs
│   ├── trie.rs       # trie implémenté sans crate externe
│   └── plantuml.rs   # génération du MindMap PlantUML
├── data/             # fichiers JSON de test
├── graph/            # fichiers PlantUML (.puml) générés
└── .github/workflows/ci.yml
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
