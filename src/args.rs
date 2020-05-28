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

use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;

use crate::date::{Date, DateError};

type Description = String;

pub enum ParseError {
    InvalidDate,
    UnknownOption,
}

impl From<DateError> for ParseError {
    fn from(_: DateError) -> ParseError {
        ParseError::InvalidDate
    }
}

#[derive(Debug)]
pub enum Action {
    List,
    Add(Description, Date),
    AddWithTime(Description, String, String),
}

pub fn parse() -> Result<Action, ParseError> {
    let params = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new event")
                .arg(
                    Arg::with_name("date")
                        .required(true)
                        .takes_value(true)
                        .help("Date for the event, in YYYY-MM-DD format"),
                )
                .arg(
                    Arg::with_name("description")
                        .required(true)
                        .takes_value(true)
                        .help("Event description"),
                )
                .arg(
                    Arg::with_name("time")
                        .short("t")
                        .long("time")
                        .takes_value(true)
                        .required(false)
                        .help("Time for the event"),
                ),
        );
    let matches = params.get_matches();

    match matches.subcommand() {
        ("", _) => Ok(Action::List),
        ("add", Some(arguments)) => parse_add(arguments),
        (_, _) => Err(ParseError::UnknownOption),
    }
}

fn parse_add(arguments: &ArgMatches) -> Result<Action, ParseError> {
    let description = arguments.value_of("description").unwrap();
    let date = arguments.value_of("date").unwrap();

    if let Some(time) = arguments.value_of("time") {
        Ok(Action::AddWithTime(
            description.into(),
            date.into(),
            time.into(),
        ))
    } else {
        Ok(Action::Add(description.into(), Date::try_from(date)?))
    }
}
