use bevy::prelude::*;
use spacetimedb_sdk::{DbContext, Identity};

use crate::{
    module_bindings::{self, PlayerTableAccess, set_position},
    spacetime_db::{
        spacetime_channel::{SpacetimeChannel, register_spacetime_channel},
        spacetime_server::ServerConnection,
        spacetime_state::SpacetimeState,
    },
};

pub struct PlayerPlugin;

fn init_player_channel(
    channel: Res<SpacetimeChannel<module_bindings::Player>>,
    spacetime: Res<ServerConnection>,
) {
    register_spacetime_channel(channel, spacetime.0.db.player());
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpacetimeChannel<module_bindings::Player>>();
        app.add_systems(OnEnter(SpacetimeState::Initialized), init_player_channel);
        app.add_systems(Update, (move_player_system, move_other_player_system));
        app.add_systems(
            Update,
            check_channels.run_if(in_state(SpacetimeState::Initialized)),
        );
        app.add_systems(
            FixedUpdate,
            send_player_position.run_if(in_state(SpacetimeState::Initialized)),
        );
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Player {
    pub velocity: Vec3,
}

#[derive(Component)]
#[require(Transform)]
pub struct OtherPlayer {
    pub identity: Identity,
    pub velocity: Vec3,
    pub start_pos: Vec3,
    pub position: Vec3,
    pub lerp_time: f32,
}

const SPEED: f32 = 400.;

fn move_player_system(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut movement = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        movement += Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement += Vec3::NEG_Y;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement += Vec3::X;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement += Vec3::NEG_X;
    }

    if movement == Vec3::ZERO {
        for (_, mut player) in &mut player_query {
            player.velocity = movement;
        }

        return;
    }

    movement = movement.normalize() * SPEED;

    let adjusted_movement = movement * time.delta_secs();

    for (mut transform, mut player) in &mut player_query {
        player.velocity = movement;
        transform.translation += adjusted_movement;
    }
}

fn move_other_player_system(
    mut other_player_query: Query<(&mut Transform, &mut OtherPlayer)>,
    time: Res<Time>,
) {
    for (mut transform, mut other_player) in &mut other_player_query {
        other_player.lerp_time += time.delta_secs();
        let end_pos = other_player.position + (other_player.velocity);
        let new_pos = other_player
            .start_pos
            .lerp(end_pos, other_player.lerp_time.min(1.));
        transform.translation = new_pos;
    }
}

fn send_player_position(
    player_query: Query<(&Transform, &Player)>,
    server_connection: Res<ServerConnection>,
) {
    let player = player_query.single().expect("No Player Found!");

    server_connection
        .0
        .reducers
        .set_position(
            player.0.translation.x,
            player.0.translation.y,
            player.0.translation.z,
            player.1.velocity.x,
            player.1.velocity.y,
            player.1.velocity.z,
        )
        .expect("Couldn't send player pos");
}

fn check_channels(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut other_player_qeury: Query<(Entity, &mut OtherPlayer, &Transform)>,
    connection: Res<ServerConnection>,
    player_channels: Res<SpacetimeChannel<module_bindings::Player>>,
) {
    for player in player_channels.try_iter() {
        if connection.0.identity() == player.identity {
            continue;
        }

        let other_player = other_player_qeury
            .iter_mut()
            .find(|(_, other_player, _)| other_player.identity == player.identity);

        let position = Vec3::new(player.position_x, player.position_y, player.position_z);
        let velocity = Vec3::new(player.velocity_x, player.velocity_y, player.velocity_z);

        if let Some(mut other_player) = other_player {
            other_player.1.start_pos = other_player.2.translation;
            other_player.1.position = position;
            other_player.1.velocity = velocity;
            other_player.1.lerp_time = 0.;
        } else {
            let circle = meshes.add(Circle::new(20.0));
            let color = Color::linear_rgb(1., 0., 0.);

            commands.spawn((
                OtherPlayer {
                    identity: player.identity,
                    position: position,
                    start_pos: position,
                    velocity: velocity,
                    lerp_time: 0.,
                },
                Transform::from_translation(position),
                Mesh2d(circle),
                MeshMaterial2d(materials.add(color)),
            ));
        }
    }

    for deleted_player in player_channels.try_iter_delete() {
        if connection.0.identity() == deleted_player.identity {
            continue;
        }

        let other_player = other_player_qeury
            .iter_mut()
            .find(|(_, other_player, _)| other_player.identity == deleted_player.identity);

        if let Some(other_player) = other_player {
            commands.entity(other_player.0).despawn();
        }
    }
}
