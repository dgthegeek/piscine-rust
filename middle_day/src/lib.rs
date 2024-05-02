pub use chrono::{Datelike, NaiveDate, Weekday as wd};
pub fn middle_day(year: i32) -> Option<chrono::Weekday> { 
    let january_1 = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let december_31: NaiveDate = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
    let num_days = (december_31 - january_1).num_days() + 1;
    
    if num_days % 2 == 0 {
        return None;
    };
    let mid_day = january_1 + chrono::Duration::days(num_days / 2);
    Some(mid_day.weekday())
}