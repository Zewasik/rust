pub fn num_to_ordinal(x: u32) -> String {
    match x % 10 {
        1 if x != 11 => x.to_string() + "st",
        2 if x != 12 => x.to_string() + "nd",
        3 if x != 13 => x.to_string() + "rd",
        _ => x.to_string() + "th",
    }
}
