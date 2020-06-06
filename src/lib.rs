//! A Rust parser for the machine-readable zone (MRZ) of machine-readable travel documents (MRTD)
//! as defined by ICAO Document 9303.

mod document;
mod error;
mod parser;

pub use document::*;
pub use error::Error;

/// Parse a Machine-readable Zone (MRZ) returning the corresponding travel document.
/// Performs error checking using the included check digits.
pub fn parse(data: &str) -> Result<Document, Error> {
    parser::parse(data, true)
}

/// Parse a Machine-readable Zone (MRZ) returning the corresponding travel document.
/// Does not perform error checking using the included check digits.
pub fn parse_without_checks(data: &str) -> Result<Document, Error> {
    parser::parse(data, false)
}
