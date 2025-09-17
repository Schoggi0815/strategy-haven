use crate::r#match::world::{tile_grid::TileGrid, world_tile_type::WorldTileType};

#[derive(PartialEq, Eq)]
pub struct Pattern {
    pub tiles: Vec<Vec<WorldTileType>>,
}

impl Pattern {
    pub fn get_grid(&self) -> TileGrid {
        TileGrid {
            data: self.tiles.clone(),
            grid_size: [self.tiles.len(), self.tiles[0].len()],
        }
    }
}
