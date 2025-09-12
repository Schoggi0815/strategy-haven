use itertools::Itertools;

use crate::r#match::world::{wfc::pattern::Pattern, world_tile_type::WorldTileType};

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
}

#[derive(Clone, Copy)]
pub struct PatternId(usize);
