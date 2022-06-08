use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(vec![]),
        }
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let id = self.states.borrow().len();
        self.states.borrow_mut().push(false);

        (id, Thread::new_thread(id, c, &self))
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        *self.states.borrow().get(id).unwrap()
    }

    pub fn add_drop(&self, id: usize) {
        let mut temp = self.states.borrow_mut();
        if *temp.get(id).unwrap() {
            panic!("Cannot drop X, because its already dropped")
        } else {
            temp[id] = true;
            self.drops.set(self.drops.get() + 1); // repair this part
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}
