use bevy::ecs::resource::Resource;

use crate::r#match::world::world_tile_chances::WorldTileChances;

#[derive(Resource)]
pub struct GlobalChancesResource(pub WorldTileChances);

impl Default for GlobalChancesResource {
    fn default() -> Self {
        Self(WorldTileChances {
            water: 1.,
            field: 1.,
            forest: 1.,
            mountain: 1.,
            beach: 1.,
        })
    }
}
