use diamond_creation::*;

fn main() {
    println!("{:?}", get_diamond('A'));
    println!("{:?}", get_diamond('B'));
    println!("{:?}", get_diamond('C'));
    println!("{:?}", get_diamond('D'));
    // println!("{:?}", get_diamond('Z'));

    for s in get_diamond('Z') {
        println!("{}", s)
    }
}
