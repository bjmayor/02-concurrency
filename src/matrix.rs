use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use anyhow::Result;

use crate::{dot_product, Vector};

const NUM_THREADS: usize = 4;
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

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, output: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, output }
    }
}
pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    output: oneshot::Sender<MsgOutput<T>>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("Matrix size mismatch"));
    }
    // generate 4 threads which will calculate the dot product of each row and column

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col).unwrap();
                    if let Err(e) = msg.output.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Error: {:? }", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Error: {:?}", e);
            }
            receivers.push(rx);
        }
    }
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default + Send + 'static,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiplication failed")
    }
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
        let c = a * b;
        assert_eq!(format!("{}", c), "{7 10, 15 22}");
    }

    #[test]
    fn test_dot_product() {
        let a = Vector::new(vec![1, 2, 3]);
        let b = Vector::new(vec![4, 5, 6]);
        let c = dot_product(a, b).unwrap();
        assert_eq!(c, 32);
    }

    #[test]
    fn test_a_can_not_multiply_b() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b);
        assert!(c.is_err());
    }
    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let _ = a * b;
    }
}
