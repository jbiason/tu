/*
   TU - Time's Up!
   Copyright (C) 2020  Julio Biason

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Affero General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#![deny(missing_docs)]

// TODO trait TryFrom

use chrono::prelude::*;
use chrono::LocalResult;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::date_errors::DateError;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Date(chrono::NaiveDate);

impl Date {
    /// Returns Ok with the Date or Error in an invalid Date.
    pub fn new(year: u16, month: u8, day: u8) -> Result<Date, DateError> {
        match Local.ymd_opt(year as i32, month as u32, day as u32) {
            LocalResult::Single(x) => Ok(Date(x.naive_local())),
            LocalResult::None => Err(DateError::InvalidDate),
            LocalResult::Ambiguous(_, _) => Err(DateError::InvalidDate),
        }
    }

    pub fn year(&self) -> u16 {
        self.0.year() as u16
    }

    pub fn month(&self) -> u8 {
        self.0.month() as u8
    }

    pub fn day(&self) -> u8 {
        self.0.day() as u8
    }

    /// Try to convert a string to a Date.
    pub fn try_from(value: &str) -> Result<Self, DateError> {
        let mut frags = value.split("-");
        Date::new(
            frags.next().ok_or(DateError::InvalidDate)?.parse()?,
            frags.next().ok_or(DateError::InvalidDate)?.parse()?,
            frags.next().ok_or(DateError::InvalidDate)?.parse()?,
        )
    }

    /// Number of days till the date; None if the date is in the past.
    pub fn eta(&self) -> Option<u16> {
        let days = (self.0 - Local::today().naive_local()).num_days();
        if days >= 0 {
            Some(days as u16)
        } else {
            None
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.0.and_hms(23, 59, 59).timestamp()
    }
}

#[cfg(test)]
mod date_test {
    use chrono::prelude::*;
    use chrono::Duration;

    #[test]
    pub fn invalid_date() {
        assert!(super::Date::new(2020, 127, 26).is_err());
        assert!(super::Date::new(2020, 5, 127).is_err());
        assert!(super::Date::new(2020, 0, 0).is_err());
    }

    #[test]
    pub fn valid_date() {
        assert!(super::Date::new(2025, 5, 26).is_ok());
    }

    #[test]
    pub fn from_string() {
        if let Ok(dt) = super::Date::try_from("2025-05-26") {
            assert_eq!(dt.year(), 2025);
            assert_eq!(dt.month(), 5);
            assert_eq!(dt.day(), 26);
        } else {
            panic!("Can't parse 2025-05-26")
        }
    }

    #[test]
    pub fn failed_from_string() {
        assert!(super::Date::try_from("2020-127-26").is_err());
    }

    #[test]
    pub fn eta_tomorrow() {
        let future = Local::today() + Duration::days(1);
        let date = super::Date::new(
            future.year() as u16,
            future.month() as u8,
            future.day() as u8,
        )
        .unwrap();
        assert_eq!(date.eta(), Some(1));
    }

    #[test]
    pub fn eta_today() {
        let future = Local::today();
        let date = super::Date::new(
            future.year() as u16,
            future.month() as u8,
            future.day() as u8,
        )
        .unwrap();
        assert_eq!(date.eta(), Some(0));
    }

    #[test]
    pub fn eta_yesterday() {
        let future = Local::today() - Duration::days(1);
        let date = super::Date::new(
            future.year() as u16,
            future.month() as u8,
            future.day() as u8,
        )
        .unwrap();
        assert_eq!(date.eta(), None);
    }
}
