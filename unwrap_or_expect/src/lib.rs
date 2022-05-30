pub fn odd_to_even(data: Vec<u32>) -> Result<Vec<u32>, (String, Vec<u32>)> {
    let mut a = Vec::new();
    a.extend(data.iter().filter(|&value| value % 2 == 0));
    if a.len() != 0 {
        return Err(("There is a even value in the vector!".to_string(), a));
    }
    a.extend(data.iter().map(|&value| value + 1));
    Ok(a)
}
pub fn expect(v: Vec<u32>) -> Vec<u32> {
    match odd_to_even(v) {
        Ok(result) => result,
        Err(error) => panic!("ERROR : {:?}", error),
    }
}
pub fn unwrap_or(v: Vec<u32>) -> Vec<u32> {
    match odd_to_even(v) {
        Ok(result) => result,
        Err(_) => vec![],
    }
}
pub fn unwrap_err(v: Vec<u32>) -> (String, Vec<u32>) {
    match odd_to_even(v) {
        Ok(_) => panic!(),
        Err(error) => error,
    }
}
pub fn unwrap(v: Vec<u32>) -> Vec<u32> {
    match odd_to_even(v) {
        Ok(result) => result,
        Err(error) => Result::unwrap(Err(error)),
    }
}
pub fn unwrap_or_else(v: Vec<u32>) -> Vec<u32> {
    match odd_to_even(v) {
        Ok(result) => result,
        Err(error) => error.1,
    }
}
