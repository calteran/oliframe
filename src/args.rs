use std::path::PathBuf;
use clap::Parser;


#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// One or more input files
    #[arg(short, long, value_name = "FILENAME", value_parser = path_parser)]
    pub file: Vec<PathBuf>,

    /// Director(y/ies) to search for input files
    #[arg(short, long, value_name = "DIR", value_parser = path_parser)]
    pub dir: Vec<PathBuf>,
    
    /// Recursively search for input files in the specified director(y/ies)
    #[arg(short = 'R', long)]
    pub recursive: bool,

    /// File extension(s) of input files to accept (must be exact match, ie: "jpg" != "jpeg" != "JPG")
    #[arg(short = 'x', long, value_name = "FMT", value_parser = extension_parser)]
    pub extension: Vec<String>,

    /// Output destination
    #[arg(short, long, value_name = "DEST", value_parser = path_parser)]
    pub output: Option<PathBuf>,

    /// Prefix to prepend to output files
    #[arg(short, long, value_name = "PREFIX")]
    pub prefix: Option<String>,

    /// Suffix to append to output files
    #[arg(short, long, value_name = "SUFFIX")]
    pub suffix: Option<String>,

    /// Border width in percent of average dimension, ie: (width + height) / 2.0.  Default is 5%.
    #[arg(short = 'C', long = "pct", conflicts_with = "pixels", value_name = "%WIDTH", default_value = "5")]
    pub percent: Option<u32>,

    /// Border with in pixels, default is 0 (disabled)
    #[arg(short = 'X', long = "px", conflicts_with = "percent", value_name = "PIXELS")]
    pub pixels: Option<u32>,

    /// Border color, name or hex code
    #[arg(short, long, value_name = "COLOR", default_value = "white")]
    pub color: String,

    /// Border corner radius (in pixels; requires --px)
    #[arg(short, long, value_name = "RADIUS", requires = "pixels")]
    pub radius: Option<u32>,

    /// Verbose output
    #[arg(short, long, conflicts_with = "quiet")]
    pub verbose: bool,

    /// Quiet output -- suppresses all output except errors
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Dry run (don't actually create output files)
    #[arg(long)]
    pub dry_run: bool,

    /// Overwrite existing files.
    /// Default is to ask for each image before overwriting if this flag is not specified.
    #[arg(short = 'y', long)]
    pub overwrite: bool,
}

fn extension_parser(ext: &str) -> Result<String, String> {
    // strip leading dot, if present
    let ext = if ext.starts_with('.') {
        &ext[1..]
    } else {
        ext
    };

    Ok(ext.to_string())
}

fn path_parser(path: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(path))
}