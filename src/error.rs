use std::fmt;

/// Parsing error
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidDocumentType,
    InvalidFormat,
    InvalidBirthDate,
    InvalidExpiryDate,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let message = match self {
            InvalidDocumentType => "invalid document type",
            InvalidFormat => "invalid MRZ",
            InvalidBirthDate => "invalid birth date",
            InvalidExpiryDate => "invalid expiry date",
        };
        write!(f, "{}", message)
    }
}
