//! Input configuration module.

use derive_getters::Getters;
use std::ffi::OsString;
use std::path::PathBuf;

/// Input configuration parameters.
#[derive(Clone, Debug, Default, Getters)]
pub struct InputConfig {
    /// The file extensions to accept.
    extensions: Vec<OsString>,
    /// A list of input files and directories.
    inputs: Vec<PathBuf>,
    /// Whether to search for input files recursively.
    recursive: bool,
}

impl InputConfig {
    /// Create a new input configuration.
    pub fn new(extensions: Vec<OsString>, inputs: Vec<PathBuf>, recursive: bool) -> Self {
        // let extensions = extensions
        //     .into_iter()
        //     .map(|ext| OsString::from(ext.trim_start_matches('.')))
        //     .collect();

        Self {
            extensions,
            inputs,
            recursive,
        }
    }
}
