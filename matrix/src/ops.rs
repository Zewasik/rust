use crate::{Matrix, Scalar};
pub use std::ops::{Add, Sub};

impl<T> Add for Matrix<T>
where
    T: Scalar<Item = T> + PartialEq + Clone + Copy + Add<Output = T>,
{
    fn add(self, rhs: Self) -> Self::Output {
        let mut matrix = self.clone();
        let matrix1 = match self.calc_row_cal() {
            Some(val) => val,
            None => return None,
        };
        let matrix2 = match rhs.calc_row_cal() {
            Some(val) => val,
            None => return None,
        };
        if matrix1.0 != matrix2.0 || matrix1.1 != matrix2.1 {
            return None;
        }
        for (i, v1) in self.0.iter().enumerate() {
            for (j, num) in v1.iter().enumerate() {
                matrix.0[i][j] = *num + rhs.0[i][j]
            }
        }

        Some(matrix)
    }

    type Output = Option<Matrix<T>>;
}

impl<T> Sub for Matrix<T>
where
    T: Scalar<Item = T> + PartialEq + Clone + Copy + Sub<Output = T>,
{
    fn sub(self, rhs: Self) -> Self::Output {
        let mut matrix = self.clone();
        let matrix1 = match self.calc_row_cal() {
            Some(val) => val,
            None => return None,
        };
        let matrix2 = match rhs.calc_row_cal() {
            Some(val) => val,
            None => return None,
        };
        if matrix1.0 != matrix2.0 || matrix1.1 != matrix2.1 {
            return None;
        }
        for (i, v1) in self.0.iter().enumerate() {
            for (j, num) in v1.iter().enumerate() {
                matrix.0[i][j] = *num - rhs.0[i][j]
            }
        }

        Some(matrix)
    }

    type Output = Option<Matrix<T>>;
}
