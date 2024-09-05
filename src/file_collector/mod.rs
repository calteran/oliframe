//! Module to collect the file paths needed to process the input images into their outputs.

mod file_pair;
mod walker;

use crate::config::{Config, InputConfig, OutputConfig};
pub use file_pair::FilePair;
use std::ffi::OsString;
use std::path::PathBuf;

/// Struct responsible for building a list of input/output file paths
#[derive(Debug)]
pub struct FileCollector;

impl FileCollector {
    /// Collect the file pairs
    pub fn collect(config: &Config) -> Vec<FilePair> {
        let (input_config, output_config) = (config.input_config(), config.output_config());
        input_config
            .inputs()
            .iter()
            .flat_map(|base_path| collect_for_base_path(base_path, input_config, output_config))
            .filter(|file_pair| {
                authorize_overwrite(&file_pair.output_path(), output_config.overwrite())
            })
            .collect()
    }
}

fn collect_for_base_path<'a>(
    base_path: &'a PathBuf,
    input_config: &'a InputConfig,
    output_config: &'a OutputConfig,
) -> impl Iterator<Item = FilePair> + 'a {
    walker::path_walker(base_path, input_config.recursive())
        .filter(|file| match_extensions(file, &input_config.extensions()))
        .map(|input_path| FilePair::build(base_path, input_path, output_config))
}

fn authorize_overwrite(path: &PathBuf, overwrite: bool) -> bool {
    if !path.exists() {
        return true;
    }

    if overwrite {
        log::warn!("Overwriting existing file: {:?}", path);
        return true;
    }

    #[rustfmt::skip] // When the string is on its own line, tarpaulin thinks it's untested
    log::warn!("File already exists: {:?}.  Run with --y to replace it.", path);
    false
}

fn match_extensions(entry: &PathBuf, extensions: &[OsString]) -> bool {
    if extensions.is_empty() {
        return true;
    }
    if let Some(ext) = entry.extension() {
        extensions.iter().any(|e| e == ext)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FrameConfig;
    use tempfile::TempDir;

    #[test]
    fn collect_files_recursively() {
        let base_dir = TempDir::with_prefix(crate::test_utils::TEST_FS_PREFIX).unwrap();
        let _file_handles = crate::test_utils::populate_test_directory(&base_dir);
        let input_config = InputConfig::new(Vec::new(), vec![base_dir.path().to_path_buf()], true);
        let output_config = OutputConfig::new(
            false,
            false,
            Some(PathBuf::from("/output")),
            false,
            None,
            None,
        );
        let config = Config::new(input_config, output_config, FrameConfig::default());

        let file_pairs = FileCollector::collect(&config);
        assert_ne!(file_pairs.len(), 0);
        assert!(file_pairs
            .iter()
            .all(|file_pair| file_pair.input_path().exists()));
        assert!(file_pairs
            .iter()
            .any(|file_pair| file_pair.input_path().parent() == Some(base_dir.path())));
        assert!(file_pairs.iter().any(|file_pair| file_pair
            .input_path()
            .components()
            .any(|c| c.as_os_str() == "samples")));
    }

    #[test]
    fn collect_files_non_recursively() {
        let base_dir = TempDir::with_prefix(crate::test_utils::TEST_FS_PREFIX).unwrap();
        let _file_handles = crate::test_utils::populate_test_directory(&base_dir);
        let input_config = InputConfig::new(Vec::new(), vec![base_dir.path().to_path_buf()], false);
        let output_config = OutputConfig::new(
            false,
            false,
            Some(PathBuf::from("/output")),
            false,
            None,
            None,
        );
        let config = Config::new(input_config, output_config, FrameConfig::default());

        let file_pairs = FileCollector::collect(&config);
        assert_ne!(file_pairs.len(), 0);
        assert!(file_pairs
            .iter()
            .all(|file_pair| file_pair.input_path().exists()));
        assert!(file_pairs.iter().all(|file_pair| file_pair
            .input_path()
            .components()
            .all(|c| c.as_os_str() != "samples")));
    }

    #[test]
    fn overwrite_protection() {
        let _ = env_logger::builder().is_test(true).try_init();
        let base_dir = TempDir::with_prefix(crate::test_utils::TEST_FS_PREFIX).unwrap();
        let _file_handles = crate::test_utils::populate_test_directory(&base_dir);
        let input_config = InputConfig::new(Vec::new(), vec![base_dir.path().to_path_buf()], false);
        let output_config_overwrite_prohibited =
            OutputConfig::new(false, false, None, false, None, None);
        let config = Config::new(
            input_config.clone(),
            output_config_overwrite_prohibited,
            FrameConfig::default(),
        );

        let file_pairs = FileCollector::collect(&config);
        assert_eq!(file_pairs.len(), 0);

        let output_config_overwrite_allowed =
            OutputConfig::new(false, false, None, true, None, None);
        let config = Config::new(
            input_config,
            output_config_overwrite_allowed,
            FrameConfig::default(),
        );

        let file_pairs = FileCollector::collect(&config);
        assert_ne!(file_pairs.len(), 0);
    }

    #[test]
    fn collect_extensions() {
        let base_dir = TempDir::with_prefix(crate::test_utils::TEST_FS_PREFIX).unwrap();
        let _file_handles = crate::test_utils::populate_test_directory(&base_dir);
        let input_config = InputConfig::new(
            vec![OsString::from("jpg"), OsString::from("png")],
            vec![base_dir.path().to_path_buf()],
            false,
        );
        let output_config = OutputConfig::new(
            false,
            false,
            Some(PathBuf::from("/output")),
            false,
            None,
            None,
        );
        let config = Config::new(input_config, output_config, FrameConfig::default());

        let file_pairs = FileCollector::collect(&config);
        assert_ne!(file_pairs.len(), 0);
        assert!(file_pairs
            .iter()
            .all(|file_pair| file_pair.input_path().exists()));
        assert!(file_pairs.iter().all(|file_pair| file_pair
            .input_path()
            .extension()
            .map(|ext| ext == "jpg" || ext == "png")
            .unwrap_or(false)));
    }
}
