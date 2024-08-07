use chrono::NaiveDate;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Travel document
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case", tag = "type")
)]
pub enum Document {
    /// Passport
    Passport(Passport),
    /// Identity Card
    IdentityCard(IdentityCard),
}

/// Gender
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Gender {
    /// Male
    Male,
    /// Female
    Female,
    /// Other/unspecified
    Other,
}

/// Passport
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Passport {
    /// Country (ISO 3166-1 code)
    pub country: String,
    /// Surname
    pub surnames: Vec<String>,
    /// Given names
    pub given_names: Vec<String>,
    /// Passport number
    pub passport_number: String,
    /// Nationality (ISO 3166-1 code)
    pub nationality: String,
    /// Date of birth
    pub birth_date: NaiveDate,
    /// Gender
    pub gender: Gender,
    /// Date of expiry
    pub expiry_date: NaiveDate,
}

/// Identity Card
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IdentityCard {
    /// Country (ISO 3166-1 code)
    pub country: String,
    /// Surname
    pub surnames: Vec<String>,
    /// Given names
    pub given_names: Vec<String>,
    /// Document number
    pub document_number: String,
    /// Nationality (ISO 3166-1 code)
    pub nationality: String,
    /// Date of birth
    pub birth_date: NaiveDate,
    /// Gender
    pub gender: Gender,
    /// Date of expiry
    pub expiry_date: NaiveDate,
}
