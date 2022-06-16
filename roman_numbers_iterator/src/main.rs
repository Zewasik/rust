use roman_numbers_iterator::*;

fn main() {
    let mut number = RomanNumber::from(49);

    println!("{:?}", number);
    println!("{:?}", number.next());
}
