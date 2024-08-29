//! Input options

use clap::Args;
use std::path::PathBuf;

/// Input options
#[derive(Args, Debug)]
pub struct InputOptions {
    /// One or more input files or directories
    #[arg(short = 'i', long = "input", name = "FILE_OR_DIR", required = true)]
    inputs: Vec<PathBuf>,

    /// Recursively search for input files in the specified director(y/ies)
    #[arg(short = 'R', long)]
    recursive: bool,

    /// File extensions(s) to accept (must be exact match, i.e.: "jpg" != "jpeg" != "JPG")
    #[arg(short = 'x', long, value_name = "XTN")]
    extension: Vec<String>,
}
