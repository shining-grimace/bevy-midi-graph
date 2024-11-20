use crate::{assets::GameAssets, states::AppState};
use bevy::{gltf::GltfMesh, prelude::*, render::view::RenderLayers};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), init_loading_ui)
            .add_systems(OnExit(AppState::Loading), remove_loading_ui)
            .add_systems(OnEnter(AppState::Splash), init_splash_ui);
    }
}

#[derive(Component)]
struct SceneRoot;

#[derive(Component)]
struct UiRoot;

fn init_loading_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            UiRoot,
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Node::DEFAULT
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                Text("Loading...".to_owned()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE.into()),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ));
        });
}

fn remove_loading_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<UiRoot>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let ui = ui_query.get_single().unwrap();
    let camera = camera_query.get_single().unwrap();
    commands.entity(ui).despawn_recursive();
    commands.entity(camera).despawn_recursive();
}

fn init_splash_ui(
    mut commands: Commands,
    server: Res<AssetServer>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let scene: &Gltf = gltf_assets.get(&game_assets.game_scene).unwrap();
    let gltf_mesh: &GltfMesh = gltf_mesh_assets.get(&scene.meshes[0]).unwrap();
    commands.spawn((
        SceneRoot,
        Mesh3d(gltf_mesh.primitives[0].mesh.clone()),
        MeshMaterial3d(server.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.6, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Camera2d::default(),
        Camera {
            clear_color: ClearColorConfig::None,
            order: 1,
            ..default()
        },
        RenderLayers::layer(1),
    ));
    commands
        .spawn((
            UiRoot,
            RenderLayers::layer(1),
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                Text("ACORN GAME".to_owned()),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.1).into()),
                TextLayout {
                    justify: JustifyText::Center,

                    ..default()
                },
            ));
        });
}
