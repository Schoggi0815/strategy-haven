use std::{
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use itertools::Itertools;

use crate::r#match::world::{
    wfc::pattern_data::PatternData, world_tile_type::WorldTileType,
    world_tile_type_flags::WorldTileTypeFlags,
};

pub struct PatternPalette {
    patterns: Vec<PatternData>,
    size_cache: Vec<[usize; 2]>,
    max_size: [usize; 2],
}

impl PatternPalette {
    pub fn new(patterns: Vec<PatternData>) -> Self {
        let size_cache = patterns
            .iter()
            .map(|pattern| pattern.get_size())
            .collect_vec();

        let max_size = patterns
            .iter()
            .map(|pattern| pattern.get_size())
            .fold([0, 0], |acc, size| {
                [acc[0].max(size[0]), acc[1].max(size[1])]
            });

        Self {
            patterns,
            size_cache,
            max_size,
        }
    }

    pub fn get_size(&self, pattern_id: &PatternId) -> [usize; 2] {
        self.size_cache[pattern_id.0]
    }

    pub fn get_max_size(&self) -> [usize; 2] {
        self.max_size
    }

    pub fn get_tile_type(&self, pattern_id: &PatternId, position: [usize; 2]) -> WorldTileType {
        self.patterns[pattern_id.0].get_tile_type(position)
    }

    pub fn get_all_ids(&self) -> Vec<PatternId> {
        (0..self.patterns.len()).map(|i| PatternId(i)).collect_vec()
    }

    pub fn get_offset_arrays(&self, pattern_id: PatternId) -> Vec<Vec<bool>> {
        self.patterns[pattern_id.0].get_offset_arrays()
    }

    pub fn get_pattern_array<T, F>(&self, f: F) -> PatternArray<T>
    where
        F: FnMut(PatternId) -> T,
    {
        PatternArray::new(self.patterns.len(), f)
    }

    pub fn get_occurances<const GRID_X_SIZE: usize, const GRID_Y_SIZE: usize>(
        &self,
        type_flags: WorldTileTypeFlags,
        position: [usize; 2],
    ) -> impl Iterator<Item = ([usize; 2], PatternId, [usize; 2])> {
        // let grid_min_x = (position[0] as i32 - (self.max_size[0] as i32 - 1)).max(0) as usize;
        // let grid_min_y = (position[1] as i32 - (self.max_size[1] as i32 - 1)).max(0) as usize;

        // let grid_max_x = (position[0] as i32 + (self.max_size[0] as i32 - 1))
        //     .min(GRID_X_SIZE as i32 - 1) as usize;
        // let grid_max_y = (position[1] as i32 + (self.max_size[1] as i32 - 1))
        //     .min(GRID_Y_SIZE as i32 - 1) as usize;

        // (grid_min_x..grid_max_x).cartesian_product(grid_min_y..grid_max_y).map(|(grid_x, grid_y)| {
        //     let tile = grid[grid_x][grid_y];
        //     tile.enabled_pattern_ids().flat_map(|pattern_id| {
        //         self.patterns[pattern_id.0].get_tile_occurances(type_flags).map(|occurrence| )
        //     })
        // })

        self.get_type_occurances_in_patterns(type_flags).flat_map(
            move |(pattern_id, pattern_occurance_position)| {
                let pattern_size = self.patterns[pattern_id.0].get_size();

                (0..pattern_size[0])
                    .cartesian_product(0..pattern_size[1])
                    .filter(move |(pattern_x, pattern_y)| {
                        pattern_x != &pattern_occurance_position[0]
                            || pattern_y != &pattern_occurance_position[1]
                    })
                    .map(move |(pattern_x, pattern_y)| {
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
                    .map(move |(in_pattern_pos, grid_pos)| {
                        (
                            [grid_pos[0] as usize, grid_pos[1] as usize],
                            pattern_id,
                            in_pattern_pos,
                        )
                    })
            },
        )
    }

    fn get_type_occurances_in_patterns(
        &self,
        type_flags: WorldTileTypeFlags,
    ) -> impl Iterator<Item = (PatternId, [usize; 2])> {
        self.patterns
            .iter()
            .enumerate()
            .flat_map(move |(id, pattern)| {
                pattern
                    .get_tile_occurances(type_flags)
                    .map(move |to| (PatternId(id), to))
            })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PatternId(pub usize);

pub struct PatternArray<T>(pub Vec<T>);

impl<T> PatternArray<T> {
    fn new<F>(size: usize, mut f: F) -> Self
    where
        F: FnMut(PatternId) -> T,
    {
        let mut vec = Vec::with_capacity(size);
        (0..size).for_each(|i| vec.push(f(PatternId(i))));
        Self(vec)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.0.iter_mut()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (PatternId, &T)> {
        self.0.iter().enumerate().map(|(id, t)| (PatternId(id), t))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (PatternId, &mut T)> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(id, t)| (PatternId(id), t))
    }
}

impl<T> Index<PatternId> for PatternArray<T> {
    type Output = T;

    fn index(&self, index: PatternId) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<T> IndexMut<PatternId> for PatternArray<T> {
    fn index_mut(&mut self, index: PatternId) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}
