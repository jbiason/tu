use chrono::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    hour: u32,
    min: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "due", content = "datetime")]
pub enum EventDateType {
    AllDay(Date),
    AtTime(Date, Time),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    description: String,
    due: EventDateType,
}

impl Event {
    pub fn new_on_date(description: &str, date: &str) -> Self {
        let fake_datetime = format!("{} 00:00:00", date);
        if let Ok(dt) = Utc.datetime_from_str(&fake_datetime, "%Y-%m-%d %H:%M:%S") {
            Self {
                description: description.into(),
                due: EventDateType::AllDay(Date {
                    year: dt.year(),
                    month: dt.month(),
                    day: dt.day(),
                }),
            }
        } else {
            panic!("Failed to parse the date");
        }
    }
}
