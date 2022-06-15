pub fn first_fifty_even_square() -> Vec<i32> {
    let mut ans = vec![];
    for i in (2..=100).step_by(2) {
        ans.push(i * i);
    }

    ans
}
