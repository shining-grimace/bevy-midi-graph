use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::synccell::SyncCell,
};
use midi_graph::{generator::WavSource, util::wav_from_bytes, Balance, LoopRange};
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath)]
pub struct LoopFileSource {
    pub node: SyncCell<WavSource>,
}

#[derive(Default)]
pub struct LoopFileSourceLoader {}

#[derive(Deserialize, Serialize)]
pub struct LoopFileSettings {
    pub source_note: u8,
    pub loop_range: Option<(usize, usize)>,
    pub balance: Balance
}

impl Default for LoopFileSettings {
    fn default() -> Self {
        Self {
            source_note: 69,
            loop_range: None,
            balance: Balance::Both
        }
    }
}

impl AssetLoader for LoopFileSourceLoader {
    type Asset = LoopFileSource;
    type Settings = LoopFileSettings;
    type Error = midi_graph::Error;
    async fn load<'a>(
        &'a self,
        reader: &mut dyn Reader,
        settings: &LoopFileSettings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let node = wav_from_bytes(
            bytes.as_slice(),
            settings.source_note,
            settings
                .loop_range
                .map(|(start, end)| LoopRange::new_frame_range(start, end)),
            settings.balance,
            None,
        )?;
        Ok(LoopFileSource {
            node: SyncCell::new(node),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}
