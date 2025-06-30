use bevy::{
    ecs::{
        resource::Resource,
        system::{Commands, ResMut},
    },
    log::error,
    state::state::NextState,
};
use spacetimedb_sdk::{DbContext, Error, Identity, credentials};

use crate::{
    module_bindings::{DbConnection, ErrorContext},
    spacetime_db::spacetime_state::SpacetimeState,
};

const HOST: &str = "http://192.168.12.245:3002";
const DB_NAME: &str = "test-server";

#[derive(Resource)]
pub struct ServerConnection(pub DbConnection);

fn creds_store() -> credentials::File {
    credentials::File::new("player-1")
}

pub fn init_spacetime_server(
    mut commands: Commands,
    mut next_state: ResMut<NextState<SpacetimeState>>,
) {
    let connection = DbConnection::builder()
        // Register our `on_connect` callback, which will save our auth token.
        .on_connect(on_connected)
        // Register our `on_connect_error` callback, which will print a message, then exit the procesfs.
        .on_connect_error(on_connect_error)
        // Our `on_disconnect` callback, which will print a message, then exit the process.
        .on_disconnect(on_disconnected)
        // If the user has previously connected, we'll have saved a token in the `on_connect` callback.
        // In that case, we'll load it and pass it to `with_token`,
        // so we can re-authenticate as the same `Identity`.
        .with_token(None::<String>)
        // Set the database name we chose when we called `spacetime publish`.
        .with_module_name(DB_NAME)
        // Set the URI of the SpacetimeDB host that's running our database.
        .with_uri(HOST)
        // Finalize configuration and connect!
        .build()
        .expect("Failed to connect");

    connection
        .subscription_builder()
        .on_error(|_ctx, err| {
            error!("{}", err);
        })
        .subscribe("SELECT * FROM player WHERE online = true");

    connection.run_threaded();

    commands.insert_resource(ServerConnection(connection));
    next_state.set(SpacetimeState::Initialized);
}

fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
    if let Err(e) = creds_store().save(token) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

/// Our `on_connect_error` callback: print the error, then exit the process.
fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Connection error: {:?}", err);
    std::process::exit(1);
}

/// Our `on_disconnect` callback: print a note, then exit the process.
fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    if let Some(err) = err {
        eprintln!("Disconnected: {}", err);
        std::process::exit(1);
    } else {
        println!("Disconnected.");
        std::process::exit(0);
    }
}
