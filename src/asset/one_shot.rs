use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

#[derive(Asset, TypePath)]
pub struct OneShotFileSource {
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub struct OneShotFileSourceLoader {}

impl AssetLoader for OneShotFileSourceLoader {
    type Asset = OneShotFileSource;
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
        Ok(OneShotFileSource { bytes })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}
