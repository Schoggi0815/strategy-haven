use itertools::Itertools;

use crate::r#match::world::{
    wfc::{pattern::Pattern, tile_grid::TileGrid},
    world_tile_type::WorldTileType,
    world_tile_type_flags::WorldTileTypeFlags,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PatternData<const P_SIZE_X: usize, const P_SIZE_Y: usize> {
    tiles: [[WorldTileType; P_SIZE_Y]; P_SIZE_X],
}

impl<const P_SIZE_X: usize, const P_SIZE_Y: usize> PatternData<P_SIZE_X, P_SIZE_Y> {
    pub fn new(tiles: [[WorldTileType; P_SIZE_Y]; P_SIZE_X]) -> Self {
        Self { tiles }
    }

    pub fn rotation(&self) -> PatternData<P_SIZE_Y, P_SIZE_X> {
        let mut tiles = [[WorldTileType::Water; P_SIZE_X]; P_SIZE_Y];

        for y in 0..P_SIZE_X {
            for x in 0..P_SIZE_Y {
                tiles[x][y] = self.tiles[P_SIZE_X - 1 - y][x];
            }
        }

        PatternData { tiles }
    }

    pub fn get_type_at(&self, x: usize, y: usize) -> WorldTileType {
        self.tiles[x][y]
    }

    pub fn pattern_fits<const X_SIZE: usize, const Y_SIZE: usize>(
        &self,
        grid: &[[WorldTileTypeFlags; Y_SIZE]; X_SIZE],
        pattern_offset: [i32; 2],
    ) -> bool {
        for (x, y) in (0..P_SIZE_X).cartesian_product(0..P_SIZE_Y) {
            let grid_x = x as i32 + pattern_offset[0];
            let grid_y = y as i32 + pattern_offset[1];

            if grid_x < 0 || grid_x >= X_SIZE as i32 || grid_y < 0 || grid_y >= Y_SIZE as i32 {
                continue;
            }

            let tile_type_flags = grid[grid_x as usize][grid_y as usize];

            let tile_type = self.tiles[x][y];
            if !tile_type_flags.contains(tile_type.into()) {
                return false;
            }
        }

        true
    }

    pub fn to_grid(&self) -> TileGrid<P_SIZE_X, P_SIZE_Y> {
        TileGrid { data: self.tiles }
    }
}

impl<const P_SIZE_X: usize, const P_SIZE_Y: usize> Pattern for PatternData<P_SIZE_X, P_SIZE_Y> {
    fn get_size(&self) -> [usize; 2] {
        [P_SIZE_X, P_SIZE_Y]
    }

    fn get_tile_type(&self, position: [usize; 2]) -> WorldTileType {
        self.tiles[position[0]][position[1]]
    }
}
