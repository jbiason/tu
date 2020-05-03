use chrono::prelude::*;

pub fn date(input: String) -> Result<(), String> {
    let fake_datetime = format!("{} 00:00:00", input);
    if let Ok(_) = dbg!(Utc.datetime_from_str(&fake_datetime, "%Y-%m-%d %H:%M:%S")) {
        Ok(())
    } else {
        Err(format!("Invalid date: '{}'", input))
    }
}

pub fn time(input: String) -> Result<(), String> {
    let fake_datetime = format!("2020-01-01 {}:00", input);
    if let Ok(_) = dbg!(Utc.datetime_from_str(&fake_datetime, "%Y-%m-%d %H:%M:%S")) {
        Ok(())
    } else {
        Err(format!("Invalid time: '{}'", input))
    }
}
