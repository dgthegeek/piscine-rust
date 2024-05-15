use std::ops::{Add, Sub};
#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T> Add for Matrix<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Option<Matrix<T>>;
    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut result = vec![];
        for i in 0..self.0.len() {
            let mut row = vec![];
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] + other.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}
impl<T> Sub for Matrix<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Option<Matrix<T>>;
    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut result = vec![];
        for i in 0..self.0.len() {
            let mut row = vec![];
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] - other.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}
