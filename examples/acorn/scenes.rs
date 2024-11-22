use crate::{assets::GameAssets, states::AppState};
use bevy::{gltf::GltfMesh, prelude::*};

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), init_splash_scene)
            .add_systems(OnExit(AppState::Splash), remove_splash_scene)
            .add_systems(OnEnter(AppState::Game), init_game_scene);
    }
}

#[derive(Component)]
struct SceneRoot;

fn init_splash_scene(
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
}

fn remove_splash_scene(
    mut commands: Commands,
    scene_query: Query<Entity, With<SceneRoot>>,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    let scene = scene_query.get_single().unwrap();
    commands.entity(scene).despawn_recursive();

    let camera = camera_query.get_single().unwrap();
    commands.entity(camera).despawn_recursive();
}

fn init_game_scene(
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
}
