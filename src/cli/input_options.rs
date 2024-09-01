//! Input options

use crate::config::InputConfig;
use clap::Args;
use std::path::PathBuf;

/// Input options
#[derive(Args, Debug)]
pub struct InputOptions {
    /// One or more input files or directories, if not specified, the current directory is used.
    #[arg(short = 'i', long = "input", name = "FILE_OR_DIR")]
    inputs: Vec<PathBuf>,

    /// Recursively search for input files in the specified director(y/ies)
    #[arg(short = 'R', long)]
    recursive: bool,

    /// File extensions(s) to accept (must be exact match, i.e.: "jpg" != "jpeg" != "JPG")
    #[arg(short = 'x', long, value_name = "XTN")]
    extension: Vec<String>,
}

impl From<InputOptions> for InputConfig {
    fn from(opts: InputOptions) -> Self {
        InputConfig::new(opts.extension, opts.inputs, opts.recursive)
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
            extension: vec!["jpg".to_string()],
        };

        let config = InputConfig::from(opts);

        assert_eq!(config.extensions(), &["jpg"]);
        assert_eq!(config.inputs(), &[PathBuf::from("input.jpg")]);
        assert_eq!(config.recursive(), true);
    }
}
