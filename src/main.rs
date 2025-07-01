pub mod main_menu;
pub mod module_bindings;
pub mod spacetime_db;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::{
    main_menu::main_menu_plugin::MainMenuPlugin, spacetime_db::spacetime_plugin::SpacetimePlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        SpacetimePlugin,
        MainMenuPlugin,
        EguiPlugin::default(),
    ));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
