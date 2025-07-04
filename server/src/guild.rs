use spacetimedb::{
    Filter, Identity, ReducerContext, Table, client_visibility_filter, reducer, table,
};

use crate::user::user;

#[client_visibility_filter]
pub const GUILD_USER_FILTER: Filter =
    Filter::Sql("SELECT s.* FROM guild_user s JOIN user u ON s.user_id = u.identity");

#[table(name = guild_user, public)]
pub struct GuildUser {
    #[index(btree)]
    pub user_id: Identity,
    #[index(btree)]
    pub guild_id: u64,
    pub guild_manager: bool,
}

#[client_visibility_filter]
pub const GUILD_FILTER: Filter =
    Filter::Sql("SELECT g.* FROM guild g JOIN guild_user gu ON gu.guild_id = g.id");

#[table(name = guild, public)]
pub struct Guild {
    #[auto_inc]
    #[primary_key]
    id: u64,
    name: String,
}

#[reducer]
pub fn create_guild(ctx: &ReducerContext, guild_name: String) {
    let guild = ctx.db.guild().insert(Guild {
        id: 0,
        name: guild_name,
    });

    ctx.db.guild_user().insert(GuildUser {
        guild_id: guild.id,
        user_id: ctx.sender,
        guild_manager: true,
    });
}

#[reducer]
pub fn add_to_guild(ctx: &ReducerContext, guild_id: u64, friend_code: i32) {
    let guild = ctx.db.guild().id().find(guild_id);
    let Some(guild) = guild else {
        log::warn!("Guild not found: {:?}", guild_id);
        return;
    };

    let guild_user = ctx
        .db
        .guild_user()
        .user_id()
        .filter(ctx.sender)
        .find(|guild_user| guild_user.guild_id == guild.id && guild_user.guild_manager);

    if guild_user.is_none() {
        log::warn!("GuildUser not found: {:?}, {:?}", guild.id, ctx.sender);
        return;
    }

    let new_user = ctx.db.user().friend_code().find(friend_code);
    let Some(new_user) = new_user else {
        log::warn!("User with friend_code not found: {:?}", friend_code);
        return;
    };

    ctx.db.guild_user().insert(GuildUser {
        guild_id: guild.id,
        guild_manager: false,
        user_id: new_user.identity,
    });
}

pub fn get_guild_user(ctx: &ReducerContext, guild_id: u64, user_id: Identity) -> Option<GuildUser> {
    ctx.db
        .guild_user()
        .guild_id()
        .filter(guild_id)
        .find(|guild_user| guild_user.user_id == user_id)
}

pub fn is_self_manager(ctx: &ReducerContext, guild_id: u64) -> bool {
    let guild_user = get_guild_user(ctx, guild_id, ctx.sender);
    let Some(guild_user) = guild_user else {
        return false;
    };

    guild_user.guild_manager
}

#[reducer]
pub fn leave_guild(ctx: &ReducerContext, guild_id: u64) {
    let guild_user = get_guild_user(ctx, guild_id, ctx.sender);

    let Some(guild_user) = guild_user else {
        log::warn!("GuildUser not found: {:?}, {:?}", guild_id, ctx.sender);
        return;
    };

    if !ctx.db.guild_user().delete(guild_user) {
        log::warn!(
            "GuildUser not found on deletion: {:?}, {:?}",
            guild_id,
            ctx.sender
        );
    }

    if ctx.db.guild_user().guild_id().filter(guild_id).count() != 0 {
        return;
    }

    // Delete the Guild if no User is left
    if !ctx.db.guild().id().delete(guild_id) {
        log::warn!("Guild not found on deletion: {:?}", guild_id);
    }
}

#[reducer]
pub fn kick_user(ctx: &ReducerContext, guild_id: u64, user_id: Identity) {
    if ctx.sender == user_id {
        log::warn!(
            "User trying to kick themselfs: {:?}, {:?}",
            guild_id,
            user_id
        );
        return;
    }

    if !is_self_manager(ctx, guild_id) {
        log::warn!(
            "GuildUser trying to kick is not Manager: {:?}, {:?}",
            guild_id,
            ctx.sender
        );
        return;
    }

    let guild_user_to_kick = get_guild_user(ctx, guild_id, user_id);
    let Some(guild_user_to_kick) = guild_user_to_kick else {
        log::warn!("GuildUser to kick not found: {:?}, {:?}", guild_id, user_id);
        return;
    };

    if !ctx.db.guild_user().delete(guild_user_to_kick) {
        log::warn!(
            "GuildUser to kick not found on deletion: {:?}, {:?}",
            guild_id,
            user_id
        );
    }
}

#[reducer]
pub fn promote_to_guild_manager(ctx: &ReducerContext, guild_id: u64, user_to_promote_id: Identity) {
    if !is_self_manager(ctx, guild_id) {
        log::warn!(
            "GuildUser trying to promote is not manager: {:?}, {:?}",
            guild_id,
            ctx.sender
        );
        return;
    }

    let guild_user_to_promote = get_guild_user(ctx, guild_id, user_to_promote_id);
    let Some(guild_user_to_promote) = guild_user_to_promote else {
        log::warn!(
            "GuildUser to promote not found: {:?}, {:?}",
            guild_id,
            ctx.sender
        );
        return;
    };

    ctx.db.guild_user().insert(GuildUser {
        guild_manager: true,
        ..guild_user_to_promote
    });
}
