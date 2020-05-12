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

use log;

mod args;
mod eventlist;

use crate::eventlist::event::Event;
use crate::eventlist::eventlist::EventList;

fn main() {
    env_logger::init();

    if let Ok(command) = args::parse() {
        log::debug!("Command: {:?}", command);
        match command {
            args::Action::List => list(),
            args::Action::Add(description, date) => add_with_date(&description, &date),
            args::Action::AddWithTime(description, date, time) => {
                add_with_date_time(&description, &date, &time)
            }
        }
    }
}

fn list() {
    let event_list = EventList::load();
    for record in event_list.into_iter() {
        println!("{}", record);
    }
}

fn add_with_date(description: &str, date: &str) {
    let event = Event::new_on_date(description, date);
    add_event(event);
}

fn add_with_date_time(description: &str, date: &str, time: &str) {
    let event = Event::new_on_date_time(description, date, time);
    add_event(event);
}

fn add_event(event: Event) {
    println!("Adding event {}", event);

    let mut event_list = EventList::load();
    log::debug!("EventList: {:?}", event_list);
    event_list.push(event);
    event_list.save();

    println!("Done.");
}
