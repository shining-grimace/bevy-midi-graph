use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use midi_graph::{util::one_shot_from_bytes, Balance, Error, Node};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Asset, TypePath)]
pub struct OneShotFileSource {
    pub node: Mutex<Box<dyn Node + Send + 'static>>,
}

impl OneShotFileSource {
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
pub struct OneShotFileSourceLoader {}

#[derive(Deserialize, Serialize)]
pub struct OneShotFileSettings {
    pub balance: Balance,
}

impl Default for OneShotFileSettings {
    fn default() -> Self {
        Self {
            balance: Balance::Both,
        }
    }
}

impl AssetLoader for OneShotFileSourceLoader {
    type Asset = OneShotFileSource;
    type Settings = OneShotFileSettings;
    type Error = midi_graph::Error;
    async fn load<'a>(
        &'a self,
        reader: &mut dyn Reader,
        settings: &OneShotFileSettings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let node = one_shot_from_bytes(bytes.as_slice(), settings.balance, None)?;
        Ok(OneShotFileSource {
            node: Mutex::new(Box::new(node)),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}
