use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::synccell::SyncCell,
};
use midi_graph::{font::SoundFont, util::soundfont_from_bytes};
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath)]
pub struct Sf2FileSource {
    pub node: SyncCell<SoundFont>,
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
            node: SyncCell::new(node),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sf2"]
    }
}
