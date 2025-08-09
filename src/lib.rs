mod asset;
mod resource;
mod state;

use bevy::prelude::*;

pub use asset::{
    graph::{MidiGraph, MidiGraphLoader},
    loader::{AssetType, GraphAssetLoader},
    midi::{MidiFileSource, MidiFileSourceLoader},
    sf2::{Sf2FileSource, Sf2FileSourceLoader},
    wave::{WaveFileSource, WaveFileSourceLoader},
    AssetError,
};
pub use resource::MidiGraphAudioContext;

pub mod midi {
    pub mod event {
        pub use midi_graph::{
            effect::ModulationProperty, midi::CueData, Balance, Event, EventTarget, LoopRange,
            Message, MessageSender, NoteRange,
        };
    }
    pub mod node {
        pub use midi_graph::{
            abstraction::{ChildConfig, NodeConfig},
            effect::*,
            generator::*,
            group::*,
            midi::*,
            GraphNode, Node,
        };
    }
}

pub struct MidiGraphPlugin;

impl Plugin for MidiGraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<MidiGraph>()
            .init_asset_loader::<MidiGraphLoader>()
            .init_asset::<MidiFileSource>()
            .init_asset_loader::<MidiFileSourceLoader>()
            .init_asset::<Sf2FileSource>()
            .init_asset_loader::<Sf2FileSourceLoader>()
            .init_asset::<WaveFileSource>()
            .init_asset_loader::<WaveFileSourceLoader>()
            .init_resource::<resource::MidiGraphAudioContext>()
            .insert_state(state::AudioContextState::None)
            .add_systems(
                Update,
                MidiGraphAudioContext::check_loading_asset
                    .run_if(in_state(state::AudioContextState::Loading)),
            );
    }
}
