use crate::{
    state::AudioContextState, GraphAssetLoader, MidiFileSource, MidiGraph, Sf2FileSource,
    WaveFileSource,
};
use bevy::prelude::*;
use midi_graph::{abstraction::ChildConfig, AssetLoader, BaseMixer, Error, MessageSender};
use std::sync::{Arc, Mutex};

pub struct SendMixer(BaseMixer);

unsafe impl Send for SendMixer {}

#[derive(Resource)]
pub struct MidiGraphAudioContext {
    mixer: Mutex<SendMixer>,
    event_sender: Arc<MessageSender>,
    playing_program: Option<(usize, Handle<MidiGraph>)>,
    loading_program: Option<(usize, Handle<MidiGraph>)>,
}

impl Default for MidiGraphAudioContext {
    fn default() -> Self {
        let mixer = BaseMixer::builder_with_default_registry()
            .unwrap()
            .start(None)
            .unwrap();
        let event_sender = mixer.get_event_sender();
        Self {
            mixer: Mutex::new(SendMixer(mixer)),
            event_sender,
            playing_program: None,
            loading_program: None,
        }
    }
}

impl MidiGraphAudioContext {
    pub fn check_loading_asset(
        server: Res<AssetServer>,
        mut audio_context: ResMut<MidiGraphAudioContext>,
        mut next_state: ResMut<NextState<AudioContextState>>,
        asset_server: Res<AssetServer>,
        graphs: ResMut<Assets<MidiGraph>>,
        midi_assets: Res<Assets<MidiFileSource>>,
        sf2_assets: Res<Assets<Sf2FileSource>>,
        wave_assets: Res<Assets<WaveFileSource>>,
    ) -> Result<(), BevyError> {
        let (loading_program_no, loading_asset_handle) = match &audio_context.loading_program {
            Some((program_no, asset_handle)) => (*program_no, asset_handle.clone()),
            None => {
                return Err(Error::User(
                    "Internal error: checking loading state with no asset".to_owned(),
                )
                .into());
            }
        };
        if !server.is_loaded_with_dependencies(&loading_asset_handle) {
            return Ok(());
        }
        next_state.set(AudioContextState::Running);
        let mut loader =
            GraphAssetLoader::new(&asset_server, &midi_assets, &sf2_assets, &wave_assets);
        let asset = graphs.get(&loading_asset_handle).unwrap();
        let current_program_no = audio_context
            .playing_program
            .as_ref()
            .map(|(program_no, _)| *program_no);
        audio_context.store_new_program(loading_program_no, &asset.config, &mut loader)?;
        match current_program_no {
            Some(program_no) => {
                if program_no != loading_program_no {
                    audio_context.change_program(loading_program_no)?;
                }
            }
            None => {
                audio_context.change_program(loading_program_no)?;
            }
        }
        Ok(())
    }

    pub fn start_new_program(
        &mut self,
        commands: &mut Commands,
        program_no: usize,
        asset_handle: Handle<MidiGraph>,
    ) {
        self.loading_program = Some((program_no, asset_handle));
        commands.set_state(AudioContextState::Loading);
    }

    // Store a new program ready to be played later when requested.
    // Returns whether a program was already stored at the given program number.
    pub fn store_new_program(
        &mut self,
        program_no: usize,
        config: &ChildConfig,
        loader: &mut dyn AssetLoader,
    ) -> Result<bool, Error> {
        let mut mixer = match self.mixer.lock() {
            Err(err) => {
                return Err(Error::User(format!(
                    "Mixer could not be locked to store program: {:?}",
                    err
                )));
            }
            Ok(mixer) => mixer,
        };
        let node = config.0.to_node(loader)?;
        let replaced_existing = mixer.0.store_program(program_no, node);
        Ok(replaced_existing)
    }

    pub fn change_program(&mut self, program_no: usize) -> Result<(), Error> {
        let mut mixer = match self.mixer.lock() {
            Err(err) => {
                return Err(Error::User(format!(
                    "Mixer could not be locked to store program: {:?}",
                    err
                )));
            }
            Ok(mixer) => mixer,
        };
        mixer.0.change_program(program_no)?;
        Ok(())
    }

    pub fn get_event_sender(&mut self) -> Arc<MessageSender> {
        self.event_sender.clone()
    }
}
