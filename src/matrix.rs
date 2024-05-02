use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    vec,
};

#[allow(dead_code)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Matrix {
            data: data.into(),
            rows,
            cols,
        }
    }
}

pub fn multiply<T>(a: Matrix<T>, b: Matrix<T>) -> Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    assert_eq!(a.cols, b.rows);
    let mut data = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }
    Matrix::new(data, a.rows, b.cols)
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //show [1,2,3,4,5,6] 2,3 as {1 2 3,4 5 6} and show [1,2,3,4,5,6] 3,2 as {1 2,3 4,5 6}
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i < self.rows - 1 {
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
        write!(
            f,
            "Matrix:rows={},cols={},{}",
            &self.rows, &self.cols, &self
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(a, b);
        assert_eq!(c.cols, 2);
        assert_eq!(c.rows, 2);
        assert_eq!(format!("{:?}", c), "Matrix:rows=2,cols=2,{22 28,49 64}");
    }
}
