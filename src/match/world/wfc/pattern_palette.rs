use std::ops::{Index, IndexMut};

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

    pub fn get_pattern_array<T, F>(&self, f: F) -> PatternArray<T>
    where
        F: FnMut(PatternId) -> T,
    {
        PatternArray::new(self.patterns.len(), f)
    }

    pub fn get_type_occurances_in_patterns(
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
        Self((0..size).map(|i| f(PatternId(i))).collect_vec())
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
