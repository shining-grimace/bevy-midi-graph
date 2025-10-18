use crate::{
    asset::{midi::MidiFileSource, sf2::Sf2FileSource, wave::WaveFileSource},
    AssetType, GraphAssetLoader,
};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use midi_graph::abstraction::ChildConfig;

#[derive(Asset, TypePath)]
pub struct MidiGraph {
    pub config: ChildConfig,
    pub midi_assets: Vec<Handle<MidiFileSource>>,
    pub sf2_assets: Vec<Handle<Sf2FileSource>>,
    pub wave_assets: Vec<Handle<WaveFileSource>>,
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
        println!("Starting graph load...");
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let root_config: ChildConfig = serde_json::from_slice(&bytes)?;
        println!("Core graph loaded");

        let mut midi_assets = vec![];
        let mut sf2_assets = vec![];
        let mut wave_assets = vec![];
        ChildConfig::traverse_config_tree(&root_config, &mut |config: &ChildConfig| {
            if let Some(sub_asset_path) = config.0.asset_source() {
                let sub_asset_path = sub_asset_path.to_owned();
                match GraphAssetLoader::infer_asset_type(&sub_asset_path).unwrap() {
                    AssetType::Midi => {
                        println!("Queuing MIDI asset...");
                        let handle = load_context.loader().load(sub_asset_path);
                        midi_assets.push(handle);
                    }
                    AssetType::SoundFont => {
                        println!("Queuing SoundFont asset...");
                        let handle = load_context.loader().load(sub_asset_path);
                        sf2_assets.push(handle);
                    }
                    AssetType::Wave => {
                        println!("Queuing Wave asset...");
                        let handle = load_context.loader().load(sub_asset_path);
                        wave_assets.push(handle);
                    }
                }
            };
        });

        Ok(MidiGraph {
            config: root_config,
            midi_assets,
            sf2_assets,
            wave_assets,
        })
    }
}
