use bevy::prelude::*;

use crate::r#match::world::{
    global_chances_resource::GlobalChancesResource, wfc_tile::WfcTile, world_state::WorldState,
    world_tile_position::WorldTilePosition, world_tile_type::WorldTileType,
    world_tile_type_flags::WorldTileTypeFlags,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<WorldState>()
            .init_resource::<GlobalChancesResource>()
            .add_systems(OnEnter(WorldState::SpawningTiles), spawn_tiles)
            .add_systems(OnEnter(WorldState::ReplaceTiles), replace_tiles)
            .add_systems(OnEnter(WorldState::CleanupTerrain), cleanup_terrain)
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
    mut global_chances: ResMut<GlobalChancesResource>,
) {
    let Some((mut first_tile, first_position, first_entity)) = wfc_tiles
        .iter_mut()
        .sort::<&WfcTile>()
        .filter(|(tile, ..)| tile.possible_types.iter().count() > 1)
        .nth(0)
    else {
        world_state.set(WorldState::CleanupTerrain);
        return;
    };

    first_tile.collapse(&mut global_chances.0);
    let color = first_tile.possible_types.get_tile_type().get_color();
    let mesh = meshes.add(Cuboid::from_size(Vec3::ONE));
    commands.entity(first_entity).insert((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(color))),
    ));

    let position = first_position.clone();
    let allowed = first_tile.possible_types.get_allowed();

    let mut tiles = wfc_tiles.iter_mut().collect::<Vec<_>>();

    propagate_neighbours(
        &position,
        allowed,
        &mut tiles,
        &mut commands,
        mesh,
        &mut materials,
    );
}

fn propagate_neighbours(
    position: &WorldTilePosition,
    allowed_flags: WorldTileTypeFlags,
    tiles: &mut Vec<(Mut<WfcTile>, &WorldTilePosition, Entity)>,
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let neighbours = position.neighbours();

    let mut updated_neighbours = Vec::new();

    for (neighbour, neighbour_position, entity) in
        tiles.iter_mut().filter(|(_, p, _)| neighbours.contains(p))
    {
        let new_value = neighbour.possible_types & allowed_flags;

        if neighbour.possible_types != new_value {
            neighbour.possible_types = new_value;
            updated_neighbours.push((
                neighbour_position.clone(),
                neighbour.possible_types.get_allowed(),
            ));

            if new_value.iter().count() == 1 {
                let color = new_value.get_tile_type().get_color();
                commands.entity(*entity).insert((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial::from_color(color))),
                ));
            }
        }
    }

    for (position, allowed_flags) in updated_neighbours {
        propagate_neighbours(
            &position,
            allowed_flags,
            tiles,
            commands,
            mesh.clone(),
            materials,
        );
    }
}

fn cleanup_terrain(
    mut wfc_tiles: Query<(
        &mut WfcTile,
        &WorldTilePosition,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
    mut world_state: ResMut<NextState<WorldState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let beach_positions = wfc_tiles
        .iter()
        .filter(|(tile, ..)| tile.possible_types == WorldTileTypeFlags::Beach)
        .map(|(_, position, _)| position)
        .collect::<Vec<_>>();

    let beaches_without_land = beach_positions
        .into_iter()
        .filter(|position| {
            let neighbours = position.neighbours();

            let has_fields = wfc_tiles
                .iter()
                .filter(|(_, position, _)| neighbours.contains(&position))
                .any(|(tile, ..)| tile.possible_types == WorldTileTypeFlags::Field);

            !has_fields
        })
        .cloned()
        .collect::<Vec<_>>();

    let color = WorldTileType::Water.get_color();
    let material_handle = materials.add(StandardMaterial::from_color(color));

    for (mut tile, _, mut material) in wfc_tiles
        .iter_mut()
        .filter(|(_, position, _)| beaches_without_land.contains(&position))
    {
        tile.possible_types = WorldTileTypeFlags::Water;
        material.0 = material_handle.clone();
    }

    world_state.set(WorldState::ReplaceTiles);
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
