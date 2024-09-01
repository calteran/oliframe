//! Frame configuration.
use crate::geometry::{AspectRatio, Margins, RelativePosition, Size};
use derive_getters::Getters;
use image::Rgba;

/// Configuration parameters for the frame.
#[derive(Debug, Getters)]
pub struct FrameConfig {
    aspect_ratio: Option<AspectRatio>,
    color: Rgba<u8>,
    corner_radius: Option<u32>,
    margins: Margins,
    position: Option<RelativePosition>,
    upscale: Option<Size>,
}

impl FrameConfig {
    /// Create a new frame configuration.
    pub fn new(
        aspect_ratio: Option<AspectRatio>,
        color: Rgba<u8>,
        corner_radius: Option<u32>,
        margins: Margins,
        position: Option<RelativePosition>,
        upscale: Option<Size>,
    ) -> Self {
        Self {
            aspect_ratio,
            color,
            corner_radius,
            margins,
            position,
            upscale,
        }
    }
}
