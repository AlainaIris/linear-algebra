use std::error::Error;

/// Matrix format
#[derive(Clone)]
pub struct Matrix {
    pub values: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}

/// # Matrix Algebra Functions
impl Matrix {
    /// Checks if the dimensions of two matricies are the same
    ///
    /// # Examples
    ///
    /// ```
    /// let m1 = Matrix {values: ..., rows: 2, cols: 3};
    /// let m2 = Matrix {values: ..., rows: 2, cols: 3};
    ///
    /// assert!(Matrix::same_dimensions(&m1, &m2));
    /// ```
    pub fn same_dimensions(m1: &Matrix, m2: &Matrix) -> bool {
        m1.rows == m2.rows && m1.cols == m2.cols
    }

    /// Checks if two matricies can be multiplied (in the order given)
    ///
    /// # Examples
    /// 
    /// ```
    /// let m1 = Matrix {values..., rows: 3, cols: 3};
    /// let m2 = Matrix {values..., rows: 1, cols: 3};
    ///
    /// assert!(!Matrix::can_multiply(&m1, &m2));
    /// assert!(Matrix::can_multiply(&m2, &m1));
    /// ```
    pub fn can_multiply(m1: &Matrix, m2: &Matrix) -> bool {
        m1.cols == m2.rows
    }

    /// Checks if two matricies are the same size and share all the
    /// same entries
    ///
    /// # Examples
    ///
    /// ```
    /// let m1 = Matrix {values: vec![vec![2.0, 3.0], vec![4.0, 3.0]], rows: 2, cols: 2};
    /// let m2 = Matrix {values: vec![vec![2.0, 3.0], vec![4.0, 3.0]], rows: 2, cols: 2};
    ///
    /// assert!(m1.equals(&m2));
    /// ```
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

    /// Add two matricies
    ///
    /// # Examples
    ///
    /// ```
    /// let m1 = Matrix ...; // Representing [2, 3, 4]
    /// let m2 = Matrix ...; // Representing [4, 5, 9]
    /// let m3 = Matrix ...; // Representing [6, 8, 13]
    /// 
    /// assert!(m1.add(&m2).expect("Unable to add").equals(&m3));
    /// ```
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

    /// Multiply two matricies
    ///
    /// # Examples
    ///
    /// ```
    /// let m1 = Matrix ...; // Representing [1, 4, 2] (1 x 3)
    /// let m2 = Matrix ...; // Representing [[5], [8], [1]] (3 x 1)
    /// let m3 = Matrix ...; // Representing [39] (1 x 1)
    ///
    /// assert!(m1.multiply(&m2).expect("Unable to multiply").equals(&m3));
    /// ```
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

    /// Pretty print for matrix
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

/// # Row Operations
impl Matrix {
    /// Apply a scalar to a row
    ///
    /// # Examples
    /// 
    /// ```
    /// let mut m1 = Matrix ... // Representing [[1, 2], [3, 4]] (2 x 2)
    /// m1.apply_row_scalar(0, 3.0);
    /// let m2 = Matrix ... // Representing [[3, 6], [3, 4]] (2 x 2)
    ///
    /// assert!(m1.equals(&m2));
    /// ```
    pub fn apply_row_scalar(&mut self, row: usize, scalar: f64) {
        for i in &mut self.values[row] {
            *i *= scalar;
        }
    }

    /// Apply a row swap
    ///
    /// # Examples
    ///
    /// ```
    /// let mut m1 = Matrix ... // Representing [[0, 0, 0], [1, 1, 1]] (2 x 3)
    /// let m2 = Matrix ... // Representing [[1, 1, 1], [0, 0, 0]] (2 x 3)
    /// m1.apply_row_swap(0, 1);
    ///
    /// assert!(m1.equals(&m2));
    /// ```
    pub fn apply_row_swap(&mut self, row1: usize, row2: usize) {
        self.values.swap(row1, row2);
    }

    /// Apply a row addition
    ///
    /// # Examples
    ///
    /// ```
    /// let mut m1 = Matrix ... // Representing [[2, 3], [4, 5]] (2 x 2)
    /// let m2 = Matrix ... // Representing [[2, 3], [6, 8]] (2 x 2)
    /// m1.apply_row_addition(1, 0);
    ///
    /// assert!(m1.equals(&m2));
    /// ```
    pub fn apply_row_addition(&mut self, row: usize, row_addend: usize) {
        for i in 0..self.cols {
            self.values[row][i] += self.values[row_addend][i];
        }
    }
}
