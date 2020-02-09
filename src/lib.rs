//! A Rust parser for the machine-readable zone (MRZ) of machine-readable travel documents (MRTD)
//! as defined by ICAO Document 9303.

extern crate chrono;
extern crate lazy_static;
extern crate regex;

mod document;
mod error;
mod parser;

pub use document::*;
pub use error::Error;
pub use parser::parse;
