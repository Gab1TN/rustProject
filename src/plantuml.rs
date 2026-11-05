//! Génération du diagramme PlantUML (format MindMap) qui représente le trie.

use crate::trie::{Node, Trie};

/// Construit le diagramme MindMap PlantUML qui représente `trie`.
///
/// ```
/// use phone_trie::{plantuml::to_mindmap, trie::Trie};
///
/// let trie: Trie = [("15", "SAMU")].into_iter().collect();
/// assert_eq!(to_mindmap(&trie), "@startmindmap\n* 1\n** 5\n*** SAMU\n@endmindmap\n");
/// ```
pub fn to_mindmap(trie: &Trie) -> String {
    let mut output = String::from("@startmindmap\n");
    write_children(trie.root(), 1, &mut output);
    output.push_str("@endmindmap\n");
    output
}

/// Le nom passe avant les chiffres suivants (cas `112` puis `1123`).
fn write_children(node: &Node, depth: usize, output: &mut String) {
    for (digit, child) in node.children() {
        write_line(depth, &digit.to_string(), output);
        if let Some(name) = child.name() {
            write_line(depth + 1, &capitalize(name), output);
        }
        write_children(child, depth + 1, output);
    }
}

fn write_line(depth: usize, text: &str, output: &mut String) {
    output.push_str(&"*".repeat(depth));
    output.push(' ');
    output.push_str(text);
    output.push('\n');
}

/// Par caractère et non par octet, pour ne pas couper une lettre accentuée.
fn capitalize(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMON_PARTS_PUML: &str = "\
@startmindmap
* 0
** 4
*** 1
**** 2
***** 5
****** 7
******* 8
******** 4
********* 4
********** 0
*********** Alice
***** 1
****** 9
******* 9
******** 8
********* 0
********** 3
*********** Bob
*** 6
**** 8
***** 8
****** 9
******* 2
******** 0
********* 1
********** 1
*********** Patate
* 1
** 1
*** 2
**** Urgences
** 5
*** SAMU
@endmindmap
";

    #[test]
    fn subject_example_matches_expected_output() {
        let trie: Trie = [
            ("0412578440", "Alice"),
            ("0412199803", "Bob"),
            ("0468892011", "patate"),
            ("112", "Urgences"),
            ("15", "SAMU"),
        ]
        .into_iter()
        .collect();
        assert_eq!(to_mindmap(&trie), COMMON_PARTS_PUML);
    }

    #[test]
    fn name_comes_before_longer_numbers() {
        let trie: Trie = [("112", "Urgences"), ("1123", "Standard")]
            .into_iter()
            .collect();
        let expected = "\
@startmindmap
* 1
** 1
*** 2
**** Urgences
**** 3
***** Standard
@endmindmap
";
        assert_eq!(to_mindmap(&trie), expected);
    }

    #[test]
    fn empty_trie_has_only_delimiters() {
        assert_eq!(to_mindmap(&Trie::new()), "@startmindmap\n@endmindmap\n");
    }

    #[test]
    fn capitalize_handles_accents_and_empty_names() {
        assert_eq!(capitalize("patate"), "Patate");
        assert_eq!(capitalize("élodie"), "Élodie");
        assert_eq!(capitalize("SAMU"), "SAMU");
        assert_eq!(capitalize(""), "");
    }
}
