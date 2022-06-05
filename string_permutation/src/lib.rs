use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut word1: HashMap<char, u32> = HashMap::new();
    let mut word2: HashMap<char, u32> = HashMap::new();

    for ch in s1.chars() {
        let counter = word1.entry(ch).or_insert(0);
        *counter += 1;
    }

    for ch in s2.chars() {
        let counter = word2.entry(ch).or_insert(0);
        *counter += 1;
    }

    word1 == word2
}
