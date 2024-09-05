//! A point is a location in a two-dimensional space.
use derive_getters::Getters;

/// A point is a location in a two-dimensional space.
#[derive(Debug, Getters, PartialEq)]
pub struct Point {
    /// The x-coordinate of the point.
    x: u32,
    /// The y-coordinate of the point.
    y: u32,
}

impl Point {
    /// Create a new point with the given coordinates.
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
