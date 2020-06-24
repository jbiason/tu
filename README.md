# TU

Time's Up!

## About

Time's Up! is a small application that will let you control how long you have
till it is up to an event.

For example, if today is 07-May and you set an event for 22-May, it should
tell you that you have 15 days up to it.

## Commands

* Listing events: `tu`
* Adding new events: `tu add YYYY-MM-DD 'description'`
	* Optional: set a time for the event: `tu add YYYY-MM-DD 'description' --time HH:MM`
* Removing events: When you add an event, it will show up a small code for
	that event; you can remove it with `tu rm EVENTID`

## License

GNU AFFERO GENERAL PUBLIC LICENSE, Version 3.
