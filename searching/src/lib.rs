pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().position(|&to_find| to_find == key)
}
