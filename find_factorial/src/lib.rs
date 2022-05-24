pub fn factorial(num: u64) -> u64 {
    let mut ans: u64 = 1;
    for i in 1..num + 1 {
        ans *= i;
    }
    return ans;
}
