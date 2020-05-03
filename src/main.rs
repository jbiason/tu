mod args;
mod eventlist;

use crate::eventlist::event::Event;
use crate::eventlist::eventlist::EventList;

fn main() {
    if let Ok(command) = dbg!(args::parse()) {
        match command {
            args::Action::List => list(),
            args::Action::Add(description, date) => add_with_date(&description, &date),
            _ => println!("Unknown command"),
        }
    }
}

fn list() {
    unimplemented!()
}

fn add_with_date(description: &str, date: &str) {
    let event = Event::new_on_date(description, date);
    let mut event_list = dbg!(EventList::load());
    event_list.push(event);
    event_list.save();
}
