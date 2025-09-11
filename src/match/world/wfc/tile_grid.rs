use itertools::Itertools;

use crate::r#match::world::{wfc::wfc_pattern::WfcPattern, world_tile_type::WorldTileType};

pub struct TileGrid<const X_SIZE: usize, const Y_SIZE: usize> {
    pub data: [[WorldTileType; Y_SIZE]; X_SIZE],
}

impl<const X_SIZE: usize, const Y_SIZE: usize> TileGrid<X_SIZE, Y_SIZE> {
    pub fn new_filled(tile_type: WorldTileType) -> Self {
        Self {
            data: [[tile_type; Y_SIZE]; X_SIZE],
        }
    }

    pub fn get_patterns(&self) -> Vec<WfcPattern> {
        (1..X_SIZE - 1)
            .cartesian_product(1..Y_SIZE - 1)
            .flat_map(|(x, y)| {
                let slice_1 = self.data[x - 1][y - 1..y + 1];
                let slice_2 = self.data[x][y - 1..y + 1];
                let slice_3 = self.data[x + 1][y - 1..y + 1];

                let pattern = WfcPattern::new_from_slices(&slice_1, &slice_2, &slice_3);
                let pattern_rot1 = pattern.rotation();
                let pattern_rot2 = pattern_rot1.rotation();
                let pattern_rot3 = pattern_rot2.rotation();
                [pattern, pattern_rot1, pattern_rot2, pattern_rot3]
            })
            .collect::<Vec<_>>()
    }
}
