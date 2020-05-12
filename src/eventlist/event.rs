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

use std::cmp::Ordering;
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

impl From<&EventDateType> for DateTime<Local> {
    fn from(origin: &EventDateType) -> Self {
        match origin {
            EventDateType::AllDay(d) => Local.ymd(d.year, d.month, d.day).and_hms(0, 0, 0),
            EventDateType::AtTime(d, t) => {
                Local.ymd(d.year, d.month, d.day).and_hms(t.hour, t.min, 0)
            }
        }
    }
}

impl From<&EventDateType> for String {
    fn from(origin: &EventDateType) -> String {
        match origin {
            EventDateType::AllDay(d) => format!("{}{}{}0000", d.year, d.month, d.day),
            EventDateType::AtTime(d, t) => {
                format!("{}{}{}{}{}", d.year, d.month, d.day, t.hour, t.min)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    id: String,
    description: String,
    due: EventDateType,
}

fn uuid() -> String {
    let (id, _, _, _) = Uuid::new_v4().as_fields();
    format!("{:x}", id)
}

impl Event {
    pub fn new_on_date(description: &str, date: &str) -> Self {
        let fake_datetime = format!("{} 00:00:00", date);
        if let Ok(dt) = Utc.datetime_from_str(&fake_datetime, "%Y-%m-%d %H:%M:%S") {
            Self {
                id: uuid(),
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

    pub fn new_on_date_time(description: &str, date: &str, time: &str) -> Self {
        let fake_datetime = format!("{} {}:00", date, time);
        if let Ok(dt) = Utc.datetime_from_str(&fake_datetime, "%Y-%m-%d %H:%M:%S") {
            Self {
                id: uuid(),
                description: description.into(),
                due: EventDateType::AtTime(
                    Date {
                        year: dt.year(),
                        month: dt.month(),
                        day: dt.day(),
                    },
                    Time {
                        hour: dt.hour(),
                        min: dt.minute(),
                    },
                ),
            }
        } else {
            panic!("Failed to parse the date");
        }
    }

    pub fn eta(&self) -> Option<String> {
        let now = Local::now();
        let to: DateTime<Local> = (&self.due).into();
        let eta = to - now;
        log::debug!("ETA for {}: {}", self.id, eta.num_minutes());

        match self.due {
            EventDateType::AllDay(_) if eta.num_minutes() > 0 => {
                Some(format!("{}d", eta.num_days()))
            }
            EventDateType::AtTime(_, _) if eta.num_minutes() > 0 => match eta.num_days() {
                0 => Some(format!("{}h", eta.num_hours())),
                _ => Some(format!("{}d {}h", eta.num_days(), eta.num_hours())),
            },
            _ => None,
        }
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

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        let self_str = String::from(&self.due);
        let other_str = String::from(&other.due);
        self_str == other_str
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_str = String::from(&self.due);
        let other_str = String::from(&other.due);

        Some(self_str.cmp(&other_str))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_str = String::from(&self.due);
        let other_str = String::from(&other.due);

        self_str.cmp(&other_str)
    }
}
