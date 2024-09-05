use derive_getters::Getters;

#[derive(Debug, Getters, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
