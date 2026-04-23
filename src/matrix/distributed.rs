use std::collections::HashMap;

use super::layout::DistributionLayout;
use super::local::LocalMatrix;

pub struct DistributedMatrix {
    layout: DistributionLayout,
    local_tiles: HashMap<(usize, usize), LocalMatrix>,
    current_node_id: usize,
}

impl DistributedMatrix {
    pub fn new(
        global_rows: usize,
        global_cols: usize,
        tile_rows: usize,
        tile_cols: usize,
        num_nodes: usize,
        current_node_id: usize,
    ) -> Self {
        let layout =
            DistributionLayout::new(global_rows, global_cols, tile_rows, tile_cols, num_nodes);

        let num_tile_rows = layout.num_tile_rows();
        let num_tile_cols = layout.num_tile_cols();
        let num_tiles = num_tile_rows * num_tile_cols;

        let local_tiles = (current_node_id..num_tiles)
            .step_by(num_nodes)
            .map(|idx| {
                let tile_row = idx / num_tile_cols;
                let tile_col = idx % num_tile_cols;

                let (_, _, h, w) = layout.tile_bounds(tile_row, tile_col);

                ((tile_row, tile_col), LocalMatrix::zeros(h, w))
            })
            .collect();

        Self {
            layout,
            local_tiles,
            current_node_id,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.check_index_valid(i, j);

        let (tile_row, tile_col, local_i, local_j) = self.layout.locate(i, j);

        if self.layout.owner_of(tile_row, tile_col) != self.current_node_id {
            panic!("Referenced non-local tile");
        }

        let tile = self
            .local_tiles
            .get(&(tile_row, tile_col))
            .expect("Tile missing");

        tile.get(local_i, local_j)
    }

    fn check_index_valid(&self, i: usize, j: usize) {
        let (global_rows, global_cols) = self.layout.shape();
        if i >= global_rows || j >= global_cols {
            panic!("Index out of bounds");
        }
    }
}
