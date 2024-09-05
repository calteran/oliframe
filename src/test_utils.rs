use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use std::fs::create_dir_all;
use std::path::PathBuf;
use tempfile::{Builder, NamedTempFile, TempDir};

pub const TEST_FS_PREFIX: &str = "oliframe_test_";
const HIDDEN_PREFIX: &str = ".oliframe_test_";
const EXTENSIONS: [&str; 8] = ["jpg", "png", "gif", "jpeg", "bmp", "tiff", "txt", "pdf"];
const SAMPLE_DIR: &str = "samples";

pub fn populate_test_directory(base_dir: &TempDir) -> Vec<NamedTempFile> {
    let mut handles = Vec::new();
    EXTENSIONS
        .iter()
        .map(|ext| {
            Builder::new()
                .prefix(TEST_FS_PREFIX)
                .suffix(&format!(".{}", ext))
                .tempfile_in(base_dir.path())
                .unwrap()
        })
        .for_each(|handle| handles.push(handle));

    handles.push(
        Builder::new()
            .prefix(TEST_FS_PREFIX)
            .tempfile_in(base_dir.path())
            .unwrap(),
    ); // file without an extension

    handles.push(
        Builder::new()
            .prefix(HIDDEN_PREFIX)
            .tempfile_in(base_dir.path())
            .unwrap(),
    );

    let crate_sample_dir = PathBuf::from(SAMPLE_DIR).canonicalize().unwrap();
    let test_sample_dir = base_dir.path().join(SAMPLE_DIR);
    create_dir_all(&test_sample_dir).unwrap();
    dir::copy(crate_sample_dir, test_sample_dir, &CopyOptions::default()).unwrap();

    handles
}
