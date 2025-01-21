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

#[derive(Default)]
pub struct MidiGraphLoader {}

impl AssetLoader for MidiGraphLoader {
    type Asset = MidiGraph;
    type Settings = ();
    type Error = midi_graph::Error;
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
                return Err(Self::Error::User("Unknown error".to_owned()));
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
