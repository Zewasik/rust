use simple_hash::*;
use std::collections::HashMap;

fn main() {
    let mut hash: HashMap<&str, i32> = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!(
        "Does the HashMap contains the name Roman? => {}",
        contain(&hash, "Roman")
    );
    println!(
        "Does the HashMap contains the name Katie? => {}",
        contain(&hash, "Katie")
    );
    println!("Removing Robert {:?}", remove(&mut hash, "Robert"));
    println!(
        "Does the HashMap contains the name Robert? => {}",
        contain(&hash, "Robert")
    );
    println!("Hash {:?}", &hash);
}
