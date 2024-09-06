#![warn(
    missing_docs,
    missing_debug_implementations,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
//! A command-line tool to add a frame around images.
//!
//! Oliframe places a simple colored border or frame around images,
//! with a wide range of options for customizing the frame, as inspired
//! by [`@officialmumbo`](https://www.instagram.com/officialmumbo/) on Instagram.
//!

use env_logger::Builder as LogBuilder;
use rayon::prelude::*;

use crate::cli::*;
use crate::config::Config;
use crate::file_collector::FileCollector;
use crate::frame::Frame;

mod cli;
mod config;
mod errors;
mod file_collector;
mod frame;
mod geometry;
#[cfg(test)]
mod test_utils;

#[cfg(not(tarpaulin_include))]
fn main() {
    let args = Cli::parse();

    LogBuilder::new()
        .filter_level(args.verbosity().log_level())
        .parse_env("RUST_LOG")
        .init();

    let config = match Config::try_from(args) {
        Ok(c) => c,
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1);
        }
    };

    log::debug!("Configuration: {:#?}", config);

    FileCollector::collect(&config)
        .into_par_iter()
        .for_each(|file_pair| {
            Frame::process(
                file_pair,
                config.frame_config(),
                config.output_config().dry_run(),
            )
            .unwrap_or_else(|e| log::error!("{}", e));
        });
}
