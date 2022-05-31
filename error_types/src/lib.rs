pub use chrono::{NaiveDate, Utc};

#[derive(Debug, Eq, PartialEq)]
pub struct FErr {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FErr {
    pub fn new(name: String, error: String, err: String) -> FErr {
        FErr {
            form_values: (name, error),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub fav_colour: Color,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        fav_colour: Color,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            fav_colour,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FErr> {
        if self.first_name.is_empty() {
            return Err(FErr::new(
                String::from("first_name"),
                self.first_name.clone(),
                String::from("No user name"),
            ));
        } else if self.password.chars().count() < 8 {
            return Err(FErr::new(
                String::from("password"),
                self.password.clone(),
                String::from("At least 8 characters"),
            ));
        } else if !self.password.contains(|ch: char| ch.is_ascii_digit())
            || !self.password.contains(|ch: char| ch.is_ascii_alphabetic())
            || !self.password.contains(|ch: char| ch.is_ascii_punctuation())
        {
            return Err(FErr::new(String::from("password"), self.password.clone(),String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)")));
        }

        Ok(vec!["Valid first name", "Valid password"])
    }
}
