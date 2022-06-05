use bigger::bigger;
use std::collections::HashMap;

fn main() {
    let mut hash = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!(
        "The biggest of the elements in the HashMap is {}",
        bigger(hash)
    );
}
