use std::{
    fs::File,
    io::{Read, Write},
};

use bevy::{picking::pointer::PointerInteraction, prelude::*};
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass},
    egui,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use itertools::Itertools;
use ron::ser::PrettyConfig;
use strategy_haven::{game::world_tile_type::WorldTileType, r#match::world::tile_grid::TileGrid};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            MeshPickingPlugin,
            PanOrbitCameraPlugin,
        ))
        .insert_resource(EditorResource {
            file_name: "wfc_preset".into(),
            selected_tile: WorldTileType::Water,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (reset_grid, edit_tiles))
        .add_systems(EguiPrimaryContextPass, render_gui)
        .run();
}

const PLACEABLE_TILES: [WorldTileType; 5] = [
    WorldTileType::Water,
    WorldTileType::Beach,
    WorldTileType::Field,
    WorldTileType::Forest,
    WorldTileType::Mountain,
];

#[derive(Resource)]
struct EditorResource {
    selected_tile: WorldTileType,
    file_name: String,
}

#[derive(Component)]
struct GridComponent {
    grid: TileGrid,
}

#[derive(Component)]
struct GridEntity {
    pos: [usize; 2],
}

fn setup(mut commands: Commands, editor_resource: Res<EditorResource>) {
    if let Ok(mut file) = File::open(format!("assets/{}.ron", editor_resource.file_name)) {
        let mut ron_string = String::new();
        file.read_to_string(&mut ron_string)
            .expect("Could not read file.");

        let grid: TileGrid = ron::from_str(&ron_string).expect("Could not parse file.");

        // grid.resize([36, 36]);
        // grid.shift([1, 1], WorldTileType::Water);

        commands.spawn(GridComponent { grid });
    } else {
        let mut grid = TileGrid::new_filled(WorldTileType::Water, [9, 11]);
        for (x, y) in (2..9).cartesian_product(4..11) {
            grid.set(x, y, WorldTileType::Beach);
        }
        for (x, y) in (3..9).cartesian_product(5..11) {
            grid.set(x, y, WorldTileType::Field);
        }
        for (x, y) in (6..9).cartesian_product(7..11) {
            grid.set(x, y, WorldTileType::Forest);
        }
        for (x, y) in (6..9).cartesian_product(0..5) {
            grid.set(x, y, WorldTileType::Beach);
        }
        for (x, y) in (7..9).cartesian_product(1..6) {
            grid.set(x, y, WorldTileType::Field);
        }
        for (x, y) in (5..6).cartesian_product(9..11) {
            grid.set(x, y, WorldTileType::Forest);
        }

        commands.spawn(GridComponent { grid });
    }

    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_translation(Vec3::new(0., 3., 5.)),
    ));
}

fn edit_tiles(
    pointers: Query<&PointerInteraction>,
    input: Res<ButtonInput<MouseButton>>,
    grid_entites: Query<(Entity, &GridEntity)>,
    mut grid: Single<&mut GridComponent>,
    editor_resource: Res<EditorResource>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, _) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
    {
        let Some((_, grid_entity)) = grid_entites.iter().find(|(e, _)| e == entity) else {
            continue;
        };

        grid.grid.set(
            grid_entity.pos[0],
            grid_entity.pos[1],
            editor_resource.selected_tile,
        );
    }
}

fn reset_grid(
    grid: Single<&GridComponent, Changed<GridComponent>>,
    grid_entities: Query<Entity, With<GridEntity>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in grid_entities {
        commands.entity(entity).despawn();
    }

    let mesh = meshes.add(Cuboid::from_size(Vec3::ONE));

    let water_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Water.get_color(),
    ));
    let beach_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Beach.get_color(),
    ));
    let field_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Field.get_color(),
    ));
    let forest_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Forest.get_color(),
    ));
    let mountain_material = materials.add(StandardMaterial::from_color(
        WorldTileType::Mountain.get_color(),
    ));

    for x in 0..grid.grid.grid_size[0] {
        for y in 0..grid.grid.grid_size[1] {
            let tile_type = grid.grid.get(x, y);

            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(match tile_type {
                    WorldTileType::Water => water_material.clone(),
                    WorldTileType::Field => field_material.clone(),
                    WorldTileType::Forest => forest_material.clone(),
                    WorldTileType::Mountain => mountain_material.clone(),
                    WorldTileType::Beach => beach_material.clone(),
                    _ => beach_material.clone(),
                }),
                Transform::from_xyz(x as f32, 0., y as f32),
                GridEntity { pos: [x, y] },
            ));
        }
    }
}

fn render_gui(
    mut contexts: EguiContexts,
    mut editor_resource: ResMut<EditorResource>,
    grid: Single<&GridComponent>,
) -> Result {
    egui::TopBottomPanel::top("top").show(contexts.ctx_mut()?, |ui| {
        ui.horizontal(|ui| {
            for tile in PLACEABLE_TILES {
                let mut button = ui.button(format!("{:?}", tile));

                if editor_resource.selected_tile == tile {
                    button = button.highlight();
                }

                if button.clicked() {
                    editor_resource.selected_tile = tile;
                }
            }
        });
    });

    egui::TopBottomPanel::bottom("structure_builder").show(contexts.ctx_mut()?, |ui| {
        let filename_label = ui.label("Filename:");
        ui.text_edit_singleline(&mut editor_resource.file_name)
            .labelled_by(filename_label.id);

        if ui.button("Save").clicked() {
            let filename = editor_resource.file_name.clone();

            let mut file =
                File::create(format!("assets/{}.ron", filename)).expect("Could not open file.");
            let ron_string = ron::ser::to_string_pretty(&grid.grid, PrettyConfig::default())
                .expect("Could not parse model.");
            file.write_all(ron_string.as_bytes())
                .expect("Could not write to file.");
            file.flush().expect("Could not flush file.")
        }
    });

    Ok(())
}
