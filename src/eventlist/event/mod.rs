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

mod date;
mod eventtype;
mod time;

use date::Date as EventDate;
use eventtype::EventType;
use time::Time as EventTime;

static DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub description: String,
    due: EventType,
}

fn uuid() -> String {
    let (id, _, _, _) = Uuid::new_v4().as_fields();
    format!("{:x}", id)
}

#[derive(Debug)]
pub enum EventError {
    InvalidDate(String),
    TooOld,
}

impl From<chrono::format::ParseError> for EventError {
    fn from(error: chrono::format::ParseError) -> EventError {
        EventError::InvalidDate(error.to_string())
    }
}

impl Event {
    pub fn new_on_date(description: &str, date: &str) -> Result<Self, EventError> {
        let fake_datetime = format!("{} 00:00:00", date);
        let dt = Local.datetime_from_str(&fake_datetime, DATE_FORMAT)?;

        if dt < Local::now() {
            Err(EventError::TooOld)
        } else {
            Ok(Self {
                id: uuid(),
                description: description.into(),
                due: EventType::AllDay(EventDate::from(&dt)),
            })
        }
    }

    pub fn new_on_date_time(description: &str, date: &str, time: &str) -> Result<Self, EventError> {
        let fake_datetime = format!("{} {}:00", date, time);
        let dt = Local.datetime_from_str(&fake_datetime, DATE_FORMAT)?;

        if dt < Local::now() {
            Err(EventError::TooOld)
        } else {
            Ok(Self {
                id: uuid(),
                description: description.into(),
                due: EventType::AtTime(EventDate::from(&dt), EventTime::from(&dt)),
            })
        }
    }

    pub fn eta(&self) -> Option<String> {
        let now = Local::now();
        let to: DateTime<Local> = (&self.due).into();
        let eta = to - now;
        log::debug!("ETA for {}: {}", self.id, eta.num_minutes());

        match self.due {
            EventType::AllDay(_) if eta.num_minutes() > 0 => Some(format!("{}d", eta.num_days())),
            EventType::AtTime(_, _) if eta.num_days() > 0 => {
                Some(format!("{}d {}h", eta.num_days(), eta.num_hours()))
            }
            EventType::AtTime(_, _) if eta.num_hours() > 0 => Some(format!("{}h", eta.num_hours())),
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
