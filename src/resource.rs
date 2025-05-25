use crate::GraphAssetLoader;
use bevy::prelude::*;
use midi_graph::{BaseMixer, Config, Error, GraphLoader, MessageSender};
use std::sync::{Arc, Mutex};

pub struct SendMixer(BaseMixer);

unsafe impl Send for SendMixer {}

#[derive(Resource)]
pub struct MidiGraphAudioContext {
    mixer: Mutex<SendMixer>,
    event_sender: Arc<MessageSender>,
}

impl Default for MidiGraphAudioContext {
    fn default() -> Self {
        let mixer = BaseMixer::start_empty().unwrap();
        let event_sender = mixer.get_event_sender();
        Self {
            mixer: Mutex::new(SendMixer(mixer)),
            event_sender,
        }
    }
}

impl MidiGraphAudioContext {
    // Store a new program ready to be played later when requested.
    // Returns whether a program was already stored at the given program number.
    pub fn store_new_program(
        &mut self,
        program_no: usize,
        config: &Config,
        loader: &GraphAssetLoader,
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
        let source = loader.load_source_with_dependencies(&config.root)?;
        let replaced_existing = mixer.0.store_program(program_no, source);
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
