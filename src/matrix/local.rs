pub struct LocalMatrix {
    data: Vec<Vec<f64>>,
}

impl LocalMatrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let zero_matrix = vec![vec![0.0; cols]; rows];
        Self { data: zero_matrix }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i][j]
    }
}

