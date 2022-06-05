pub fn get_diamond(c: char) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    let wide: usize = ((c as u8 - b'A' + 1) * 2 - 1) as usize;
    println!("{}", wide);

    for ch in 'A'..=c {
        if ch == 'A' {
            ans.push(" ".repeat(wide / 2) + &ch.to_string() + &" ".repeat(wide / 2));
        } else {
            let inner_wide = ((ch as u8 - b'A') * 2 - 1) as usize;
            let outer_wide = (wide - 2 - inner_wide) / 2;

            ans.push(
                " ".repeat(outer_wide)
                    + &ch.to_string()
                    + &" ".repeat(inner_wide)
                    + &ch.to_string()
                    + &" ".repeat(outer_wide),
            );
        }
    }

    let mut copy = ans.clone();
    copy.pop();
    copy.reverse();
    ans.append(&mut copy);

    ans
}
