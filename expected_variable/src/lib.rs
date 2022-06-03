pub mod edit_distance;
use edit_distance::edit_distance;

pub fn expected_variable(to_compare: &str, expected: &str) -> Option<String> {
    if to_compare.contains(' ')
        || to_compare.contains('-')
        || expected.contains(' ')
        || expected.contains('-')
    {
        return None;
    };
    let a = to_compare.to_lowercase();
    let b = expected.to_lowercase();
    let test = edit_distance(&a, &b);
    let temp = ((1.0 - (test as f64 / expected.len() as f64)) * 100.0).round() as i32;

    if temp < 50 {
        return None;
    };
    let temp_str = temp.to_string();
    let res: String = temp_str + "%";
    return Some(res);
}
