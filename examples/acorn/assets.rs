use crate::{material::ArrayTextureMaterialExtension, states::AppState};
use bevy::{
    asset::LoadState,
    gltf::{GltfMesh, GltfNode},
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    pbr::ExtendedMaterial,
    prelude::*,
};

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .add_systems(OnEnter(AppState::Loading), init_game_assets)
            .add_systems(
                Update,
                check_game_assets_ready.run_if(in_state(AppState::Loading)),
            );
    }
}

pub struct ExtractedMeshes {
    pub terrain: Handle<Mesh>,
    pub squirrel: Handle<Mesh>,
    pub monkey: Handle<Mesh>,
    pub fan: Handle<Mesh>,
    pub acorn: Handle<Mesh>,
    pub marker_splash_squirrel: Transform,
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pub texture_0: Handle<Image>,
    pub texture_1: Handle<Image>,
    pub game_scene: Handle<Gltf>,
    pub array_material: Handle<ExtendedMaterial<StandardMaterial, ArrayTextureMaterialExtension>>,
}

impl GameAssets {
    fn get_all_file_assets_untyped(&self) -> [UntypedHandle; 3] {
        [
            self.texture_0.clone_weak().untyped(),
            self.texture_1.clone_weak().untyped(),
            self.game_scene.clone_weak().untyped(),
        ]
    }

    pub fn extract(
        &self,
        gltf_assets: &Res<Assets<Gltf>>,
        gltf_mesh_assets: &Res<Assets<GltfMesh>>,
        gltf_node_assets: &Res<Assets<GltfNode>>,
    ) -> ExtractedMeshes {
        fn mesh_primitive(meshes: &Vec<&GltfMesh>, name: &str) -> Handle<Mesh> {
            meshes
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| format!("Could not find GLTF mesh: {}", name))
                .unwrap()
                .primitives[0]
                .mesh
                .clone()
        }
        fn scene_marker(nodes: &Vec<&GltfNode>, name: &str) -> Transform {
            nodes
                .iter()
                .find(|n| n.name == name)
                .ok_or_else(|| format!("Could not find GLTF node: {}", name))
                .unwrap()
                .transform
        }
        let scene: &Gltf = gltf_assets.get(&self.game_scene).unwrap();
        let meshes: Vec<&GltfMesh> = scene
            .meshes
            .iter()
            .map(|handle| gltf_mesh_assets.get(handle).unwrap())
            .collect();
        let nodes: Vec<&GltfNode> = scene
            .nodes
            .iter()
            .map(|handle| gltf_node_assets.get(handle).unwrap())
            .collect();
        ExtractedMeshes {
            terrain: mesh_primitive(&meshes, "Terrain"),
            squirrel: mesh_primitive(&meshes, "Squirrel"),
            monkey: mesh_primitive(&meshes, "Monkey"),
            fan: mesh_primitive(&meshes, "Fan"),
            acorn: mesh_primitive(&meshes, "Acorn"),
            marker_splash_squirrel: scene_marker(&nodes, "Marker_SplashSquirrel"),
        }
    }
}

fn init_game_assets(
    server: Res<AssetServer>,
    mut assets: ResMut<GameAssets>,
    mut materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, ArrayTextureMaterialExtension>>,
    >,
) {
    assets.texture_0 = server.load_with_settings("acorn/terrain.jpg", |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            }),
            ..default()
        }
    });
    assets.texture_1 = server.load_with_settings("acorn/terrain-ext.jpg", |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            }),
            ..default()
        }
    });
    assets.game_scene = server.load::<Gltf>("acorn/game.glb");
    assets.array_material = materials.add(ExtendedMaterial {
        base: StandardMaterial::default(),
        extension: ArrayTextureMaterialExtension {
            array_texture_0: assets.texture_0.clone(),
            array_texture_1: assets.texture_1.clone(),
        },
    })
}

fn check_game_assets_ready(
    server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let handles = assets.get_all_file_assets_untyped();
    for (index, handle) in handles.iter().enumerate() {
        if !is_ready(&server, &handle, index) {
            return;
        }
    }

    // This bit taken from Bevy array-texture example
    let array_layers = 4;
    let image = images.get_mut(&assets.texture_0).unwrap();
    image.reinterpret_stacked_2d_as_array(array_layers);
    let image = images.get_mut(&assets.texture_1).unwrap();
    image.reinterpret_stacked_2d_as_array(array_layers);

    next_state.set(AppState::Splash);
}

fn is_ready(server: &Res<AssetServer>, handle: &UntypedHandle, index: usize) -> bool {
    match server.load_state(handle.id()) {
        LoadState::Failed(error) => panic!("Asset load failed at index {}: {:?}", index, error),
        LoadState::NotLoaded => panic!("Asset not loading at index {}", index),
        LoadState::Loaded | LoadState::Loading => server.is_loaded_with_dependencies(handle.id()),
    }
}
