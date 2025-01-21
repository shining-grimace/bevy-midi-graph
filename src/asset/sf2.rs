use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

#[derive(Asset, TypePath)]
pub struct Sf2FileSource {
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub struct Sf2FileSourceLoader {}

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
        Ok(Sf2FileSource { bytes })
    }

    fn extensions(&self) -> &[&str] {
        &["sf2"]
    }
}
