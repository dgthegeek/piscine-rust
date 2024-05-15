use crate::{Matrix, Scalar};
use std::ops::{ Add, Sub };

impl <T: Scalar<Item = T>> Matrix<T> {
    pub fn dim(&self) -> (usize, usize) {
        let rows = self.0.len();
        let columns = if rows > 0 { self.0[0].len() } else { 0 };
        (rows, columns)
    }

    pub fn is_same_dim(&self, other: &Matrix<T>) -> bool {
        self.dim().0 == other.dim().0 && self.dim().1 == other.dim().1
    }
}

impl <T: Scalar<Item = T> + Add<Output = T> + Clone> Add for Matrix<T> {
    type Output = Option<Matrix<T>> ;

    fn add(self, other: Self) -> Self::Output {
        if !self.is_same_dim(&other) {
            return None;
        };

        let new_data = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(row_a, row_b)| {
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .map(|(&x, &y)| x.clone() + y.clone())
                    .collect()
            })
            .collect();

        Some(Matrix(new_data))
    }
}

impl <T: Scalar<Item = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Self) -> Self::Output {
        if !self.is_same_dim(&other) {
            return None;
        };

        let new_data = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(row_a, row_b)| {
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .map(|(&x, &y)| x.clone() - y.clone())
                    .collect()
            })
            .collect();

        Some(Matrix(new_data))
    }
}