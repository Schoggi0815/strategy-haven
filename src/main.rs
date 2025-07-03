pub mod main_menu;
pub mod module_bindings;
pub mod spacetime_db;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_ui_text_input::TextInputPlugin;

use crate::{
    main_menu::main_menu_plugin::MainMenuPlugin, spacetime_db::spacetime_plugin::SpacetimePlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        SpacetimePlugin,
        MainMenuPlugin,
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        WorldInspectorPlugin::default(),
        TextInputPlugin,
    ));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
