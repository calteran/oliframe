//! Frame configuration.
use crate::geometry::{AspectRatio, Margins, RelativePosition};
use derive_getters::Getters;
use image::Rgba;

/// Configuration parameters for the frame.
#[derive(Debug, Getters)]
pub struct FrameConfig {
    aspect_ratio: Option<AspectRatio>,
    color: Rgba<u8>,
    corner_radius: Option<u32>,
    margins: Margins,
    position: RelativePosition,
}

impl FrameConfig {
    /// Create a new frame configuration.
    pub fn new(
        aspect_ratio: Option<AspectRatio>,
        color: Rgba<u8>,
        corner_radius: Option<u32>,
        margins: Margins,
        position: RelativePosition,
    ) -> Self {
        Self {
            aspect_ratio,
            color,
            corner_radius,
            margins,
            position,
        }
    }
}
