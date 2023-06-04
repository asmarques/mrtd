use crate::document::*;
use crate::error::Error;
use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::str;
use std::str::FromStr;

lazy_static! {
    static ref VALID_PASSPORT_MRZ: Regex = Regex::new(r"^[A-Z0-9<]{88}$").unwrap();
    static ref VALID_IDENTITY_CARD_MRZ: Regex = Regex::new(r"^[A-Z0-9<]{90}$").unwrap();
}

const DATE_FORMAT: &str = "%y%m%d";

// Field specification from https://www.icao.int/publications/Documents/9303_p4_cons_en.pdf
// and
// Field specification from https://www.icao.int/publications/Documents/9303_p5_cons_en.pdf
pub(crate) fn parse(data: &str, check: bool) -> Result<Document, Error> {
    if VALID_PASSPORT_MRZ.is_match(data) {
        parse_passport(data, check)
    } else if VALID_IDENTITY_CARD_MRZ.is_match(data) {
        parse_identity_card(data, check)
    } else {
        Err(Error::InvalidFormat)
    }
}

fn char_to_num(full_str: &str, ind: usize) -> Result<u32, Error> {
    let c = full_str.chars().nth(ind).ok_or(Error::InvalidFormat)?;

    if c.is_ascii_digit() {
        Ok(c.to_digit(10).ok_or(Error::ExpectedDigit)?)
    } else {
        Err(Error::ExpectedDigit)
    }
}

// Check digit calculation from https://www.icao.int/publications/Documents/9303_p3_cons_en.pdf (section 4.9)
fn verify_check_digit(slice: &str, check_digit: u32) -> Result<(), Error> {
    let mut weighting_iter = [7, 3, 1].iter().cycle();

    let mut next = || weighting_iter.next().expect("cycle iter stopped");

    let char_weighting = |c: char| -> Result<u32, Error> {
        let num = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => 10,
            'B' => 11,
            'C' => 12,
            'D' => 13,
            'E' => 14,
            'F' => 15,
            'G' => 16,
            'H' => 17,
            'I' => 18,
            'J' => 19,
            'K' => 20,
            'L' => 21,
            'M' => 22,
            'N' => 23,
            'O' => 24,
            'P' => 25,
            'Q' => 26,
            'R' => 27,
            'S' => 28,
            'T' => 29,
            'U' => 30,
            'V' => 31,
            'W' => 32,
            'X' => 33,
            'Y' => 34,
            'Z' => 35,
            '<' => 0,
            _ => return Err(Error::InvalidChar),
        };

        Ok(num * next())
    };

    let sum: u32 = slice
        .chars()
        .map(char_weighting)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    let expected_check_digit = sum % 10;

    if check_digit == expected_check_digit {
        Ok(())
    } else {
        Err(Error::BadCheckDigit)
    }
}

fn parse_passport(data: &str, check: bool) -> Result<Document, Error> {
    let mrz = data.as_bytes();

    if mrz[0] != b'P' {
        return Err(Error::InvalidDocumentType);
    }

    let country = str::from_utf8(&mrz[2..5]).unwrap().replace('<', "");
    let mut names = str::from_utf8(&mrz[5..43])
        .unwrap()
        .split("<<")
        .collect::<Vec<_>>();

    names.reverse();

    let surnames = names
        .pop()
        .ok_or(Error::InvalidFormat)?
        .split('<')
        .filter(|name| !name.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();

    let given_names = names
        .pop()
        .ok_or(Error::InvalidFormat)?
        .split('<')
        .filter(|name| !name.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();

    let passport_number = str::from_utf8(&mrz[44..53]).unwrap().replace('<', "");
    if check {
        verify_check_digit(&data[44..53], char_to_num(data, 53)?)?;
    }

    let nationality = str::from_utf8(&mrz[54..57]).unwrap().replace('<', "");
    let mut birth_date =
        NaiveDate::parse_from_str(str::from_utf8(&mrz[57..63]).unwrap(), DATE_FORMAT)
            .map_err(|_| Error::InvalidBirthDate)?;

    let birth_year = birth_date.year();
    let current_year = Utc::now().year();

    if birth_year > current_year {
        birth_date = birth_date.with_year(birth_year - 100).unwrap();
    }

    if check {
        verify_check_digit(&data[57..63], char_to_num(data, 63)?)?;
    }

    let gender = match mrz[64] {
        b'M' => Gender::Male,
        b'F' => Gender::Female,
        _ => Gender::Other,
    };

    let expiry_date = NaiveDate::parse_from_str(str::from_utf8(&mrz[65..71]).unwrap(), DATE_FORMAT)
        .map_err(|_| Error::InvalidExpiryDate)?;

    if check {
        verify_check_digit(&data[65..71], char_to_num(data, 71)?)?;
        verify_check_digit(&data[72..86], char_to_num(data, 86)?)?;

        let comp_check_digit_str = format!("{}{}{}", &data[44..54], &data[57..64], &data[65..87]);
        verify_check_digit(&comp_check_digit_str, char_to_num(data, 87)?)?;
    }

    Ok(Document::Passport(Passport {
        country,
        surnames,
        given_names,
        passport_number,
        nationality,
        birth_date,
        gender,
        expiry_date,
    }))
}

fn parse_identity_card(data: &str, check: bool) -> Result<Document, Error> {
    let mrz = data.as_bytes();

    if (mrz[0] != b'I') && (mrz[0] != b'A') && (mrz[0] != b'C') {
        return Err(Error::InvalidDocumentType);
    }

    let country = str::from_utf8(&mrz[2..5]).unwrap().replace('<', "");

    let mut names = str::from_utf8(&mrz[60..])
        .unwrap()
        .split("<<")
        .collect::<Vec<_>>();

    names.reverse();

    let surnames = names
        .pop()
        .ok_or(Error::InvalidFormat)?
        .split('<')
        .filter(|name| !name.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();

    let given_names = names
        .pop()
        .ok_or(Error::InvalidFormat)?
        .split('<')
        .filter(|name| !name.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();

    let document_number = str::from_utf8(&mrz[5..14]).unwrap().replace('<', "");
    if check {
        verify_check_digit(&data[5..14], char_to_num(data, 14)?)?;
    }

    let nationality = str::from_utf8(&mrz[2..5]).unwrap().replace('<', "");
    let mut birth_date =
        NaiveDate::parse_from_str(str::from_utf8(&mrz[30..36]).unwrap(), DATE_FORMAT)
            .map_err(|_| Error::InvalidBirthDate)?;

    let birth_year = birth_date.year();
    let current_year = Utc::now().year();

    if birth_year > current_year {
        birth_date = birth_date.with_year(birth_year - 100).unwrap();
    }

    if check {
        verify_check_digit(&data[30..36], char_to_num(data, 36)?)?;
    }

    let gender = match mrz[37] {
        b'M' => Gender::Male,
        b'F' => Gender::Female,
        _ => Gender::Other,
    };

    let expiry_date = NaiveDate::parse_from_str(str::from_utf8(&mrz[38..44]).unwrap(), DATE_FORMAT)
        .map_err(|_| Error::InvalidExpiryDate)?;

    if check {
        verify_check_digit(&data[38..44], char_to_num(data, 44)?)?;

        let comp_check_digit_str = format!(
            "{}{}{}{}",
            &data[5..30],
            &data[30..37],
            &data[38..45],
            &data[48..59]
        );
        verify_check_digit(&comp_check_digit_str, char_to_num(data, 59)?)?;
    }

    Ok(Document::IdentityCard(IdentityCard {
        country,
        surnames,
        given_names,
        document_number,
        nationality,
        birth_date,
        gender,
        expiry_date,
    }))
}

impl FromStr for Document {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_passport_with_fillers() {
        let mrz = "P<CANMARTIN<<SARAH<<<<<<<<<<<<<<<<<<<<<<<<<<\
                   ZE000509<9CAN8501019F2301147<<<<<<<<<<<<<<08";
        match parse(mrz, true).unwrap() {
            Document::Passport(passport) => {
                assert_eq!(passport.country, "CAN");
                assert_eq!(passport.surnames, vec!["MARTIN"]);
                assert_eq!(passport.given_names, vec!["SARAH"]);
                assert_eq!(passport.passport_number, "ZE000509");
                assert_eq!(passport.nationality, "CAN");
            }
            _ => {}
        }
    }

    #[test]
    fn parse_passport() {
        let mrz = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
                   L898902C36UTO7408122F1204159ZE184226B<<<<<10";
        match parse(mrz, true).unwrap() {
            Document::Passport(passport) => {
                assert_eq!(passport.country, "UTO");
                assert_eq!(passport.surnames, vec!["ERIKSSON"]);
                assert_eq!(passport.given_names, vec!["ANNA", "MARIA"]);
                assert_eq!(passport.passport_number, "L898902C3");
                assert_eq!(passport.nationality, "UTO");
                assert_eq!(passport.birth_date.year(), 1974);
                assert_eq!(passport.birth_date.month(), 8);
                assert_eq!(passport.birth_date.day(), 12);
                assert_eq!(passport.gender, Gender::Female);
                assert_eq!(passport.expiry_date.year(), 2012);
                assert_eq!(passport.expiry_date.month(), 4);
                assert_eq!(passport.expiry_date.day(), 15);
            }
            _ => {}
        }
    }

    #[test]
    fn parse_passport_multiple_names() {
        let mrz = "P<UTOERIKSSON<JOHNSON<<ANNA<MARIA<<<<<<<<<<<\
                   L898902C36UTO7408122F1204159ZE184226B<<<<<10";
        match parse(mrz, true).unwrap() {
            Document::Passport(passport) => {
                assert_eq!(passport.country, "UTO");
                assert_eq!(passport.surnames, vec!["ERIKSSON", "JOHNSON"]);
                assert_eq!(passport.given_names, vec!["ANNA", "MARIA"]);
                assert_eq!(passport.passport_number, "L898902C3");
                assert_eq!(passport.nationality, "UTO");
                assert_eq!(passport.birth_date.year(), 1974);
                assert_eq!(passport.birth_date.month(), 8);
                assert_eq!(passport.birth_date.day(), 12);
                assert_eq!(passport.gender, Gender::Female);
                assert_eq!(passport.expiry_date.year(), 2012);
                assert_eq!(passport.expiry_date.month(), 4);
                assert_eq!(passport.expiry_date.day(), 15);
            }
            _ => {}
        }
    }

    #[test]
    fn parse_passport_invalid_length() {
        let mrz = "ABC<<";
        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidFormat);
    }

    #[test]
    fn parse_passport_invalid_encoding() {
        let mrz = "🕶️";
        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidFormat);
    }

    #[test]
    fn parse_passport_invalid_document_type() {
        let mrz = "X<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
                   L898902C36UTO7408122F1204159ZE184226B<<<<<10";
        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidDocumentType);
    }

    #[test]
    fn parse_passport_invalid_birth_date() {
        let mrz = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
                   L898902C36UTO7A08122F1204159ZE184226B<<<<<10";
        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidBirthDate);
    }

    #[test]
    fn parse_passport_invalid_expiry_date() {
        let mrz = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
                   L898902C36UTO7408122F1<0A159ZE184226B<<<<<10";
        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidExpiryDate);
    }

    #[test]
    fn parse_passport_invalid_check_digit() {
        let mrz = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
                   L898902C36UTO7408122F1204159ZE184226B<<<<<11";
        parse(mrz, false).unwrap();
        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::BadCheckDigit);
    }

    #[test]
    fn parse_from_str() {
        let valid_mrz = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
                   L898902C36UTO7408122F1204159ZE184226B<<<<<10";
        valid_mrz.parse::<Document>().unwrap();
        let invalid_mrz = "<";
        invalid_mrz.parse::<Document>().unwrap_err();
    }

    #[test]
    fn parse_identity_card() {
        let mrz = "C<ITACA00000AA4<<<<<<<<<<<<<<<\
        6412308F2212304ITA<<<<<<<<<<<0\
        ROSSI<<BIANCA<<<<<<<<<<<<<<<<<";

        match parse(mrz, true).unwrap() {
            Document::IdentityCard(identity_card) => {
                assert_eq!(identity_card.country, "ITA");
                assert_eq!(identity_card.surnames, vec!["ROSSI"]);
                assert_eq!(identity_card.given_names, vec!["BIANCA"]);
                assert_eq!(identity_card.document_number, "CA00000AA");
                assert_eq!(identity_card.nationality, "ITA");
                assert_eq!(identity_card.birth_date.year(), 1964);
                assert_eq!(identity_card.birth_date.month(), 12);
                assert_eq!(identity_card.birth_date.day(), 30);
                assert_eq!(identity_card.gender, Gender::Female);
                assert_eq!(identity_card.expiry_date.year(), 2022);
                assert_eq!(identity_card.expiry_date.month(), 12);
                assert_eq!(identity_card.expiry_date.day(), 30);
            }
            _ => {}
        }
    }

    #[test]
    fn parse_identity_card_multiple_names() {
        let mrz = "I<NLDSPECI20212<<<<<<<<<<<<<<<\
        6503101F3108022NLD<<<<<<<<<<<8\
        DE<BRUIJN<<WILLEKE<LISELOTTE<<";

        match parse(mrz, true).unwrap() {
            Document::IdentityCard(identity_card) => {
                assert_eq!(identity_card.country, "NLD");
                assert_eq!(identity_card.surnames, vec!["DE", "BRUIJN"]);
                assert_eq!(identity_card.given_names, vec!["WILLEKE", "LISELOTTE"]);
                assert_eq!(identity_card.document_number, "SPECI2021");
                assert_eq!(identity_card.nationality, "NLD");
                assert_eq!(identity_card.birth_date.year(), 1965);
                assert_eq!(identity_card.birth_date.month(), 3);
                assert_eq!(identity_card.birth_date.day(), 10);
                assert_eq!(identity_card.gender, Gender::Female);
                assert_eq!(identity_card.expiry_date.year(), 2031);
                assert_eq!(identity_card.expiry_date.month(), 8);
                assert_eq!(identity_card.expiry_date.day(), 2);
            }
            _ => {}
        }
    }

    #[test]
    fn parse_identity_card_invalid_length() {
        let mrz = "I<<<";
        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidFormat);
    }

    #[test]
    fn parse_identity_card_invalid_encoding() {
        let mrz = "👺";
        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidFormat);
    }

    #[test]
    fn parse_identity_card_invalid_document_type() {
        let mrz = "Z<NLDSPECI20212<<<<<<<<<<<<<<<\
        6503101F3108022NLD<<<<<<<<<<<8\
        DE<BRUIJN<<WILLEKE<LISELOTTE<<";

        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidDocumentType);
    }

    #[test]
    fn parse_identity_card_invalid_birth_date() {
        let mrz = "I<NLDSPECI20212<<<<<<<<<<<<<<<\
        6K03101F3108022NLD<<<<<<<<<<<8\
        DE<BRUIJN<<WILLEKE<LISELOTTE<<";

        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidBirthDate);
    }

    #[test]
    fn parse_identity_card_invalid_expiry_date() {
        let mrz = "I<NLDSPECI20212<<<<<<<<<<<<<<<\
        6503101F31080W2NLD<<<<<<<<<<<8\
        DE<BRUIJN<<WILLEKE<LISELOTTE<<";

        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::InvalidExpiryDate);
    }

    #[test]
    fn parse_identity_card_invalid_check_digit() {
        let mrz = "I<NLDSPECI20212<<<<<<<<<<<<<<<\
        6503101F3108022NLD<<<<<<<<<<<9\
        DE<BRUIJN<<WILLEKE<LISELOTTE<<";

        let error = parse(mrz, true).unwrap_err();
        assert_eq!(error, Error::BadCheckDigit);
    }
}
