use std::cmp::min;

pub struct DistributionLayout {
    global_rows: usize,
    global_cols: usize,
    tile_rows: usize,
    tile_cols: usize,
    num_nodes: usize,
}

impl DistributionLayout {
    pub fn new(
        global_rows: usize,
        global_cols: usize,
        tile_rows: usize,
        tile_cols: usize,
        num_nodes: usize,
    ) -> Self {
        Self {
            global_rows,
            global_cols,
            tile_rows,
            tile_cols,
            num_nodes,
        }
    }

    pub fn tile_bounds(&self, tile_row: usize, tile_col: usize) -> (usize, usize, usize, usize) {
        let row_start = tile_row * self.tile_rows;
        let col_start = tile_col * self.tile_cols;

        let height = min(self.tile_rows, self.global_rows - row_start);
        let width = min(self.tile_cols, self.global_cols - col_start);

        (row_start, col_start, height, width)
    }

    pub fn locate(&self, i: usize, j: usize) -> (usize, usize, usize, usize) {
        let tile_row = i / self.tile_rows;
        let tile_col = j / self.tile_cols;

        let local_i = i % self.tile_rows;
        let local_j = j % self.tile_cols;

        (tile_row, tile_col, local_i, local_j)
    }

    pub fn num_tile_rows(&self) -> usize {
        self.global_rows.div_ceil(self.tile_rows)
    }

    pub fn num_tile_cols(&self) -> usize {
        self.global_cols.div_ceil(self.tile_cols)
    }

    pub fn owner_of(&self, tile_row: usize, tile_col: usize) -> usize {
        debug_assert!(tile_row < self.num_tile_rows());
        debug_assert!(tile_col < self.num_tile_cols());

        let tile_index = tile_row * self.num_tile_cols() + tile_col;
        tile_index % self.num_nodes
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.global_rows, self.global_cols)
    }
}
