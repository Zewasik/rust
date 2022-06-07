use std::cell::RefCell;

trait Logger {
    fn warning(&self, msg: &str) {}
    fn info(&self, msg: &str) {}
    fn error(&self, msg: &str) {}
}

pub struct Tracker<'a> {
    logger: &'a dyn Logger,
    value: usize,
    max: usize,
}

impl Tracker<'_> {
    fn new(logger: &dyn Logger, max: usize) -> Tracker {
        Tracker {
            logger: logger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize, max: usize) {
        self.value = value;

        let diff = (value as f64 / max as f64 * 100.0).round() as usize;

        match diff {
            100.. => self.logger.error("Error: you are over your quota!"),
            70..=99 => self.logger.warning(
                format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                    diff
                )
                .as_str(),
            ),
            _ => self
                .logger
                .info(format!("Info: you are using up too {}% of your quote", diff).as_str()),
        }
    }
}
