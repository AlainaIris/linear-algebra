use crate::matrix::Matrix;

pub mod matrix;

fn main() {
    let _m: Matrix = Matrix::new(vec![vec![2.0, 3.5]]);
    _m.print();
}
