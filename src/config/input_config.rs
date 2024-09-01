//! Input configuration module.
use derive_getters::Getters;
use std::path::PathBuf;

/// Input configuration parameters.
#[derive(Debug, Default, Getters)]
pub struct InputConfig {
    extensions: Vec<String>,
    inputs: Vec<PathBuf>,
    recursive: bool,
}

impl InputConfig {
    /// Create a new input configuration.
    pub fn new(extensions: Vec<String>, inputs: Vec<PathBuf>, recursive: bool) -> Self {
        Self {
            extensions,
            inputs,
            recursive,
        }
    }
}
