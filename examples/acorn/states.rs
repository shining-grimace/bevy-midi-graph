use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Splash,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
    }
}
