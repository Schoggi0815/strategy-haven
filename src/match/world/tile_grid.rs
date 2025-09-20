use std::fmt::Display;

use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::networking::world_tile_type::WorldTileType;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn shift(&mut self, shift: [usize; 2], new: WorldTileType) {
        self.data.iter_mut().for_each(|column| {
            *column = (0..shift[1])
                .map(|_| new.clone())
                .chain(column.iter().take(self.grid_size[1] - shift[1]).cloned())
                .collect_vec()
        });

        self.data = (0..shift[0])
            .map(|_| (0..self.grid_size[1]).map(|_| new.clone()).collect_vec())
            .chain(self.data.iter().take(self.grid_size[0] - shift[0]).cloned())
            .collect_vec();
    }

    pub fn chain_below(&self, chain: Self) -> Self {
        let new_size = [self.grid_size[0], self.grid_size[1] + chain.grid_size[1]];

        let new_grid = self
            .data
            .iter()
            .enumerate()
            .map(|(x, column)| {
                column
                    .iter()
                    .chain(chain.data[x].iter())
                    .cloned()
                    .collect_vec()
            })
            .collect_vec();

        Self {
            data: new_grid,
            grid_size: new_size,
        }
    }

    pub fn chain_right(&self, chain: Self) -> Self {
        let new_size = [self.grid_size[0] + chain.grid_size[0], self.grid_size[1]];

        let new_grid = self
            .data
            .iter()
            .chain(chain.data.iter())
            .cloned()
            .collect_vec();

        Self {
            data: new_grid,
            grid_size: new_size,
        }
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
