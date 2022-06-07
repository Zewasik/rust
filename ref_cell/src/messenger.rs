pub use std::rc::Rc;
pub trait Logger {
    fn warning(&self, msg: &str) {
        println!("Warning: {}", msg)
    }
    fn info(&self, msg: &str) {
        println!("Info: {}", msg)
    }
    fn error(&self, msg: &str) {
        println!("Error: {}", msg)
    }
}

pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    max: usize,
}

impl<'a, T> Tracker<'a, T>
where
    T: Logger,
{
    pub fn new(logger: &T, max: usize) -> Tracker<T> {
        Tracker { logger, max }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let diff = Rc::strong_count(&value) * 100 / self.max;

        match diff {
            100.. => self.logger.error("Error: you are over your quota!"), //Error:
            70..=99 => self.logger.warning(
                format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution", //Warning:
                    diff
                )
                .as_str(),
            ),
            _ => (),
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let diff = Rc::strong_count(&value) * 100 / self.max;

        self.logger
            .info(format!("Info: you are using up too {}% of your quote", diff).as_str())
        //Info:
    }
}
