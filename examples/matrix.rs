use anyhow::Result;
use concurrency::Matrix;

fn main() -> Result<()> {
    let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
    let b = Matrix::new(vec![1, 2, 3, 4], 2, 2);
    let c = a * b;
    println!("{}", c); // {7 10, 15 22}
    Ok(())
}
