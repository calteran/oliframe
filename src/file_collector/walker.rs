//! Module responsible for walking directories to locate candidate input paths.

use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Create a new walker for the given path and recursive flag.
pub fn path_walker(base_path: &Path, recursive: bool) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(base_path)
        .max_depth(if recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
}

/// Determine if the given directory entry is hidden.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
