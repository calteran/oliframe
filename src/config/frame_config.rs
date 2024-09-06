//! Frame configuration.

use crate::geometry::{AspectRatio, Margins, RelativePosition};
use derive_getters::Getters;
use image::Rgba;
use std::fmt::Debug;

/// Configuration parameters for the frame.
#[derive(Debug, Getters)]
pub struct FrameConfig {
    /// The desired aspect ratio of the final output image.
    aspect_ratio: Option<AspectRatio>,
    /// The color of the frame around the image
    color: Rgba<u8>,
    /// The radius of the frame's corners.
    corner_radius: Option<u32>,
    /// The relative margins around the image.
    margins: Margins,
    /// The relative position of the image within the frame.
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

impl Default for FrameConfig {
    fn default() -> Self {
        Self {
            aspect_ratio: None,
            color: Rgba([255, 255, 255, 255]),
            corner_radius: None,
            margins: Margins::default(),
            position: RelativePosition::default(),
        }
    }
}
