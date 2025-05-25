use crate::{LoopFileSource, MidiFileSource, OneShotFileSource, Sf2FileSource};
use bevy::prelude::*;
use midi_graph::{
    effect::{AdsrEnvelope, Fader},
    font::SoundFontBuilder,
    generator::{LfsrNoiseSource, SawtoothWaveSource, SquareWaveSource, TriangleWaveSource},
    group::{CombinerSource, MixerSource, Polyphony},
    Error, FontSource, GraphLoader, LoopRange, MidiDataSource, Node, NoteRange, SoundSource,
};

pub struct GraphAssetLoader<'a> {
    asset_server: &'a Res<'a, AssetServer>,
    midi_assets: &'a Res<'a, Assets<MidiFileSource>>,
    sf2_assets: &'a Res<'a, Assets<Sf2FileSource>>,
    loop_assets: &'a Res<'a, Assets<LoopFileSource>>,
    one_shot_assets: &'a Res<'a, Assets<OneShotFileSource>>,
}

impl<'a> GraphAssetLoader<'a> {
    pub fn new(
        asset_server: &'a Res<AssetServer>,
        midi_assets: &'a Res<Assets<MidiFileSource>>,
        sf2_assets: &'a Res<Assets<Sf2FileSource>>,
        loop_assets: &'a Res<Assets<LoopFileSource>>,
        one_shot_assets: &'a Res<Assets<OneShotFileSource>>,
    ) -> Self {
        Self {
            asset_server,
            midi_assets,
            sf2_assets,
            loop_assets,
            one_shot_assets,
        }
    }
}

impl<'a> GraphLoader for GraphAssetLoader<'a> {
    fn load_source_with_dependencies(
        &self,
        source: &SoundSource,
    ) -> Result<Box<dyn Node + Send + 'static>, Error> {
        let consumer = match source {
            SoundSource::Midi {
                node_id,
                source,
                channels,
            } => {
                let mut midi = match source {
                    MidiDataSource::FilePath(path) => {
                        let handle: Handle<MidiFileSource> = self.asset_server.load(path);
                        let asset = self
                            .midi_assets
                            .get(handle.id())
                           .ok_or_else(|| Error::User(format!("File not loaded: {}", path)))?;
                        asset.clone_node()?
                    }
                };
                let channel_sources = channels.values()
                    .map(|source| {
                        self.load_source_with_dependencies(source)
                    })
                    .collect::<Result<Vec<Box<dyn Node + Send + 'static>>, Error>>()?;
                midi.replace_children(&channel_sources)?;
                if let Some(node_id) = node_id {
                    midi.set_node_id(*node_id);
                };
                midi
            }
            SoundSource::Font { node_id, config } => match config {
                FontSource::Ranges(ranges) => {
                    let mut font_builder = SoundFontBuilder::new(*node_id);
                    for range in ranges {
                        let note_range = NoteRange::new_inclusive_range(range.lower, range.upper);
                        let source = self.load_source_with_dependencies(&range.source)?;
                        font_builder = font_builder.add_range(note_range, source)?;
                    }
                    let source: Box<dyn Node + Send + 'static> = Box::new(font_builder.build());
                    source
                }
                FontSource::Sf2FilePath {
                    path,
                    instrument_index,
                    polyphony_voices,
                } => {
                    let handle: Handle<Sf2FileSource> = self.asset_server.load(path);
                    let asset = self.sf2_assets.get(handle.id()).ok_or_else(|| {
                        Error::User(format!("Soundfont file not loaded: {}", path))
                    })?;
                    let mut source = asset.clone_node()?;
                    if let Some(node_id) = node_id {
                        source.set_node_id(*node_id);
                    };
                    source
                }
            },
            SoundSource::SquareWave {
                node_id,
                balance,
                amplitude,
                duty_cycle,
            } => {
                let source = SquareWaveSource::new(*node_id, *balance, *amplitude, *duty_cycle);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
            SoundSource::TriangleWave {
                node_id,
                balance,
                amplitude,
            } => {
                let source = TriangleWaveSource::new(*node_id, *balance, *amplitude);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
            SoundSource::SawtoothWave {
                node_id,
                balance,
                amplitude,
            } => {
                let source = SawtoothWaveSource::new(*node_id, *balance, *amplitude);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
            SoundSource::LfsrNoise {
                node_id,
                balance,
                amplitude,
                inside_feedback,
                note_for_16_shifts,
            } => {
                let source = LfsrNoiseSource::new(
                    *node_id,
                    *balance,
                    *amplitude,
                    *inside_feedback,
                    *note_for_16_shifts,
                );
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
            SoundSource::SampleFilePath {
                node_id,
                balance,
                path,
                base_note,
                looping,
            } => {
                let handle: Handle<LoopFileSource> = self.asset_server.load(path);
                let asset = self
                    .loop_assets
                    .get(handle.id())
                    .ok_or_else(|| Error::User(format!("Loop file not loaded: {}", path)))?;
                let loop_range = looping.as_ref().map(LoopRange::from_config);
                let mut source = asset.clone_node()?;
                if let Some(node_id) = node_id {
                    source.set_node_id(*node_id);
                };
                source
            }
            SoundSource::OneShotFilePath {
                node_id,
                balance,
                path,
            } => {
                let handle: Handle<OneShotFileSource> = self.asset_server.load(path);
                let asset = self
                    .one_shot_assets
                    .get(handle.id())
                    .ok_or_else(|| Error::User(format!("One shot file not loaded: {}", path)))?;
                let mut source = asset.clone_node()?;
                if let Some(node_id) = node_id {
                    source.set_node_id(*node_id);
                };
                source
            }
            SoundSource::AdsrEnvelope {
                node_id,
                attack_time,
                decay_time,
                sustain_multiplier,
                release_time,
                source,
            } => {
                let source = self.load_source_with_dependencies(source)?;
                let source = AdsrEnvelope::from_parameters(
                    *node_id,
                    *attack_time,
                    *decay_time,
                    *sustain_multiplier,
                    *release_time,
                    source,
                );
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
            SoundSource::Combiner { node_id, sources } => {
                let mut inner_sources: Vec<Box<dyn Node + Send + 'static>> = vec![];
                for source in sources.iter() {
                    let source = self.load_source_with_dependencies(source)?;
                    inner_sources.push(source);
                }
                let source = CombinerSource::new(*node_id, inner_sources);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
            SoundSource::Mixer {
                node_id,
                balance,
                source_0,
                source_1,
            } => {
                let source_0 = self.load_source_with_dependencies(source_0)?;
                let source_1 = self.load_source_with_dependencies(source_1)?;
                let source = MixerSource::new(*node_id, *balance, source_0, source_1);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
            SoundSource::Polyphony {
                node_id,
                max_voices,
                source,
            } => {
                let inner_source = self.load_source_with_dependencies(source)?;
                let source = Polyphony::new(*node_id, *max_voices, inner_source)?;
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
            SoundSource::Fader {
                node_id,
                initial_volume,
                source,
            } => {
                let source = self.load_source_with_dependencies(source)?;
                let source = Fader::new(*node_id, *initial_volume, source);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                source
            }
        };
        Ok(consumer)
    }
}
