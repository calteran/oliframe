//! Error types for the oliframe crate.

/// Standard error type for the oliframe crate.
#[derive(Debug, thiserror::Error)]
pub enum OliframeError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
