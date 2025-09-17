use itertools::Itertools;

use crate::r#match::world::{
    tile_grid::TileGrid, world_tile_type::WorldTileType, world_tile_type_flags::WorldTileTypeFlags,
};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct PatternData {
    size: [usize; 2],
    pub tiles: Vec<Vec<WorldTileType>>,
    pub occurrence_count: u32,
}

impl PatternData {
    pub fn new(tiles: Vec<Vec<WorldTileType>>, size: [usize; 2], occurrence_count: u32) -> Self {
        Self {
            tiles,
            size,
            occurrence_count,
        }
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
            occurrence_count: self.occurrence_count,
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
            occurrence_count: self.occurrence_count,
        }
    }

    pub fn to_grid(&self) -> TileGrid {
        TileGrid {
            data: self
                .tiles
                .iter()
                .map(|inner| inner.iter().cloned().collect_vec())
                .collect_vec(),
            grid_size: self.size,
        }
    }

    pub fn get_size(&self) -> [usize; 2] {
        self.size
    }

    pub fn get_tile_type(&self, position: [usize; 2]) -> WorldTileType {
        self.tiles[position[0]][position[1]]
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
