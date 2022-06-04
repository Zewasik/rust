#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let a = if self.r == first {
            "r"
        } else if self.g == first {
            "g"
        } else if self.b == first {
            "b"
        } else {
            "a"
        };

        let b = if self.r == second {
            "r"
        } else if self.g == second {
            "g"
        } else if self.b == second {
            "b"
        } else {
            "a"
        };

        println!("first: {}, second: {}", a, b);

        match a {
            "r" => self.r = second,
            "g" => self.g = second,
            "b" => self.b = second,
            _ => self.a = second,
        };

        match b {
            "r" => self.r = first,
            "g" => self.g = first,
            "b" => self.b = first,
            _ => self.a = first,
        };

        self
    }
}
