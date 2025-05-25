use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use midi_graph::{Node, Error, util::midi_builder_from_bytes};
use std::sync::Mutex;

#[derive(Asset, TypePath)]
pub struct MidiFileSource {
    pub node: Mutex<Box<dyn Node + Send + 'static>>,
}

impl MidiFileSource {
    pub fn clone_node(&self) -> Result<Box<dyn Node + Send + 'static>, Error> {
        let lock = self.node.lock()
            .map_err(|e| Error::Internal(format!("Lock: {:?}", e)))?;
        let node = lock.duplicate();
        node
    }
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
            node: Mutex::new(Box::new(node)),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["mid", "midi", "smf"]
    }
}
