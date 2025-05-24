use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::synccell::SyncCell,
};
use midi_graph::{midi::MidiSource, util::midi_builder_from_bytes};

#[derive(Asset, TypePath)]
pub struct MidiFileSource {
    pub node: SyncCell<MidiSource>,
}

#[derive(Default)]
pub struct MidiFileSourceLoader {}

impl AssetLoader for MidiFileSourceLoader {
    type Asset = MidiFileSource;
    type Settings = ();
    type Error = midi_graph::Error;
    async fn load<'a>(
        &'a self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let builder = midi_builder_from_bytes(None, bytes.as_slice())?;
        let node = builder.build()?;
        Ok(MidiFileSource {
            node: SyncCell::new(node),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["mid", "midi", "smf"]
    }
}
