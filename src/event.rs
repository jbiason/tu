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

use serde_derive::Deserialize;
use serde_derive::Serialize;
use uuid::Uuid;

use crate::date::Date;
use crate::datetime::DateTime;
use crate::eventtype::EventType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub description: String,
    pub due: EventType,
}

/// TODO inject this
fn uuid() -> String {
    let (id, _, _, _) = Uuid::new_v4().as_fields();
    format!("{:x}", id)
}

#[derive(Debug)]
pub enum EventError {
    InvalidDate(String),
    TooOld,
}

impl Event {
    pub fn new_on_date(description: &str, date: &Date) -> Result<Self, EventError> {
        Ok(Self {
            id: uuid(),
            description: description.into(),
            due: EventType::AllDay(date.clone()),
        })
    }

    pub fn new_on_date_time(description: &str, datetime: &DateTime) -> Result<Self, EventError> {
        Ok(Self {
            id: uuid(),
            description: description.into(),
            due: EventType::AtTime(datetime.clone()),
        })
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.due.timestamp() == other.due.timestamp()
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.due.timestamp().cmp(&other.due.timestamp())
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.due.timestamp().cmp(&other.due.timestamp()))
    }
}
