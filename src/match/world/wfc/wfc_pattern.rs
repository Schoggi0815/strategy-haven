use crate::r#match::world::world_tile_type::WorldTileType;

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
}
