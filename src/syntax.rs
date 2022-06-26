use std::str::FromStr;

use chrono::{Datelike, Local, NaiveDate};

use crate::errors::Error;

pub fn parse_date(input: &str) -> Result<NaiveDate, Error> {
    let d = NaiveDate::from_str(input)
        .or_else(|_| NaiveDate::parse_from_str(input, "%d/%m/%Y"))
        .or_else(|_| {
            let year = Local::today().year().to_string();
            let mut timestr = String::from(input);
            timestr.push('/');
            timestr.push_str(&year);
            NaiveDate::parse_from_str(&timestr, "%d/%m/%Y")
        })
        .or_else(|_| {
            let year = Local::today().year().to_string();
            let month = Local::today().month().to_string();
            let mut timestr = String::from(input);
            timestr.push('/');
            timestr.push_str(&month);
            timestr.push('/');
            timestr.push_str(&year);
            NaiveDate::parse_from_str(&timestr, "%d/%m/%Y")
        })?;

    Ok(d)
}

#[test]
fn test_parse_date() {
    let date = parse_date("01/01/2018").unwrap();
    assert_eq!("2018-01-01", date.to_string());
    let date = parse_date("01/12").unwrap();
    assert_eq!("2022-12-01", date.to_string());
    let date = parse_date("01").unwrap();
    assert_eq!("2022-06-01", date.to_string());
}
