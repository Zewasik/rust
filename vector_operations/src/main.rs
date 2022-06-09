use vector_operations::ThreeDVector;

fn main() {
    let a = ThreeDVector {
        i: 3.0,
        j: 5.0,
        k: 2.0,
    };
    let b = ThreeDVector {
        i: 2.0,
        j: 7.0,
        k: 4.0,
    };
    println!("{:?}", a + b);
}
