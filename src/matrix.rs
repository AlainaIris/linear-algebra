use std::error::Error;

#[derive(Clone)]
pub struct Matrix {
    pub values: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn same_dimensions(m1: &Matrix, m2: &Matrix) -> bool {
        m1.rows == m2.rows && m1.cols == m2.cols
    }

    pub fn can_multiply(m1: &Matrix, m2: &Matrix) -> bool {
        m1.cols == m2.rows
    }

    pub fn equals(&self, m: &Matrix) -> bool {
        if !Self::same_dimensions(&self, m) {
            return false;
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.values[i][j] != m.values[i][j] {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn add(&self, m: &Matrix) -> Result<Matrix, Box<dyn Error>> {
        if !Self::same_dimensions(&self, m) {
            return Err("Invalid Addition".into());
        } else {
            let mut matrix = m.clone();
            for i in 0..self.rows {
                for j in 0..self.cols {
                    matrix.values[i][j] += self.values[i][j];
                }
            }
            return Ok(matrix);
        }
    }

    pub fn multiply(&self, m: &Matrix) -> Result<Matrix, Box<dyn Error>> {
        if !Self::can_multiply(&self, m) {
            return Err("Invalid multiplication".into());
        } else {
            let mut matrix = vec![];
            let rows = self.rows;
            let cols = m.cols;
            for i in 0..self.rows {
                let mut row = vec![];
                for j in 0..m.cols {
                    let mut sum = 0.0;
                    for k in 0..self.cols {
                        sum += self.values[i][k] * m.values[k][j];
                    }
                    row.push(sum);
                }
                matrix.push(row);
            }
            Ok(Matrix {values: matrix, rows, cols})
        }
    }

    pub fn print(&self) {
        let mut text = String::new();
        for i in &self.values {
            for j in i {
                text.push_str(&j.to_string());
                text.push_str("  ");
            }
            text.push_str("\n");
        }
        print!("{}", text);
    }
}
