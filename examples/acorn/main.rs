use avian3d::prelude::*;
use bevy::prelude::*;

mod assets;
mod hud;
mod input;
mod scenes;
mod states;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            states::AppStatePlugin,
            hud::HudPlugin,
            input::InputPlugin,
            assets::GameAssetsPlugin,
            scenes::ScenesPlugin,
        ))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
        })
        .run();
}

#[derive(Component)]
pub struct Squirrel;
