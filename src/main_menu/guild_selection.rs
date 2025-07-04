use bevy::prelude::*;
use spacetimedb_sdk::DbContext;

use crate::{
    module_bindings::{Guild, GuildUser, User, UserTableAccess},
    spacetime_db::{
        spacetime_channel::{InsertEvent, SpacetimeChannel, register_update_channel},
        spacetime_server::ServerConnection,
    },
};

#[derive(Component)]
pub struct GuildSelectionMenu;

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
        .subscribe("SELECT * FROM guild_user");
}

pub fn on_insert_guild(mut events: EventReader<InsertEvent<GuildUser>>) {
    for event in events.read() {
        info!("Inserted guild: {:?}", event.entity)
    }
}
