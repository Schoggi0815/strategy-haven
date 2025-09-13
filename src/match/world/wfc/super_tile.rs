use itertools::Itertools;

use crate::r#match::world::{
    wfc::{
        pattern_overlap::PatternOverlap,
        pattern_palette::{PatternId, PatternPalette},
    },
    world_tile_type::WorldTileType,
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

    pub fn entropy(&self) -> usize {
        self.pattern_states
            .iter()
            .filter(|(_, enabled)| *enabled)
            .count()
    }

    pub fn to_tile_type(&self) -> WorldTileType {
        self.possible_flags.get_tile_type()
    }

    pub fn pop_random_pattern(&mut self, palette: &PatternPalette) -> WorldTileTypeFlags {
        let overlaps = self
            .pattern_states
            .iter()
            .filter(|(overlap, enabled)| *enabled && self.possible_flags.contains(overlap.get_tile_type(palette).into()))
            .collect_vec();

        let random = rand::random_range(0..overlaps.len());
        let new_flag = overlaps[random].0.get_tile_type(palette).into();
        let possible = self.possible_flags;
        self.set_flag(new_flag, palette);
        possible ^ new_flag
    }

    pub fn disable_pattern(&mut self, pattern_id: PatternId, offset_pos: [usize; 2]) -> usize {
        let updated_count = self
            .pattern_states
            .iter()
            .filter(|(overlap, enabled)| {
                *enabled && overlap.pattern == pattern_id && overlap.offset == offset_pos
            })
            .count();

        self.pattern_states
            .iter_mut()
            .filter(|(overlap, enabled)| {
                *enabled && overlap.pattern == pattern_id && overlap.offset == offset_pos
            })
            .for_each(|(_, enabled)| *enabled = false);

        updated_count
    }

    pub fn set_flag(
        &mut self,
        tile_flags: WorldTileTypeFlags,
        palette: &PatternPalette,
    ) -> WorldTileTypeFlags {
        let new_flags = self.possible_flags & tile_flags;
        let removed_flags = self.possible_flags ^ new_flags;
        self.possible_flags = new_flags;

        if removed_flags.bits().count_ones() == 0 {
            return removed_flags;
        }

        self.pattern_states
            .iter_mut()
            .for_each(|(overlap, enabled)| {
                *enabled = tile_flags.contains(overlap.get_tile_type(palette).into())
            });

        removed_flags
    }

    pub fn recalculate_possible_flags(&mut self, palette: &PatternPalette) -> WorldTileTypeFlags {
        let pattern_offsets = self
            .pattern_states
            .iter()
            .filter(|(_, enabled)| *enabled)
            .map(|(overlap, _)| overlap.offset)
            .sorted()
            .dedup()
            .collect_vec();

        let new_flags = pattern_offsets
            .iter()
            .map(|offset| {
                self.pattern_states
                    .iter()
                    .filter(|(overlap, enabled)| *enabled && overlap.offset == *offset)
                    .fold(WorldTileTypeFlags::empty(), |acc, (overlap, _)| {
                        acc | overlap.get_tile_type(palette).into()
                    })
            })
            .fold(WorldTileTypeFlags::all(), |acc, flags| acc & flags);

        let removed_flags = self.possible_flags ^ new_flags;
        self.possible_flags = new_flags;
        removed_flags
    }
}
