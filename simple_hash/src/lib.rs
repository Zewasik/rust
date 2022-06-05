use std::collections::HashMap;

pub fn contain(h: &HashMap<&str, i32>, s: &str) -> bool {
    h.contains_key(s)
}

pub fn remove(h: &mut HashMap<&str, i32>, s: &str) {
    h.remove(s);
}
