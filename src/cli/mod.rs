//! Command line interface

use crate::config::Config;
use crate::errors::OliframeError;
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

impl TryFrom<Cli> for Config {
    type Error = OliframeError;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        Ok(Config::new(
            cli.input_opts.into(),
            cli.output_opts.into(),
            cli.frame_opts.try_into()?,
        ))
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

    #[test]
    fn parse_cli() {
        let opts = vec![
            "oliframe",
            "-i",
            "input.jpg",
            "-R",
            "-x",
            "jpg",
            "-o",
            "output",
            "-f",
            "-p",
            "prefix",
            "-s",
            "suffix",
            "-y",
            "-c",
            "white",
            "-m",
            "10px,20px",
            "-P",
            "top-left",
            "-u",
            "200%",
        ];
        let args = Cli::parse_from(opts);
        let config = Config::try_from(args).unwrap();
        assert_eq!(config.input_config().extensions(), &["jpg"]);
    }
}
