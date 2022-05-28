pub fn sum(a: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for num in a {
        sum += num
    }
    return sum;
}

pub fn thirtytwo_tens() -> [i32; 32] {
    return [10; 32];
}
