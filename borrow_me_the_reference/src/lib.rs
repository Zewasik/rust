extern crate meval;

pub fn delete_and_backspace(s: &mut String) {
    let mut temp = String::new();

    for value in s.chars() {
        if value == '-' {
            temp.pop();
        } else {
            temp += &value.to_string();
        }
    }

    let mut temp2 = String::new();

    for value in temp.chars().rev() {
        if value == '+' {
            temp2.pop();
        } else {
            temp2 += &value.to_string();
        }
    }
    *s = temp2.chars().rev().collect::<String>();
}

pub fn is_correct(v: &mut Vec<&str>) -> usize {
    let mut temp: Vec<&str> = Vec::new();
    let mut correct = 0;
    for value in v.iter() {
        let (left, right) = value.split_once("=").unwrap();
        if meval::eval_str(left).unwrap() == right.parse::<f64>().unwrap() {
            temp.push("✔");
            correct += 1
        } else {
            temp.push("✘");
        }
    }
    *v = temp;
    return 100 / v.len() * correct;
}
