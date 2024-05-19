pub use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        return None;
    }
    
    let date = NaiveDate::from_ymd_opt(year, 7, 2);
    match date {
        Some(d) => Some(d.weekday()),
        None => None,
    }
}
