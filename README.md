# phone_trie

Projet Rust : on stocke des numéros de téléphone dans un trie (arbre de préfixes) codé à la main. Le programme lit un fichier JSON de contacts, construit le trie et génère un fichier PlantUML pour visualiser l'arbre.

Rust 1.85 minimum (édition 2024).

## Commandes

Générer le graphe d'un fichier (sortie dans `graph/`) :

```bash
cargo run -- data/04_common_parts.json
```

Lancer les tests :

```bash
cargo test
```

Formatage et lint :

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
```

Pour voir un graphe, lancer un serveur PlantUML et coller le contenu d'un fichier `.puml` sur http://localhost:8080 :

```bash
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```

## Équipe

- Gabin
- Maxime
- Erwin
