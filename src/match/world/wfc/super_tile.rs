use itertools::Itertools;

use crate::r#match::world::{
    wfc::{
        pattern_overlap::PatternOverlap,
        pattern_palette::{PatternArray, PatternId, PatternPalette},
    },
    world_tile_type::WorldTileType,
    world_tile_type_flags::WorldTileTypeFlags,
};

pub struct SuperTile {
    pattern_states: PatternArray<Box<[Box<[bool]>]>>,
    possible_flags: WorldTileTypeFlags,
    entropy: usize,
}

impl SuperTile {
    pub fn new(palette: &PatternPalette) -> Self {
        let pattern_states = palette.get_pattern_array(|id| palette.get_offset_arrays(id));

        Self {
            possible_flags: WorldTileTypeFlags::all(),
            entropy: pattern_states
                .iter()
                .flat_map(|offsets| offsets.iter().flatten())
                .count(),
            pattern_states,
        }
    }

    pub fn get_type_count(&self) -> u32 {
        self.possible_flags.bits().count_ones()
    }

    pub fn entropy(&self) -> usize {
        self.entropy
    }

    pub fn to_tile_type(&self) -> WorldTileType {
        self.possible_flags.get_tile_type()
    }

    fn enabled_patterns(&self) -> impl Iterator<Item = PatternOverlap> {
        self.pattern_states.enumerate().flat_map(|(id, offsets)| {
            offsets.iter().enumerate().flat_map(move |(x, offset)| {
                offset
                    .iter()
                    .enumerate()
                    .filter(|(_, enabled)| **enabled)
                    .map(move |(y, _)| PatternOverlap {
                        offset: [x, y],
                        pattern: id,
                    })
            })
        })
    }

    pub fn pop_random_pattern(&mut self, palette: &PatternPalette) -> WorldTileTypeFlags {
        let overlaps = self
            .enabled_patterns()
            .filter(|overlap| {
                self.possible_flags
                    .contains(overlap.get_tile_type(palette).into())
            })
            .collect_vec();

        let random = rand::random_range(0..overlaps.len());
        let new_flag = overlaps[random].get_tile_type(palette).into();
        let possible = self.possible_flags;
        self.set_flag(new_flag, palette);
        println!("New flag: {:?}", new_flag);
        possible ^ new_flag
    }

    pub fn disable_pattern(&mut self, pattern_id: PatternId, offset_pos: [usize; 2]) -> bool {
        let pattern_state = &mut self.pattern_states[pattern_id][offset_pos[0]][offset_pos[1]];
        if *pattern_state {
            *pattern_state = false;
            self.entropy -= 1;
            return true;
        }

        false
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
            .enumerate_mut()
            .for_each(|(id, offsets)| {
                offsets.iter_mut().enumerate().for_each(|(x, offsets)| {
                    offsets
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, enabled)| **enabled)
                        .for_each(|(y, enabled)| {
                            let new_value =
                                tile_flags.contains(palette.get_tile_type(&id, [x, y]).into());
                            if !new_value {
                                self.entropy -= 1;
                            }
                            *enabled = new_value
                        })
                })
            });

        removed_flags
    }

    pub fn recalculate_possible_flags(&mut self, palette: &PatternPalette) -> WorldTileTypeFlags {
        let max_size = palette.get_max_size();

        let new_flags = (0..max_size[0]).cartesian_product(0..max_size[1]).fold(
            WorldTileTypeFlags::all(),
            |acc, (x, y)| {
                acc & self
                    .pattern_states
                    .enumerate()
                    .filter(|(id, offsets)| {
                        palette.get_size(id)[0] > x && palette.get_size(id)[1] > y && offsets[x][y]
                    })
                    .fold(WorldTileTypeFlags::empty(), |acc, (id, _)| {
                        acc | palette.get_tile_type(&id, [x, y]).into()
                    })
            },
        );

        let removed_flags = self.possible_flags ^ new_flags;
        self.possible_flags = new_flags;
        removed_flags
    }
}
