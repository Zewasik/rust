use collect::*;

fn main() {
    let ref mut v = vec![8, 2, 3, 7, 5, 1, 4, 6];
    let mut b = v.clone();
    bubble_sort(v);
    println!("{:?}", v);

    b.sort();
    println!("{:?}", b);
}
