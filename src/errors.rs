//! Error types for the oliframe crate.

use std::path::PathBuf;

/// Standard error type for the oliframe crate.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OliframeError {
    /// An error occurred while reading or decoding an image file.
    #[error("Unable to open image file at: {0}")]
    ImageUnreadable(PathBuf),
    /// User input was invalid.
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    /// An error occurred while saving an image.
    #[error("Unable to save image: {0}. {1}")]
    SaveError(PathBuf, String),
}
