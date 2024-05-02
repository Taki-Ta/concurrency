use crate::Vector;
use anyhow::Result;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    vec,
};

#[allow(dead_code)]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix {
            data: data.into(),
            row,
            col,
        }
    }
}

pub fn multiply<T>(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    assert_eq!(a.col, b.row);
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            let a_part = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let b_data = b
                .data
                .iter()
                .skip(j)
                .step_by(b.col)
                .cloned()
                .collect::<Vec<_>>();
            let b_part = Vector::new(b_data);
            data[i * b.col + j] = dot_product(a_part, b_part)?;
        }
    }
    Ok(Matrix::new(data, a.row, b.col))
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

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //show [1,2,3,4,5,6] 2,3 as {1 2 3,4 5 6} and show [1,2,3,4,5,6] 3,2 as {1 2,3 4,5 6}
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j < self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i < self.row - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix:rows={},cols={},{}", &self.row, &self.col, &self)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    #[test]
    fn test_matrix() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(a, b)?;
        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(format!("{:?}", c), "Matrix:rows=2,cols=2,{22 28,49 64}");
        Ok(())
    }
}
