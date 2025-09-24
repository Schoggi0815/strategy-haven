use crate::{game::world_tile_type::WorldTileType, r#match::world::tile_grid::TileGrid};

#[derive(PartialEq, Eq, Clone)]
pub struct Pattern {
    pub tiles: Vec<Vec<WorldTileType>>,
    pub size: [usize; 2],
}

impl Pattern {
    pub fn get_grid(&self) -> TileGrid {
        TileGrid {
            data: self.tiles.clone(),
            grid_size: [self.tiles.len(), self.tiles[0].len()],
        }
    }

    pub fn get_rotation(&self) -> Self {
        let new_size = [self.size[1], self.size[0]];
        let mut tiles = Vec::with_capacity(new_size[0]);

        for x in 0..new_size[0] {
            tiles.push(Vec::with_capacity(new_size[1]));
            for y in 0..new_size[1] {
                tiles[x].push(self.tiles[self.size[0] - 1 - y][x]);
            }
        }

        Pattern {
            tiles,
            size: new_size,
        }
    }

    pub fn flip_x(&self) -> Self {
        let mut tiles = Vec::with_capacity(self.size[0]);

        for x in 0..self.size[0] {
            tiles.push(Vec::with_capacity(self.size[1]));
            for y in 0..self.size[1] {
                tiles[x].push(self.tiles[self.size[0] - 1 - x][y]);
            }
        }

        Pattern {
            tiles,
            size: self.size,
        }
    }

    pub fn flip_y(&self) -> Self {
        let mut tiles = Vec::with_capacity(self.size[0]);

        for x in 0..self.size[0] {
            tiles.push(Vec::with_capacity(self.size[1]));
            for y in 0..self.size[1] {
                tiles[x].push(self.tiles[x][self.size[1] - 1 - y]);
            }
        }

        Pattern {
            tiles,
            size: self.size,
        }
    }
}
