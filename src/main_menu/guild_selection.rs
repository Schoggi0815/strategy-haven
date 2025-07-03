use bevy::prelude::*;
use spacetimedb_sdk::DbContext;

use crate::{
    module_bindings::{Guild, User, UserTableAccess},
    spacetime_db::{
        spacetime_channel::{InsertEvent, SpacetimeChannel, register_spacetime_channel},
        spacetime_server::ServerConnection,
    },
};

#[derive(Component)]
pub struct GuildSelectionMenu;

pub fn register_guild_callbacks(
    spacetime: Res<ServerConnection>,
    channel: Res<SpacetimeChannel<User>>,
) {
    register_spacetime_channel(channel, &spacetime.0.db.user());
}

pub fn on_enter_guild_selection(
    mut commands: Commands,
    assets: Res<AssetServer>,
    spacetime: Res<ServerConnection>,
) {
    info!("GUILD SELECTION!");

    spacetime
        .0
        .subscription_builder()
        .on_error(|_, e| {
            error!("{}", e);
        })
        .subscribe("SELECT * FROM user");
}

pub fn on_insert_guild(mut events: EventReader<InsertEvent<User>>) {
    for event in events.read() {
        info!("Inserted guild: {:?}", event.entity)
    }
}
