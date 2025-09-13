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

    pub fn get_patterns<const P_SIZE_X: usize, const P_SIZE_Y: usize>(
        &self,
    ) -> (
        Vec<PatternData<P_SIZE_X, P_SIZE_Y>>,
        Vec<PatternData<P_SIZE_Y, P_SIZE_X>>,
    ) {
        let (all, all_rotated): (Vec<_>, Vec<_>) = (0..X_SIZE - P_SIZE_X)
            .cartesian_product(0..Y_SIZE - P_SIZE_Y)
            .map(|(x, y)| {
                let mut pattern_array = [[WorldTileType::Water; P_SIZE_Y]; P_SIZE_X];

                for (pattern_x, pattern_y) in (0..P_SIZE_X).cartesian_product(0..P_SIZE_Y) {
                    pattern_array[pattern_x][pattern_y] = self.data[x + pattern_x][y + pattern_y];
                }

                let pattern = PatternData::new(pattern_array);
                let pattern_rot1 = pattern.rotation();
                let pattern_rot2 = pattern_rot1.rotation();
                let pattern_rot3 = pattern_rot2.rotation();
                ([pattern, pattern_rot2], [pattern_rot1, pattern_rot3])
            })
            .unzip();

        let mut all = all.into_flattened();
        all.sort();
        all.dedup();
        let mut all_rotated = all_rotated.into_flattened();
        all_rotated.sort();
        all_rotated.dedup();

        (all, all_rotated)
    }

    pub fn get_patterns_square<const P_SIZE: usize>(&self) -> Vec<PatternData<P_SIZE, P_SIZE>> {
        (0..X_SIZE - P_SIZE)
            .cartesian_product(0..Y_SIZE - P_SIZE)
            .flat_map(|(x, y)| {
                let mut pattern_array = [[WorldTileType::Water; P_SIZE]; P_SIZE];

                for (pattern_x, pattern_y) in (0..P_SIZE).cartesian_product(0..P_SIZE) {
                    pattern_array[pattern_x][pattern_y] = self.data[x + pattern_x][y + pattern_y];
                }

                let pattern = PatternData::new(pattern_array);
                let pattern_rot1 = pattern.rotation();
                let pattern_rot2 = pattern_rot1.rotation();
                let pattern_rot3 = pattern_rot2.rotation();
                [pattern, pattern_rot2, pattern_rot1, pattern_rot3]
            })
            .sorted()
            .dedup()
            .collect_vec()
    }
}

impl<const X_SIZE: usize, const Y_SIZE: usize> Display for TileGrid<X_SIZE, Y_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                let tile = self.data[x][y];

                let color = tile.get_color().to_linear();
                let colored = format!("{:?}", tile).truecolor(
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
