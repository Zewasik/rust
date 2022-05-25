pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split(" ").collect();
    let mut correct_phrase: Vec<String> = vec!["".to_string(); words.len() - 1];
    for word in &words {
        let mut index: usize = 0;
        let mut temp_str = "".to_string();
        for ch in word.chars() {
            if ch.is_ascii_digit() {
                index = ch as usize - 48;
            } else {
                temp_str += &ch.to_string();
            }
        }
        if index < correct_phrase.len() {
            correct_phrase.remove(index - 1);
        }
        correct_phrase.insert(index - 1, temp_str);
    }

    return correct_phrase.join(" ").trim().to_string();
}
