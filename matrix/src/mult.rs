use crate::{Matrix, Scalar};
use std::ops::{Add, Mul};

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut ans = vec![];

        for col in &self.0 {
            ans.push(col[n])
        }
        ans
    }
}

impl<T> Mul for Matrix<T>
where
    T: Scalar<Item = T> + PartialEq + Clone + Copy + Add<Output = T> + Mul<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let matrix1 = match self.calc_row_cal() {
            Some(val) => val,
            None => return None,
        };
        let matrix2 = match rhs.calc_row_cal() {
            Some(val) => val,
            None => return None,
        };

        if matrix1.1 != matrix2.0 {
            return None;
        }

        let mut matrix: Matrix<T> = Matrix::zero(matrix1.1, matrix1.1);

        for i in 0..matrix1.1 {
            for j in 0..matrix1.1 {
                let mut temp = T::zero();

                for k in 0..matrix1.1 {
                    temp = temp + self.0[i][k] * rhs.0[k][j];
                }
                matrix.0[i][j] = temp;
            }
        }
        Some(matrix)
    }
}
