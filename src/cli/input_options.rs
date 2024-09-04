//! Input options

use crate::config::InputConfig;
use crate::errors::OliframeError;
use clap::Args;
use derive_getters::Getters;
use std::ffi::OsString;
use std::path::PathBuf;

/// Input options
#[derive(Args, Debug, Getters)]
pub struct InputOptions {
    /// One or more input file_collector or directories, if not specified, the current directory is used.
    #[arg(short = 'i', long = "input", name = "FILE_OR_DIR")]
    inputs: Vec<PathBuf>,

    /// Recursively search for input file_collector in the specified director(y/ies)
    #[arg(short = 'R', long)]
    recursive: bool,

    /// File extensions(s) to accept (must be exact match, i.e.: "jpg" != "jpeg" != "JPG")
    #[arg(short = 'x', long = "extension", value_name = "XTN")]
    extensions: Vec<String>,
}

impl TryFrom<InputOptions> for InputConfig {
    type Error = OliframeError;

    fn try_from(opts: InputOptions) -> Result<Self, Self::Error> {
        let extensions = opts
            .extensions()
            .into_iter()
            .map(|ext| OsString::from(ext.trim_start_matches('.')))
            .collect();

        let inputs = if opts.inputs.is_empty() {
            match std::env::current_dir() {
                Ok(dir) => vec![dir],
                Err(e) => {
                    return Err(OliframeError::InvalidInput(format!(
                        "No input(s) specified and unable to determine current directory: {}",
                        e
                    )))
                }
            }
        } else {
            opts.inputs
        };

        Ok(InputConfig::new(extensions, inputs, opts.recursive))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_input_options() {
        let opts = InputOptions {
            inputs: vec![PathBuf::from("input.jpg")],
            recursive: true,
            extensions: vec!["jpg".into()],
        };

        let config = InputConfig::try_from(opts).expect("Failed to parse input options.");

        assert_eq!(config.extensions(), &["jpg"]);
        assert_eq!(config.inputs(), &[PathBuf::from("input.jpg")]);
        assert_eq!(config.recursive(), true);
    }
}
