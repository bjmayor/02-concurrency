use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

use anyhow::Result;

// [[1,2],[1,2],[1,2] ] => [1,2,1,2,1,2 ]

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            row: rows,
            col: cols,
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    // display a 2x3 as {1 2 3,4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                // if not the last element in the row
                if j != self.col - 1 {
                    write!(f, " ")?;
                } else {
                    // if not the last row
                    if i != self.row - 1 {
                        write!(f, ", ")?;
                    }
                }
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
        write!(f, "Matrix(row={},col={}) {}", self.row, self.col, self)
    }
}
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("Matrix size mismatch"));
    }
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_display() {
        let m = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(format!("{}", m), "{1 2 3, 4 5 6}");
    }

    #[test]
    fn test_matrix_debug() {
        let m = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(format!("{:?}", m), "Matrix(row=2,col=3) {1 2 3, 4 5 6}");
    }

    #[test]
    fn test_matrix_multiply() {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b).unwrap();
        assert_eq!(format!("{}", c), "{7 10, 15 22}");
    }
}
