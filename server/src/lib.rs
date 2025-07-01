use spacetimedb::{reducer, ReducerContext};

mod game;
mod guild;
mod name_generator;
mod player;
mod user;

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // Called when the module is initially published
}
