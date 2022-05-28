pub fn capitalize_first(input: &str) -> String {
    return input[0..1].to_uppercase().to_string() + &input[1..];
}

pub fn title_case(input: &str) -> String {
    let mut temp_str: String = String::new();
    let mut is_new = true;
    for ch in input.chars() {
        if is_new == true {
            temp_str += &ch.to_uppercase().to_string();
            is_new = false;
        } else {
            temp_str += &ch.to_string();
        }

        if ch.is_ascii_whitespace() == true {
            is_new = true;
        }
    }

    return temp_str;
}

pub fn change_case(input: &str) -> String {
    let mut temp_str: String = String::new();

    for ch in input.chars() {
        if ch.is_ascii_lowercase() {
            temp_str += &ch.to_uppercase().to_string();
        } else if ch.is_ascii_uppercase() {
            temp_str += &ch.to_lowercase().to_string();
        } else {
            temp_str += &ch.to_string();
        }
    }

    return temp_str;
}
