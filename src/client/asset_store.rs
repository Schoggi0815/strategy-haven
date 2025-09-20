use bevy::prelude::*;

#[derive(Resource)]
pub struct AssetStore {
    pub tile_mesh: Handle<Mesh>,
    pub water_material: Handle<StandardMaterial>,
    pub beach_material: Handle<StandardMaterial>,
    pub field_material: Handle<StandardMaterial>,
    pub forest_material: Handle<StandardMaterial>,
    pub mountain_material: Handle<StandardMaterial>,
}
