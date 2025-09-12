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

    pub fn set(&mut self, x: usize, y: usize, flags: WorldTileTypeFlags) {
        self.grid[x][y] = flags;
        self.recalculate_around([x, y]);
    }

    pub fn collapse_grid(&mut self) {
        let mut step_count = 0;

        loop {
            println!("Step {}:", step_count);
            println!("{}", self.to_tile_grid());
            step_count += 1;

            thread::sleep(Duration::from_secs(1));

            let next = self
                .grid
                .iter()
                .flatten()
                .enumerate()
                .sorted_by(|(_, flags_a), (_, flags_b)| {
                    flags_a
                        .bits()
                        .count_ones()
                        .cmp(&flags_b.bits().count_ones())
                })
                .filter(|(_, flags)| flags.bits().count_ones() > 1)
                .collect::<Vec<_>>();

            let Some((_, first_flag)) = next.first() else {
                break;
            };

            let next = next
                .iter()
                .filter(|(_, flags)| flags.bits().count_ones() == first_flag.bits().count_ones())
                .collect::<Vec<_>>();

            let (index, _) = next[rand::random_range(0..next.len())];

            let x = index / Y_SIZE;
            let y = index % Y_SIZE;

            println!("POP: {:?}, {:?}", x, y);

            self.pop_position([x, y]);
        }
    }

    pub fn to_tile_grid(&self) -> TileGrid<X_SIZE, Y_SIZE> {
        let data: [[WorldTileType; Y_SIZE]; X_SIZE] = self
            .grid
            .map(|column| column.map(|flag| flag.get_tile_type()));

        TileGrid { data }
    }

    fn pop_position(&mut self, position: [usize; 2]) {
        let possible = self.grid[position[0]][position[1]]
            .iter()
            .collect::<Vec<_>>();

        let new = possible[rand::random_range(0..possible.len())];

        self.grid[position[0]][position[1]] = new;

        self.recalculate_around(position);
    }

    fn recalculate_around(&mut self, position: [usize; 2]) {
        let mut updated_positions = Vec::new();

        let p_size_max = P_SIZE_X.max(P_SIZE_Y);
        let p_size_range = (p_size_max as f32 / 2.).ceil() as i32;

        for (x_offset, y_offset) in
            (-p_size_range..=p_size_range).cartesian_product(-p_size_range..=p_size_range)
        {
            if x_offset == 0 && y_offset == 0 {
                continue;
            }

            let x = position[0] as i32 + x_offset;
            let y = position[1] as i32 + y_offset;

            if x < 0 || x >= X_SIZE as i32 || y < 0 || y >= Y_SIZE as i32 {
                continue;
            }

            // if self.grid[x as usize][y as usize].bits().count_ones() <= 1 {
            //     continue;
            // }

            let new_pos = [x as usize, y as usize];

            if self.recalculate_super_position(new_pos) {
                updated_positions.push(new_pos);
            }
        }

        println!("{}", self.to_tile_grid());

        thread::sleep(Duration::from_secs(1));

        updated_positions
            .into_iter()
            .for_each(|pos| self.recalculate_around(pos));
    }

    fn recalculate_super_position(&mut self, position: [usize; 2]) -> bool {
        let alloweds = (0..P_SIZE_X)
            .cartesian_product(0..P_SIZE_Y)
            .map(|(x_offset, y_offset)| {
                let pattern_offset_x = position[0] as i32 + x_offset as i32 - P_SIZE_X as i32 + 1;
                let pattern_offset_y = position[1] as i32 + y_offset as i32 - P_SIZE_Y as i32 + 1;

                let pattern_x = P_SIZE_X - 1 - x_offset as usize;
                let pattern_y = P_SIZE_Y - 1 - y_offset as usize;

                let allowed = self
                    .patterns
                    .iter()
                    .filter(|pattern| {
                        pattern.pattern_fits(&self.grid, [pattern_offset_x, pattern_offset_y])
                    })
                    .fold(WorldTileTypeFlags::empty(), |acc, pattern| {
                        acc | pattern.get_type_at(pattern_x, pattern_y).into()
                    });

                allowed
            });

        let alloweds_rot =
            (0..P_SIZE_Y)
                .cartesian_product(0..P_SIZE_X)
                .map(|(x_offset, y_offset)| {
                    let pattern_offset_x =
                        position[0] as i32 + x_offset as i32 - P_SIZE_Y as i32 + 1;
                    let pattern_offset_y =
                        position[1] as i32 + y_offset as i32 - P_SIZE_X as i32 + 1;

                    let pattern_x = P_SIZE_Y - 1 - x_offset as usize;
                    let pattern_y = P_SIZE_X - 1 - y_offset as usize;

                    let allowed = self
                        .patterns_rot
                        .iter()
                        .filter(|pattern| {
                            pattern.pattern_fits(&self.grid, [pattern_offset_x, pattern_offset_y])
                        })
                        .fold(WorldTileTypeFlags::empty(), |acc, pattern| {
                            acc | pattern.get_type_at(pattern_x, pattern_y).into()
                        });

                    allowed
                });

        let allowed = alloweds
            .chain(alloweds_rot)
            .fold(WorldTileTypeFlags::all(), |acc, flag| acc & flag);

        let current = self.grid[position[0]][position[1]];
        let new = current & allowed;

        let is_same = current == new;
        self.grid[position[0]][position[1]] = new;

        !is_same
    }
}
