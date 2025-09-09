use bevy::prelude::*;

use crate::r#match::world::{
    world_tile_chances::WorldTileChances, world_tile_type_flags::WorldTileTypeFlags,
};

#[derive(Component, Clone, PartialEq, Eq)]
pub struct WfcTile {
    pub possible_types: WorldTileTypeFlags,
}

impl PartialOrd for WfcTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.possible_types
            .bits()
            .count_ones()
            .partial_cmp(&other.possible_types.bits().count_ones())
    }
}

impl Ord for WfcTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.possible_types
            .bits()
            .count_ones()
            .cmp(&other.possible_types.bits().count_ones())
    }
}

impl WfcTile {
    pub fn new() -> Self {
        Self {
            possible_types: WorldTileTypeFlags::all(),
        }
    }

    pub fn collapse(&mut self, chances: &mut WorldTileChances) {
        self.possible_types = self.possible_types.get_random(chances);
        chances.reduce_matching(self.possible_types);
    }
}
