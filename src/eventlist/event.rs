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

use std::convert::From;
use std::fmt;

use chrono::prelude::*;
use chrono::DateTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    hour: u32,
    min: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "due", content = "datetime")]
pub enum EventDateType {
    AllDay(Date),
    AtTime(Date, Time),
}

impl From<&EventDateType> for DateTime<Utc> {
    fn from(origin: &EventDateType) -> Self {
        match origin {
            EventDateType::AllDay(d) => Utc.ymd(d.year, d.month, d.day).and_hms(0, 0, 0),
            _ => Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    id: String,
    description: String,
    due: EventDateType,
}

impl Event {
    pub fn new_on_date(description: &str, date: &str) -> Self {
        let fake_datetime = format!("{} 00:00:00", date);
        if let Ok(dt) = Utc.datetime_from_str(&fake_datetime, "%Y-%m-%d %H:%M:%S") {
            let (id, _, _, _) = Uuid::new_v4().as_fields();
            Self {
                id: format!("{:x}", id),
                description: description.into(),
                due: EventDateType::AllDay(Date {
                    year: dt.year(),
                    month: dt.month(),
                    day: dt.day(),
                }),
            }
        } else {
            panic!("Failed to parse the date");
        }
    }

    pub fn eta(&self) -> Option<String> {
        let now = Utc::now();
        let to: DateTime<Utc> = (&self.due).into();
        let eta = to - now;
        Some(format!("{}d", eta.num_days()))
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:8} - {:>6} - {}",
            self.id,
            match self.eta() {
                Some(x) => x,
                None => "".into(),
            },
            self.description
        )
    }
}
