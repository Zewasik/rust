use arrays::{sum, thirtytwo_tens};

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a1: Vec<i32> = (1..11).collect();
    let b = [100; 10];

    println!("The Sum of the elements in {:?} = {}", a, sum(&a));
    println!("The Sum of the elements in {:?} = {}", a1, sum(&a1));
    println!("The Sum of the elements in {:?} = {}", b, sum(&b));
    println!(
        "Array size {} with only 10's in it {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}
