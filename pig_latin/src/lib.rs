pub fn pig_latin(text: &str) -> String {
    let is_vowels = |ch: char| ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u';

    if text.len() == 0 {
        return text.to_owned();
    }

    if !is_vowels(text[0..1].chars().next().unwrap()) {
        if text.len() >= 3 {
            if &text[..2] == "qu" {
                return text[2..].to_owned() + &text[0..2] + "ay";
            } else if &text[1..3] == "qu" {
                return text[3..].to_owned() + &text[0..3] + "ay";
            }
        }
        match text.find(is_vowels) {
            Some(i) => return text[i..].to_owned() + &text[0..i] + "ay",
            None => return text.to_owned() + "ay",
        }
    }

    text.to_owned() + "ay"
}
