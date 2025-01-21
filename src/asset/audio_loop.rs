use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

#[derive(Asset, TypePath)]
pub struct LoopFileSource {
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub struct LoopFileSourceLoader {}

impl AssetLoader for LoopFileSourceLoader {
    type Asset = LoopFileSource;
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
        Ok(LoopFileSource { bytes })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}
