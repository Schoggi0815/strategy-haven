use bevy::prelude::*;
use bevy_ui_text_input::{TextInputContents, TextInputMode, TextInputNode, TextInputPrompt};

use crate::{
    main_menu::main_menu_state::MainMenuState,
    spacetime_db::{
        spacetime_connection_details::{
            DEFAULT_DATABASE_NAME, DEFAULT_PORT, DEFAULT_SERVER, SpacetimeConnectionDetails,
        },
        spacetime_state::SpacetimeState,
    },
};

#[derive(Component)]
pub struct ServerSelectionMenu;

#[derive(Component)]
pub struct ServerSelectionPlayButton;

#[derive(Component)]
pub struct ServerAdressInput;

#[derive(Component)]
pub struct ServerPortInput;

#[derive(Component)]
pub struct ServerDatabaseInput;

pub fn spawn_server_selection(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/Roboto-VariableFont_wdth,wght.ttf");

    commands.spawn((
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ServerSelectionMenu,
        children![
            (
                ServerSelectionPlayButton,
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new("Play Online"),
                    TextFont {
                        font: font.clone(),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            ),
            (
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    ServerAdressInput,
                    get_text_input(font.clone(), "Server. (Leave empty for default)".into())
                )]
            ),
            (
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    ServerPortInput,
                    get_text_input(font.clone(), "Port. (Leave empty for default)".into())
                )]
            ),
            (
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    ServerDatabaseInput,
                    get_text_input(font.clone(), "Database. (Leave empty for default)".into())
                )]
            )
        ],
    ));
}

fn get_text_input(font: Handle<Font>, placeholder: String) -> impl Bundle {
    (
        TextInputNode {
            mode: TextInputMode::SingleLine,
            max_chars: Some(40),
            clear_on_submit: false,
            ..Default::default()
        },
        TextInputContents::default(),
        TextInputPrompt {
            text: placeholder,
            ..default()
        },
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        TextFont {
            font: font,
            font_size: 20.,
            ..Default::default()
        },
    )
}

pub fn delete_server_selection(
    mut commands: Commands,
    server_selection_menus: Query<Entity, With<ServerSelectionMenu>>,
) {
    for server_selection_menu in server_selection_menus {
        commands.entity(server_selection_menu).despawn();
    }
}

pub fn update_server_info(
    server_address_query: Query<
        &TextInputContents,
        (With<ServerAdressInput>, Changed<TextInputContents>),
    >,
    server_port_query: Query<
        &TextInputContents,
        (With<ServerPortInput>, Changed<TextInputContents>),
    >,
    server_database_query: Query<
        &TextInputContents,
        (With<ServerDatabaseInput>, Changed<TextInputContents>),
    >,
    mut spacetime_connection_details: ResMut<SpacetimeConnectionDetails>,
) {
    let server_address = server_address_query.single();
    if let Ok(server_address) = server_address {
        let server_address = server_address.get().into();
        let server_address = if server_address == "" {
            DEFAULT_SERVER.into()
        } else {
            server_address
        };
        spacetime_connection_details.server_address = server_address;
    }

    let server_port = server_port_query.single();
    if let Ok(server_port) = server_port {
        let server_port = server_port.get().into();
        let server_port = if server_port == "" {
            DEFAULT_PORT.into()
        } else {
            server_port
        };
        spacetime_connection_details.server_port = server_port;
    }

    let server_database = server_database_query.single();
    if let Ok(server_database) = server_database {
        let server_database = server_database.get().into();
        let server_database = if server_database == "" {
            DEFAULT_DATABASE_NAME.into()
        } else {
            server_database
        };
        spacetime_connection_details.database_name = server_database;
    }
}

pub fn read_server_selection_button_input(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ServerSelectionPlayButton>)>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut spacetime_state: ResMut<NextState<SpacetimeState>>,
) {
    let Ok(interaction) = interaction_query.single() else {
        return;
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    spacetime_state.set(SpacetimeState::Ready);
    main_menu_state.set(MainMenuState::ServerSelectionLoading);
}
