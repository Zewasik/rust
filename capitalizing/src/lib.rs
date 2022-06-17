pub fn capitalize_first(input: &str) -> String {
    if input != "" {
        input[..1].to_uppercase() + &input[1..]
    } else {
        "".to_string()
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(|s| capitalize_first(s))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect()
}
