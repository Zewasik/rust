pub fn first_subword(mut s: String) -> String {
    let mut i = 0;
    for value in s.chars() {
        if i != 0 && (value >= 'A' && value <= 'Z' || value == '_') {
            break;
        }
        i += 1
    }
    s[..i].to_string()
}
