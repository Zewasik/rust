pub use chrono::{Datelike, NaiveDate};
pub use std::collections::HashMap;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut ans: HashMap<String, u32> = HashMap::new();

    for value in data.members() {
        let temp = &value["commit"]["author"]["date"].to_string();

        let time = NaiveDate::parse_from_str(temp, "%Y-%m-%dT%H:%M:%S%Z")
            .unwrap()
            .iso_week();

        let s = format!("{}-W{}", time.year(), time.week());

        let counter = ans.entry(s).or_insert(0);
        *counter += 1;
    }
    ans
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut ans: HashMap<String, u32> = HashMap::new();
    for value in data.members() {
        let temp = value["author"]["login"].to_string();

        let counter = ans.entry(temp).or_insert(0);
        *counter += 1;
    }
    ans
}
