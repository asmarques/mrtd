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
    /// MRZ failed check digit verification
    BadCheckDigit,
    /// Expected digit at location but got something else
    ExpectedDigit,
    /// Encountered an invalid character (not [A-Z], [0-9] or <)
    InvalidChar,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let message = match self {
            InvalidDocumentType => "invalid document type",
            InvalidFormat => "invalid MRZ format",
            InvalidBirthDate => "invalid date of birth",
            InvalidExpiryDate => "invalid date of expiry",
            BadCheckDigit => "provided MRZ failed check digit verification",
            ExpectedDigit => "expected digit at location but found something else",
            InvalidChar => "encountered a invalid character",
        };
        write!(f, "{}", message)
    }
}
