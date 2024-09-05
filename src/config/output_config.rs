//! Output configuration
use derive_getters::Getters;
use std::path::PathBuf;

/// Output configuration parameters.
#[derive(Clone, Debug, Default, Getters)]
pub struct OutputConfig {
    dry_run: bool,
    flatten: bool,
    output_root: Option<PathBuf>,
    overwrite: bool,
    prefix: Option<String>,
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
