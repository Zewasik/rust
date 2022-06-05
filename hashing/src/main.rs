use hashing::*;

fn main() {
    println!("Hello, world!");
    let v = vec![2, 1, 5, 2, 7, 4];
    println!("mean {}", hashing::mean(&v));
    println!("median {}", hashing::median(&v));
    println!("mode {}", hashing::mode(&v));
}
