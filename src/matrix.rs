use crate::{dot_product, Vector};
use anyhow::{Ok, Result};
use oneshot::channel;
use std::{
    fmt,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread, vec,
};

const NUM_PRODUCTERS: usize = 4;

#[allow(dead_code)]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    output: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, output: oneshot::Sender<MsgOutput<T>>) -> Self {
        Msg { input, output }
    }
}

impl<T> MsgOutput<T> {
    fn new(idx: usize, value: T) -> Self {
        MsgOutput { idx, value }
    }
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        MsgInput { idx, row, col }
    }
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
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign + Send + 'static,
{
    assert_eq!(a.col, b.row);
    //为每个任务 创建channel、和线程
    //在线程内创建oneshot channel，在channel send时 将线程内创建的oneshot channel的tx传入到线程中，保存rx接收运算结果
    let senders = (0..NUM_PRODUCTERS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    //接收msg 调用dot_product 使用output send回去
                    let res = dot_product(msg.input.row, msg.input.col)?;
                    let output = MsgOutput::new(msg.input.idx, res);
                    println!("thread {} finished!", &output.idx);
                    if let Err(err) = msg.output.send(output) {
                        eprintln!("send error:{}", err);
                    }
                }
                Ok(())
            });
            tx
        })
        .collect::<Vec<_>>();
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
            let (shot_tx, shot_rx) = channel();
            let msg: Msg<T> = Msg::new(MsgInput::new(i * b.col + j, a_part, b_part), shot_tx);
            let res = senders[i % NUM_PRODUCTERS].send(msg);
            if let Err(err) = res {
                eprintln!("send error:{}", err);
            } else {
                let output = shot_rx.recv()?;
                data[i * b.col + j] = output.value;
            }
        }
    }
    Ok(Matrix::new(data, a.row, b.col))
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        std::fmt::Result::Ok(())
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
