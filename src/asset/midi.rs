use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

#[derive(Asset, TypePath)]
pub struct MidiFileSource {
    pub bytes: Vec<u8>,
}

// TODO: Can this be replaced by midi_graph::Error?
#[derive(Debug)]
pub enum MidiFileSourceLoaderError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
    MidiGraph(midi_graph::Error),
    Unknown(String),
}

impl From<std::io::Error> for MidiFileSourceLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for MidiFileSourceLoaderError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl From<midi_graph::Error> for MidiFileSourceLoaderError {
    fn from(value: midi_graph::Error) -> Self {
        Self::MidiGraph(value)
    }
}

impl std::fmt::Display for MidiFileSourceLoaderError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(fmt),
            Self::Ron(e) => e.fmt(fmt),
            Self::MidiGraph(e) => fmt.write_fmt(format_args!("{}", e)),
            Self::Unknown(e) => e.fmt(fmt),
        }
    }
}

impl std::error::Error for MidiFileSourceLoaderError {}

#[derive(Default)]
pub struct MidiFileSourceLoader {}

impl AssetLoader for MidiFileSourceLoader {
    type Asset = MidiFileSource;
    type Settings = ();
    type Error = MidiFileSourceLoaderError;
    async fn load<'a>(
        &'a self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        Ok(MidiFileSource { bytes })
    }

    fn extensions(&self) -> &[&str] {
        &["mid", "midi", "smf"]
    }
}
