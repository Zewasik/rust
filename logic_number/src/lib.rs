pub fn number_logic(num: u32) -> bool {
    let mut nums: Vec<u32> = Vec::new();
    let mut temp = num;
    loop {
        nums.push(temp % 10);
        if temp / 10 == 0 {
            break;
        }
        temp /= 10;
    }

    let mut sum = 0.0;

    for value in &nums {
        sum += (*value as f64).powi(nums.len() as i32);
    }

    sum as u32 == num
}
