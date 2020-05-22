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

use crate::eventlist::eventlist::EventList;

fn main() {
    env_logger::init();

    if let Ok(command) = args::parse() {
        log::debug!("Command: {:?}", command);
        match command {
            args::Action::List => list(),
            args::Action::Add(description, date) => {
                let event_id = EventList::add_event_with_date(&description, &date).unwrap();
                println!("Created new event {}", event_id);
            }
            args::Action::AddWithTime(description, date, time) => {
                let event_id =
                    EventList::add_event_with_date_and_time(&description, &date, &time).unwrap();
                println!("Created new event {}", event_id);
            }
        }
    }
}

fn list() {
    let event_list = EventList::load(); // TODO hide load from outside
    println!("{:^8} | {:^7} | {}", "ID", "ETA", "Description");
    // TODO: EventList::iter()
    for record in event_list.into_iter() {
        let eta = if let Some(eta) = record.eta() {
            // TODO: "1d" == Tomorrow; "0d" == Today
            eta
        } else {
            "Over".into()
        };

        println!("{:>8} | {:>7} | {}", record.id, eta, record.description);
    }
}
