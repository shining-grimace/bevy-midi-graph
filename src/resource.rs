use bevy::prelude::*;
use bevy::utils::synccell::SyncCell;
use midi_graph::{util::source_from_config, BaseMixer, Config, Error, EventChannel, SoundSource};
use std::collections::HashMap;
use std::sync::Mutex;

const NODE_ID_ROOT_EVENTS: u64 = 0x10000000f;

pub struct SendMixer(BaseMixer);

unsafe impl Send for SendMixer {}

#[derive(Resource)]
pub struct MidiGraphAudioContext {
    mixer: Mutex<SendMixer>,
    program_event_channels: SyncCell<HashMap<usize, Vec<EventChannel>>>,
}

impl Default for MidiGraphAudioContext {
    fn default() -> Self {
        let mixer = BaseMixer::start_empty().unwrap();
        Self {
            mixer: Mutex::new(SendMixer(mixer)),
            program_event_channels: SyncCell::new(HashMap::new()),
        }
    }
}

impl MidiGraphAudioContext {
    // Store a new program ready to be played later when requested.
    // Returns whether a program was already stored at the given program number.
    pub fn store_new_program(&mut self, program_no: usize, config: &Config) -> Result<bool, Error> {
        let mut mixer = match self.mixer.lock() {
            Err(err) => {
                return Err(Error::User(format!(
                    "Mixer could not be locked to store program: {:?}",
                    err
                )));
            }
            Ok(mixer) => mixer,
        };
        let wrapped_config = Self::wrap_in_root_node(config);
        let (channels, source) = source_from_config(&wrapped_config.root)?;
        let replaced_existing = mixer.0.store_program(program_no, source);
        let existing_channels = self.program_event_channels.get();
        let _ = existing_channels.insert(program_no, channels);
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

    pub fn root_event_channel(&mut self) -> Result<Option<&mut EventChannel>, Error> {
        self.event_channel(NODE_ID_ROOT_EVENTS)
    }

    pub fn event_channel(&mut self, for_node_id: u64) -> Result<Option<&mut EventChannel>, Error> {
        let mixer = match self.mixer.lock() {
            Err(err) => {
                return Err(Error::User(format!(
                    "Mixer could not be locked to store program: {:?}",
                    err
                )));
            }
            Ok(mixer) => mixer,
        };
        let Some(program_no) = mixer.0.get_current_program_no() else {
            return Ok(None);
        };
        let channels = self.program_event_channels.get();
        let program_channels = match channels.get_mut(&program_no) {
            Some(channels) => channels,
            None => {
                return Ok(None);
            }
        };
        let channels = program_channels
            .iter_mut()
            .find(|channel| channel.for_node_id == for_node_id);
        Ok(channels)
    }

    fn wrap_in_root_node(config: &Config) -> Config {
        Config {
            root: SoundSource::EventReceiver {
                node_id: Some(NODE_ID_ROOT_EVENTS),
                source: Box::new(config.root.clone()),
            },
        }
    }
}
