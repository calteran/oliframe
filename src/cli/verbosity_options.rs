//! Verbosity options
use clap::Args;

/// Verbosity options
#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct VerbosityOptions {
    /// Verbose output
    #[arg(short = 'v', long, conflicts_with = "quiet")]
    verbose: bool,

    /// Quiet output -- suppresses everything except errors
    #[arg(short = 'q', long, conflicts_with = "verbose")]
    quiet: bool,
}

impl VerbosityOptions {
    /// Returns a log-level filter based on the verbosity options
    pub fn log_level(&self) -> log::LevelFilter {
        if self.quiet {
            log::LevelFilter::Error
        } else if self.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiet_option_sets_log_level_to_error() {
        let opts = VerbosityOptions {
            quiet: true,
            verbose: false,
        };
        assert_eq!(opts.log_level(), log::LevelFilter::Error);
    }

    #[test]
    fn verbose_option_sets_log_level_to_debug() {
        let opts = VerbosityOptions {
            quiet: false,
            verbose: true,
        };
        assert_eq!(opts.log_level(), log::LevelFilter::Debug);
    }

    #[test]
    fn default_log_level_is_info() {
        let opts = VerbosityOptions {
            quiet: false,
            verbose: false,
        };
        assert_eq!(opts.log_level(), log::LevelFilter::Info);
    }
}
