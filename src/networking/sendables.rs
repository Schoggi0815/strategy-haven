use serde::{Deserialize, Serialize};

use crate::{
    common::{player_id::PlayerId, player_name::PlayerName, player_private_id::PlayerPrivateId},
    networking::{world_tile_position::WorldTilePosition, world_tile_type::WorldTileType},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sendables {
    WorldTileType(WorldTileType),
    WorldTilePosition(WorldTilePosition),
    PlayerId(PlayerId),
    PlayerPrivateId(PlayerPrivateId),
    PlayerName(PlayerName),
}
