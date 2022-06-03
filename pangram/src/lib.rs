pub fn is_pangram(s: &str) -> bool {
    for ch in 'a'..='z' {
        if !s.contains(ch) && !s.contains(ch.to_ascii_uppercase()) {
            return false;
        }
    }
    true
}
