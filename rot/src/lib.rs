pub fn rotate(input: &str, key: i8) -> String {
    let mut temp_str = String::new();

    for ch in input.chars() {
        temp_str.push(if ch.is_ascii_alphabetic() {
            let char_key = if ch.is_ascii_lowercase() {
                (
                    'a' as u32,
                    if key < 0 {
                        (26 + key) as u32
                    } else {
                        key as u32
                    },
                )
            } else {
                (
                    'A' as u32,
                    if key < 0 {
                        (26 + key) as u32
                    } else {
                        key as u32
                    },
                )
            };

            char::from_u32(((ch as u32 - char_key.0 + char_key.1) % 26) + char_key.0).unwrap()
        } else {
            ch
        })
    }
    temp_str
}
