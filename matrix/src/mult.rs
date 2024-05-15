use crate::{Matrix, Scalar};
use std::ops::Mul;

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
        self.dim().1
	}

	pub fn number_of_rows(&self) -> usize {
        self.dim().0
	}

	pub fn row(&self, n: usize) -> Vec<T> {
        if self.number_of_rows() > n {
            self.0[n].clone()
        } else {
            Vec::new()
        }
	}

	pub fn col(&self, n: usize) -> Vec<T> {
        let mut cols = Vec::new();
        for r in self.0.iter() {
            if self.number_of_cols() > n {
                cols.push(r[n]);
            } else {
                return Vec::new();
            }
        };
        cols
	}
}

impl <T: Scalar<Item = T> + Mul<Output = T>+ Clone> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows()  {
            return None;
        };

        let mut result = Vec::new();

        for row in 0..self.number_of_rows() {
            let mut columns = Vec::new();
            for col in 0..self.number_of_cols() {
                let mut sum = T::zero();
                
                for k in 0..self.number_of_cols() {            
                    sum = sum.clone()  + self.row(row)[k] * other.col(col)[k];
                };
                columns.push(sum);
            }
            result.push(columns);
        };

        Some(Matrix(result))
    }
}
