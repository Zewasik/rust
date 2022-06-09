use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
