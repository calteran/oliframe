mod parser;

use clap::Parser;
use crossterm::event::read;
use image::RgbaImage;
use indicatif::ParallelProgressIterator;
use parser::Args;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

enum Error {
    OverwriteProhibitedError,
}

fn collect_files(args: &Args) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    if !args.file.is_empty() {
        files.extend(args.file.clone());
    } else if let Some(dir) = &args.dir {
        if !args.quiet {
            println!("Searching for files in {}", dir);
        }
        files.extend(
            std::fs::read_dir(dir)
                .expect("Unable to read directory")
                .map(|entry| {
                    entry
                        .expect("Unable to read directory entry")
                        .path()
                        .display()
                        .to_string()
                }),
        );
    } else {
        files.extend(
            std::fs::read_dir(".")
                .expect("Unable to read directory")
                .map(|entry| {
                    entry
                        .expect("Unable to read directory entry")
                        .path()
                        .display()
                        .to_string()
                }),
        );
    }

    files
}

fn filter_files_by_extension(files: Vec<String>, args: &Args) -> Vec<String> {
    if !args.extension.is_empty() {
        files
            .into_iter()
            .filter(|file| args.extension.iter().any(|ext| file.ends_with(ext)))
            .collect()
    } else {
        files
    }
}

fn create_progress_bar_style(args: &Args) -> indicatif::ProgressStyle {
    if args.verbose {
        indicatif::ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
        )
    } else {
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7}")
    }
    .unwrap()
}

fn get_output_path(path: &Path, args: &Args) -> PathBuf {
    let mut output_path = PathBuf::new();

    if let Some(output) = &args.output {
        output_path.push(output);
    } else if let Some(parent) = path.parent() {
        output_path.push(parent);
    }

    output_path
}

fn get_output_file_name(path: &Path, args: &Args) -> String {
    let mut output_file_name = path
        .file_stem()
        .unwrap_or_else(|| panic!("Unable to get file stem for {}", path.display()))
        .to_str()
        .unwrap_or_else(|| {
            panic!(
                "Unable to convert file stem to string for {}",
                path.display()
            )
        })
        .to_string();

    if let Some(prefix) = &args.prefix {
        output_file_name = format!("{}{}", prefix, output_file_name);
    }
    if let Some(suffix) = &args.suffix {
        output_file_name = format!("{}{}", output_file_name, suffix);
    }

    output_file_name = format!(
        "{}.{}",
        output_file_name,
        path.extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    );

    output_file_name
}

fn write_image(new_img: RgbaImage, output_path: PathBuf, args: &Args) {
    if args.dry_run {
        println!("Would write to {}", output_path.display());
    } else {
        match new_img.save(output_path.clone()) {
            Ok(_) => {
                println!("Wrote to {}", output_path.display());
            }
            Err(e) => {
                eprintln!("Unable to write to {}: {}", output_path.display(), e);
            }
        }
    }
}

fn process_file(path: &Path, args: &Args) {
    match image::open(path) {
        Ok(img) => {
            let new_img = qwikborder::add_border(
                &img,
                args.width as f32,
                args.border_px,
                args.radius,
                args.verbose,
            );

            let mut output_path = get_output_path(path, args);
            let output_file_name = get_output_file_name(path, args);

            output_path.push(output_file_name);

            write_image(new_img, output_path, args);
        }
        Err(e) => {
            eprintln!("Unable to open {}: {}", path.display(), e);
        }
    }
}

fn validate_overwrite(args: &Args, files: &Vec<String>) -> Result<(), Error> {
    if !args.overwrite && args.suffix.is_none() && args.prefix.is_none() && args.output.is_none() {
        println!(
            "This operation will overwrite {} files.  Continue (y/N)?",
            files.len()
        );

        return match read() {
            Ok(event) => match event {
                crossterm::event::Event::Key(key_event) => match key_event.code {
                    crossterm::event::KeyCode::Char('y') => Ok(()),
                    _ => Err(Error::OverwriteProhibitedError),
                },
                _ => Err(Error::OverwriteProhibitedError),
            },
            Err(_) => Err(Error::OverwriteProhibitedError),
        };
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    let files = collect_files(&args);
    let files = filter_files_by_extension(files, &args);
    let bar_style = create_progress_bar_style(&args);

    if let Err(e) = validate_overwrite(&args, &files) {
        match e {
            Error::OverwriteProhibitedError => {
                eprintln!("Overwrite prohibited.  Exiting.");
                std::process::exit(1);
            }
        }
    }

    if args.quiet {
        files
            .par_iter()
            .for_each(|file| process_file(Path::new(file), &args));
    } else {
        files
            .par_iter()
            .progress_with_style(bar_style.clone())
            .for_each(|file| process_file(Path::new(file), &args));
    };
}
