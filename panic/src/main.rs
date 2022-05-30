use panic::*;
use std::fs;
use std::fs::File;

fn main() {
    let filename = "created.txt";
    File::create(filename).unwrap();

    let a = open_file(filename);
    println!("{:?}", a);

    fs::remove_file(filename).unwrap();

    //It must panic
    let b = open_file(filename);
}
