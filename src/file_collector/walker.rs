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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn file_walker() {
        let tmpdir = TempDir::with_prefix("file_walker").unwrap();
        let base_path = tmpdir.path().to_owned();
        let _ = vec!["file1.txt", "file2.txt", "file3", ".im_hidden"]
            .into_iter()
            .map(|f| {
                File::create(base_path.join(f))
                    .unwrap()
                    .write_all(&1234_u32.to_be_bytes())
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let inner_dir = TempDir::with_prefix_in("inner_dir", &base_path).unwrap();
        let _ = vec!["file4.txt", "file5.txt"]
            .into_iter()
            .map(|f| {
                File::create(inner_dir.path().join(f))
                    .unwrap()
                    .write_all(&1234_u32.to_be_bytes())
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let walker = path_walker(&base_path, true);

        let paths: Vec<PathBuf> = walker.collect();
        assert_eq!(paths.len(), 5);
        assert!(paths.contains(&tmpdir.path().join("file1.txt")));
        assert!(paths.contains(&tmpdir.path().join("file2.txt")));
        assert!(paths.contains(&tmpdir.path().join("file3")));
        assert!(paths.contains(&inner_dir.path().join("file4.txt")));
        assert!(paths.contains(&inner_dir.path().join("file5.txt")));
        assert!(!paths.contains(&tmpdir.path().join(".im_hidden")));
    }
}
