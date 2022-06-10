use std::collections::HashMap;

use chrono::*;
use json::*;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut ans: HashMap<String, u32> = HashMap::new();
    println!("{}", data);
    ans
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut ans: HashMap<String, u32> = HashMap::new();
    println!("{}", data);
    ans
}
