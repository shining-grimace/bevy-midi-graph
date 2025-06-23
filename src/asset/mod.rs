use std::fmt::Display;

pub(crate) mod graph;
pub(crate) mod loader;
pub(crate) mod midi;
pub(crate) mod sf2;
pub(crate) mod wave;

#[derive(Debug)]
pub struct AssetError(pub midi_graph::Error);

impl Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for AssetError {}
