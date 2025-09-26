use bevy::prelude::*;

#[derive(Resource)]
pub struct MenuResource {
    pub font: Handle<Font>,
}

impl FromWorld for MenuResource {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load("fonts/Roboto-VariableFont_wdth,wght.ttf");

        Self { font }
    }
}
