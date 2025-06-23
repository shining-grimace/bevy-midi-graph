use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use std::sync::Mutex;

#[derive(Asset, TypePath)]
pub struct WaveFileSource {
    pub data: Mutex<Vec<u8>>,
}

#[derive(Default)]
pub struct WaveFileSourceLoader;

impl WaveFileSourceLoader {
    pub fn file_extensions<'a>() -> &'a [&'static str] {
        &["wav"]
    }
}

impl AssetLoader for WaveFileSourceLoader {
    type Asset = WaveFileSource;
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
        Ok(WaveFileSource {
            data: Mutex::new(bytes),
        })
    }

    fn extensions(&self) -> &[&str] {
        Self::file_extensions()
    }
}
