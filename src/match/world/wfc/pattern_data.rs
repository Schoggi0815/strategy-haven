use itertools::Itertools;

use crate::r#match::world::{
    wfc::tile_grid::TileGrid, world_tile_type::WorldTileType,
    world_tile_type_flags::WorldTileTypeFlags,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PatternData {
    size: [usize; 2],
    tiles: Vec<Vec<WorldTileType>>,
}

impl PatternData {
    pub fn new(tiles: Vec<Vec<WorldTileType>>, size: [usize; 2]) -> Self {
        Self { tiles, size }
    }

    pub fn rotation(&self) -> PatternData {
        let new_size = [self.size[1], self.size[0]];
        let mut tiles = Vec::with_capacity(new_size[0]);

        for x in 0..new_size[0] {
            tiles.push(Vec::with_capacity(new_size[1]));
            for y in 0..new_size[1] {
                tiles[x].push(self.tiles[self.size[0] - 1 - y][x]);
            }
        }

        PatternData {
            tiles,
            size: new_size,
        }
    }

    pub fn flip_x(&self) -> PatternData {
        let mut tiles = Vec::with_capacity(self.size[0]);

        for x in 0..self.size[0] {
            tiles.push(Vec::with_capacity(self.size[1]));
            for y in 0..self.size[1] {
                tiles[x].push(self.tiles[self.size[0] - 1 - x][y]);
            }
        }

        PatternData {
            tiles,
            size: self.size,
        }
    }

    pub fn get_type_at(&self, x: usize, y: usize) -> WorldTileType {
        self.tiles[x][y]
    }

    pub fn pattern_fits<const X_SIZE: usize, const Y_SIZE: usize>(
        &self,
        grid: &[[WorldTileTypeFlags; Y_SIZE]; X_SIZE],
        pattern_offset: [i32; 2],
    ) -> bool {
        for (x, y) in (0..self.size[0]).cartesian_product(0..self.size[1]) {
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

    pub fn to_grid<const X_SIZE: usize, const Y_SIZE: usize>(&self) -> TileGrid<X_SIZE, Y_SIZE> {
        TileGrid {
            data: self
                .tiles
                .iter()
                .map(|inner| inner.iter().cloned().collect_array::<Y_SIZE>().unwrap())
                .collect_array::<X_SIZE>()
                .unwrap(),
        }
    }

    pub fn get_size(&self) -> [usize; 2] {
        self.size
    }

    pub fn get_tile_type(&self, position: [usize; 2]) -> WorldTileType {
        self.tiles[position[0]][position[1]]
    }

    pub fn get_offset_arrays(&self) -> Vec<Vec<bool>> {
        (0..self.size[0])
            .map(|_| (0..self.size[1]).map(|_| true).collect_vec())
            .collect_vec()
    }

    pub fn get_tile_occurances(
        &self,
        type_flags: WorldTileTypeFlags,
    ) -> impl Iterator<Item = [usize; 2]> {
        let size = self.get_size();

        (0..size[0])
            .cartesian_product(0..size[1])
            .filter(move |(x, y)| type_flags.contains(self.get_tile_type([*x, *y]).into()))
            .map(|(x, y)| [x, y])
    }
}
