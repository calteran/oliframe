//! Output configuration
use derive_getters::Getters;
use std::path::PathBuf;

/// Output configuration parameters.
#[derive(Clone, Debug, Default, Getters)]
pub struct OutputConfig {
    /// Whether to suppress writing to the filesystem.
    dry_run: bool,
    /// Whether to flatten the output directory.
    flatten: bool,
    /// The root directory for output files.
    output_root: Option<PathBuf>,
    /// Whether to overwrite existing files.
    overwrite: bool,
    /// A prefix to add to output filenames.
    prefix: Option<String>,
    /// A suffix to add to output filenames.
    suffix: Option<String>,
}

impl OutputConfig {
    /// Create a new output configuration.
    pub fn new(
        dry_run: bool,
        flatten: bool,
        output_root: Option<PathBuf>,
        overwrite: bool,
        prefix: Option<String>,
        suffix: Option<String>,
    ) -> Self {
        Self {
            dry_run,
            flatten,
            output_root,
            overwrite,
            prefix,
            suffix,
        }
    }
}
