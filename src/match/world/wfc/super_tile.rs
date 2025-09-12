use itertools::Itertools;

use crate::r#match::world::{
    wfc::{pattern_overlap::PatternOverlap, pattern_palette::PatternPalette},
    world_tile_type_flags::WorldTileTypeFlags,
};

pub struct SuperTile {
    pattern_states: Vec<(PatternOverlap, bool)>,
    possible_flags: WorldTileTypeFlags,
}

impl SuperTile {
    pub fn new(palette: &PatternPalette) -> Self {
        let pattern_states = palette
            .get_all_ids()
            .into_iter()
            .flat_map(|id| PatternOverlap::get_all_possible(palette, id))
            .map(|overlap| (overlap, true))
            .collect_vec();

        Self {
            pattern_states,
            possible_flags: WorldTileTypeFlags::all(),
        }
    }

    pub fn get_type_count(&self) -> u32 {
        self.possible_flags.bits().count_ones()
    }

    pub fn set_flag(&mut self, tile_flags: WorldTileTypeFlags) {
        let new_flags = self.possible_flags & tile_flags;
        let removed_flags = self.possible_flags ^ new_flags;

        if removed_flags.bits().count_ones() == 0 {
            return;
        }
    }
}
