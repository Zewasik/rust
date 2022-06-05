use edit_distance::*;

fn main() {
    let source = "nill";
    let target = "kigger";
    println!(
        "It's necessary to make {} change(s) to {}, to get {}",
        edit_distance(source, target),
        source,
        target
    );
}
