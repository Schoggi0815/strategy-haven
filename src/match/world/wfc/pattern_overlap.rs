use itertools::Itertools;

use crate::r#match::world::{
    wfc::pattern_palette::{PatternId, PatternPalette},
    world_tile_type::WorldTileType,
};

pub struct PatternOverlap {
    pub pattern: PatternId,
    pub offset: [usize; 2],
}

impl PatternOverlap {
    pub fn get_all_possible(palette: &PatternPalette, pattern_id: PatternId) -> Vec<Self> {
        let pattern_size = palette.get_size(pattern_id);

        (0..pattern_size[0])
            .cartesian_product(0..pattern_size[1])
            .map(|(x, y)| Self {
                offset: [x, y],
                pattern: pattern_id,
            })
            .collect_vec()
    }

    pub fn get_tile_type(&self, palette: &PatternPalette) -> WorldTileType {
        palette.get_tile_type(self.pattern, self.offset)
    }
}
