pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() < 2 {
        return vec![];
    }
    arr.iter()
        .map(|x| arr.iter().product::<usize>() / x)
        .collect()
}
