use std::fs::File;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(result) => result,
        Err(_) => panic!("File not found"),
    }
}
