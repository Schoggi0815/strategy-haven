use bevy::prelude::*;
use bevy_hookup_core::{owner_component::Owner, shared::Shared, sync_entity::SyncEntityOwner};
use bevy_ui_text_input::{
    TextInputContents, TextInputMode, TextInputNode, TextInputQueue,
    actions::{TextInputAction, TextInputEdit},
};

use crate::{
    common::{player_id::PlayerId, player_name::PlayerName, player_private_id::PlayerPrivateId},
    main_menu::menu_resource::MenuResource,
};

#[derive(Component)]
pub struct MatchLobbyRoot;

#[derive(Component)]
pub struct MatchUiPlayer(PlayerId);

#[derive(Component)]
pub struct MatchLobbyPlayerMarker;

pub fn spawn_match_lobby_ui(mut commands: Commands, menu_resource: Res<MenuResource>) {
    let font = &menu_resource.font;

    commands.spawn((
        MatchLobbyRoot,
        Node {
            height: Val::Vh(100.),
            width: Val::Vw(100.),
            margin: UiRect::all(Val::Px(10.0)),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![(
            Text::new("Lobby"),
            TextFont {
                font: font.clone(),
                font_size: 33.,
                ..default()
            }
        )],
    ));
}

pub fn remove_player_names(
    trigger: Trigger<OnRemove, Shared<PlayerId>>,
    player_ids: Query<&Shared<PlayerId>>,
    match_ui_players: Query<(Entity, &MatchUiPlayer)>,
    mut commands: Commands,
) {
    let Ok(player_id) = player_ids.get(trigger.target()) else {
        warn!("Removed Player ID not found!");
        return;
    };

    let Some((entity, _)) = match_ui_players
        .iter()
        .find(|(_, match_ui)| match_ui.0 == player_id.inner)
    else {
        return;
    };

    commands.entity(entity).despawn();
}

pub fn despawn_match_lobby_ui(
    mut commands: Commands,
    lobby_root: Single<Entity, With<MatchLobbyRoot>>,
) {
    commands.entity(lobby_root.into_inner()).despawn();
}

pub fn extend_with_player(
    mut commands: Commands,
    menu_resource: Res<MenuResource>,
    lobby_root: Single<Entity, With<MatchLobbyRoot>>,
    added_player: Query<
        (&Shared<PlayerName>, &Shared<PlayerId>, Entity),
        (
            Without<MatchLobbyPlayerMarker>,
            Without<Shared<PlayerPrivateId>>,
        ),
    >,
) {
    let lobby_root = lobby_root.into_inner();
    let font = &menu_resource.font;

    for (added_player_name, added_player_id, added_player_entity) in added_player {
        info!("ADDED OTHER PLAYER!");

        commands
            .entity(added_player_entity)
            .insert(MatchLobbyPlayerMarker);

        commands.entity(lobby_root).with_child((
            Text::new(added_player_name.0.clone()),
            TextFont {
                font: font.clone(),
                font_size: 25.,
                ..default()
            },
            MatchUiPlayer(added_player_id.inner.clone()),
        ));
    }
}

pub fn extend_with_player_self(
    mut commands: Commands,
    menu_resource: Res<MenuResource>,
    lobby_root: Single<Entity, With<MatchLobbyRoot>>,
    added_player: Query<
        (&Shared<PlayerName>, &Shared<PlayerId>, Entity),
        (With<Shared<PlayerPrivateId>>, Without<Owner<PlayerName>>),
    >,
) {
    let lobby_root = lobby_root.into_inner();
    let font = &menu_resource.font;

    for (added_player_name, added_player_id, entity) in added_player {
        info!("ADDED OWNED PLAYER!");

        commands
            .entity(entity)
            .insert(Owner::new(added_player_name.inner.clone()));

        commands.entity(lobby_root).with_child((
            TextInputNode {
                mode: TextInputMode::SingleLine,
                max_chars: Some(40),
                clear_on_submit: false,
                ..Default::default()
            },
            TextInputContents::default(),
            text_input_queue(&added_player_name.0),
            TextFont {
                font: font.clone(),
                font_size: 25.,
                ..default()
            },
            Node {
                width: Val::Px(300.),
                height: Val::Px(50.),
                ..default()
            },
            MatchUiPlayer(added_player_id.inner.clone()),
        ));
    }
}

pub fn update_player_name(
    updated_names: Query<
        (&Shared<PlayerName>, &Shared<PlayerId>),
        (
            Changed<Shared<PlayerName>>,
            Without<Shared<PlayerPrivateId>>,
        ),
    >,
    mut all_name_texts: Query<(&mut Text, &MatchUiPlayer)>,
) {
    for (new_name, player_id) in updated_names {
        for (mut name_text, text_player_id) in all_name_texts.iter_mut() {
            if player_id.inner != text_player_id.0 {
                continue;
            }

            name_text.0 = new_name.0.clone();
        }
    }
}

pub fn update_player_name_self(
    updated_name_input: Single<
        &TextInputContents,
        (Changed<TextInputContents>, With<MatchUiPlayer>),
    >,
    self_name: Single<&mut Owner<PlayerName>, Without<SyncEntityOwner>>,
) {
    self_name.into_inner().0 = updated_name_input.get().into();
}

fn text_input_queue(initial_text: &str) -> TextInputQueue {
    let mut queue = TextInputQueue::default();
    let overwrite_mode = false;
    for char in initial_text.chars() {
        queue.add(TextInputAction::Edit(TextInputEdit::Insert(
            char,
            overwrite_mode,
        )));
    }
    queue
}
