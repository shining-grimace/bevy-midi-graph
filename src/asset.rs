use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use midi_graph::{util::config_from_bytes, Config};

#[derive(Asset, TypePath)]
pub struct MidiGraphAsset {
    pub config: Config,
}

// TODO: Can this be replaced by midi_graph::Error?
#[derive(Debug)]
pub enum MidiGraphAssetLoaderError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
    MidiGraph(midi_graph::Error),
    Unknown(String),
}

impl From<std::io::Error> for MidiGraphAssetLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for MidiGraphAssetLoaderError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl From<midi_graph::Error> for MidiGraphAssetLoaderError {
    fn from(value: midi_graph::Error) -> Self {
        Self::MidiGraph(value)
    }
}

impl std::fmt::Display for MidiGraphAssetLoaderError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(fmt),
            Self::Ron(e) => e.fmt(fmt),
            Self::MidiGraph(e) => fmt.write_fmt(format_args!("{}", e)),
            Self::Unknown(e) => e.fmt(fmt),
        }
    }
}

impl std::error::Error for MidiGraphAssetLoaderError {}

#[derive(Default)]
pub struct MidiGraphAssetLoader {}

impl AssetLoader for MidiGraphAssetLoader {
    type Asset = MidiGraphAsset;
    type Settings = ();
    type Error = MidiGraphAssetLoaderError;
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let config = match config_from_bytes(&bytes) {
            Ok(config) => config,
            Err(midi_graph::Error::Ron(error)) => {
                return Err(Self::Error::Ron(error));
            }
            Err(_) => {
                return Err(Self::Error::Unknown("Unknown error".to_owned()));
            }
        };
        Ok(MidiGraphAsset { config })
    }
}
