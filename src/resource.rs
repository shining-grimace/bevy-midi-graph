use bevy::prelude::*;
use midi_graph::{abstraction::NodeConfigData, AssetLoader, BaseMixer, Error, MessageSender};
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
        let mixer = BaseMixer::builder_with_default_registry()
            .unwrap()
            .start(None)
            .unwrap();
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
        config: &NodeConfigData,
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
