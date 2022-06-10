// use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar: Sized {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl<T> Scalar for T
where
    T: From<u8>,
{
    type Item = T;

    fn zero() -> Self::Item {
        T::from(0)
    }

    fn one() -> Self::Item {
        T::from(1)
    }
}
