//! Lecture et validation du fichier JSON de contacts.

use crate::error::Error;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Contact tel qu'il est écrit dans le JSON.
#[derive(Deserialize, Debug)]
pub struct Contact {
    // Une chaîne garde les zéros au début du numéro.
    nb: String,
    name: String,
}

impl Contact {
    /// Numéro du contact.
    pub fn number(&self) -> &str {
        &self.nb
    }

    /// Nom du contact, sans changer sa casse.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Lit les contacts et refuse le fichier si un numéro est vide ou contient autre chose que 0 à 9.
pub fn read_contacts(path: impl AsRef<Path>) -> Result<Vec<Contact>, Error> {
    // Le ? renvoie l'erreur à l'appelant si la lecture ou le JSON échoue.
    let text = fs::read_to_string(path)?;
    let contacts: Vec<Contact> = serde_json::from_str(&text)?;

    for contact in &contacts {
        if contact.nb.is_empty() || !contact.nb.chars().all(|digit| digit.is_ascii_digit()) {
            return Err(Error::InvalidNumber {
                number: contact.nb.clone(),
                name: contact.name.clone(),
            });
        }
    }

    // On ne renvoie la liste qu'une fois tous les numéros vérifiés.
    Ok(contacts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trie::Trie;

    fn read_data(file: &str) -> Result<Vec<Contact>, Error> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("data")
            .join(file);
        read_contacts(path)
    }

    #[test]
    fn empty_list_is_valid() {
        assert!(read_data("01_empty.json").unwrap().is_empty());
    }

    #[test]
    fn short_numbers_are_valid() {
        let contacts = read_data("02_short_numbers.json").unwrap();
        let numbers: Vec<&str> = contacts.iter().map(|contact| contact.number()).collect();
        assert_eq!(numbers, ["0", "112", "15"]);
    }

    #[test]
    fn subject_example_keeps_order_zeros_and_names() {
        let contacts = read_data("04_common_parts.json").unwrap();
        let pairs: Vec<(&str, &str)> = contacts
            .iter()
            .map(|contact| (contact.number(), contact.name()))
            .collect();
        assert_eq!(
            pairs,
            [
                ("0412578440", "Alice"),
                ("0412199803", "Bob"),
                ("0468892011", "patate"),
                ("112", "Urgences"),
                ("15", "SAMU"),
            ]
        );
    }

    #[test]
    fn prefix_numbers_can_be_loaded_into_trie() {
        let contacts = read_data("03_prefix.json").unwrap();
        let trie: Trie = contacts.iter().map(|c| (c.number(), c.name())).collect();
        assert_eq!(trie.get("112"), Some("Urgences"));
        assert_eq!(trie.get("1123"), Some("Standard"));
    }

    #[test]
    fn duplicates_are_kept_for_trie_to_handle() {
        let contacts = read_data("05_duplicate.json").unwrap();
        assert_eq!(contacts.len(), 2);
        let trie: Trie = contacts.iter().map(|c| (c.number(), c.name())).collect();
        assert_eq!(trie.len(), 1);
        assert_eq!(trie.get("15"), Some("Urgences médicales"));
    }

    #[test]
    fn missing_file_returns_io_error() {
        assert!(matches!(
            read_data("does_not_exist.json"),
            Err(Error::Io(_))
        ));
    }

    #[test]
    fn malformed_json_returns_json_error() {
        assert!(matches!(
            read_data("invalid/malformed.json"),
            Err(Error::Json(_))
        ));
    }

    #[test]
    fn missing_name_returns_json_error() {
        assert!(matches!(
            read_data("invalid/missing_name.json"),
            Err(Error::Json(_))
        ));
    }

    #[test]
    fn missing_number_returns_json_error() {
        assert!(matches!(
            read_data("invalid/missing_number.json"),
            Err(Error::Json(_))
        ));
    }

    #[test]
    fn numeric_number_returns_json_error() {
        assert!(matches!(
            read_data("invalid/numeric_number.json"),
            Err(Error::Json(_))
        ));
    }

    #[test]
    fn object_instead_of_list_returns_json_error() {
        assert!(matches!(
            read_data("invalid/object.json"),
            Err(Error::Json(_))
        ));
    }

    #[test]
    fn letters_return_invalid_number() {
        assert!(matches!(
            read_data("invalid/letters.json"),
            Err(Error::InvalidNumber { .. })
        ));
    }

    #[test]
    fn international_prefix_returns_invalid_number() {
        assert!(matches!(
            read_data("invalid/international.json"),
            Err(Error::InvalidNumber { .. })
        ));
    }

    #[test]
    fn empty_number_returns_invalid_number() {
        assert!(matches!(
            read_data("invalid/empty_number.json"),
            Err(Error::InvalidNumber { .. })
        ));
    }

    #[test]
    fn spaces_return_invalid_number() {
        assert!(matches!(
            read_data("invalid/spaces.json"),
            Err(Error::InvalidNumber { .. })
        ));
    }

    #[test]
    fn unicode_digits_return_invalid_number() {
        assert!(matches!(
            read_data("invalid/unicode_digits.json"),
            Err(Error::InvalidNumber { .. })
        ));
    }

    #[test]
    fn one_invalid_contact_rejects_whole_file() {
        assert!(matches!(
            read_data("invalid/mixed.json"),
            Err(Error::InvalidNumber { number, name }) if number == "abc" && name == "Alice"
        ));
    }
}
