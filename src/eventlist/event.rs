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

use chrono::prelude::*;
use chrono::DateTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use uuid::Uuid;

static DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl From<&DateTime<Local>> for Date {
    fn from(origin: &DateTime<Local>) -> Date {
        Date {
            year: origin.year(),
            month: origin.month(),
            day: origin.day(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    hour: u32,
    min: u32,
}

impl From<&DateTime<Local>> for Time {
    fn from(origin: &DateTime<Local>) -> Time {
        Time {
            hour: origin.hour(),
            min: origin.minute(),
        }
    }
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
    pub id: String,
    pub description: String,
    due: EventDateType,
}

fn uuid() -> String {
    let (id, _, _, _) = Uuid::new_v4().as_fields();
    format!("{:x}", id)
}

impl Event {
    // TODO result this
    pub fn new_on_date(description: &str, date: &str) -> Self {
        let fake_datetime = format!("{} 00:00:00", date);
        if let Ok(dt) = Local.datetime_from_str(&fake_datetime, DATE_FORMAT) {
            // TODO turn format into static
            Self {
                id: uuid(),
                description: description.into(),
                due: EventDateType::AllDay(Date::from(&dt)),
            }
        } else {
            panic!("Failed to parse the date");
        }
    }

    pub fn new_on_date_time(description: &str, date: &str, time: &str) -> Self {
        let fake_datetime = format!("{} {}:00", date, time);
        if let Ok(dt) = Local.datetime_from_str(&fake_datetime, DATE_FORMAT) {
            Self {
                id: uuid(),
                description: description.into(),
                due: EventDateType::AtTime(Date::from(&dt), Time::from(&dt)),
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
            EventDateType::AtTime(_, _) if eta.num_days() > 0 => {
                Some(format!("{}d {}h", eta.num_days(), eta.num_hours()))
            }
            EventDateType::AtTime(_, _) if eta.num_hours() > 0 => {
                Some(format!("{}h", eta.num_hours()))
            }
            _ => None,
        }
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
