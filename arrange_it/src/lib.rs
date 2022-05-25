pub fn arrange_phrase(phrase: &str) -> String {
    let nbrs: Vec<&str> = phrase.matches(char::is_numeric).collect();
    let temp_str = &phrase.replace(char::is_numeric, "");
    let mut words: Vec<&str> = temp_str.split_whitespace().collect();
    for (i, value) in nbrs.iter().enumerate() {
        let strs: Vec<&str> = temp_str.split_whitespace().collect();
        words[value.parse::<usize>().unwrap() - 1] = strs[i];
    }
    words.join(" ")
}
