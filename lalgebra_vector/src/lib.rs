use std::ops::{Add, Mul};
use std::fmt::Debug;
use std::cmp::PartialEq;
use std::iter::Sum;
pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + PartialEq + Sum {}
impl<T: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + PartialEq + Sum> Scalar for T {}
pub struct Vector<T: Scalar>(pub Vec<T>);
impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        Some(self.0.iter().zip(&other.0).map(|(a, b)| *a * *b).sum())
    }
}
impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        Some(Vector(
            self.0.iter().zip(&other.0).map(|(a, b)|*a + *b).collect(),
        ))
    }
}
impl<T: Scalar> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector(self.0.clone())
    }
}
impl<T: Scalar> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Scalar> Eq for Vector<T> {}
impl<T: Scalar> Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
