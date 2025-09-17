use itertools::Itertools;

use crate::r#match::world::{
    wfc::{pattern_palette::PatternPalette, pattern_store::PatternStore, tile_grid::TileGrid},
    world_tile_type_flags::WorldTileTypeFlags,
};

pub struct SuperGrid {
    grid: Vec<Vec<WorldTileTypeFlags>>,
    pattern_palette: PatternPalette,
    pattern_store: PatternStore,
    size: [usize; 2],
}

impl SuperGrid {
    pub fn new_empty(pattern_palette: PatternPalette, size: [usize; 2]) -> Self {
        let grid = (0..size[0])
            .map(|_| {
                (0..size[1])
                    .map(|_| WorldTileTypeFlags::all())
                    .collect_vec()
            })
            .collect_vec();

        Self {
            grid,
            pattern_store: PatternStore::new(&pattern_palette, size),
            pattern_palette,
            size,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, flags: WorldTileTypeFlags) {
        let removed_flags = self.grid[x][y] ^ flags;
        self.grid[x][y] = flags;
        // self.pattern_store
        //     .disable_patterns_from_flag([x, y], flags, &self.pattern_palette);
        self.update_patterns_around([x, y], removed_flags);
    }

    pub fn update_patterns_around(
        &mut self,
        position: [usize; 2],
        removed_flags: WorldTileTypeFlags,
    ) {
        let mut positions_to_recalculate = vec![(position, removed_flags)];
        let mut updated_positions = Vec::new();

        loop {
            while let Some((position, removed_flags)) = positions_to_recalculate.pop() {
                let occurances = self
                    .pattern_palette
                    .get_type_occurances_in_patterns(removed_flags);

                for (pattern_id, flag_pos) in occurances {
                    if self.pattern_store.disable_pattern(
                        &self.pattern_palette,
                        pattern_id,
                        position,
                        flag_pos,
                    ) {
                        let pattern_size = self.pattern_palette.get_size(&pattern_id);

                        let min_x = position[0].max(flag_pos[0]) - flag_pos[0];
                        let max_x = (position[0] + pattern_size[0] - flag_pos[0]).min(self.size[0]);
                        let min_y = position[1].max(flag_pos[1]) - flag_pos[1];
                        let max_y = (position[1] + pattern_size[1] - flag_pos[1]).min(self.size[1]);

                        (min_x..max_x)
                            .cartesian_product(min_y..max_y)
                            .for_each(|(x, y)| {
                                if !updated_positions.contains(&[x, y]) {
                                    updated_positions.push([x, y]);
                                }
                            });
                    }
                }
            }

            let mut done = true;

            for grid_pos in &updated_positions {
                let possible_flags = self.grid[grid_pos[0]][grid_pos[1]]
                    & self
                        .pattern_store
                        .get_possible_flags(&self.pattern_palette, *grid_pos);
                let removed_flags = self.grid[grid_pos[0]][grid_pos[1]] ^ possible_flags;

                if removed_flags.bits().count_ones() > 0 {
                    self.grid[grid_pos[0]][grid_pos[1]] &= possible_flags;

                    done = false;
                    positions_to_recalculate.push((*grid_pos, removed_flags));
                }
            }

            // println!("Updated positions!");
            // println!("{}", self.to_tile_grid());

            if done {
                break;
            }

            updated_positions.clear();
        }
    }

    pub fn collapse_grid(&mut self) {
        // let mut step_count = 0;

        loop {
            // println!("Step {}:", step_count);
            // println!("{}", self.to_tile_grid());
            // step_count += 1;

            let pattern_store = &self.pattern_store;

            let next = self
                .grid
                .iter()
                .flatten()
                .zip(pattern_store.inverse_entropy_cache.iter().flatten())
                .enumerate()
                .filter(|(_, (flag, _))| flag.bits().count_ones() > 1)
                .max_by(|(_, (_, a)), (_, (_, b))| a.cmp(b));

            let Some((index, _)) = next else {
                break;
            };

            let x = index / self.size[1];
            let y = index % self.size[1];

            let random_flag = self
                .pattern_store
                .get_random_allowed_flag([x, y], &self.pattern_palette);

            // println!("POP: {:?}, {:?}, with flag: {:?}", x, y, random_flag);

            let removed_flags = self.grid[x][y] ^ random_flag;
            self.grid[x][y] = random_flag;

            self.update_patterns_around([x, y], removed_flags);
        }
    }

    pub fn to_tile_grid(&self) -> TileGrid {
        let data = self
            .grid
            .iter()
            .map(|column| column.iter().map(|tile| tile.get_tile_type()).collect_vec())
            .collect_vec();

        TileGrid {
            data,
            grid_size: self.size,
        }
    }
}
