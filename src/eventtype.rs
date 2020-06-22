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

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::date::Date;
use crate::datetime::DateTime;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "due", content = "datetime")]
pub enum EventType {
    AllDay(Date),
    AtTime(DateTime),
}

impl EventType {
    pub fn timestamp(&self) -> i64 {
        match self {
            EventType::AllDay(date) => date.timestamp(),
            EventType::AtTime(datetime) => datetime.timestamp(),
        }
    }
}
