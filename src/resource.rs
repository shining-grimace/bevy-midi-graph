use bevy::prelude::*;
use bevy::utils::synccell::SyncCell;
use midi_graph::{
    util::source_from_config, AsyncEventReceiver, BaseMixer, Config, Error, EventChannel,
    LfsrNoiseSource,
};
use std::sync::Mutex;

pub struct SendMixer(BaseMixer);

unsafe impl Send for SendMixer {}

#[derive(Resource)]
pub struct MidiGraphAudioContext {
    mixer: Mutex<SendMixer>,
    pub event_channel: SyncCell<EventChannel>,
}

impl Default for MidiGraphAudioContext {
    fn default() -> Self {
        let source = LfsrNoiseSource::new(None, 0.25, false, 69);
        let (channel, source) = AsyncEventReceiver::new(None, Box::new(source));
        let mixer = BaseMixer::start_with(Box::from(source)).unwrap();
        Self {
            mixer: Mutex::new(SendMixer(mixer)),
            event_channel: SyncCell::new(channel),
        }
    }
}

impl MidiGraphAudioContext {
    pub fn swap_graph(&mut self, config: &Config) -> Result<(), Error> {
        let mut mixer = match self.mixer.lock() {
            Err(err) => {
                return Err(Error::User(format!(
                    "Mixer could not be locked to replace source: {:?}",
                    err
                )));
            }
            Ok(mixer) => mixer,
        };
        let source = source_from_config(&config.root)?;
        let (channel, source) = AsyncEventReceiver::new(None, source);
        mixer.0.swap_consumer(Box::new(source));
        self.event_channel = SyncCell::new(channel);
        Ok(())
    }
}
