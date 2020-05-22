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
use crate::eventlist::event::EventError;

static FILENAME: &str = "events.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct EventList {
    pub events: Vec<Event>, // TODO remove pub
}
// XXX new type?  pub struct EventList(Vec<Event>);

// TODO expose Vec iterator?
pub struct EventListIterator<'a> {
    index: usize,
    max: usize,
    list: &'a Vec<Event>,
}

#[derive(Debug)]
pub enum EventListError {
    InvalidDate,
    TooOld,
}

impl From<EventError> for EventListError {
    fn from(error: EventError) -> EventListError {
        match error {
            EventError::InvalidDate(_) => EventListError::InvalidDate,
            EventError::TooOld => EventListError::TooOld,
        }
    }
}

// TODO separate business rule from repository
impl EventList {
    fn empty() -> Self {
        Self { events: Vec::new() }
    }

    // TODO hide this
    pub fn load() -> Self {
        if let Ok(mut fp) = File::open(FILENAME) {
            let mut content = String::new();
            fp.read_to_string(&mut content)
                .expect("Your event file is corrupted");
            // TODO remove toml
            toml::from_str(&content).unwrap_or(EventList::empty())
        } else {
            EventList::empty()
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
        self.events.sort();
    }

    // TODO turn this into the destructor
    // TODO if so, track changes
    pub fn save(&self) {
        // TODO remove toml
        let content = toml::to_string(&self).unwrap();
        if let Ok(mut fp) = File::create(FILENAME) {
            fp.write_all(content.as_bytes()).unwrap();
        }
    }

    /// Load the event list, add an all day event, and save it back.
    /// Returns the ID of the new event.
    pub fn add_event_with_date(description: &str, date: &str) -> Result<String, EventListError> {
        let mut list = EventList::load();
        let event = Event::new_on_date(description, date)?;
        let id = String::from(&event.id);
        list.push(event);
        list.save();
        Ok(id)
    }

    /// Load the event list, add an event with date and time, and save it back.
    /// Returns the ID of the new event.
    pub fn add_event_with_date_and_time(
        description: &str,
        date: &str,
        time: &str,
    ) -> Result<String, EventListError> {
        let mut list = EventList::load();
        let event = Event::new_on_date_time(description, date, time).unwrap();
        let id = String::from(&event.id);
        list.push(event);
        list.save();
        Ok(id)
    }
}

impl<'a> IntoIterator for &'a EventList {
    type Item = &'a Event;
    type IntoIter = EventListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EventListIterator {
            index: 0,
            max: self.events.len(),
            list: &self.events,
        }
    }
}

impl<'a> Iterator for EventListIterator<'a> {
    type Item = &'a Event;

    fn next(&mut self) -> Option<&'a Event> {
        if self.index >= self.max {
            None
        } else {
            self.index += 1;
            Some(&self.list[self.index - 1])
        }
    }
}
