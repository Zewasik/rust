#[derive(Debug)]

pub struct Student(pub i32, pub String, pub String);

pub fn id(student: &Student) -> i32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    (student.1).clone()
}

pub fn last_name(student: &Student) -> String {
    (student.2).clone()
}
