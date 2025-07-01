use bevy::prelude::*;

#[derive(Component)]
pub struct ServerSelectionMenu;

pub fn spawn_server_selection(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(150.0),
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
                Text::new("Start"),
                TextFont {
                    font: assets.load("fonts/Roboto-VariableFont_wdth,wght.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    ));
}

pub fn delete_server_selection(
    mut commands: Commands,
    server_selection_menus: Query<Entity, With<ServerSelectionMenu>>,
) {
    for server_selection_menu in server_selection_menus {
        commands.entity(server_selection_menu).despawn();
    }
}
