//! Erreurs renvoyées par la bibliothèque.

use std::fmt;
use std::io;

/// Les trois causes d'échec de la lecture des contacts.
#[derive(Debug)]
pub enum Error {
    /// Fichier impossible à lire ou à écrire.
    Io(io::Error),
    /// JSON mal formé ou champs incompatibles avec Contact.
    Json(serde_json::Error),
    /// Numéro vide ou contenant autre chose que des chiffres de 0 à 9.
    InvalidNumber {
        /// Numéro refusé.
        number: String,
        /// Nom du contact concerné.
        name: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(error) => write!(f, "Erreur de fichier : {error}"),
            Error::Json(error) => write!(f, "Erreur JSON : {error}"),
            Error::InvalidNumber { number, name } => {
                write!(
                    f,
                    "Numéro invalide pour {name} : {number:?} (attendu : chiffres de 0 à 9, non vide)"
                )
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // On garde l'erreur d'origine pour pouvoir retrouver la cause.
        match self {
            Error::Io(error) => Some(error),
            Error::Json(error) => Some(error),
            Error::InvalidNumber { .. } => None,
        }
    }
}

// Ces conversions permettent au ? de passer d'une erreur de fichier ou JSON à notre Error.
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn io_conversion_keeps_original_cause() {
        let error = Error::from(io::Error::new(io::ErrorKind::NotFound, "fichier absent"));
        assert!(matches!(error, Error::Io(_)));
        assert_eq!(error.source().unwrap().to_string(), "fichier absent");
        assert_eq!(error.to_string(), "Erreur de fichier : fichier absent");
    }

    #[test]
    fn json_conversion_keeps_original_cause() {
        let cause = serde_json::from_str::<Vec<String>>("[").unwrap_err();
        let message = cause.to_string();
        let error = Error::from(cause);
        assert!(matches!(error, Error::Json(_)));
        assert_eq!(error.source().unwrap().to_string(), message);
        assert_eq!(error.to_string(), format!("Erreur JSON : {message}"));
    }

    #[test]
    fn invalid_number_message_identifies_contact() {
        let error = Error::InvalidNumber {
            number: "06 12".to_string(),
            name: "Alice".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Numéro invalide pour Alice : \"06 12\" (attendu : chiffres de 0 à 9, non vide)"
        );
        assert!(error.source().is_none());
    }
}
