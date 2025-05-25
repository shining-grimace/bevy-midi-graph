use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use midi_graph::{font::SoundFont, util::soundfont_from_bytes, Error, Node};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Asset, TypePath)]
pub struct Sf2FileSource {
    pub node: Mutex<SoundFont>,
}

impl Sf2FileSource {
    pub fn clone_node(&self) -> Result<Box<dyn Node + Send + 'static>, Error> {
        let lock = self
            .node
            .lock()
            .map_err(|e| Error::Internal(format!("Lock: {:?}", e)))?;
        let node = lock.duplicate();
        node
    }
}

#[derive(Default)]
pub struct Sf2FileSourceLoader {}

#[derive(Default, Deserialize, Serialize)]
pub struct Sf2FileSettings {
    pub instrument_index: usize,
    pub polyphony_voices: usize,
}

impl AssetLoader for Sf2FileSourceLoader {
    type Asset = Sf2FileSource;
    type Settings = Sf2FileSettings;
    type Error = midi_graph::Error;
    async fn load<'a>(
        &'a self,
        reader: &mut dyn Reader,
        settings: &Sf2FileSettings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let node = soundfont_from_bytes(
            None,
            bytes.as_slice(),
            settings.instrument_index,
            settings.polyphony_voices,
        )?;
        Ok(Sf2FileSource {
            node: Mutex::new(node),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sf2"]
    }
}
