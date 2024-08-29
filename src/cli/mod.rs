//! Command line interface

pub use clap::Parser;
use frame_options::FrameOptions;
use input_options::InputOptions;
use output_options::OutputOptions;
use verbosity_options::VerbosityOptions;

mod frame_options;
mod input_options;
mod output_options;
mod verbosity_options;

/// Command line interface options
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Verbosity options
    #[command(flatten)]
    verbosity: VerbosityOptions,

    /// Input options
    #[command(flatten, next_help_heading = "Input Options")]
    input_opts: InputOptions,

    /// Output options
    #[command(flatten, next_help_heading = "Output Options")]
    output_opts: OutputOptions,

    /// Frame options
    #[command(flatten, next_help_heading = "Framing Options")]
    frame_opts: FrameOptions,
}

impl Cli {
    /// Returns the verbosity options
    pub fn verbosity(&self) -> &VerbosityOptions {
        &self.verbosity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
