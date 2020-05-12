# TU

Time's Up!

## About

Time's Up! is a small application that will let you control how long you have
till it is up to an event.

For example, if today is 07-May and you set an event for 22-May, it should
tell you that you have 15 days up to it.

## Commands

* Adding new events: `tu add YYYY-MM-DD 'description'`
	* Optional: set a time for the event: `tu add YYYY-MM-DD 'description' --time HH:MM`
* Listing events: `tu`
* Removing events: When you add an event, it will show up a small code for
	that event; you can remove it with `tu rm EVENTID`

## TODO

- [x] Add unique identifier for each event
- [x] List events
- [x] Add Events with time
- [x] List events with time
- [x] Sort events by ETA
- [ ] Option to remove events
- [ ] Automatically remove past events
- [ ] Move the app "db" to a fixed space
- [x] Replace `dbg!` with [env_logger](https://crates.io/crates/env_logger)
	(reasoning: Although `dbg!` is nice and dandy, it can't be disabled, and
	that's bad UI)

## License

GNU AFFERO GENERAL PUBLIC LICENSE, Version 3.
