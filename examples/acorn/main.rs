use bevy::prelude::*;

mod assets;
mod splash;
mod states;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins((
            DefaultPlugins,
            states::AppStatePlugin,
            splash::SplashPlugin,
            assets::GameAssetsPlugin,
        ))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
        })
        .add_systems(PreUpdate, input_system)
        .run();
}

fn input_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut quit_signal: EventWriter<AppExit>) {
    let quit = keyboard_input.pressed(KeyCode::Escape);
    if quit {
        quit_signal.send(AppExit::Success);
    }
}
