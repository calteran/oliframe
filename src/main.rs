mod args;
mod file_collector;
mod image_file;

use args::Args;
use clap::Parser;
use file_collector::FileCollector;
use rayon::prelude::*;

fn main() {
    let args = Args::parse();
    let files = FileCollector::from_args(&args).unwrap();

    if args.verbose {
        println!("Processing {} files..", files.len());
    }

    files
        .files()
        .par_iter()
        .for_each(|image| match image.add_border(&args) {
            Ok(_) => {
                if !args.quiet && !args.dry_run {
                    println!("Processed {}", image);
                }
            }
            Err(e) => {
                eprintln!("Unable to process {}: {}", image, e);
            }
        });
}
