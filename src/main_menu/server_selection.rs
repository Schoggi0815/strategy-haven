use bevy::prelude::*;
use bevy_ui_text_input::{TextInputContents, TextInputMode, TextInputNode, TextInputPrompt};

use crate::main_menu::main_menu_state::MainMenuState;

#[derive(Component)]
pub struct ServerSelectionMenu;

#[derive(Component)]
pub struct ServerSelectionPlayOnlineButton;

#[derive(Component)]
pub struct ServerSelectionPlayOfflineButton;

#[derive(Component)]
pub struct ServerAdressInput;

#[derive(Component)]
pub struct ServerPortInput;

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
                ServerSelectionPlayOfflineButton,
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
                    Text::new("Play Offline"),
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
                ServerSelectionPlayOnlineButton,
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

pub fn play_offline(
    play_button: Single<
        &Interaction,
        (Changed<Interaction>, With<ServerSelectionPlayOfflineButton>),
    >,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if *play_button.into_inner() != Interaction::Pressed {
        return;
    }

    main_menu_state.set(MainMenuState::Hidden);
}

pub fn read_server_selection_button_input(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<ServerSelectionPlayOnlineButton>),
    >,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    let Ok(interaction) = interaction_query.single() else {
        return;
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    main_menu_state.set(MainMenuState::ServerSelectionLoading);
}
