use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct SpacetimeConnectionDetails {
    pub server_address: String,
    pub server_port: String,
    pub database_name: String,
}

impl Default for SpacetimeConnectionDetails {
    fn default() -> Self {
        Self {
            server_address: "schoggi.net".into(),
            server_port: "3002".into(),
            database_name: "test-server".into(),
        }
    }
}
