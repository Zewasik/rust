#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Worker {
    pub worker_type: String,
    pub worker_name: String,
    pub next_worker: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: Link::None }
    }

    pub fn add_worker(&mut self, t: String, name: String) {
        self.grade = Link::Some(Box::new(Worker {
            worker_type: t,
            worker_name: name,
            next_worker: self.grade.clone(),
        }));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.clone() {
            Link::Some(wok) => {
                self.grade = wok.next_worker;
                Some(wok.worker_name.clone())
            }
            Link::None => None,
        }
    }

    pub fn search_worker(&self) -> Option<(String, String)> {
        match &self.grade {
            Some(find) => Some((find.worker_name.clone(), find.worker_type.clone())),
            None => None,
        }
    }
}
