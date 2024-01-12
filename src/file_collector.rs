use crate::args::Args;
use crate::image_file::ImageFile;
use junk_file::is_not_junk;
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::path::{Path, PathBuf};

enum OverwriteResult {
    NoConflict,
    Overwrite,
    OverwriteAll,
    Skip,
    Error,
}

pub struct FileCollector {
    files: Vec<ImageFile>,
}

impl FileCollector {
    /// Returns a new FileCollector based on the given Args
    ///
    /// When returned, the FileCollector will contain a list of ImageFile objects
    /// that match the given Args.
    /// For any ImageFile objects that would overwrite an existing file, the user will be prompted
    /// to confirm the overwrite, unless `args.overwrite` is true.
    /// The user will be given the option to overwrite all remaining files each time an overwrite is confirmed.
    pub fn from_args(args: &Args) -> Result<Self, String> {
        let mut overwrite_all = args.overwrite;
        let files = input_files(args)
            .into_iter()
            .filter_map(|input_path| match output_path(args, &input_path) {
                Ok(output_path) => {
                    if overwrite_all {
                        Some(ImageFile::new(input_path, output_path))
                    } else {
                        match validate_overwrite(&output_path) {
                            OverwriteResult::Overwrite | OverwriteResult::NoConflict => {
                                Some(ImageFile::new(input_path, output_path))
                            }
                            OverwriteResult::OverwriteAll => {
                                overwrite_all = true;
                                Some(ImageFile::new(input_path, output_path))
                            }
                            _ => None,
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Unable to get output path for {}: {}",
                        input_path.display(),
                        e
                    );
                    None
                }
            })
            .collect();
        Ok(Self { files })
    }

    /// Returns the number of files in the FileCollector
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns a slice of ImageFile objects in the FileCollector
    pub fn files(&self) -> &[ImageFile] {
        &self.files
    }
}

fn validate_overwrite(output_path: &Path) -> OverwriteResult {
    match output_path.try_exists() {
        Ok(true) => {
            if atty::is(atty::Stream::Stdout) {
                println!(
                    "{} already exists.  Overwrite? (Y)es (N)o (A)ll ",
                    output_path.display()
                );
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                match input.to_lowercase().trim() {
                    "y" | "yes" => OverwriteResult::Overwrite,
                    "a" | "all" => OverwriteResult::OverwriteAll,
                    _ => OverwriteResult::Skip,
                }
            } else {
                eprintln!(
                    "{} already exists.  Run with --overwrite to overwrite.",
                    output_path.display()
                );
                OverwriteResult::Skip
            }
        }
        Ok(false) => OverwriteResult::NoConflict,
        Err(e) => {
            eprintln!(
                "Unable to check for existing file {}: {}",
                output_path.display(),
                e
            );
            OverwriteResult::Error
        }
    }
}

/// Returns a list of input files
///
/// This function will return a list of input files based on the following rules:
///
/// 1. If the `--dir` option is specified, all files in that directory will be returned.
/// 2. If the `--file` option is specified, all files specified will be returned.
/// 3. If both `--dir` and `--file` are specified, all files in the directory will be returned, and all files specified will be returned.
/// 4. If neither `--dir` nor `--file` are specified, all files in the current directory will be returned.
/// 5. If the `--extension` option is specified,
///     only files with the specified extension(s) will be returned.
/// 6. The `junk_file` crate is used to filter out OS-specific junk files.
fn input_files(args: &Args) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    if !args.dir.is_empty() {
        for dir in &args.dir {
            if args.verbose {
                println!(
                    "Searching for files in {}...",
                    dir.canonicalize().unwrap_or_default().display()
                );
            }
            files.append(&mut extract_files(args, dir, 0));
        }
    } else if !args.file.is_empty() {
        // we don't need
        // to screen for junk files here or extensions,
        // because we're explicitly specifying the files
        for file in &args.file {
            files.push(PathBuf::from(file));
        }
    } else {
        // no dir or file specified, so search current directory
        files.append(&mut extract_files(args, &PathBuf::from("."), 0));
    }

    if args.verbose {
        println!("Found {} valid files matching input criteria.", files.len());
    }

    files.sort();
    files
}

/// Given a path, returns a list of files in that path that match the specified extension(s)
fn extract_files(args: &Args, path: &PathBuf, depth: u8) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if path.is_file() && is_not_junk(path.file_name().unwrap_or_default()) {
        if args.extension.is_empty() || has_specified_extension(args, path) {
            files.push(path.to_path_buf());
        }
    } else if (args.recursive || depth == 0) && path.is_dir() {
        match std::fs::read_dir(path) {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(entry) => {
                            files.append(&mut extract_files(args, &entry.path(), depth + 1));
                            // Note:
                            // Integer overflow is not a concern here,
                            // because the only time we could overflow is when we're recursively searching
                            // anyway.
                            // If we overflow, we'll just continue to recurse.
                        }
                        Err(e) => {
                            eprintln!("Unable to read {}: {}", path.display(), e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Unable to read {}: {}", path.display(), e);
            }
        }
    }

    files
}

/// Returns true if the given path's extension matches one of the specified extensions.
fn has_specified_extension(args: &Args, path: &Path) -> bool {
    match path.extension() {
        Some(path_ext) => args.extension.iter().any(|ext| path_ext == OsStr::new(ext)),
        None => false,
    }
}

/// Returns the output path for the given input path
fn output_path(args: &Args, input_path: &Path) -> Result<PathBuf, FileCollectorError> {
    let mut output_path = PathBuf::new();
    let extension = input_path.extension().unwrap_or(OsStr::new(""));
    let stem = if let Some(stem) = input_path.file_stem() {
        stem
    } else {
        return Err(FileCollectorError::InvalidFileStemError(
            input_path.display().to_string(),
        ));
    };

    if let Some(output) = &args.output {
        output_path.push(output);
    } else {
        output_path.push(input_path.parent().unwrap());
    }

    let mut output_file_name = OsString::new();

    if let Some(prefix) = &args.prefix {
        output_file_name.push(prefix);
    }

    output_file_name.push(stem);

    if let Some(suffix) = &args.suffix {
        output_file_name.push(suffix);
    }

    output_file_name.push(".");
    output_file_name.push(extension);
    output_path.push(output_file_name);

    Ok(output_path)
}

#[derive(Debug)]
pub enum FileCollectorError {
    InvalidFileStemError(String),
}

impl Display for FileCollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileCollectorError::InvalidFileStemError(path) => {
                write!(f, "Unable to get file stem for {}", path)
            }
        }
    }
}
