mod asset;
mod resource;

use bevy::prelude::*;

pub use asset::MidiGraphAsset;
pub use resource::MidiGraphAudioContext;

pub mod config {
    pub use midi_graph::{
        util::source_from_config, Config, FontSource, Loop, MidiDataSource, RangeSource,
        SoundSource,
    };
}

pub mod midi {
    pub use midi_graph::{EventChannel, NodeControlEvent, NodeEvent, NoteEvent};
}

pub struct MidiGraphPlugin;

impl Plugin for MidiGraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<asset::MidiGraphAsset>()
            .init_asset_loader::<asset::MidiGraphAssetLoader>()
            .init_resource::<resource::MidiGraphAudioContext>();
    }
}
