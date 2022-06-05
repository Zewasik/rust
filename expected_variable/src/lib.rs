pub mod edit_distance;
use edit_distance::edit_distance;

pub fn expected_variable(to_compare: &str, expected: &str) -> Option<String> {
    if to_compare.contains(|ch| ch == ' ' || ch == '-')
        || expected.contains(|ch| ch == ' ' || ch == '-')
    {
        return None;
    };

    let dist = edit_distance(&to_compare.to_lowercase(), &expected.to_lowercase());
    let ans = ((1.0 - (dist as f64 / expected.len() as f64)) * 100.0).round() as i32;

    if ans < 50 {
        return None;
    };

    Some(ans.to_string() + "%")
}
