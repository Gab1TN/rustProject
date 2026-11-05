//! Trie (prefix tree) de numéros de téléphone, implémenté sans crate externe.
//!
//! Chaque nœud représente un chiffre : le chemin entre la racine et un nœud
//! forme un préfixe, et les numéros qui commencent pareil partagent les mêmes
//! nœuds. Le nom d'un contact est posé sur le nœud de son dernier chiffre.

/// Nœud du trie : ses enfants et, si un numéro se termine ici, un nom.
#[derive(Debug, Default)]
pub struct Node {
    /// Enfants dans l'ordre d'insertion, sous la forme `(chiffre, nœud)`.
    children: Vec<(char, Node)>,
    /// Nom du contact dont le numéro se termine sur ce nœud.
    name: Option<String>,
}

impl Node {
    /// Enfants du nœud dans l'ordre d'insertion, sous la forme `(chiffre, nœud)`.
    pub fn children(&self) -> impl Iterator<Item = (char, &Node)> {
        self.children.iter().map(|(digit, node)| (*digit, node))
    }

    /// Nom du contact dont le numéro se termine sur ce nœud, s'il y en a un.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Enfant qui correspond à `digit`, s'il existe.
    fn child(&self, digit: char) -> Option<&Node> {
        self.children
            .iter()
            .find(|(child_digit, _)| *child_digit == digit)
            .map(|(_, node)| node)
    }

    /// Enfant qui correspond à `digit`, ajouté à la fin des enfants s'il n'existe pas encore.
    fn child_or_insert(&mut self, digit: char) -> &mut Node {
        let index = match self
            .children
            .iter()
            .position(|(child_digit, _)| *child_digit == digit)
        {
            Some(index) => index,
            None => {
                self.children.push((digit, Node::default()));
                self.children.len() - 1
            }
        };
        &mut self.children[index].1
    }
}

/// Trie qui associe des numéros de téléphone à des noms.
///
/// # Exemple
///
/// ```
/// use phone_trie::trie::Trie;
///
/// let mut trie = Trie::new();
/// trie.insert("112", "Urgences");
/// trie.insert("15", "SAMU");
///
/// assert_eq!(trie.get("112"), Some("Urgences"));
/// assert_eq!(trie.get("11"), None);
/// assert_eq!(trie.len(), 2);
/// ```
#[derive(Debug, Default)]
pub struct Trie {
    /// Racine : elle ne représente aucun chiffre.
    root: Node,
    /// Nombre de numéros enregistrés.
    len: usize,
}

impl Trie {
    /// Crée un trie vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute `number` avec le nom `name`.
    ///
    /// Si le numéro existait déjà, le nouveau nom remplace l'ancien, qui est
    /// renvoyé, et le nombre de numéros ne change pas.
    ///
    /// Le trie ne valide pas le numéro : la vérification (chiffres uniquement,
    /// numéro non vide) est faite à la lecture du fichier JSON.
    pub fn insert(&mut self, number: &str, name: &str) -> Option<String> {
        let mut node = &mut self.root;
        for digit in number.chars() {
            node = node.child_or_insert(digit);
        }
        let previous = node.name.replace(name.to_string());
        if previous.is_none() {
            self.len += 1;
        }
        previous
    }

    /// Nom associé exactement à `number`, ou `None` si ce numéro n'a pas été ajouté.
    pub fn get(&self, number: &str) -> Option<&str> {
        let mut node = &self.root;
        for digit in number.chars() {
            node = node.child(digit)?;
        }
        node.name()
    }

    /// Nombre de numéros enregistrés.
    pub fn len(&self) -> usize {
        self.len
    }

    /// `true` si aucun numéro n'a été ajouté.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Racine du trie. Elle ne représente aucun chiffre : ses enfants sont les
    /// premiers chiffres des numéros.
    pub fn root(&self) -> &Node {
        &self.root
    }
}

/// Construit un trie à partir de paires `(numéro, nom)`.
///
/// ```
/// use phone_trie::trie::Trie;
///
/// let trie: Trie = [("112", "Urgences"), ("15", "SAMU")].into_iter().collect();
/// assert_eq!(trie.get("15"), Some("SAMU"));
/// ```
impl<'a> FromIterator<(&'a str, &'a str)> for Trie {
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(pairs: I) -> Self {
        let mut trie = Trie::new();
        for (number, name) in pairs {
            trie.insert(number, name);
        }
        trie
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Chiffres des enfants d'un nœud, dans l'ordre.
    fn digits(node: &Node) -> Vec<char> {
        node.children().map(|(digit, _)| digit).collect()
    }

    /// Nœud atteint en suivant `prefix` depuis la racine.
    fn node_at<'a>(trie: &'a Trie, prefix: &str) -> &'a Node {
        let mut node = trie.root();
        for digit in prefix.chars() {
            node = node.child(digit).expect("le préfixe doit exister");
        }
        node
    }

    /// Contacts de l'exemple du sujet (`data/04_common_parts.json`).
    fn subject_example() -> Trie {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice");
        trie.insert("0412199803", "Bob");
        trie.insert("0468892011", "patate");
        trie.insert("112", "Urgences");
        trie.insert("15", "SAMU");
        trie
    }

    #[test]
    fn new_trie_is_empty() {
        let trie = Trie::new();
        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn insert_new_number_returns_none_and_counts_it() {
        let mut trie = Trie::new();
        assert_eq!(trie.insert("112", "Urgences"), None);
        assert_eq!(trie.len(), 1);
    }

    #[test]
    fn insert_counts_numbers_sharing_a_prefix_separately() {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice");
        trie.insert("0412199803", "Bob");
        assert_eq!(trie.len(), 2);
    }

    #[test]
    fn insert_duplicate_number_returns_previous_name() {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice");
        assert_eq!(trie.insert("0412578440", "Bob"), Some("Alice".to_string()));
        assert_eq!(trie.len(), 1);
    }

    #[test]
    fn get_returns_name_of_inserted_number() {
        let trie = subject_example();
        assert_eq!(trie.get("0412199803"), Some("Bob"));
    }

    #[test]
    fn get_unknown_number_returns_none() {
        let trie = subject_example();
        assert_eq!(trie.get("0699999999"), None);
    }

    #[test]
    fn get_prefix_without_name_returns_none() {
        let trie = subject_example();
        assert_eq!(trie.get("0412"), None);
    }

    #[test]
    fn get_number_longer_than_stored_returns_none() {
        let trie = subject_example();
        assert_eq!(trie.get("1123"), None);
    }

    #[test]
    fn number_can_be_prefix_of_another() {
        let mut trie = Trie::new();
        trie.insert("112", "Urgences");
        trie.insert("1123", "Standard");
        assert_eq!(trie.get("112"), Some("Urgences"));
        assert_eq!(trie.get("1123"), Some("Standard"));
    }

    #[test]
    fn duplicate_number_keeps_last_name() {
        let mut trie = Trie::new();
        trie.insert("15", "SAMU");
        trie.insert("15", "Urgences médicales");
        assert_eq!(trie.get("15"), Some("Urgences médicales"));
    }

    #[test]
    fn shared_prefix_is_stored_once() {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice");
        trie.insert("0412199803", "Bob");
        assert_eq!(digits(trie.root()), ['0']);
        assert_eq!(digits(node_at(&trie, "041")), ['2']);
    }

    #[test]
    fn children_keep_insertion_order() {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice");
        trie.insert("0412199803", "Bob");
        assert_eq!(digits(node_at(&trie, "0412")), ['5', '1']);
    }

    #[test]
    fn distinct_first_digits_are_children_of_root() {
        let trie = subject_example();
        assert_eq!(digits(trie.root()), ['0', '1']);
    }

    #[test]
    fn name_is_only_on_last_digit() {
        let trie = subject_example();
        assert_eq!(node_at(&trie, "11").name(), None);
        assert_eq!(node_at(&trie, "112").name(), Some("Urgences"));
    }

    #[test]
    fn subject_example_has_expected_branches() {
        let trie = subject_example();
        assert_eq!(digits(node_at(&trie, "04")), ['1', '6']);
        assert_eq!(digits(node_at(&trie, "1")), ['1', '5']);
    }

    #[test]
    fn collect_builds_trie_from_pairs() {
        let pairs = [("112", "Urgences"), ("15", "SAMU"), ("112", "Pompiers")];
        let trie: Trie = pairs.into_iter().collect();
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.get("112"), Some("Pompiers"));
    }
}
