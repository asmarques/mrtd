use std::fmt;

/// Parsing error
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid or unsupported document type
    InvalidDocumentType,
    /// Invalid MRZ format
    InvalidFormat,
    /// Invalid format for date of birth
    InvalidBirthDate,
    /// Invalid format for date of expiry
    InvalidExpiryDate,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let message = match self {
            InvalidDocumentType => "invalid document type",
            InvalidFormat => "invalid MRZ format",
            InvalidBirthDate => "invalid date of birth",
            InvalidExpiryDate => "invalid date of expiry",
        };
        write!(f, "{}", message)
    }
}
