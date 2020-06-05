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

use chrono::prelude::*;
use chrono::LocalResult;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::date_errors::DateError;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct DateTime(chrono::DateTime<Local>);

impl DateTime {
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8) -> Result<Self, DateError> {
        match Local.ymd_opt(year as i32, month as u32, day as u32) {
            LocalResult::None => Err(DateError::InvalidDate),
            LocalResult::Ambiguous(_, _) => Err(DateError::InvalidDate),
            LocalResult::Single(x) => match x.and_hms_opt(hour as u32, minute as u32, 59) {
                Some(x) => Ok(DateTime(x)),
                None => Err(DateError::InvalidDate),
            },
        }
    }

    pub fn year(&self) -> u16 {
        self.0.date().year() as u16
    }

    pub fn month(&self) -> u8 {
        self.0.date().month() as u8
    }

    pub fn day(&self) -> u8 {
        self.0.date().day() as u8
    }

    pub fn hour(&self) -> u8 {
        self.0.time().hour() as u8
    }

    pub fn minute(&self) -> u8 {
        self.0.time().minute() as u8
    }

    /// Try to convert a string to a Date.
    pub fn try_from(date: &str, time: &str) -> Result<Self, DateError> {
        let mut date_frags = date.split("-");
        let mut time_frags = time.split(":");

        DateTime::new(
            date_frags.next().ok_or(DateError::InvalidDate)?.parse()?,
            date_frags.next().ok_or(DateError::InvalidDate)?.parse()?,
            date_frags.next().ok_or(DateError::InvalidDate)?.parse()?,
            time_frags.next().ok_or(DateError::InvalidDate)?.parse()?,
            time_frags.next().ok_or(DateError::InvalidDate)?.parse()?,
        )
    }

    pub fn eta(&self) -> Option<(u16, u16)> {
        let diff = self.0 - Local::now();
        let days = diff.num_days();
        let hours = diff.num_hours() - (24 * diff.num_days());

        if hours >= 0 {
            Some((days as u16, hours as u16))
        } else {
            None
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }
}

#[cfg(test)]
mod datetime_test {
    use chrono::prelude::*;
    use chrono::Duration;

    #[test]
    pub fn invalid_date_time() {
        assert!(super::DateTime::new(2020, 127, 2, 0, 0).is_err());
        assert!(super::DateTime::new(2020, 6, 127, 0, 0).is_err());
        assert!(super::DateTime::new(2020, 0, 0, 0, 0).is_err());
        assert!(super::DateTime::new(2020, 6, 2, 24, 0).is_err());
        assert!(super::DateTime::new(2020, 6, 2, 0, 60).is_err());
    }

    #[test]
    pub fn valid_date_time() {
        assert!(super::DateTime::new(2020, 6, 2, 20, 17).is_ok());
    }

    #[test]
    pub fn from_string() {
        if let Ok(x) = super::DateTime::try_from("2020-06-02", "20:18") {
            assert_eq!(x.year(), 2020);
            assert_eq!(x.month(), 6);
            assert_eq!(x.day(), 2);
            assert_eq!(x.hour(), 20);
            assert_eq!(x.minute(), 18);
        } else {
            panic!("Can't parse 2020-06-02, 20:18");
        }
    }

    #[test]
    pub fn string_leading_zeroes() {
        if let Ok(x) = super::DateTime::try_from("2020-09-09", "09:09") {
            assert_eq!(x.year(), 2020);
            assert_eq!(x.month(), 9);
            assert_eq!(x.day(), 9);
            assert_eq!(x.hour(), 9);
            assert_eq!(x.minute(), 9);
        } else {
            panic!("Can't parse 2020-09-09, 09:09");
        }
    }

    #[test]
    pub fn eta_two_hours() {
        let future = Local::now() + Duration::hours(2);
        let datetime = super::DateTime::new(
            future.year() as u16,
            future.month() as u8,
            future.day() as u8,
            future.hour() as u8,
            future.minute() as u8,
        );

        assert!(datetime.is_ok());
        assert_eq!(datetime.unwrap().eta(), Some((0, 2)));
    }
}
