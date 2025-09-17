use std::fmt::Display;

use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::r#match::world::{wfc::pattern_data::PatternData, world_tile_type::WorldTileType};

#[derive(Debug, Serialize, Deserialize)]
pub struct TileGrid {
    pub data: Vec<Vec<WorldTileType>>,
    pub grid_size: [usize; 2],
}

impl TileGrid {
    pub fn new_filled(tile_type: WorldTileType, grid_size: [usize; 2]) -> Self {
        Self {
            grid_size,
            data: (0..grid_size[0])
                .map(|_| (0..grid_size[1]).map(|_| tile_type).collect_vec())
                .collect_vec(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile_type: WorldTileType) {
        self.data[x][y] = tile_type;
    }

    pub fn get(&self, x: usize, y: usize) -> WorldTileType {
        self.data[x][y]
    }

    pub fn resize(&mut self, new_size: [usize; 2]) {
        self.grid_size = new_size;

        self.data
            .iter_mut()
            .for_each(|column| column.resize(new_size[1], WorldTileType::Water));

        self.data.resize(
            new_size[0],
            (0..new_size[1]).map(|_| WorldTileType::Water).collect_vec(),
        );
    }

    pub fn get_patterns<const P_SIZE_X: usize, const P_SIZE_Y: usize>(&self) -> Vec<PatternData> {
        let all: Vec<_> = (0..self.grid_size[0] - P_SIZE_X)
            .cartesian_product(0..self.grid_size[1] - P_SIZE_Y)
            .flat_map(|(x, y)| {
                let pattern_array = (0..P_SIZE_X)
                    .map(|pattern_x| {
                        (0..P_SIZE_Y)
                            .map(|pattern_y| self.data[x + pattern_x][y + pattern_y])
                            .collect_vec()
                    })
                    .collect_vec();

                let pattern = PatternData::new(pattern_array, [P_SIZE_X, P_SIZE_Y], 1);
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
            .sorted_by(|a, b| a.tiles.cmp(&b.tiles))
            .fold(Vec::new(), |mut acc, current| {
                let Some(last) = acc.last_mut() else {
                    acc.push(current);
                    return acc;
                };

                if current.tiles != last.tiles {
                    acc.push(current);
                    return acc;
                }

                last.occurrence_count += 1;
                acc
            });

        all
    }
}

impl Display for TileGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<4}", "")?;

        for x in 0..self.grid_size[0] {
            write!(f, "{:<6}", x)?;
        }

        writeln!(f)?;

        for y in 0..self.grid_size[1] {
            write!(f, "{:<4}", y)?;

            for x in 0..self.grid_size[0] {
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
