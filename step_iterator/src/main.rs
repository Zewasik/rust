use step_iterator::*;

fn main() {
    for v in StepIterator::new(0, 100, 10) {
        print!("{},", v);
    }
    println!();

    for v in StepIterator::new(0, 100, 0) {
        print!("{},", v)
    }
    println!();
}
