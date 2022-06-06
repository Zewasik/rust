pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut temp: Vec<u32> = Vec::new();

    for int in s.split_ascii_whitespace() {
        temp.push(if int.contains("k") {
            (int.replace("k", "").parse::<f64>().unwrap() * 1000.0) as u32
        } else {
            int.parse().unwrap()
        })
    }

    Box::new(temp)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_it_test() {
        let new_str = String::from("5.5k 8.9k 32");
        let a_h = transform_and_save_on_heap(new_str);

        assert_eq!(Box::new(vec![5500, 8900, 32]), a_h);
        assert_eq!(8, std::mem::size_of_val(&a_h));

        let a_b_v = take_value_ownership(a_h);

        assert_eq!(vec![5500, 8900, 32], a_b_v);
        assert_eq!(24, std::mem::size_of_val(&a_b_v));
    }
}
