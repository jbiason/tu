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
use log;

pub fn date(input: String) -> Result<(), String> {
    let fake_datetime = format!("{} 00:00:00", input);
    if let Ok(parse) = Utc.datetime_from_str(&fake_datetime, "%Y-%m-%d %H:%M:%S") {
        log::debug!("Parsing {} ({})= {:?}", input, fake_datetime, parse);
        Ok(())
    } else {
        Err(format!("Invalid date: '{}'", input))
    }
}

pub fn time(input: String) -> Result<(), String> {
    let fake_datetime = format!("2020-01-01 {}:00", input);
    if let Ok(parse) = Utc.datetime_from_str(&fake_datetime, "%Y-%m-%d %H:%M:%S") {
        log::debug!("Parsing {} ({}) = {:?}", input, fake_datetime, parse);
        Ok(())
    } else {
        Err(format!("Invalid time: '{}'", input))
    }
}
