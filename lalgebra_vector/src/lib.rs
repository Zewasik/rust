use std::ops::{Add, Div, Mul, Sub};
pub trait Scalar: Sized {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
impl<T> Scalar for T
where
    T: From<u8> + PartialEq + Add + Sub + Mul + Div,
{
    type Item = T;
    fn zero() -> Self::Item {
        T::from(0)
    }
    fn one() -> Self::Item {
        T::from(1)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);
impl<
        T: Clone
            + Scalar
            + std::ops::Mul<Output = T>
            + std::ops::Add<Output = T>
            + std::cmp::PartialEq
            + Scalar<Item = T>,
    > Vector<T>
{
    pub fn new() -> Self {
        Vector(Vec::<T>::new())
    }
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() == other.0.len() {
            Some({
                let mut temp = T::zero();

                for i in 0..self.0.len() {
                    temp = temp + self.0[i].clone() * other.0[i].clone()
                }

                temp
            })
        } else {
            None
        }
    }
}
impl<
        T: Clone
            + Scalar
            + std::ops::Add<Output = T>
            + std::cmp::PartialEq
            + Mul<Output = T>
            + Scalar<Item = T>,
    > Add for Vector<T>
{
    type Output = Option<Vector<T>>;
    fn add(self, rhs: Self) -> Option<Vector<T>> {
        if self.0.len() == rhs.0.len() {
            let mut new: Vector<T> = Vector::new();
            for index in 0..self.0.len() {
                new.0.push(self.0[index].clone() + rhs.0[index].clone())
            }
            Some(new)
        } else {
            None
        }
    }
}
