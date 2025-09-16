use itertools::Itertools;

use crate::r#match::world::{
    wfc::pattern_palette::{PatternArray, PatternId, PatternPalette},
    world_tile_type_flags::WorldTileTypeFlags,
};

pub struct PatternStore {
    patterns: PatternArray<Vec<Vec<bool>>>,
    pub inverse_entropy_cache: Vec<Vec<usize>>,
}

impl PatternStore {
    pub fn new(pattern_palette: &PatternPalette, grid_size: [usize; 2]) -> Self {
        let patterns = pattern_palette.get_pattern_array(|pattern_id| {
            (0..grid_size[0] + pattern_palette.get_size(&pattern_id)[0] - 1)
                .map(|_| {
                    (0..grid_size[1] + pattern_palette.get_size(&pattern_id)[1] - 1)
                        .map(|_| true)
                        .collect_vec()
                })
                .collect_vec()
        });

        let inverse_entropy_cache = (0..grid_size[0])
            .map(|_| (0..grid_size[1]).map(|_| 0).collect_vec())
            .collect_vec();

        Self {
            patterns,
            inverse_entropy_cache,
        }
    }

    pub fn disable_pattern(
        &mut self,
        palette: &PatternPalette,
        pattern_id: PatternId,
        grid_pos: [usize; 2],
        offset: [usize; 2],
    ) -> bool {
        let size = palette.get_size(&pattern_id);
        let pattern_x = grid_pos[0] + (size[0] - offset[0] - 1);
        let pattern_y = grid_pos[1] + (size[1] - offset[1] - 1);

        if self.patterns[pattern_id][pattern_x][pattern_y] {
            self.patterns[pattern_id][pattern_x][pattern_y] = false;

            let grid_x_min = pattern_x.max(size[0] - 1) - (size[0] - 1);
            let grid_y_min = pattern_y.max(size[1] - 1) - (size[1] - 1);
            let grid_x_max = pattern_x;
            let grid_y_max = pattern_y;

            (grid_x_min..=grid_x_max.min(self.inverse_entropy_cache.len() - 1)).for_each(|x| {
                (grid_y_min..=grid_y_max.min(self.inverse_entropy_cache[x].len() - 1)).for_each(
                    |y| {
                        self.inverse_entropy_cache[x][y] += 1;
                    },
                );
            });

            return true;
        }

        false
    }

    pub fn get_random_allowed_flag(
        &self,
        position: [usize; 2],
        palette: &PatternPalette,
    ) -> WorldTileTypeFlags {
        let enableds = self
            .patterns
            .0
            .iter()
            .enumerate()
            .flat_map(|(id, enableds)| {
                let size = palette.get_size(&PatternId(id));

                enableds
                    .iter()
                    .skip(position[0])
                    .take(size[0])
                    .enumerate()
                    .flat_map(move |(x, column)| {
                        column
                            .iter()
                            .skip(position[1])
                            .take(size[1])
                            .enumerate()
                            .filter(move |(_, value)| **value)
                            .map(move |(y, _)| (PatternId(id), [size[0] - x - 1, size[0] - y - 1]))
                    })
            })
            .collect_vec();

        let random = enableds[rand::random_range(0..enableds.len())];
        return palette.get_tile_type(&random.0, random.1).into();
    }

    pub fn get_possible_flags(
        &self,
        palette: &PatternPalette,
        grid_pos: [usize; 2],
    ) -> WorldTileTypeFlags {
        let max_size = palette.get_max_size();

        let all_allowed = (0..max_size[0])
            .map(|_| {
                (0..max_size[1])
                    .map(|_| WorldTileTypeFlags::empty())
                    .collect_vec()
            })
            .collect_vec();

        let allowed = self
            .patterns
            .0
            .iter()
            .enumerate()
            .map(|(id, positions)| (PatternId(id), positions, palette.get_size(&PatternId(id))))
            .map(|(id, positions, size)| {
                (0..size[0]).map(move |x| {
                    (0..size[1]).map(move |y| {
                        if positions[x + grid_pos[0]][y + grid_pos[1]] {
                            palette
                                .get_tile_type(&id, [size[0] - x - 1, size[1] - y - 1])
                                .into()
                        } else {
                            WorldTileTypeFlags::empty()
                        }
                    })
                })
            })
            .fold(all_allowed, |mut acc, current| {
                acc.iter_mut()
                    .zip(current)
                    .for_each(|(acc_column, current_column)| {
                        acc_column
                            .iter_mut()
                            .zip(current_column)
                            .for_each(|(acc_value, current_value)| *acc_value |= current_value)
                    });

                acc
            })
            .into_iter()
            .flatten()
            .fold(WorldTileTypeFlags::all(), |acc, flags| acc & flags);

        allowed
    }
}
