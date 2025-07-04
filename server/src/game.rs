use spacetimedb::{Filter, ReducerContext, Table, client_visibility_filter, reducer, table};

use crate::guild::is_self_manager;

#[client_visibility_filter]
pub const GAME_FILTER: Filter = Filter::Sql(
    "
    SELECT game.*
    FROM game
    JOIN guild
    ON game.guild_id = guild.id
    ",
);

#[table(name = game, public)]
pub struct Game {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub guild_id: u64,
    pub has_started: bool,
}

#[reducer]
pub fn create_game(ctx: &ReducerContext, guild_id: u64) {
    if !is_self_manager(ctx, guild_id) {
        log::warn!("GuildUser not manager: {:?}, {:?}", guild_id, ctx.sender);
        return;
    }

    let game = Game {
        id: 0,
        guild_id,
        has_started: false,
    };
    ctx.db.game().insert(game);
}
