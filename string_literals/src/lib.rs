pub fn is_empty(v: &str) -> bool {
    return v.is_empty();
}

pub fn is_ascii(v: &str) -> bool {
    return v.is_ascii();
}

pub fn contains(v: &str, pat: &str) -> bool {
    return v.contains(pat);
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    return v.split_at(index);
}

pub fn find(v: &str, pat: char) -> usize {
    return v.find(pat).unwrap();
}
