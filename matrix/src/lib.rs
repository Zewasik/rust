mod mult;
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

impl<T: Scalar<Item = T> + Clone + PartialEq> Matrix<T> {
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

    pub fn calc_row_cal(&self) -> Option<(usize, usize)> {
        let row = self.0[0].len();
        let cal = self.0.len();

        for v in &self.0 {
            if v.len() != row {
                return None;
            }
        }

        Some((row, cal))
    }
}
