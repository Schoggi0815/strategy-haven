use itertools::Itertools;

use crate::r#match::world::{
    world_tile_type::WorldTileType, world_tile_type_flags::WorldTileTypeFlags,
};

pub struct WfcPattern {
    tiles: [WorldTileType; 9],
}

impl WfcPattern {
    pub fn new_from_slices(
        x_slice_1: &[WorldTileType],
        x_slice_2: &[WorldTileType],
        x_slice_3: &[WorldTileType],
    ) -> Self {
        Self {
            tiles: [
                x_slice_1[0],
                x_slice_2[0],
                x_slice_3[0],
                x_slice_1[1],
                x_slice_2[1],
                x_slice_3[1],
                x_slice_1[2],
                x_slice_2[2],
                x_slice_3[2],
            ],
        }
    }

    pub fn rotation(&self) -> Self {
        Self {
            tiles: [
                self.tiles[0],
                self.tiles[3],
                self.tiles[6],
                self.tiles[1],
                self.tiles[4],
                self.tiles[7],
                self.tiles[2],
                self.tiles[5],
                self.tiles[8],
            ],
        }
    }

    pub fn get_type_at(&self, x: usize, y: usize) -> WorldTileType {
        self.tiles[x + y * 3]
    }

    pub fn pattern_fits(&self, slices: &[&[WorldTileTypeFlags]]) -> bool {
        for (x, y) in (0..3).cartesian_product(0..3) {
            let Some(tile_type_flags) = slices.get(x).and_then(|column| column.get(y)) else {
                continue;
            };

            let index = x + (y * 3);

            let tile_type = self.tiles[index];
            if !tile_type_flags.contains(tile_type.into()) {
                return false;
            }
        }

        true
    }
}
