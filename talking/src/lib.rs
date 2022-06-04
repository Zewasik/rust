pub fn talking(text: &str) -> &str {
    let yell_0 = "There is no need to yell, calm down!";
    let ask_1 = "Sure.";
    let yell_q_2 = "Quiet, I am thinking!";
    let nothing_3 = "Just say something!";
    let deafult_4 = "Interesting";
    let mut variant = 4;
    if text.len() > 0 {
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
        0 => return yell_0,
        1 => return ask_1,
        2 => return yell_q_2,
        3 => return nothing_3,
        4 => return deafult_4,
        _ => return deafult_4,
    }
}
