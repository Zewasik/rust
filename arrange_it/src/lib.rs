pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split_ascii_whitespace().collect();
    let mut correct_phrase: Vec<String> = vec!["".to_string(); words.len()];
    let mut index = 0;
    for word in words {
        let mut temp_str = "".to_string();
        for ch in word.chars() {
            if ch >= '0' && ch <= '9' {
                index = ch as usize - 48; // 48 is '0' at ascii
            } else {
                temp_str += &ch.to_string();
            }
        }
        correct_phrase[index - 1] = temp_str.to_string();
    }
    return correct_phrase.join(" ");
}
