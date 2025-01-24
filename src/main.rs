use crate::matrix::Matrix;

pub mod matrix;

fn m1() -> Matrix {
    Matrix {
        values: vec![
            vec![2.0, 3.0],
            vec![4.0, 5.0]
        ],
        rows: 2,
        cols: 2
    }
}

fn m2() -> Matrix {
    Matrix {
        values: vec![
            vec![-5.0, 2.0],
            vec![5.0, -1.0]
        ],
        rows: 2,
        cols: 2
    }
}

fn m3() -> Matrix {
    Matrix {
        values: vec![
            vec![4.0, 3.0, 8.0],
            vec![-1.0, 0.0, 3.0],
            vec![5.0, -7.0, -4.0]
        ],
        cols: 3,
        rows: 3
    }
}

fn m4() -> Matrix {
    Matrix {
        values: vec![
            vec![4.0],
            vec![3.0],
            vec![5.0],
        ],
        cols: 1,
        rows: 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_dimensions_test() {
        let m1 = m1();
        let m2 = m2();
        let m3 = m3();
        assert!(Matrix::same_dimensions(&m1, &m2));
        assert!(!Matrix::same_dimensions(&m2, &m3));
    }
    
    #[test]
    fn add_test() {
        let m1 = m1();
        let m2 = m2();
        assert!(m1.add(&m2).expect("").equals(&Matrix { values: vec![
            vec![-3.0, 5.0],
            vec![9.0, 4.0],
        ],
            cols: 2,
            rows: 2
        }));
    }

    #[test]
    fn can_multiply() {
        let m1 = m1();
        let m2 = m2();
        let m3 = m3();
        let m4 = m4();
        assert!(!Matrix::can_multiply(&m4, &m3));
        assert!(!Matrix::can_multiply(&m2, &m3));
        assert!(Matrix::can_multiply(&m1, &m2));
        assert!(Matrix::can_multiply(&m3, &m4));
    }

    #[test]
    fn mult_test() {
        let m1 = m1();
        let m2 = m2();
        let m3 = m3();
        let m4 = m4();
        assert!(m1.multiply(&m2).expect("Something went wrong with this multiplication").equals(&Matrix { values: vec![
            vec![5.0, 1.0],
            vec![5.0, 3.0]
        ],
            cols: 2,
            rows: 2
        }));
        assert!(m3.multiply(&m4).expect("Something went wrong with this multiplicaton").equals(&Matrix { values: vec![
            vec![65.0],
            vec![11.0],
            vec![-21.0],
        ],
        cols: 1,
        rows: 3,
        }));
    }

    #[test]
    fn row_scale_test() {
        let mut m1 = m1();
        let mut m2 = m2();
        m1.apply_row_scalar(0, 2.0);
        m2.apply_row_scalar(1, -1.0);
        assert!(m1.equals(&Matrix {values: vec![
            vec![4.0, 6.0],
            vec![4.0, 5.0]
        ], rows: 2, cols: 2}));
        assert!(m2.equals(&Matrix {values: vec![
            vec![-5.0, 2.0],
            vec![-5.0, 1.0]
        ], rows: 2, cols: 2}));
    }

    #[test]
    fn row_swap_test() {
        let mut m1 = m1();
        let mut m2 = m2();
        m1.apply_row_swap(0, 1);
        m2.apply_row_swap(1, 0);

        assert!(m1.equals(&Matrix {values: vec![
            vec![4.0, 5.0],
            vec![2.0, 3.0]
        ], rows: 2, cols: 2}));
        assert!(m2.equals(&Matrix {values: vec![
            vec![5.0, -1.0],
            vec![-5.0, 2.0]
        ], rows: 2, cols: 2}));
    }

    #[test]
    fn row_addition_test() {
        let mut m1 = m1();
        let mut m2 = m2();
        m1.apply_row_addition(1, 0);
        m2.apply_row_addition(0, 1);

        assert!(m1.equals(&Matrix {values: vec![
            vec![2.0, 3.0],
            vec![6.0, 8.0]
        ], rows: 2, cols: 2}));
        assert!(m2.equals(&Matrix {values: vec![
            vec![0.0, 1.0],
            vec![5.0, -1.0]
        ], rows: 2, cols: 2}));
    }
}
