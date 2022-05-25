pub fn arrange_phrase(phrase: &str) -> String {
    let ans = {
        let words: Vec<&str> = phrase.split(" ").collect();
        let mut correct_phrase: Vec<String> = vec!["".to_string(); words.len() - 1];
        let mut index = 0;
        for &word in &words {
            let mut temp_str = "".to_string();
            for ch in word.chars() {
                if ch.is_ascii_digit() {
                    index = ch as usize - 48;
                } else {
                    temp_str += &ch.to_string();
                }
            }
            if index < correct_phrase.len() {
                correct_phrase.remove(&index - 1);
            }
            correct_phrase.insert(&index - 1, temp_str);
        }
        correct_phrase
    };
    return ans.join(" ");
}
