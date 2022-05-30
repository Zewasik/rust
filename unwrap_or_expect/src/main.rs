use unwrap_or_expect::*;

fn main() {
    // // if uncommented, the below line will give an expect "ERROR "
    // println!("{:?}", expect(vec![1, 3, 2, 5]));

    println!("{:?}", unwrap_or(vec![1, 3, 2, 5]));
    println!("{:?}", unwrap_or(vec![1, 3, 5]));

    println!("{:?}", unwrap_err(vec![1, 3, 2, 5]));

    // // if uncommented, the below line will give an unwraped error
    // println!("{:?}", unwrap_err(vec![1, 3, 5]));

    println!("{:?}", unwrap(vec![1, 3, 5]));

    //// if uncommented, the below line will give an error
    // println!("{:?}", unwrap(vec![1, 3, 2, 5]));

    println!("{:?}", unwrap_or_else(vec![1, 3, 5]));
    println!("{:?}", unwrap_or_else(vec![3, 2, 6, 5]));
}
