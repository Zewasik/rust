pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    arr.iter()
        .map(|x| arr.iter().product::<usize>() / x)
        .collect()
}
