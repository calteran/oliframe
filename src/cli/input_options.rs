//! Input options

use crate::config::InputConfig;
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

impl From<InputOptions> for InputConfig {
    fn from(opts: InputOptions) -> Self {
        let extensions = opts
            .extensions()
            .iter()
            .map(|ext| OsString::from(ext.trim_start_matches('.')))
            .collect();

        let inputs = if opts.inputs.is_empty() {
            vec![std::env::current_dir().expect("Failed to determine current directory.")]
        } else {
            opts.inputs
        };

        InputConfig::new(extensions, inputs, opts.recursive)
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

        let config = InputConfig::from(opts);

        assert_eq!(config.extensions(), &["jpg"]);
        assert_eq!(config.inputs(), &[PathBuf::from("input.jpg")]);
        assert_eq!(config.recursive(), true);
    }

    #[test]
    fn parse_input_options_no_inputs() {
        let opts = InputOptions {
            inputs: vec![],
            recursive: true,
            extensions: vec!["jpg".into()],
        };

        let config = InputConfig::from(opts);

        assert_eq!(config.extensions(), &["jpg"]);
        assert_eq!(
            config.inputs(),
            &[std::env::current_dir().expect("Failed to determine current directory.")]
        );
        assert_eq!(config.recursive(), true);
    }
}
