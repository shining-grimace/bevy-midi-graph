use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
struct ModeStartTime(Duration);

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .insert_resource(ModeStartTime(Duration::ZERO))
            .add_systems(OnEnter(AppState::Splash), on_splash_init)
            .add_systems(Update, leave_splash_after_delay);
    }
}
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Splash,
    Game,
}

fn on_splash_init(mut mode_start_time: ResMut<ModeStartTime>, time: Res<Time>) {
    mode_start_time.0 = time.elapsed();
}

fn leave_splash_after_delay(
    mut next_state: ResMut<NextState<AppState>>,
    mode_start_time: Res<ModeStartTime>,
    time: Res<Time>,
) {
    let mode_duration = time.elapsed() - mode_start_time.0;
    if mode_duration > Duration::from_secs(3) {
        next_state.set(AppState::Game);
    }
}
