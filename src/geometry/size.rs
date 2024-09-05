//! A module for parsing and storing size values.
use derive_getters::Getters;

/// A struct for storing size values.
#[derive(Debug, Getters, PartialEq)]
pub struct Size {
    /// The distance from the left to right side.
    width: u32,
    /// The distance from the top to bottom.
    height: u32,
}

impl Size {
    /// Extract the width and height values as a tuple.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Create a new `Size` instance for testing purposes.
    #[cfg(test)]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl From<(u32, u32)> for Size {
    fn from(values: (u32, u32)) -> Self {
        Self {
            width: values.0,
            height: values.1,
        }
    }
}
