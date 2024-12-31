use bevy::prelude::*;
use bevy::utils::synccell::SyncCell;
use midi_graph::{util::source_from_config, BaseMixer, Config, Error, EventChannel, SoundSource};
use std::sync::Mutex;

const NODE_ID_ROOT_EVENTS: u64 = 0x10000000f;

pub struct SendMixer(BaseMixer);

unsafe impl Send for SendMixer {}

#[derive(Resource)]
pub struct MidiGraphAudioContext {
    mixer: Mutex<SendMixer>,
    event_channels: SyncCell<Vec<EventChannel>>,
}

impl Default for MidiGraphAudioContext {
    fn default() -> Self {
        let mixer = BaseMixer::start_empty().unwrap();
        Self {
            mixer: Mutex::new(SendMixer(mixer)),
            event_channels: SyncCell::new(vec![]),
        }
    }
}

impl MidiGraphAudioContext {
    // Store a new program ready to be played later when requested.
    // Returns whether a program was already stored at the given program number.
    pub fn store_new_program(&mut self, config: &Config) -> Result<bool, Error> {
        let Some(program_no) = config.program_no else {
            return Err(Error::User(
                "Missing program_no from configuration".to_owned(),
            ));
        };
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
        self.event_channels = SyncCell::new(channels);
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

    pub fn root_event_channel(&mut self) -> Option<&mut EventChannel> {
        self.event_channel(NODE_ID_ROOT_EVENTS)
    }

    pub fn event_channel(&mut self, for_node_id: u64) -> Option<&mut EventChannel> {
        let channels = self.event_channels.get();
        channels
            .iter_mut()
            .find(|channel| channel.for_node_id == for_node_id)
    }

    fn wrap_in_root_node(config: &Config) -> Config {
        Config {
            program_no: config.program_no,
            root: SoundSource::EventReceiver {
                node_id: Some(NODE_ID_ROOT_EVENTS),
                source: Box::new(config.root.clone()),
            },
        }
    }
}
