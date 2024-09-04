use derive_getters::Getters;

#[derive(Debug, Getters)]
pub struct Border {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
}

impl Border {
    pub fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Border {
            top,
            right,
            bottom,
            left,
        }
    }
}
