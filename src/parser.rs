use clap::Parser;

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// One or more input files
    #[arg(short, long, value_name = "FILENAME")]
    pub file: Vec<String>,

    /// Directory to search for input files
    #[arg(short, long, value_name = "DIR")]
    pub dir: Option<String>,

    /// File extension of input files to accept (use multiple times for multiple formats)
    #[arg(short = 'x', long, value_name = "FMT")]
    pub extension: Vec<String>,

    /// Output destination
    #[arg(short, long, value_name = "DEST")]
    pub output: Option<String>,

    /// Prefix to prepend to output files
    #[arg(short, long, value_name = "PREFIX")]
    pub prefix: Option<String>,

    /// Suffix to append to output files
    #[arg(short, long, value_name = "SUFFIX")]
    pub suffix: Option<String>,

    /// Border width (in percent, unless --border-px is specified)
    #[arg(short, long, value_name = "WIDTH", default_value = "5")]
    pub width: u32,

    /// Interpret border width in pixels instead of percentage
    #[arg(long)]
    pub border_px: bool,

    /// Border color, name or hex code
    #[arg(short, long, value_name = "COLOR", default_value = "white")]
    pub color: String,

    /// Border corner radius (in pixels; overrides width to be in pixels also)
    #[arg(short, long, value_name = "RADIUS")]
    pub radius: Option<u32>,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Quiet output -- suppresses all output except errors
    #[arg(short, long)]
    pub quiet: bool,

    /// Dry run (don't actually create output files)
    #[arg(long)]
    pub dry_run: bool,

    /// Overwrite existing files.
    /// Default is to ask for each image before overwriting if this flag is not specified.
    #[arg(short = 'y', long)]
    pub overwrite: bool,
}
