//! Error types for the oliframe crate.

use std::path::PathBuf;

/// Standard error type for the oliframe crate.
#[derive(Debug, thiserror::Error)]
pub enum OliframeError {
    #[error("Unable to open image file at: {0}")]
    ImageUnreadable(PathBuf),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unable to save image: {0}. {1}")]
    SaveError(PathBuf, String),
}
