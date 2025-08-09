use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudioContextState {
    None,
    Loading,
    Running,
}
