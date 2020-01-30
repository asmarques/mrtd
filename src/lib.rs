//! A Rust parser for the machine-readable zone (MRZ) of machine-readable travel documents (MRTD)
//! as defined by ICAO Document 9303.

extern crate chrono;
extern crate failure;
extern crate failure_derive;
extern crate lazy_static;
extern crate regex;

use chrono::NaiveDate;
use failure_derive::Fail;
use lazy_static::lazy_static;
use regex::Regex;
use std::str;

/// Parsing error
#[derive(Debug, Fail, PartialEq, Eq)]
pub enum Error {
    #[fail(display = "invalid document type")]
    InvalidDocumentType,
    #[fail(display = "invalid MRZ")]
    InvalidFormat,
}

/// Travel document
#[derive(Debug)]
pub enum Document {
    /// Passport
    Passport(Passport),
}

/// Gender
#[derive(Debug, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    Undefined,
}

/// Passport
#[derive(Debug)]
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
    pub birth_date: chrono::NaiveDate,
    /// gender
    pub gender: Gender,
    /// expiry date
    pub expiry_date: chrono::NaiveDate,
}

lazy_static! {
    static ref VALID_MRZ: Regex = Regex::new(r"^[A-Z0-9<]{88}$").unwrap();
}

const DATE_FORMAT: &str = "%y%m%d";

/// Parse a Machine-readable Zone (MRZ) returning the corresponding travel document
pub fn parse(data: &str) -> Result<Document, Error> {
    if !VALID_MRZ.is_match(data) {
        return Err(Error::InvalidFormat);
    }

    let mrz = data.as_bytes();

    if mrz[0] != b'P' {
        return Err(Error::InvalidDocumentType);
    }

    let country = String::from(str::from_utf8(&mrz[2..5]).unwrap().replace("<", ""));
    let names = str::from_utf8(&mrz[5..43])
        .unwrap()
        .split("<")
        .collect::<Vec<_>>();
    let surname = String::from(*names.first().unwrap());
    let given_names = names[2..]
        .into_iter()
        .filter(|name| !name.is_empty())
        .map(|name| String::from(*name))
        .collect::<Vec<_>>();

    let passport_number = String::from(str::from_utf8(&mrz[44..53]).unwrap().replace("<", ""));
    let nationality = String::from(str::from_utf8(&mrz[54..57]).unwrap().replace("<", ""));
    let birth_date =
        NaiveDate::parse_from_str(str::from_utf8(&mrz[57..63]).unwrap(), DATE_FORMAT).unwrap();

    let gender = match mrz[64] {
        b'M' => Gender::Male,
        b'F' => Gender::Female,
        _ => Gender::Undefined,
    };

    let expiry_date =
        NaiveDate::parse_from_str(str::from_utf8(&mrz[65..71]).unwrap(), DATE_FORMAT).unwrap();

    return Ok(Document::Passport(Passport {
        country,
        surname,
        given_names,
        passport_number,
        nationality,
        birth_date,
        gender,
        expiry_date,
    }));
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use *;

    #[test]
    fn parse_passport() {
        let mrz = "P<UT<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
                   L898902C<6UT<7408122F1204159ZE184226B<<<<<10";
        match parse(mrz).unwrap() {
            Document::Passport(passport) => {
                assert_eq!(passport.country, "UT");
                assert_eq!(passport.surname, "ERIKSSON");
                assert_eq!(passport.given_names, vec!["ANNA", "MARIA"]);
                assert_eq!(passport.passport_number, "L898902C");
                assert_eq!(passport.nationality, "UT");
                assert_eq!(passport.birth_date.year(), 1974);
                assert_eq!(passport.birth_date.month(), 08);
                assert_eq!(passport.birth_date.day(), 12);
                assert_eq!(passport.gender, Gender::Female);
                assert_eq!(passport.expiry_date.year(), 2012);
                assert_eq!(passport.expiry_date.month(), 04);
                assert_eq!(passport.expiry_date.day(), 15);
            }
        }
    }

    #[test]
    fn parse_passport_invalid_length() {
        let mrz = "ABC<<";
        let error = parse(mrz).unwrap_err();
        assert_eq!(error, Error::InvalidFormat);
    }

    #[test]
    fn parse_passport_invalid_encoding() {
        let mrz = "🕶️";
        let error = parse(mrz).unwrap_err();
        assert_eq!(error, Error::InvalidFormat);
    }

    #[test]
    fn parse_passport_invalid_document_type() {
        let mrz = "X<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
                   L898902C36UTO7408122F1204159ZE184226B<<<<<10";
        let error = parse(mrz).unwrap_err();
        assert_eq!(error, Error::InvalidDocumentType);
    }
}
