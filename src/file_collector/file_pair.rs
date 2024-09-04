//! Module responsible for building file pairs from input paths & configuration options.

use crate::config::OutputConfig;
use derive_getters::Getters;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Struct responsible for building a list of input/output file paths
#[derive(Debug, Getters)]
pub struct FilePair {
    /// The input file path.
    input_path: PathBuf,
    /// The output file path.
    output_path: PathBuf,
}

impl FilePair {
    /// Build a new file pair from the given input path and output configuration.
    pub fn build(base_path: &PathBuf, input_path: PathBuf, output_config: &OutputConfig) -> Self {
        let relative_path = input_path
            .strip_prefix(base_path)
            .expect(format!("Failed to strip prefix: {:?}", input_path).as_str());
        let output_filename = filename(&input_path, output_config);
        let output_path = output_path(output_config, &input_path, relative_path, output_filename);

        Self {
            input_path,
            output_path,
        }
    }

    /// Split the file pair into its parts.
    pub fn into_parts(self) -> (PathBuf, PathBuf) {
        (self.input_path, self.output_path)
    }
}

/// Determine the output filename for the given input path and output configuration.
fn filename(input_path: &PathBuf, output_config: &OutputConfig) -> OsString {
    let mut filename = OsString::new();
    if let Some(prefix) = output_config.prefix() {
        filename.push(prefix);
    }
    filename.push(input_path.file_stem().expect("Failed to get file stem"));
    if let Some(suffix) = output_config.suffix() {
        filename.push(suffix);
    }
    if let Some(ext) = input_path.extension() {
        filename.push(".");
        filename.push(ext);
    }
    filename
}

/// Determine the output path for the given input path and output configuration.
fn output_path(
    output_config: &OutputConfig,
    input_path: &PathBuf,
    relative_path: &Path,
    filename: OsString,
) -> PathBuf {
    output_config
        .output_root()
        .as_ref()
        .map(|root| {
            if output_config.flatten() {
                root.clone()
            } else {
                root.join(relative_path)
            }
        })
        .unwrap_or_else(|| input_path.to_path_buf())
        .with_file_name(filename)
}
