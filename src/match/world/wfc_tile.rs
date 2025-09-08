use bevy::prelude::*;

use crate::r#match::world::world_tile_type_flags::WorldTileTypeFlags;

#[derive(Component, Clone)]
pub struct WfcTile {
    pub possible_types: WorldTileTypeFlags,
}

impl PartialEq for WfcTile {
    fn eq(&self, other: &Self) -> bool {
        self.possible_types.bits() == other.possible_types.bits()
    }
}

impl Eq for WfcTile {}

impl PartialOrd for WfcTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.possible_types
            .iter()
            .count()
            .partial_cmp(&other.possible_types.iter().count())
    }
}

impl Ord for WfcTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.possible_types
            .iter()
            .count()
            .cmp(&other.possible_types.iter().count())
    }
}

impl WfcTile {
    pub fn new() -> Self {
        Self {
            possible_types: WorldTileTypeFlags::all(),
        }
    }

    pub fn collapse(&mut self) {
        self.possible_types = self.possible_types.get_random();
    }
}
