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

use crate::date::Date;
use crate::date_errors::DateError;
use crate::datetime::DateTime;

type Description = String;
type Id = String;

pub enum ParseError {
    InvalidDate,
    UnknownOption,
    MissingDescription,
    MissingDate,
    MissingEventId,
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
    AddWithTime(Description, DateTime),
    RemoveById(Id),
    RemoveOutdated,
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
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove an event")
                .arg(
                    Arg::with_name("id")
                        .takes_value(true)
                        .required(false)
                        .conflicts_with("outdated")
                        .value_name("ID")
                        .help("Remove a specific event by its ID"),
                )
                .arg(
                    Arg::with_name("outdated")
                        .short("o")
                        .long("outdated")
                        .takes_value(false)
                        .required(false)
                        .conflicts_with("id"),
                ),
        );

    let matches = params.get_matches();
    match matches.subcommand() {
        ("", _) => Ok(Action::List),
        ("add", Some(arguments)) => parse_add(arguments),
        ("rm", Some(arguments)) => parse_rm(arguments),
        (_, _) => Err(ParseError::UnknownOption),
    }
}

fn parse_add(arguments: &ArgMatches) -> Result<Action, ParseError> {
    let description = arguments
        .value_of("description")
        .ok_or(ParseError::MissingDescription)?;
    let date = arguments.value_of("date").ok_or(ParseError::MissingDate)?;

    if let Some(time) = arguments.value_of("time") {
        Ok(Action::AddWithTime(
            description.into(),
            DateTime::try_from(date, time)?,
        ))
    } else {
        Ok(Action::Add(description.into(), Date::try_from(date)?))
    }
}

fn parse_rm(arguments: &ArgMatches) -> Result<Action, ParseError> {
    if arguments.is_present("outdated") {
        Ok(Action::RemoveOutdated)
    } else {
        let id = arguments.value_of("id").ok_or(ParseError::MissingEventId)?;
        Ok(Action::RemoveById(id.into()))
    }
}
