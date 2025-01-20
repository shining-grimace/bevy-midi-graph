use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

#[derive(Asset, TypePath)]
pub struct LoopFileSource {
    pub bytes: Vec<u8>,
}

// TODO: Can this be replaced by midi_graph::Error?
#[derive(Debug)]
pub enum LoopFileSourceLoaderError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
    MidiGraph(midi_graph::Error),
    Unknown(String),
}

impl From<std::io::Error> for LoopFileSourceLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for LoopFileSourceLoaderError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl From<midi_graph::Error> for LoopFileSourceLoaderError {
    fn from(value: midi_graph::Error) -> Self {
        Self::MidiGraph(value)
    }
}

impl std::fmt::Display for LoopFileSourceLoaderError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(fmt),
            Self::Ron(e) => e.fmt(fmt),
            Self::MidiGraph(e) => fmt.write_fmt(format_args!("{}", e)),
            Self::Unknown(e) => e.fmt(fmt),
        }
    }
}

impl std::error::Error for LoopFileSourceLoaderError {}

#[derive(Default)]
pub struct LoopFileSourceLoader {}

impl AssetLoader for LoopFileSourceLoader {
    type Asset = LoopFileSource;
    type Settings = ();
    type Error = LoopFileSourceLoaderError;
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
