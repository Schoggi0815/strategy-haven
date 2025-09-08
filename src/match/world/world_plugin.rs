use bevy::prelude::*;

use crate::r#match::world::{
    wfc_tile::WfcTile, world_state::WorldState, world_tile_position::WorldTilePosition,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<WorldState>()
            .add_systems(OnEnter(WorldState::SpawningTiles), spawn_tiles)
            .add_systems(OnEnter(WorldState::ReplaceTiles), replace_tiles)
            .add_systems(
                Update,
                wfc_collapse.run_if(in_state(WorldState::GeneratingTerrain)),
            );
    }
}

fn spawn_tiles(mut commands: Commands, mut world_state: ResMut<NextState<WorldState>>) {
    for x in 0..32 {
        for y in 0..32 {
            let position = WorldTilePosition::new(x, y);

            commands.spawn((
                position,
                WfcTile::new(),
                Transform::from_xyz(x as f32, 0., y as f32),
            ));
        }
    }

    world_state.set(WorldState::GeneratingTerrain);
}

fn wfc_collapse(
    mut wfc_tiles: Query<(&mut WfcTile, &WorldTilePosition, Entity)>,
    mut world_state: ResMut<NextState<WorldState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some((mut first_tile, first_position, first_entity)) = wfc_tiles
        .iter_mut()
        .sort::<&WfcTile>()
        .filter(|(tile, ..)| tile.possible_types.iter().count() > 1)
        .nth(0)
    else {
        world_state.set(WorldState::ReplaceTiles);
        return;
    };

    first_tile.collapse();
    let color = first_tile.possible_types.get_tile_type().get_color();
    commands.entity(first_entity).insert((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::ONE))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(color))),
    ));

    let possible_type = first_tile.possible_types.clone();
    let first_position = first_position.clone();
    let neighbours = first_position.neighbours();
    let allowed = possible_type.get_allowed();

    for (mut neighbour, ..) in wfc_tiles
        .iter_mut()
        .filter(|(_, p, _)| neighbours.contains(p))
    {
        neighbour.possible_types &= allowed;
    }
}

fn replace_tiles(
    wfc_tiles: Query<(Entity, &WfcTile)>,
    mut commands: Commands,
    mut world_state: ResMut<NextState<WorldState>>,
) {
    for (entity, wfc_tile) in wfc_tiles {
        let tile_type = wfc_tile.possible_types.get_tile_type();
        commands
            .entity(entity)
            .remove::<WfcTile>()
            .insert(tile_type);
    }

    world_state.set(WorldState::PlacingPlayers);
}
