pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    return (
        c.clone(),
        f64::from(c.clone()).exp(),
        (c.clone() as f64).ln().abs(),
    );
}

pub fn str_function(a: String) -> (String, String) {
    let mut expo = String::new();
    for num in a.clone().split(" ") {
        expo.push_str(num.parse::<f64>().unwrap().exp().to_string().as_str());
        expo.push(' ');
    }
    expo.pop();
    return (a.clone().into(), expo);
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut expo = Vec::<f64>::new();
    for num in b.clone() {
        expo.push((num as f64).ln().abs());
    }
    return (b.clone(), expo);
}
