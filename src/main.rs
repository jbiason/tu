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
mod date;
mod date_errors;
mod datetime;
mod event;
mod eventlist;
mod eventtype;

use eventlist::EventList;
use eventtype::EventType;

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
            args::Action::AddWithTime(description, datetime) => {
                let event_id =
                    EventList::add_event_with_date_and_time(&description, &datetime).unwrap();
                println!("Created new event {}", event_id);
            }
        }
    } else {
        println!("Error!");
    }
}

fn list() {
    let event_list = EventList::load(); // TODO hide load from outside
    println!("{:^8} | {:^7} | {}", "ID", "ETA", "Description");
    // TODO: EventList::iter()
    for event in event_list.into_iter() {
        let eta = match event.due {
            EventType::AllDay(date) => {
                let eta = date.eta();
                match eta {
                    None => "Over".into(),
                    Some(0) => "Today".into(),
                    Some(1) => "Tomorrow".into(),
                    Some(x) => format!("{}d", x),
                }
            }
            EventType::AtTime(datetime) => {
                let eta = datetime.eta();
                match eta {
                    None => "Over".into(),
                    Some((0, hours)) => format!("{}h", hours),
                    Some((days, hours)) => format!("{}d {}h", days, hours),
                }
            }
        };

        println!("{:>8} | {:>7} | {}", event.id, eta, event.description);
    }
}
