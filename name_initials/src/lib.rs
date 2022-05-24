pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for value in names.iter() {
        let (a, b) = value.split_once(" ").unwrap();
        v.push(
            a.chars().next().unwrap().to_string()
                + ". "
                + &b.chars().next().unwrap().to_string()
                + ".",
        );
    }
    return v;
}
