use crate::{LoopFileSource, MidiFileSource, OneShotFileSource, Sf2FileSource};
use bevy::prelude::*;
use midi_graph::{
    effect::{AdsrEnvelope, AsyncEventReceiver, EventChannel, Fader},
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
    fn load_source_recursive(
        &self,
        source: &SoundSource,
    ) -> Result<(Vec<EventChannel>, Box<dyn Node + Send + 'static>), Error> {
        let (event_channels, consumer) = match source {
            SoundSource::Midi {
                node_id,
                source,
                channels,
            } => {
                let mut midi_builder = match source {
                    MidiDataSource::FilePath(path) => {
                        let handle: Handle<MidiFileSource> = self.asset_server.load(path);
                        let asset = self
                            .midi_assets
                            .get(handle.id())
                            .ok_or_else(|| Error::User(format!("File not loaded: {}", path)))?;
                        asset.node.get().duplicate_without_sources()
                    }
                };
                let mut event_channels = vec![];
                for (channel, source) in channels.iter() {
                    let (channels, font) = self.load_source_recursive(source)?;
                    event_channels.extend(channels);
                    midi_builder = midi_builder.add_channel_source(*channel, font);
                }
                let mut source = midi_builder.build()?;
                if let Some(node_id) = node_id {
                    source.set_node_id(*node_id);
                };
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (event_channels, source)
            }
            SoundSource::EventReceiver { node_id, source } => {
                let (mut channels, source) = self.load_source_recursive(source)?;
                let (channel, source) = AsyncEventReceiver::new(*node_id, source);
                channels.push(channel);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (channels, source)
            }
            SoundSource::Font { node_id, config } => match config {
                FontSource::Ranges(ranges) => {
                    let mut all_channels = vec![];
                    let mut font_builder = SoundFontBuilder::new(*node_id);
                    for range in ranges {
                        let note_range = NoteRange::new_inclusive_range(range.lower, range.upper);
                        let (channels, source) = self.load_source_recursive(&range.source)?;
                        all_channels.extend(channels);
                        font_builder = font_builder.add_range(note_range, source)?;
                    }
                    let source: Box<dyn Node + Send + 'static> = Box::new(font_builder.build());
                    (all_channels, source)
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
                    let mut source = asset.node.get().duplicate()?;
                    if let Some(node_id) = node_id {
                        source.set_node_id(*node_id);
                    };
                    (vec![], source)
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
                (vec![], source)
            }
            SoundSource::TriangleWave {
                node_id,
                balance,
                amplitude,
            } => {
                let source = TriangleWaveSource::new(*node_id, *balance, *amplitude);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (vec![], source)
            }
            SoundSource::SawtoothWave {
                node_id,
                balance,
                amplitude,
            } => {
                let source = SawtoothWaveSource::new(*node_id, *balance, *amplitude);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (vec![], source)
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
                (vec![], source)
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
                let mut source = asset.node.get().duplicate()?;
                if let Some(node_id) = node_id {
                    source.set_node_id(*node_id);
                };
                (vec![], source)
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
                let mut source = asset.node.read().duplicate()?;
                if let Some(node_id) = node_id {
                    source.set_node_id(*node_id);
                };
                (vec![], source)
            }
            SoundSource::AdsrEnvelope {
                node_id,
                attack_time,
                decay_time,
                sustain_multiplier,
                release_time,
                source,
            } => {
                let (channels, source) = self.load_source_recursive(source)?;
                let source = AdsrEnvelope::from_parameters(
                    *node_id,
                    *attack_time,
                    *decay_time,
                    *sustain_multiplier,
                    *release_time,
                    source,
                );
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (channels, source)
            }
            SoundSource::Combiner { node_id, sources } => {
                let mut event_channels: Vec<EventChannel> = vec![];
                let mut inner_sources: Vec<Box<dyn Node + Send + 'static>> = vec![];
                for source in sources.iter() {
                    let (channels, source) = self.load_source_recursive(source)?;
                    event_channels.extend(channels);
                    inner_sources.push(source);
                }
                let source = CombinerSource::new(*node_id, inner_sources);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (event_channels, source)
            }
            SoundSource::Mixer {
                node_id,
                balance,
                source_0,
                source_1,
            } => {
                let (mut channels, source_0) = self.load_source_recursive(source_0)?;
                let (more_channels, source_1) = self.load_source_recursive(source_1)?;
                let source = MixerSource::new(*node_id, *balance, source_0, source_1);
                channels.extend(more_channels);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (channels, source)
            }
            SoundSource::Polyphony {
                node_id,
                max_voices,
                source,
            } => {
                let (channels, inner_source) = self.load_source_recursive(source)?;
                let source = Polyphony::new(*node_id, *max_voices, inner_source)?;
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (channels, source)
            }
            SoundSource::Fader {
                node_id,
                initial_volume,
                source,
            } => {
                let (channels, source) = self.load_source_recursive(source)?;
                let source = Fader::new(*node_id, *initial_volume, source);
                let source: Box<dyn Node + Send + 'static> = Box::new(source);
                (channels, source)
            }
        };
        Ok((event_channels, consumer))
    }
}
