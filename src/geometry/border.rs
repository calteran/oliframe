//! A border is the specific pixel widths of the top, right, bottom,
//! and left sides surrounding the image.
use derive_getters::Getters;

/// A border is the specific pixel widths of the top, right, bottom,
/// and left sides of surrounding the image.
#[derive(Debug, Getters)]
pub struct Border {
    /// The width of the top border.
    top: u32,
    /// The width of the right border.
    right: u32,
    /// The width of the bottom border.
    bottom: u32,
    /// The width of the left border.
    left: u32,
}

impl Border {
    /// Create a new border with the given widths.
    pub fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Border {
            top,
            right,
            bottom,
            left,
        }
    }
}
