use std::fmt;

/// Parsing error
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidDocumentType,
    InvalidFormat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let message = match self {
            InvalidDocumentType => "invalid document type",
            InvalidFormat => "invalid MRZ",
        };
        write!(f, "{}", message)
    }
}
