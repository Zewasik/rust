pub fn talking(text: &str) -> &str {
    let text = text.trim();
    let mut variant = 4;

    if text.len() > 0 {
        if text.chars().any(|c| matches!(c, 'a'..='z')) {
            if text.chars().last().unwrap() == '?' {
                variant = 1;
            } else {
                variant = 4;
            }
        } else if text.chars().any(|c| matches!(c, 'A'..='Z')) {
            if text.chars().last().unwrap() == '?' {
                variant = 2;
            } else {
                variant = 0;
            }
        } else {
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
