use bevy::prelude::*;

use crate::r#match::world::{
    global_chances_resource::GlobalChancesResource, wfc_tile::WfcTile, world_state::WorldState,
    world_tile_position::WorldTilePosition, world_tile_type::WorldTileType,
    world_tile_type_flags::WorldTileTypeFlags,
};

const COLLAPSES_PER_FRAME: usize = 16;

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
    for x in 0..128 {
        for y in 0..128 {
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
    let mesh = meshes.add(Cuboid::from_size(Vec3::ONE));

    for _ in 0..COLLAPSES_PER_FRAME {
        if collapse_first(
            &mut wfc_tiles,
            &mut commands,
            mesh.clone(),
            &mut materials,
            &mut global_chances,
        ) {
            info!("DONE");
            world_state.set(WorldState::CleanupTerrain);
            return;
        }
    }
}

fn collapse_first(
    tiles: &mut Query<(&mut WfcTile, &WorldTilePosition, Entity)>,
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    global_chances: &mut ResMut<GlobalChancesResource>,
) -> bool {
    let Some((mut tile, position, entity)) = tiles
        .iter_mut()
        .sort::<&WfcTile>()
        .filter(|(tile, ..)| tile.possible_types.bits().count_ones() > 1)
        .nth(0)
    else {
        return true;
    };

    tile.collapse(&mut global_chances.0);
    let color = tile.possible_types.get_tile_type().get_color();

    commands.entity(entity).insert((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(color))),
    ));

    let position = position.clone();
    let allowed = tile.possible_types.get_allowed();

    propagate_neighbours(&position, allowed, tiles, commands, mesh.clone(), materials);

    false
}

fn propagate_neighbours(
    position: &WorldTilePosition,
    mut allowed_flags: WorldTileTypeFlags,
    tiles: &mut Query<(&mut WfcTile, &WorldTilePosition, Entity)>,
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    if allowed_flags == WorldTileTypeFlags::empty() {
        warn!("Impossible state at {:?}", position);
        allowed_flags = WorldTileTypeFlags::Field;
    }

    let neighbours = position.neighbours();

    let mut updated_neighbours = Vec::new();

    for (mut neighbour, neighbour_position, entity) in tiles.iter_mut().filter(|(tile, p, _)| {
        tile.possible_types.bits().count_ones() > 1 && neighbours.contains(p)
    }) {
        let new_value = neighbour.possible_types & allowed_flags;

        if neighbour.possible_types != new_value {
            neighbour.possible_types = new_value;
            updated_neighbours.push((
                neighbour_position.clone(),
                neighbour.possible_types.get_allowed(),
            ));

            if new_value.bits().count_ones() == 1 {
                let color = new_value.get_tile_type().get_color();
                commands.entity(entity).insert((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial::from_color(color))),
                ));
            }
        }
    }

    for (position, allowed_flags) in updated_neighbours {
        if allowed_flags == WorldTileTypeFlags::all() {
            continue;
        }

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
