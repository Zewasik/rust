pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    return vec[index].to_string();
}
