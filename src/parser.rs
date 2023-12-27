use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// One or more input files
    #[arg(short, long, value_name = "FILENAME")]
    file: Vec<String>,

    /// Directory to search for input files
    #[arg(short, long, value_name = "DIR")]
    dir: Option<String>,

    /// Output destination
    #[arg(short, long, value_name = "DEST")]
    output: Option<String>,

    /// Prefix to prepend to output files
    #[arg(short, long, value_name = "PREFIX")]
    prefix: Option<String>,

    /// Suffix to append to output files
    #[arg(short, long, value_name = "SUFFIX")]
    suffix: Option<String>,

    /// Border width (in percent, unless --border-px is specified)
    #[arg(short, long, value_name = "WIDTH", default_value = "5")]
    width: u32,

    /// Interpret border width in pixels instead of percentage
    #[arg(long)]
    border_px: bool,

    /// Border color, name or hex code
    #[arg(short, long, value_name = "COLOR", default_value = "white")]
    color: String,

    /// Border corner radius (in pixels; overrides width to be in pixels also)
    #[arg(short, long, value_name = "RADIUS")]
    radius: Option<u32>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Dry run (don't actually create output files)
    #[arg(long)]
    dry_run: bool,
}
