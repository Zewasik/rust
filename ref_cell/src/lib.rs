pub use std::{cell::RefCell, collections::HashMap};
mod messenger;
pub use messenger::*;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(track_value: usize) -> Worker {
        Worker {
            track_value: Rc::new(track_value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(vec![]),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert(String::from("Warning"), msg.replace("Warning: ", ""));
        self.all_messages.borrow_mut().push(msg.to_string())
    }

    fn info(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert(String::from("Info"), msg.replace("Info: ", ""));
        self.all_messages.borrow_mut().push(msg.to_string())
    }

    fn error(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert(String::from("Error"), msg.replace("Error: ", ""));
        self.all_messages.borrow_mut().push(msg.to_string())
    }
}
