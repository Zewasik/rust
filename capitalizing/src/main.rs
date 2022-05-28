use capitalizing::*;

fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}", change_case("heLLo THere"));
    println!("{}", change_case(""));
    println!("{}", capitalize_first(""));
    println!("{}", title_case(""));
}
