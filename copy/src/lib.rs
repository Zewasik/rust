pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let copy: f64 = c.clone() as f64;
    return (c, copy.exp(), copy.ln());
}

pub fn str_function(a: String) -> (String, String) {
    let copy = a.clone();
    let mut nums: Vec<String> = Vec::new();

    for value in copy.split(" ") {
        let temp: f64 = value.parse().unwrap();
        nums.push(temp.exp().to_string());
    }

    return (a, nums.join(" "));
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut copy: Vec<f64> = Vec::new();

    for value in b.iter() {
        copy.push((*value as f64).ln());
    }

    return (b, copy);
}
