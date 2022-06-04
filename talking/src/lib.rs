pub fn talking(text: &str) -> &str {
    let mut variant = 4;
    if text.chars().count() > 0 {
        if text
            .chars()
            .all(|c| c.is_uppercase() || c == ' ' || c == '!' || c == '?' || c == '\'' || c == ',')
        {
            if text.chars().last().unwrap() != '?' {
                variant = 0;
            } else {
                variant = 2;
            }
        }
        if text.chars().any(|c| matches!(c, 'a'..='z')) && text.chars().last().unwrap() == '?' {
            variant = 1;
        }
    } else {
        variant = 3;
    }

    match variant {
        0 => return "There is no need to yell, calm down!",
        1 => return "Sure.",
        2 => return "Quiet, I am thinking!",
        3 => return "Just say something!",
        _ => return "Interesting",
    }
}
