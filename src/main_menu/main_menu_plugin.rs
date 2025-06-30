use bevy::{
    app::{App, Plugin},
    ecs::system::{Res, ResMut},
    state::{
        app::AppExtStates,
        state::{NextState, State},
    },
};
use bevy_egui::{EguiContextPass, EguiContexts, egui};

use crate::{
    main_menu::main_menu_state::MainMenuState,
    spacetime_db::{
        spacetime_connection_details::SpacetimeConnectionDetails, spacetime_state::SpacetimeState,
    },
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>()
            .add_systems(EguiContextPass, render_egui_main_menu);
    }
}

fn render_egui_main_menu(
    mut main_menu_state_set: ResMut<NextState<MainMenuState>>,
    main_menu_state: Res<State<MainMenuState>>,
    mut spacetime_state: ResMut<NextState<SpacetimeState>>,
    mut spacetime_connection_details: ResMut<SpacetimeConnectionDetails>,
    mut contexts: EguiContexts,
) {
    if *main_menu_state.get() != MainMenuState::Shown {
        return;
    }

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Strategy Haven");

            ui.text_edit_singleline(&mut spacetime_connection_details.server_address);
            ui.text_edit_singleline(&mut spacetime_connection_details.server_port);
            ui.text_edit_singleline(&mut spacetime_connection_details.database_name);

            if ui.button("Start").clicked() {
                main_menu_state_set.set(MainMenuState::Hidden);
                spacetime_state.set(SpacetimeState::Ready);
            }
        });
    });
}
