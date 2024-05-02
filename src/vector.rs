use anyhow::{Ok, Result};
use std::ops::{AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Vector { data: data.into() }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Default + Mul<Output = T> + AddAssign + Copy,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("dot product error: a.len()!=b.len()"));
    }
    let mut res = T::default();
    for i in 0..a.len() {
        res += a[i] * b[i];
    }
    Ok(res)
}
