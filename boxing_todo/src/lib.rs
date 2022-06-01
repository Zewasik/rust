#[path = "err.rs"]
mod err;
use err::{ParseErr, ReadErr};

pub use json::{parse, stringify};
pub use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let str = match fs::read_to_string(path) {
            Ok(result) => result,
            Err(err) => {
                return Err(Box::new(ReadErr {
                    child_err: Box::new(err),
                }))
            }
        };

        let raw_json = match parse(&str) {
            Ok(result) => result,
            Err(err) => return Err(Box::new(ParseErr::Malformed(Box::new(err)))),
        };

        let title = raw_json["title"].to_string();
        let mut temp = Vec::new();
        let tasks = raw_json["tasks"].members();

        if tasks.clone().count() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        for obj in tasks {
            let mut temp_obj = obj.entries();
            temp.push(Task {
                id: temp_obj.next().unwrap().1.as_u32().unwrap(),
                description: temp_obj.next().unwrap().1.to_string(),
                level: temp_obj.next().unwrap().1.as_u32().unwrap(),
            });
        }

        return Ok(TodoList { title, tasks: temp });
    }
}
