use crate::states::AppState;
use bevy::{asset::LoadState, prelude::*};

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

#[derive(Resource, Default)]
pub struct GameAssets {
    pub texture: Handle<Image>,
    pub game_scene: Handle<Gltf>,
}

impl GameAssets {
    fn get_all_untyped(&self) -> [UntypedHandle; 2] {
        [
            self.texture.clone_weak().untyped(),
            self.game_scene.clone_weak().untyped(),
        ]
    }
}

fn init_game_assets(server: Res<AssetServer>, mut assets: ResMut<GameAssets>) {
    assets.texture = server.load::<Image>("acorn/terrain.jpg");
    assets.game_scene = server.load::<Gltf>("acorn/game.glb");
}

fn check_game_assets_ready(
    server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let handles = assets.get_all_untyped();
    for handle in handles {
        if !is_ready(&server, &handle) {
            return;
        }
    }
    next_state.set(AppState::Splash);
}

fn is_ready(server: &Res<AssetServer>, handle: &UntypedHandle) -> bool {
    match server.load_state(handle.id()) {
        LoadState::Failed(error) => panic!("Asset load failed: {:?}", error),
        LoadState::NotLoaded => panic!("Asset not loading"),
        LoadState::Loaded | LoadState::Loading => server.is_loaded_with_dependencies(handle.id()),
    }
}
