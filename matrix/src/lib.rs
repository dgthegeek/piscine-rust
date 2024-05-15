extern crate lalgebra_scalar;
pub use lalgebra_scalar::Scalar;
mod ops;
mod mult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut row_values: Vec<Vec<T>> = vec![];

        for _r in 0..row {
            let mut col_values: Vec<T> = vec![];
            for _c in 0..col {
                col_values.push(T::zero());
            }
            row_values.push(col_values);
        }
        Matrix(row_values)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut row_values: Vec<Vec<T>> = vec![];

        for r in 0..n {
            let mut col_values: Vec<T> = vec![];
            for c in 0..n {
                if c == r {
                    col_values.push(T::one());
                } else {
                    col_values.push(T::zero());
                }
            }
            row_values.push(col_values);
        }
        Matrix(row_values)
    }
}
