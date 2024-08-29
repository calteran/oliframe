//! Output options
use clap::Args;
use std::path::PathBuf;

/// Output options
#[derive(Args, Debug)]
pub struct OutputOptions {
    /// Dry run (don't create output files)
    #[arg(long)]
    dry_run: bool,

    /// Output destination.  
    /// If skipped, each output file will be saved in the same directory as the input file.
    #[arg(short = 'o', long, value_name = "FILE/FOLDER")]
    output: Option<PathBuf>,

    /// Flatten the output directory structure when processing multiple input files
    #[arg(short = 'f', long, requires = "output", requires = "recursive")]
    flatten: bool,

    /// Prefix to prepend to output files
    #[arg(short = 'p', long, value_name = "PREFIX")]
    prefix: Option<String>,

    /// Suffix to append to output files
    #[arg(short = 's', long, value_name = "SUFFIX")]
    suffix: Option<String>,

    /// Overwrite existing files.  If not specified, existing files will not be overwritten.
    #[arg(short = 'y', long)]
    overwrite: bool,
}
