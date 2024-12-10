use bevy::prelude::*;
use bevy::utils::synccell::SyncCell;
use midi_graph::{
    util::source_from_config, AsyncEventReceiver, BaseMixer, Config, Error, EventChannel,
    LfsrNoiseSource,
};
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
        let source = LfsrNoiseSource::new(None, 0.25, false, 69);
        let (_, source) = AsyncEventReceiver::new(None, Box::new(source));
        let mixer = BaseMixer::start_with(Box::from(source)).unwrap();
        Self {
            mixer: Mutex::new(SendMixer(mixer)),
            event_channels: SyncCell::new(vec![]),
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
        let (mut channels, source) = source_from_config(&config.root)?;
        let (channel, source) = AsyncEventReceiver::new(Some(NODE_ID_ROOT_EVENTS), source);
        channels.push(channel);
        mixer.0.swap_consumer(Box::new(source));
        self.event_channels = SyncCell::new(channels);
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
}
