use std::time::Duration;

use bevy::platform::thread;
use itertools::Itertools;

use crate::r#match::world::{
    wfc::{pattern_palette::PatternPalette, super_tile::SuperTile, tile_grid::TileGrid},
    world_tile_type::WorldTileType,
    world_tile_type_flags::WorldTileTypeFlags,
};

pub struct SuperGrid<const X_SIZE: usize, const Y_SIZE: usize> {
    grid: [[SuperTile; Y_SIZE]; X_SIZE],
    pattern_palette: PatternPalette,
}

impl<const X_SIZE: usize, const Y_SIZE: usize> SuperGrid<X_SIZE, Y_SIZE> {
    pub fn new_empty(pattern_palette: PatternPalette) -> Self {
        let grid = (0..X_SIZE)
            .map(|_| {
                (0..Y_SIZE)
                    .map(|_| SuperTile::new(&pattern_palette))
                    .collect_array::<Y_SIZE>()
                    .unwrap()
            })
            .collect_array()
            .unwrap();

        Self {
            grid,
            pattern_palette,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &SuperTile {
        &self.grid[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, flags: WorldTileTypeFlags) {
        let removed_flags = self.grid[x][y].set_flag(flags, &self.pattern_palette);
        self.update_patterns_around([x, y], removed_flags);
    }

    pub fn update_patterns_around(
        &mut self,
        position: [usize; 2],
        removed_flags: WorldTileTypeFlags,
    ) {
        let mut updated_positions = Vec::new();

        let occurances = self
            .pattern_palette
            .get_occurances::<X_SIZE, Y_SIZE>(removed_flags, position);

        for (grid_pos, pattern_id, offset_pos) in occurances.iter() {
            if self.grid[grid_pos[0]][grid_pos[1]].disable_pattern(*pattern_id, *offset_pos) > 0
                && !updated_positions.contains(grid_pos)
            {
                updated_positions.push(*grid_pos);
            }
        }

        for grid_pos in updated_positions {
            let removed_flags = self.grid[grid_pos[0]][grid_pos[1]]
                .recalculate_possible_flags(&self.pattern_palette);

            if removed_flags.bits().count_ones() > 0 {
                self.update_patterns_around(grid_pos, removed_flags);
            }
        }
    }

    pub fn collapse_grid(&mut self) {
        // let mut step_count = 0;

        loop {
            // println!("Step {}:", step_count);
            // println!("{}", self.to_tile_grid());
            // step_count += 1;

            // thread::sleep(Duration::from_secs(1));

            let next = self
                .grid
                .iter()
                .flatten()
                .enumerate()
                .sorted_by(|(_, flags_a), (_, flags_b)| flags_a.entropy().cmp(&flags_b.entropy()))
                .filter(|(_, flags)| flags.get_type_count() > 1)
                .collect::<Vec<_>>();

            let Some((index, _)) = next.first() else {
                break;
            };

            let x = index / Y_SIZE;
            let y = index % Y_SIZE;

            // println!("POP: {:?}, {:?}", x, y);

            let removed_flags = self.grid[x][y].pop_random_pattern(&self.pattern_palette);
            self.update_patterns_around([x, y], removed_flags);
        }
    }

    pub fn to_tile_grid(&self) -> TileGrid<X_SIZE, Y_SIZE> {
        let data: [[WorldTileType; Y_SIZE]; X_SIZE] = self
            .grid
            .iter()
            .map(|column| {
                column
                    .iter()
                    .map(|tile| tile.to_tile_type())
                    .collect_array::<Y_SIZE>()
                    .unwrap()
            })
            .collect_array::<X_SIZE>()
            .unwrap();

        TileGrid { data }
    }
}
