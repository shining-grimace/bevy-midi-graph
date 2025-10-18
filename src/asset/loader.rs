use crate::{
    MidiFileSource, MidiFileSourceLoader, Sf2FileSource, Sf2FileSourceLoader, WaveFileSource,
    WaveFileSourceLoader,
};
use bevy::prelude::*;
use midi_graph::{AssetLoadPayload, AssetLoader, Error, SampleBuffer, SerializedFileMetadata};

pub enum AssetType {
    Midi,
    SoundFont,
    Wave,
}

pub struct GraphAssetLoader<'a> {
    asset_server: &'a Res<'a, AssetServer>,
    midi_assets: &'a Res<'a, Assets<MidiFileSource>>,
    sf2_assets: &'a Res<'a, Assets<Sf2FileSource>>,
    wave_assets: &'a Res<'a, Assets<WaveFileSource>>,
}

impl<'a> GraphAssetLoader<'a> {
    pub fn new(
        asset_server: &'a Res<AssetServer>,
        midi_assets: &'a Res<Assets<MidiFileSource>>,
        sf2_assets: &'a Res<Assets<Sf2FileSource>>,
        wave_assets: &'a Res<Assets<WaveFileSource>>,
    ) -> Self {
        Self {
            asset_server,
            midi_assets,
            sf2_assets,
            wave_assets,
        }
    }

    pub fn infer_asset_type(asset_path: &str) -> Result<AssetType, Error> {
        let path = std::path::Path::new(asset_path);
        let os_extension = path
            .extension()
            .ok_or_else(|| Error::User(format!("Cannot parse asset path: {}", asset_path)))?;
        let extension = os_extension
            .to_str()
            .ok_or_else(|| Error::User(format!("Cannot read asset extension: {}", asset_path)))?;
        if MidiFileSourceLoader::file_extensions()
            .iter()
            .any(|ext| ext == &extension)
        {
            Ok(AssetType::Midi)
        } else if Sf2FileSourceLoader::file_extensions()
            .iter()
            .any(|ext| ext == &extension)
        {
            Ok(AssetType::SoundFont)
        } else if WaveFileSourceLoader::file_extensions()
            .iter()
            .any(|ext| ext == &extension)
        {
            Ok(AssetType::Wave)
        } else {
            Err(Error::User(format!("Unknown asset type: {}", asset_path)))
        }
    }
}

impl<'a> AssetLoader for GraphAssetLoader<'a> {
    fn load_asset_data(&mut self, path: &str) -> Result<AssetLoadPayload, Error> {
        let path = path.to_owned();
        let asset_type = Self::infer_asset_type(&path)?;
        match asset_type {
            AssetType::Midi => {
                let handle = self.asset_server.get_handle(&path).ok_or_else(|| {
                    Error::User(format!("Asset has not started loading: {}", &path))
                })?;
                let asset_data = self
                    .midi_assets
                    .get(&handle)
                    .ok_or_else(|| Error::User(format!("Asset not finished loading: {}", &path)))?;
                let locked = asset_data
                    .data
                    .lock()
                    .map_err(|e| Error::Internal(format!("Error locking asset data: {:?}", e)))?;
                let cloned_data = locked.clone();
                Ok(AssetLoadPayload::RawAssetData(cloned_data))
            }
            AssetType::SoundFont => {
                let handle = self.asset_server.get_handle(&path).ok_or_else(|| {
                    Error::User(format!("Asset has not started loading: {}", &path))
                })?;
                let asset_data = self
                    .sf2_assets
                    .get(&handle)
                    .ok_or_else(|| Error::User(format!("Asset not finished loading: {}", &path)))?;
                let locked = asset_data
                    .data
                    .lock()
                    .map_err(|e| Error::Internal(format!("Error locking asset data: {:?}", e)))?;
                let cloned_data = locked.clone();
                Ok(AssetLoadPayload::RawAssetData(cloned_data))
            }
            AssetType::Wave => {
                let handle = self.asset_server.get_handle(&path).ok_or_else(|| {
                    Error::User(format!("Asset has not started loading: {}", &path))
                })?;
                let asset_data = self
                    .wave_assets
                    .get(&handle)
                    .ok_or_else(|| Error::User(format!("Asset not finished loading: {}", path)))?;
                let locked = asset_data
                    .data
                    .lock()
                    .map_err(|e| Error::Internal(format!("Error locking asset data: {:?}", e)))?;
                let cloned_data = locked.clone();
                Ok(AssetLoadPayload::RawAssetData(cloned_data))
            }
        }
    }

    fn store_prepared_data(
        &mut self,
        _path: &str,
        _metadata: SerializedFileMetadata,
        _sample_buffer: SampleBuffer,
    ) {
        // No need to cache in a separate mechanism here; assets already stored as Bevy Assets as
        // needed
    }
}
