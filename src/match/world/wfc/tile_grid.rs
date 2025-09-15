use std::fmt::Display;

use colored::Colorize;
use itertools::Itertools;

use crate::r#match::world::{wfc::pattern_data::PatternData, world_tile_type::WorldTileType};

#[derive(Debug)]
pub struct TileGrid<const X_SIZE: usize, const Y_SIZE: usize> {
    pub data: [[WorldTileType; Y_SIZE]; X_SIZE],
}

impl<const X_SIZE: usize, const Y_SIZE: usize> TileGrid<X_SIZE, Y_SIZE> {
    pub fn new_filled(tile_type: WorldTileType) -> Self {
        Self {
            data: [[tile_type; Y_SIZE]; X_SIZE],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile_type: WorldTileType) {
        self.data[x][y] = tile_type;
    }

    pub fn get(&self, x: usize, y: usize) -> WorldTileType {
        self.data[x][y]
    }

    pub fn get_patterns<const P_SIZE_X: usize, const P_SIZE_Y: usize>(&self) -> Vec<PatternData> {
        let all: Vec<_> = (0..X_SIZE - P_SIZE_X)
            .cartesian_product(0..Y_SIZE - P_SIZE_Y)
            .flat_map(|(x, y)| {
                let pattern_array = (0..P_SIZE_X)
                    .map(|pattern_x| {
                        (0..P_SIZE_Y)
                            .map(|pattern_y| self.data[x + pattern_x][y + pattern_y])
                            .collect_vec()
                    })
                    .collect_vec();

                let pattern = PatternData::new(pattern_array, [P_SIZE_X, P_SIZE_Y]);
                let pattern_rot1 = pattern.rotation();
                let pattern_rot2 = pattern_rot1.rotation();
                let pattern_rot3 = pattern_rot2.rotation();
                [
                    pattern.flip_x(),
                    pattern_rot2.flip_x(),
                    pattern_rot1.flip_x(),
                    pattern_rot3.flip_x(),
                    pattern,
                    pattern_rot2,
                    pattern_rot1,
                    pattern_rot3,
                ]
            })
            .sorted()
            .dedup()
            .collect_vec();

        all
    }
}

impl<const X_SIZE: usize, const Y_SIZE: usize> Display for TileGrid<X_SIZE, Y_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                let tile = self.data[x][y];

                let color = tile.get_color().to_linear();
                let colored = format!("{}", tile.name()).truecolor(
                    (color.red * 255.) as u8,
                    (color.green * 255.) as u8,
                    (color.blue * 255.) as u8,
                );
                write!(f, "{} ", colored)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
