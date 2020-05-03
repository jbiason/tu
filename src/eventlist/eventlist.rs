use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::io::{Read, Write};
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
