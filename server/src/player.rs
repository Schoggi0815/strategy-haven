use spacetimedb::{
    Filter, Identity, ReducerContext, Table, client_visibility_filter, reducer, table,
};

use crate::{
    game::game,
    guild::{get_guild_user, is_self_manager},
};

#[client_visibility_filter]
pub const PLAYER_FILTER: Filter = Filter::Sql(
    "
    SELECT player.*
    FROM player
    JOIN game
    ON player.game_id = game.id
    ",
);

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    #[auto_inc]
    id: u64,
    #[index(btree)]
    game_id: u64,
    #[index(btree)]
    user_id: Identity,
}

#[reducer]
pub fn add_player(ctx: &ReducerContext, game_id: u64, user_id: Identity) {
    let game = ctx.db.game().id().find(game_id);
    let Some(game) = game else {
        log::warn!("Game not found: {:?}", game_id);
        return;
    };

    if game.has_started {
        log::warn!("Game has already started: {:?}", game_id);
        return;
    }

    if !is_self_manager(ctx, game.guild_id) {
        log::warn!("User is not manager: {:?}", game_id);
        return;
    }

    let other_guild_user = get_guild_user(ctx, game.guild_id, user_id);
    if other_guild_user.is_none() {
        log::warn!("GuildUser not found: {:?}, {:?}", game.guild_id, user_id);
        return;
    }

    ctx.db.player().insert(Player {
        id: 0,
        game_id,
        user_id,
    });
}

#[reducer]
pub fn remove_player(ctx: &ReducerContext, player_id: u64) {
    let player = ctx.db.player().id().find(player_id);
    let Some(player) = player else {
        log::warn!("Player not found: {:?}", player_id);
        return;
    };

    let game = ctx.db.game().id().find(player.game_id);
    let Some(game) = game else {
        log::warn!("Game not found: {:?}", player.game_id);
        return;
    };

    if game.has_started {
        log::warn!("Game has already started: {:?}", game.id);
        return;
    }

    if !is_self_manager(ctx, game.guild_id) {
        log::warn!("User is not manager: {:?}, {:?}", game.guild_id, ctx.sender);
        return;
    }

    if !ctx.db.player().id().delete(player_id) {
        log::warn!("Player not found: {:?}", player_id);
    }
}

#[reducer]
pub fn replace_player(ctx: &ReducerContext, player_id: u64, user_id: Identity) {
    let player = ctx.db.player().id().find(player_id);
    let Some(player) = player else {
        log::warn!("Player not found: {:?}", player_id);
        return;
    };

    let game = ctx.db.game().id().find(player.game_id);
    let Some(game) = game else {
        log::warn!("Game not found: {:?}", player.game_id);
        return;
    };

    if !is_self_manager(ctx, game.guild_id) {
        log::warn!("User is not manager: {:?}, {:?}", game.guild_id, ctx.sender);
        return;
    }

    let guild_user = get_guild_user(ctx, game.guild_id, user_id);
    if guild_user.is_none() {
        log::warn!("User not in Guild: {:?}, {:?}", game.guild_id, user_id);
        return;
    }

    ctx.db.player().id().update(Player {
        user_id: user_id,
        ..player
    });
}
