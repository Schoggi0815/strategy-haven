use log::warn;
use spacetimedb::{
    client_visibility_filter, reducer, table, Filter, Identity, ReducerContext, Table,
};

use crate::name_generator::generate_username;

#[client_visibility_filter]
pub const USER_FILTER: Filter = Filter::Sql("SELECT * FROM user WHERE identity = :sender");

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub online: bool,
    pub avatar_id: i32,
    #[auto_inc]
    #[unique]
    pub friend_code: i32,
}

#[reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: true,
            ..user
        });
    } else {
        ctx.db.user().insert(User {
            identity: ctx.sender,
            name: generate_username(ctx.rng()),
            online: true,
            avatar_id: 0,
            friend_code: 0,
        });
    }
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: false,
            ..user
        });
    } else {
        // This branch should be unreachable,
        // as it doesn't make sense for a client to disconnect without connecting first.
        log::warn!(
            "Disconnect event for unknown user with identity {:?}",
            ctx.sender
        );
    }
}

#[reducer]
pub fn rename_self(ctx: &ReducerContext, new_name: String) {
    let Some(user) = ctx.db.user().identity().find(ctx.sender) else {
        warn!(
            "Rename request from unknown user with identity {:?}",
            ctx.sender
        );

        return;
    };

    ctx.db.user().identity().update(User {
        name: new_name,
        ..user
    });
}
