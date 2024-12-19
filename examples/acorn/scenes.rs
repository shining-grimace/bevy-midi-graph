use crate::{assets::GameAssets, states::AppState, Squirrel};
use avian3d::prelude::*;
use bevy::{
    gltf::{GltfMesh, GltfNode},
    prelude::*,
};

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
    gltf_node_assets: Res<Assets<GltfNode>>,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let extracted_assets = game_assets.extract(&gltf_assets, &gltf_mesh_assets, &gltf_node_assets);
    let character_material = server.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.6, 0.2),
        ..default()
    });
    commands
        .spawn((
            SceneRoot,
            Mesh3d(extracted_assets.terrain),
            MeshMaterial3d(game_assets.array_material.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(extracted_assets.squirrel),
                MeshMaterial3d(character_material.clone()),
                extracted_assets.marker_splash_squirrel,
            ));
            parent.spawn((
                Mesh3d(extracted_assets.monkey),
                MeshMaterial3d(character_material.clone()),
            ));
            parent.spawn((
                Mesh3d(extracted_assets.fan),
                MeshMaterial3d(character_material.clone()),
            ));
            parent.spawn((
                Mesh3d(extracted_assets.acorn),
                MeshMaterial3d(character_material.clone()),
            ));
        });
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
    gltf_node_assets: Res<Assets<GltfNode>>,
    mesh_assets: Res<Assets<Mesh>>,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 24.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let extracted_assets = game_assets.extract(&gltf_assets, &gltf_mesh_assets, &gltf_node_assets);
    let character_material = server.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.6, 0.2),
        ..default()
    });

    let terrain_mesh = mesh_assets.get(&extracted_assets.terrain).unwrap();

    commands
        .spawn((
            SceneRoot,
            Mesh3d(extracted_assets.terrain.clone()),
            Collider::trimesh_from_mesh(terrain_mesh).unwrap(),
            RigidBody::Static,
            MeshMaterial3d(game_assets.array_material.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Squirrel,
                LinearVelocity::ZERO,
                Collider::cylinder(0.5, 2.0),
                RigidBody::Dynamic,
                Mesh3d(extracted_assets.squirrel),
                MeshMaterial3d(character_material.clone()),
            ));
            parent.spawn((
                Mesh3d(extracted_assets.monkey),
                MeshMaterial3d(character_material.clone()),
            ));
            parent.spawn((
                Mesh3d(extracted_assets.fan),
                MeshMaterial3d(character_material.clone()),
            ));
            parent.spawn((
                Mesh3d(extracted_assets.acorn),
                MeshMaterial3d(character_material.clone()),
            ));
        });
}
