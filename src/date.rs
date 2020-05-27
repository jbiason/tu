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

use std::convert::TryFrom;

use chrono::prelude::*;
use chrono::LocalResult;

#[derive(Debug, Eq, PartialEq)]
pub enum DateError {
    InvalidDate,
}

impl From<std::num::ParseIntError> for DateError {
    fn from(_: std::num::ParseIntError) -> DateError {
        DateError::InvalidDate
    }
}

#[derive(Debug)]
pub struct Date(chrono::Date<Local>);

impl Date {
    /// Returns Ok with the Date or Error in an invalid Date.
    pub fn new(year: u16, month: u8, day: u8) -> Result<Date, DateError> {
        match Local.ymd_opt(year as i32, month as u32, day as u32) {
            LocalResult::Single(x) => Ok(Date(x)),
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
}

impl TryFrom<&str> for Date {
    type Error = DateError;

    fn try_from(origin: &str) -> Result<Self, Self::Error> {
        let mut frags = origin.split("-");
        Date::new(
            frags.next().ok_or(DateError::InvalidDate)?.parse()?,
            frags.next().ok_or(DateError::InvalidDate)?.parse()?,
            frags.next().ok_or(DateError::InvalidDate)?.parse()?,
        )
    }
}

#[cfg(test)]
#[test]
pub fn invalid_date() {
    assert!(Date::new(2020, 127, 26).is_err());
    assert!(Date::new(2020, 5, 127).is_err());
    assert!(Date::new(2020, 0, 0).is_err());
}

#[cfg(test)]
#[test]
pub fn valid_date() {
    assert!(Date::new(2020, 5, 26).is_ok());
}

#[cfg(test)]
#[test]
pub fn from_string() {
    if let Ok(dt) = Date::try_from("2020-05-26") {
        assert_eq!(dt.year(), 2020);
        assert_eq!(dt.month(), 5);
        assert_eq!(dt.day(), 26);
    } else {
        panic!("Can't parse 2020-05-26")
    }
}
