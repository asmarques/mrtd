use chrono::NaiveDate;

/// Travel document
#[derive(Debug, PartialEq, Clone)]
pub enum Document {
    /// Passport
    Passport(Passport),
}

/// Gender
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Gender {
    Male,
    Female,
    Undefined,
}

/// Passport
#[derive(Debug, PartialEq, Clone)]
pub struct Passport {
    /// country (ISO 3166-1 code)
    pub country: String,
    /// surname
    pub surname: String,
    /// given names,
    pub given_names: Vec<String>,
    /// passport number
    pub passport_number: String,
    /// nationality (ISO 3166-1 code)
    pub nationality: String,
    /// birth date
    pub birth_date: NaiveDate,
    /// gender
    pub gender: Gender,
    /// expiry date
    pub expiry_date: NaiveDate,
}
