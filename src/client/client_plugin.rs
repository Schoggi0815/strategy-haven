use bevy::prelude::*;
use bevy_hookup_core::{owner_component::Owner, shared::Shared};
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{
    client::{asset_store::AssetStore, client_state::ClientState},
    common::{player_name::PlayerName, player_private_id::PlayerPrivateId},
    game::{world_tile_position::WorldTilePosition, world_tile_type::WorldTileType},
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>()
            .add_systems(OnEnter(ClientState::Connected), setup)
            .add_systems(
                Update,
                (spawn_world_tiles, spawn_tile_transform).run_if(in_state(ClientState::Connected)),
            );
    }
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    // commands.spawn((
    //     Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
    //     PanOrbitCamera::default(),
    //     PrimaryEguiContext,
    // ));

    let mesh = meshes.add(Cuboid::from_size(Vec3::ONE));

    let water_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Water.get_color(),
    ));
    let beach_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Beach.get_color(),
    ));
    let field_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Field.get_color(),
    ));
    let forest_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Forest.get_color(),
    ));
    let mountain_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Mountain.get_color(),
    ));

    commands.insert_resource(AssetStore {
        beach_material,
        field_material,
        forest_material,
        mountain_material,
        tile_mesh: mesh,
        water_material,
    });
}

fn spawn_world_tiles(
    added_tiles: Query<(Entity, &Shared<WorldTileType>), Added<Shared<WorldTileType>>>,
    asset_store: Res<AssetStore>,
    mut commands: Commands,
) {
    for (entity, tile_type) in added_tiles {
        let material = match tile_type.inner {
            WorldTileType::Water => &asset_store.water_material,
            WorldTileType::Field => &asset_store.field_material,
            WorldTileType::Forest => &asset_store.forest_material,
            WorldTileType::Mountain => &asset_store.mountain_material,
            WorldTileType::Beach => &asset_store.beach_material,
            WorldTileType::Empty => &asset_store.mountain_material,
            WorldTileType::Uncertain => &asset_store.mountain_material,
        };

        let mesh = &asset_store.tile_mesh;

        commands
            .entity(entity)
            .insert((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone())));
    }
}

fn spawn_tile_transform(
    added_tile_positions: Query<
        (Entity, &Shared<WorldTilePosition>),
        Added<Shared<WorldTilePosition>>,
    >,
    mut commands: Commands,
) {
    for (entity, tile_position) in added_tile_positions {
        commands.entity(entity).insert(Transform::from_xyz(
            tile_position.x as f32,
            0.,
            tile_position.y as f32,
        ));
    }
}
