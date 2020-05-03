use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;

mod validators;

type Description = String;
type Date = String;
type Time = String;

#[derive(Debug)]
pub enum Action {
    List,
    Add(Description, Date),
    AddWithTime(Description, Date, Time),
}

pub fn parse() -> Result<Action, ()> {
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
                        .validator(validators::date)
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
                        .validator(validators::time)
                        .help("Time for the event"),
                ),
        );
    let matches = params.get_matches();

    match matches.subcommand() {
        ("", _) => Ok(Action::List),
        ("add", Some(arguments)) => parse_add(arguments),
        (_, _) => Err(()),
    }
}

fn parse_add(arguments: &ArgMatches) -> Result<Action, ()> {
    let description = arguments.value_of("description").unwrap();
    let date = arguments.value_of("date").unwrap();
    if let Some(time) = arguments.value_of("time") {
        Ok(Action::AddWithTime(
            description.into(),
            date.into(),
            time.into(),
        ))
    } else {
        Ok(Action::Add(description.into(), date.into()))
    }
}
