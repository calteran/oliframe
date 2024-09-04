//! A module for parsing and storing size values.
use derive_getters::Getters;

/// A struct for storing size values.
#[derive(Debug, Getters)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    /// Extract the width and height values as a tuple.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
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
