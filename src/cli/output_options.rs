//! Output options
use crate::config::OutputConfig;
use clap::Args;
use std::path::PathBuf;

/// Output options
#[derive(Args, Debug)]
pub struct OutputOptions {
    /// Dry run (don't create output file_collector)
    #[arg(long)]
    dry_run: bool,

    /// Output destination.  
    /// If skipped, each output file will be saved in the same directory as the input file.
    #[arg(short = 'o', long, value_name = "FILE/FOLDER")]
    output: Option<PathBuf>,

    /// Flatten the output directory structure when processing multiple input file_collector
    #[arg(short = 'f', long, requires = "output", requires = "recursive")]
    flatten: bool,

    /// Prefix to prepend to output file_collector
    #[arg(short = 'p', long, value_name = "PREFIX")]
    prefix: Option<String>,

    /// Suffix to append to output file_collector
    #[arg(short = 's', long, value_name = "SUFFIX")]
    suffix: Option<String>,

    /// Overwrite existing files.  Defaults to no.
    #[arg(short = 'y', long)]
    overwrite: bool,
}

impl From<OutputOptions> for OutputConfig {
    fn from(opts: OutputOptions) -> Self {
        OutputConfig::new(
            opts.dry_run,
            opts.flatten,
            opts.output,
            opts.overwrite,
            opts.prefix,
            opts.suffix,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_output_options() {
        let opts = OutputOptions {
            dry_run: true,
            output: Some(PathBuf::from("output")),
            flatten: true,
            prefix: Some("prefix".to_string()),
            suffix: Some("suffix".to_string()),
            overwrite: true,
        };

        let config = OutputConfig::from(opts);

        assert_eq!(config.dry_run(), true);
        assert_eq!(config.flatten(), true);
        assert_eq!(config.output_root(), &Some(PathBuf::from("output")));
        assert_eq!(config.overwrite(), true);
        assert_eq!(config.prefix(), &Some("prefix".to_string()));
        assert_eq!(config.suffix(), &Some("suffix".to_string()));
    }
}
