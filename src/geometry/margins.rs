//! Margins module.
use crate::errors::OliframeError;
use crate::geometry::border::Border;
use crate::geometry::Size;
use derive_getters::Getters;
use std::str::FromStr;

/// Margins define the relative space around an image, proportionate to the size of the image.
///
/// The user can specify the width of the margins on each side
/// of the image in pixels (px) or percentage (%).
#[derive(Clone, Debug, Getters)]
pub struct Margins {
    values: Vec<f32>,
}

impl Margins {
    pub fn to_border_with_size(&self, size: &Size) -> Border {
        match self.values.len() {
            1 => {
                let side = (self.values[0] * (size.width() + size.height()) as f32 / 2.) as u32;
                Border::new(side, side, side, side)
            }
            2 => {
                let vertical = (self.values[0] * size.height() as f32) as u32;
                let horizontal = (self.values[1] * size.width() as f32) as u32;
                Border::new(vertical, horizontal, vertical, horizontal)
            }
            3 => {
                let top = (self.values[0] * size.height() as f32) as u32;
                let side = (self.values[1] * size.width() as f32) as u32;
                let bottom = (self.values[2] * size.height() as f32) as u32;
                Border::new(top, side, bottom, side)
            }
            4 => {
                let top = (self.values[0] * size.height() as f32) as u32;
                let right = (self.values[1] * size.width() as f32) as u32;
                let bottom = (self.values[2] * size.height() as f32) as u32;
                let left = (self.values[3] * size.width() as f32) as u32;
                Border::new(top, right, bottom, left)
            }
            _ => unreachable!("Invalid number of margin values."),
        }
    }

    fn try_new(values: Vec<f32>) -> Result<Self, OliframeError> {
        let len = values.len();
        if len < 1 || len > 4 {
            return Err(OliframeError::InvalidInput(format!(
                "Margins must be specified with 1-4 positive values (received {} values).",
                len
            )));
        }

        let values = values
            .into_iter()
            .map(|v| if v > 1. { v / 100. } else { v })
            .map(|v| {
                if v < 0. {
                    return Err(OliframeError::InvalidInput(
                        "Negative margin values are not allowed.".to_string(),
                    ));
                }
                Ok(v)
            })
            .collect::<Result<Vec<f32>, OliframeError>>()?;

        Ok(Margins { values })
    }
}

impl Default for Margins {
    fn default() -> Self {
        Margins::try_new(vec![5.]).expect("Failed to create default Margins.")
    }
}

impl FromStr for Margins {
    type Err = OliframeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"(?P<value>-?\d*\.?\d+)").expect("Invalid regex");
        let values = re
            .captures_iter(input)
            .map(|caps| {
                caps.name("value")
                    .expect("Expected value not found.")
                    .as_str()
                    .parse::<f32>()
                    .map_err(|_| OliframeError::InvalidInput("Invalid margin value.".to_string()))
            })
            .collect::<Result<Vec<f32>, OliframeError>>()?;

        Margins::try_new(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_margins() {
        let margins = Margins::default();
        assert_eq!(margins.values(), &[0.05]);
    }

    #[test]
    fn margins_from_str() {
        let margins = Margins::from_str("10").unwrap();
        assert_eq!(margins.values(), &[0.1]);

        let margins = Margins::from_str("10 20").unwrap();
        assert_eq!(margins.values(), &[0.1, 0.2]);

        let margins = Margins::from_str("10 20 30").unwrap();
        assert_eq!(margins.values(), &[0.1, 0.2, 0.3]);

        let margins = Margins::from_str("10 20 30 40").unwrap();
        assert_eq!(margins.values(), &[0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn too_many_margin_values_is_err() {
        let margins = Margins::from_str("10 20 30 40 50");
        assert!(margins.is_err());
    }

    #[test]
    fn negative_margin_values_is_err() {
        let margins = Margins::from_str("-10");
        assert!(margins.is_err());
    }

    #[test]
    fn zero_margin_values_is_err() {
        let margins = Margins::from_str("no numeric values here");
        assert!(margins.is_err());
    }
}
