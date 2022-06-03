pub fn num_to_ordinal(x: u32) -> String {
    match x % 10 {
        1 => x.to_string() + if x == 11 { "th" } else { "st" },
        2 => x.to_string() + if x == 12 { "th" } else { "nd" },
        3 => x.to_string() + if x == 13 { "th" } else { "rd" },
        _ => x.to_string() + "th",
    }
}
