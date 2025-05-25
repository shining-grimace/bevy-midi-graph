use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use midi_graph::{util::wav_from_bytes, Balance, Error, LoopRange, Node};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Asset, TypePath)]
pub struct LoopFileSource {
    pub node: Mutex<Box<dyn Node + Send + 'static>>,
}

impl LoopFileSource {
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
pub struct LoopFileSourceLoader {}

#[derive(Deserialize, Serialize)]
pub struct LoopFileSettings {
    pub source_note: u8,
    pub loop_range: Option<(usize, usize)>,
    pub balance: Balance,
}

impl Default for LoopFileSettings {
    fn default() -> Self {
        Self {
            source_note: 69,
            loop_range: None,
            balance: Balance::Both,
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
            node: Mutex::new(Box::new(node)),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}
