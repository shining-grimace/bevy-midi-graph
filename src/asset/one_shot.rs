use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::synccell::SyncCell,
};
use midi_graph::{generator::OneShotSource, util::one_shot_from_bytes, Balance};
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath)]
pub struct OneShotFileSource {
    pub node: SyncCell<OneShotSource>,
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
            node: SyncCell::new(node),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}
