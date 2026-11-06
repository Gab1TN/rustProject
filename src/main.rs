use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use phone_trie::contact::read_contacts;
use phone_trie::error::Error;
use phone_trie::plantuml::to_mindmap;
use phone_trie::trie::Trie;

const GRAPH_DIR: &str = "graph";

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let [input] = args.as_slice() else {
        eprintln!("Usage : cargo run --release -- <fichier.json>");
        return ExitCode::FAILURE;
    };

    match run(Path::new(input)) {
        Ok(output) => {
            println!("Diagramme écrit dans {}", output.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

/// Lit `input`, construit le trie et écrit `graph/<nom>.puml`.
fn run(input: &Path) -> Result<PathBuf, Error> {
    let contacts = read_contacts(input)?; // Maxime
    let mut trie = Trie::new();
    for contact in &contacts {
        trie.insert(contact.number(), contact.name());
    }

    let stem = input
        .file_stem()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "nom de fichier invalide"))?;
    let output = Path::new(GRAPH_DIR).join(stem).with_extension("puml");

    fs::create_dir_all(GRAPH_DIR)?;
    fs::write(&output, to_mindmap(&trie))?; // Erwin
    Ok(output)
}
