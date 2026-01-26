use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use std::sync::Mutex;

#[derive(Asset, TypePath)]
pub struct MidiFileSource {
    pub data: Mutex<Vec<u8>>,
}

#[derive(TypePath, Default)]
pub struct MidiFileSourceLoader;

impl MidiFileSourceLoader {
    pub fn file_extensions<'a>() -> &'a [&'static str] {
        &["mid", "midi", "smf"]
    }
}

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
        Ok(MidiFileSource {
            data: Mutex::new(bytes),
        })
    }

    fn extensions(&self) -> &[&str] {
        Self::file_extensions()
    }
}
