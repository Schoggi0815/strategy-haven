use bevy_hookup_macros::Sendable;
use serde::{Deserialize, Serialize};

use crate::{
    common::{player_id::PlayerId, player_name::PlayerName, player_private_id::PlayerPrivateId},
    game::{world_tile_position::WorldTilePosition, world_tile_type::WorldTileType},
};

#[derive(Debug, Sendable, Clone, Serialize, Deserialize)]
pub enum Sendables {
    #[sendable]
    WorldTileType(WorldTileType),
    #[sendable]
    WorldTilePosition(WorldTilePosition),
    #[sendable]
    PlayerId(PlayerId),
    #[sendable]
    PlayerPrivateId(PlayerPrivateId),
    #[sendable]
    PlayerName(PlayerName),
}
