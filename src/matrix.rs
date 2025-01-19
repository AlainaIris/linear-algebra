pub struct Matrix {
    pub values: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(values: Vec<Vec<f64>>) -> Self {
        Matrix {values}
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
