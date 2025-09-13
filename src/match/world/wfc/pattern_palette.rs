use itertools::Itertools;

use crate::r#match::world::{
    wfc::pattern::Pattern, world_tile_type::WorldTileType,
    world_tile_type_flags::WorldTileTypeFlags,
};

pub struct PatternPalette {
    patterns: Vec<Box<dyn Pattern>>,
}

impl PatternPalette {
    pub fn new(patterns: Vec<Box<dyn Pattern>>) -> Self {
        Self { patterns }
    }

    pub fn get_size(&self, pattern_id: PatternId) -> [usize; 2] {
        self.patterns[pattern_id.0].get_size()
    }

    pub fn get_tile_type(&self, pattern_id: PatternId, position: [usize; 2]) -> WorldTileType {
        self.patterns[pattern_id.0].get_tile_type(position)
    }

    pub fn get_all_ids(&self) -> Vec<PatternId> {
        (0..self.patterns.len()).map(|i| PatternId(i)).collect_vec()
    }

    pub fn get_occurances<const GRID_X_SIZE: usize, const GRID_Y_SIZE: usize>(
        &self,
        type_flags: WorldTileTypeFlags,
        position: [usize; 2],
    ) -> Vec<([usize; 2], PatternId, [usize; 2])> {
        let type_occurances = self.get_type_occurances_in_patterns(type_flags);

        type_occurances
            .iter()
            .flat_map(|(pattern_id, pattern_occurance_position)| {
                let pattern_size = self.patterns[pattern_id.0].get_size();

                (0..pattern_size[0])
                    .cartesian_product(0..pattern_size[1])
                    .filter(|(pattern_x, pattern_y)| {
                        pattern_x != &pattern_occurance_position[0]
                            || pattern_y != &pattern_occurance_position[1]
                    })
                    .map(|(pattern_x, pattern_y)| {
                        (
                            [pattern_x, pattern_y],
                            [
                                position[0] as i32 - pattern_occurance_position[0] as i32
                                    + pattern_x as i32,
                                position[1] as i32 - pattern_occurance_position[1] as i32
                                    + pattern_y as i32,
                            ],
                        )
                    })
                    .filter(|(_, grid_pos)| {
                        grid_pos[0] >= 0
                            && grid_pos[0] < GRID_X_SIZE as i32
                            && grid_pos[1] >= 0
                            && grid_pos[1] < GRID_Y_SIZE as i32
                    })
                    .map(|(in_pattern_pos, grid_pos)| {
                        (
                            [grid_pos[0] as usize, grid_pos[1] as usize],
                            *pattern_id,
                            in_pattern_pos,
                        )
                    })
            })
            .collect_vec()
    }

    fn get_type_occurances_in_patterns(
        &self,
        type_flags: WorldTileTypeFlags,
    ) -> Vec<(PatternId, [usize; 2])> {
        self.patterns
            .iter()
            .enumerate()
            .flat_map(|(id, pattern)| {
                pattern
                    .get_tile_occurances(type_flags)
                    .iter()
                    .map(|to| (PatternId(id), *to))
                    .collect_vec()
            })
            .collect_vec()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PatternId(usize);
