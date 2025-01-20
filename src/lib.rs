mod asset;
mod resource;

use bevy::prelude::*;

pub use asset::{
    audio_loop::{LoopFileSource, LoopFileSourceLoader, LoopFileSourceLoaderError},
    graph::{MidiGraph, MidiGraphLoader, MidiGraphLoaderError},
    loader::GraphAssetLoader,
    midi::{MidiFileSource, MidiFileSourceLoader, MidiFileSourceLoaderError},
    one_shot::{OneShotFileSource, OneShotFileSourceLoader, OneShotFileSourceLoaderError},
    sf2::{Sf2FileSource, Sf2FileSourceLoader, Sf2FileSourceLoaderError},
};
pub use resource::MidiGraphAudioContext;

pub mod config {
    pub use midi_graph::{Config, FontSource, Loop, MidiDataSource, RangeSource, SoundSource};
}

pub mod midi {
    pub use midi_graph::{EventChannel, NodeControlEvent, NodeEvent, NoteEvent};
}

pub struct MidiGraphPlugin;

impl Plugin for MidiGraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<MidiGraph>()
            .init_asset_loader::<MidiGraphLoader>()
            .init_asset::<LoopFileSource>()
            .init_asset_loader::<LoopFileSourceLoader>()
            .init_asset::<MidiFileSource>()
            .init_asset_loader::<MidiFileSourceLoader>()
            .init_asset::<OneShotFileSource>()
            .init_asset_loader::<OneShotFileSourceLoader>()
            .init_asset::<Sf2FileSource>()
            .init_asset_loader::<Sf2FileSourceLoader>()
            .init_resource::<resource::MidiGraphAudioContext>();
    }
}
