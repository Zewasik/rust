use crate::{Matrix, Scalar};
use std::ops::{Add, Sub};

impl<T> Add for Matrix<T>
where
    T: PartialEq + Clone + Copy + Add<Output = T>,
{
    fn add(self, rhs: Self) -> Self::Output {
        let mut matrix = self.clone();
        if self.0.len() == rhs.0.len() {
            for (i, v1) in self.0.iter().enumerate() {
                if self.0[i].len() != rhs.0[i].len() {
                    return None;
                }
                for (j, num) in v1.iter().enumerate() {
                    matrix.0[i][j] = *num + rhs.0[i][j]
                }
            }
        } else {
            return None;
        }

        Some(matrix)
    }

    type Output = Option<Matrix<T>>;
}

impl<T> Sub for Matrix<T>
where
    T: PartialEq + Clone + Copy + Sub<Output = T>,
{
    fn sub(self, rhs: Self) -> Self::Output {
        let mut matrix = self.clone();
        if self.0.len() == rhs.0.len() {
            for (i, v1) in self.0.iter().enumerate() {
                if self.0[i].len() != rhs.0[i].len() {
                    return None;
                }
                for (j, num) in v1.iter().enumerate() {
                    matrix.0[i][j] = *num - rhs.0[i][j]
                }
            }
        } else {
            return None;
        }

        Some(matrix)
    }

    type Output = Option<Matrix<T>>;
}
