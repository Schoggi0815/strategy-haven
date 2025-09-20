use serde::{Deserialize, Serialize};

use crate::networking::{world_tile_position::WorldTilePosition, world_tile_type::WorldTileType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sendables {
    WorldTileType(WorldTileType),
    WorldTilePosition(WorldTilePosition),
}
