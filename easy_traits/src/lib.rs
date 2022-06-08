#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(self, new_str: String) -> Self;

    fn append_number(self, new_number: f64) -> Self;

    fn remove_punctuation_marks(self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(self, new_str: String) -> Self {
        Self {
            value: self.value + &new_str,
        }
    }

    fn append_number(self, new_number: f64) -> Self {
        Self {
            value: self.value + &new_number.to_string(),
        }
    }

    fn remove_punctuation_marks(self) -> Self {
        Self {
            value: self
                .value
                .replace(|ch| ch == '.' || ch == ',' || ch == '?' || ch == '!', ""),
        }
    }
}
