use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct SpacetimeConnectionDetails {
    pub server_address: String,
    pub server_port: String,
    pub database_name: String,
}

pub const DEFAULT_SERVER: &str = "schoggi.net";
pub const DEFAULT_PORT: &str = "3002";
pub const DEFAULT_DATABASE_NAME: &str = "strategy-haven";

impl Default for SpacetimeConnectionDetails {
    fn default() -> Self {
        Self {
            server_address: DEFAULT_SERVER.into(),
            server_port: DEFAULT_PORT.into(),
            database_name: DEFAULT_DATABASE_NAME.into(),
        }
    }
}
