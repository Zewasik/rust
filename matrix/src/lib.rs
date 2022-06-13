mod ops;
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Self::zero(n, n);
        for i in 0..n {
            matrix.0[i][i] = T::one();
        }
        matrix
    }
}
