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

use std::fs::File;
use std::io::{Read, Write};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use toml;

use crate::eventlist::event::Event;

static FILENAME: &str = "events.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct EventList {
    events: Vec<Event>,
}

impl EventList {
    fn empty() -> Self {
        Self { events: Vec::new() }
    }

    pub fn load() -> Self {
        if let Ok(mut fp) = File::open(FILENAME) {
            let mut content = String::new();
            fp.read_to_string(&mut content)
                .expect("Your event file is corrupted");
            toml::from_str(&content).unwrap_or(EventList::empty())
        } else {
            EventList::empty()
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn save(&self) {
        let content = toml::to_string(&self).unwrap();
        if let Ok(mut fp) = File::create(FILENAME) {
            fp.write_all(content.as_bytes()).unwrap();
        }
    }
}
