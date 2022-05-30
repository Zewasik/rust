use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file)
    {
        Ok(mut result) => result.write_all(content.as_bytes()),
        Err(error) => panic!("{}", error),
    };
}
