use crate::{
    asset::{
        audio_loop::LoopFileSource, midi::MidiFileSource, one_shot::OneShotFileSource,
        sf2::Sf2FileSource,
    },
    GraphAssetLoader,
};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use midi_graph::{Config, FontSource, GraphLoader, MidiDataSource, SoundSource};

#[derive(Asset, TypePath)]
pub struct MidiGraph {
    pub config: Config,
    pub midi_assets: Vec<Handle<MidiFileSource>>,
    pub sf2_assets: Vec<Handle<Sf2FileSource>>,
    pub loop_assets: Vec<Handle<LoopFileSource>>,
    pub one_shot_assets: Vec<Handle<OneShotFileSource>>,
}

// TODO: Can this be replaced by midi_graph::Error?
#[derive(Debug)]
pub enum MidiGraphLoaderError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
    MidiGraph(midi_graph::Error),
    Unknown(String),
}

impl From<std::io::Error> for MidiGraphLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for MidiGraphLoaderError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl From<midi_graph::Error> for MidiGraphLoaderError {
    fn from(value: midi_graph::Error) -> Self {
        Self::MidiGraph(value)
    }
}

impl std::fmt::Display for MidiGraphLoaderError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(fmt),
            Self::Ron(e) => e.fmt(fmt),
            Self::MidiGraph(e) => fmt.write_fmt(format_args!("{}", e)),
            Self::Unknown(e) => e.fmt(fmt),
        }
    }
}

impl std::error::Error for MidiGraphLoaderError {}

#[derive(Default)]
pub struct MidiGraphLoader {}

impl AssetLoader for MidiGraphLoader {
    type Asset = MidiGraph;
    type Settings = ();
    type Error = MidiGraphLoaderError;
    async fn load<'a>(
        &'a self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let config = match Config::from_bytes(&bytes) {
            Ok(config) => config,
            Err(midi_graph::Error::Ron(error)) => {
                return Err(Self::Error::Ron(error));
            }
            Err(_) => {
                return Err(Self::Error::Unknown("Unknown error".to_owned()));
            }
        };

        let mut midi_assets = vec![];
        let mut sf2_assets = vec![];
        let mut loop_assets = vec![];
        let mut one_shot_assets = vec![];
        <GraphAssetLoader as GraphLoader>::traverse_sources(&config.root, |source| {
            match source.clone() {
                SoundSource::Midi {
                    source: MidiDataSource::FilePath(path),
                    ..
                } => {
                    let handle = load_context.loader().load(path);
                    midi_assets.push(handle);
                }
                SoundSource::Font {
                    config: FontSource::Sf2FilePath { path, .. },
                    ..
                } => {
                    let handle = load_context.loader().load(path);
                    sf2_assets.push(handle);
                }
                SoundSource::SampleFilePath { path, .. } => {
                    let handle = load_context.loader().load(path);
                    loop_assets.push(handle);
                }
                SoundSource::OneShotFilePath { path, .. } => {
                    let handle = load_context.loader().load(path);
                    one_shot_assets.push(handle);
                }
                _ => {}
            }
        });

        Ok(MidiGraph {
            config,
            midi_assets,
            sf2_assets,
            loop_assets,
            one_shot_assets,
        })
    }
}
