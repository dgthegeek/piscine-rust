pub use lalgebra_scalar;
use core::ops::{ Add, Mul };
use std::fmt;

pub trait Scalar: Sized + Copy + Add<Self, Output = Self> + Mul<Self, Output = Self> {
    // Définissez ici les fonctions spécifiques à Scalar, telles que zero() et one()
    fn zero() -> Self;
    fn one() -> Self;
}
impl Scalar for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut result = self.clone();
        for (i, value) in rhs.0.into_iter().enumerate() {
            result.0[i] = result.0[i] + value;
        }
        Some(result)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = T::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            result = result + (*a * *b);
        }
        Some(result)
    }
}

// Implement the Mul trait for Vector to allow scalar multiplication
impl<T: Scalar> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        let mut result = self.clone();
        for value in result.0.iter_mut() {
            *value = *value * scalar;
        }
        result
    }
}

// Implement the Display trait for Vector to allow printing
impl<T: fmt::Display + Scalar> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector([")?;
        for (i, val) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", val)?;
        }
        write!(f, "])")
    }
}