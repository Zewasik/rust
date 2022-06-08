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
    fn append_str(mut self, new_str: String) -> Self {
        self.value.push_str(&new_str);
        self
    }

    fn append_number(mut self, new_number: f64) -> Self {
        self.value.push_str(&new_number.to_string());
        self
    }

    fn remove_punctuation_marks(mut self) -> Self {
        self.value = self
            .value
            .replace(|ch| ch == '.' || ch == ',' || ch == '?' || ch == '!', "");
        self
    }
}
