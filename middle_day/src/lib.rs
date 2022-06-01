use chrono::Weekday as wd;
use chrono::*;

pub fn middle_day(year: i32) -> Option<wd> {
    return if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        None
    } else {
        Some(Utc.ymd(year, 7, 2).weekday())
    };
}
