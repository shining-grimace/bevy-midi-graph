use crate::{states::AppState, Squirrel};
use avian3d::prelude::*;
use bevy::prelude::*;

const PLAYER_VELOCITY: f32 = 3.0;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharacterInputs::default()).add_systems(
            PreUpdate,
            (read_inputs, move_characters)
                .chain()
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Resource, Default)]
pub struct CharacterInputs {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

fn read_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut inputs: ResMut<CharacterInputs>,
    mut quit_signal: EventWriter<AppExit>,
) {
    let quit = keyboard_input.pressed(KeyCode::Escape);
    if quit {
        quit_signal.send(AppExit::Success);
        return;
    }

    inputs.up = keyboard_input.pressed(KeyCode::ArrowUp);
    inputs.down = keyboard_input.pressed(KeyCode::ArrowDown);
    inputs.left = keyboard_input.pressed(KeyCode::ArrowLeft);
    inputs.right = keyboard_input.pressed(KeyCode::ArrowRight);
}

fn move_characters(
    mut squirrel_query: Query<&mut LinearVelocity, With<Squirrel>>,
    inputs: Res<CharacterInputs>,
) {
    let mut squirrel_velocity = squirrel_query.single_mut();
    let input_velocity = if inputs.up && inputs.left {
        Vec3::new(-0.707, 0.0, -0.707)
    } else if inputs.up && inputs.right {
        Vec3::new(0.707, 0.0, -0.707)
    } else if inputs.down && inputs.left {
        Vec3::new(-0.707, 0.0, 0.707)
    } else if inputs.down && inputs.right {
        Vec3::new(0.707, 0.0, 0.707)
    } else if inputs.left {
        Vec3::new(-1.0, 0.0, 0.0)
    } else if inputs.right {
        Vec3::new(1.0, 0.0, 0.0)
    } else if inputs.up {
        Vec3::new(0.0, 0.0, -1.0)
    } else if inputs.down {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::ZERO
    };
    let camera_rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
    *squirrel_velocity = LinearVelocity(PLAYER_VELOCITY * (camera_rot * input_velocity));
}
