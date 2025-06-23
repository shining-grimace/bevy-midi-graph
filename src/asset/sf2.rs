use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use std::sync::Mutex;

#[derive(Asset, TypePath)]
pub struct Sf2FileSource {
    pub data: Mutex<Vec<u8>>,
}

#[derive(Default)]
pub struct Sf2FileSourceLoader;

impl Sf2FileSourceLoader {
    pub fn file_extensions<'a>() -> &'a [&'static str] {
        &["sf2"]
    }
}

impl AssetLoader for Sf2FileSourceLoader {
    type Asset = Sf2FileSource;
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
        Ok(Sf2FileSource {
            data: Mutex::new(bytes),
        })
    }

    fn extensions(&self) -> &[&str] {
        Self::file_extensions()
    }
}
