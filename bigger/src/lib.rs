use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut comp = 0;

    for i in h.values() {
        comp = if comp < *i { *i } else { comp }
    }

    comp
}
