//! Module to collect the file paths needed to process the input images into their outputs.

mod file_pair;
mod walker;

use crate::config::{Config, InputConfig, OutputConfig};
pub use file_pair::FilePair;
use std::ffi::OsString;
use std::path::PathBuf;

/// Struct responsible for building a list of input/output file paths
#[derive(Debug)]
pub struct FileCollector;

impl FileCollector {
    /// Collect the file pairs
    pub fn collect(config: &Config) -> Vec<FilePair> {
        let (input_config, output_config) = (config.input_config(), config.output_config());
        input_config
            .inputs()
            .iter()
            .flat_map(|base_path| collect_for_base_path(base_path, input_config, output_config))
            .filter(|file_pair| {
                authorize_overwrite(&file_pair.output_path(), output_config.overwrite())
            })
            .collect()
    }
}

fn collect_for_base_path<'a>(
    base_path: &'a PathBuf,
    input_config: &'a InputConfig,
    output_config: &'a OutputConfig,
) -> impl Iterator<Item = FilePair> + 'a {
    walker::path_walker(base_path, input_config.recursive())
        .filter(|file| match_extensions(file, &input_config.extensions()))
        .map(|input_path| FilePair::build(base_path, input_path, output_config))
}

fn authorize_overwrite(path: &PathBuf, overwrite: bool) -> bool {
    if !path.exists() {
        return true;
    }

    if overwrite {
        log::warn!("Overwriting existing file: {:?}", path);
        true
    } else {
        log::warn!(
            "File already exists: {:?}.  Run with --overwrite to replace it.",
            path
        );
        false
    }
}

fn match_extensions(entry: &PathBuf, extensions: &[OsString]) -> bool {
    if extensions.is_empty() {
        return true;
    }
    if let Some(ext) = entry.extension() {
        extensions.iter().any(|e| e == ext)
    } else {
        false
    }
}
