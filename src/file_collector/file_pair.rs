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
            .unwrap_or_else(|_| panic!("Failed to strip prefix: {:?}", input_path));
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

    #[cfg(test)]
    /// Create a new file pair for testing purposes.
    pub fn new(input_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            input_path,
            output_path,
        }
    }
}

/// Determine the output filename for the given input path and output configuration.
fn filename(input_path: &Path, output_config: &OutputConfig) -> OsString {
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
    input_path: &Path,
    relative_path: &Path,
    filename: OsString,
) -> PathBuf {
    output_config
        .output_root()
        .as_ref()
        .map(|root| {
            if output_config.flatten() {
                root.join(&filename)
            } else {
                root.join(relative_path).with_file_name(&filename)
            }
        })
        .unwrap_or_else(|| input_path.with_file_name(filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn build_paths() {
        let base_path = PathBuf::from("/base");
        let input_path = PathBuf::from("/base/input/file.txt");
        let output_config = OutputConfig::default();
        let file_pair = FilePair::build(&base_path, input_path.clone(), &output_config);
        assert_eq!(file_pair.input_path, input_path);
        assert_eq!(file_pair.output_path, input_path);

        let output_config = OutputConfig::new(
            false,
            false,
            Some(PathBuf::from("/output")),
            false,
            Some("prefix".to_string()),
            Some("suffix".to_string()),
        );
        let file_pair = FilePair::build(&base_path, input_path.clone(), &output_config);
        assert_eq!(file_pair.input_path, input_path);
        assert_eq!(
            file_pair.output_path,
            PathBuf::from("/output/input/prefixfilesuffix.txt")
        );

        let output_config = OutputConfig::new(
            false,
            true,
            Some(PathBuf::from("/output")),
            false,
            Some("prefix".to_string()),
            Some("suffix".to_string()),
        );
        let file_pair = FilePair::build(&base_path, input_path.clone(), &output_config);
        assert_eq!(file_pair.input_path, input_path);
        assert_eq!(
            file_pair.output_path,
            PathBuf::from("/output/prefixfilesuffix.txt")
        );
    }

    #[test]
    fn pair_into_parts() {
        let input_path = PathBuf::from("/base/input/file.txt");
        let output_path = PathBuf::from("/output/input/file.txt");
        let file_pair = FilePair {
            input_path,
            output_path,
        };
        let (input, output) = file_pair.into_parts();
        assert_eq!(input, PathBuf::from("/base/input/file.txt"));
        assert_eq!(output, PathBuf::from("/output/input/file.txt"));
    }
}
