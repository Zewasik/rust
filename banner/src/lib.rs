use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: String::from("-") + &l_h[0..1],
            long_hand: String::from("--") + l_h,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags.get(&(flag.0, flag.1)).unwrap()(argv[0], argv[1]) {
            Ok(result) => result,
            Err(error) => error.to_string(),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_num = match a.parse::<f32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    let b_num = match b.parse::<f32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    Ok((a_num / b_num).to_string())
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_num = match a.parse::<f32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    let b_num = match b.parse::<f32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    Ok((a_num % b_num).to_string())
}
