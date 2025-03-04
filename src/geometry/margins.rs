//! Margins module.
use crate::errors::OliframeError;
use crate::geometry::Size;
use crate::geometry::border::Border;
use derive_getters::Getters;
use std::str::FromStr;

/// Margins define the relative space around an image, proportionate to the size of the image.
///
/// The user can specify the width of the margins on each side
/// of the image in percentage (%).
#[derive(Clone, Debug, Getters)]
pub struct Margins {
    /// The values of the margins.
    ///
    /// There should be between one and four values.
    /// - one value: all sides have the same margin.
    /// - two values: top/bottom and left/right margins are the same.
    /// - three values: top, left/right, and bottom margins are different.
    /// - four values: top, right, bottom, and left margins are different.
    values: Vec<f32>,
}

impl Margins {
    /// Given the size of a specific image, return the pixel-specific border that represents the margins.
    pub fn to_border_with_size(&self, size: &Size) -> Border {
        let dim = (size.width() + size.height()) as f32 / 2.;
        match self.values.len() {
            1 => {
                let side = (self.values[0] * dim).round() as u32;
                Border::new(side, side, side, side)
            }
            2 => {
                let vertical = (self.values[0] * dim).round() as u32;
                let horizontal = (self.values[1] * dim).round() as u32;
                Border::new(vertical, horizontal, vertical, horizontal)
            }
            3 => {
                let top = (self.values[0] * dim).round() as u32;
                let side = (self.values[1] * dim).round() as u32;
                let bottom = (self.values[2] * dim).round() as u32;
                Border::new(top, side, bottom, side)
            }
            4 => {
                let top = (self.values[0] * dim).round() as u32;
                let right = (self.values[1] * dim).round() as u32;
                let bottom = (self.values[2] * dim).round() as u32;
                let left = (self.values[3] * dim).round() as u32;
                Border::new(top, right, bottom, left)
            }
            _ => unreachable!("Invalid number of margin values."),
        }
    }

    /// Create a new Margins instance with the given values,
    /// ensuring that there are between one and four values.
    ///
    /// Values are converted to fractions if they are greater than 1.
    fn try_new(values: Vec<f32>) -> Result<Self, OliframeError> {
        let len = values.len();
        if !(1..=4).contains(&len) {
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

    #[test]
    fn margins_to_border() {
        let margins = Margins::from_str("10").unwrap();
        let size = Size::new(100, 200);
        let border = margins.to_border_with_size(&size);
        assert_eq!(border.top(), 15);
        assert_eq!(border.right(), 15);
        assert_eq!(border.bottom(), 15);
        assert_eq!(border.left(), 15);

        let margins = Margins::from_str("10 20").unwrap();
        let border = margins.to_border_with_size(&size);
        assert_eq!(border.top(), 15);
        assert_eq!(border.right(), 30);
        assert_eq!(border.bottom(), 15);
        assert_eq!(border.left(), 30);

        let margins = Margins::from_str("10 20 30").unwrap();
        let border = margins.to_border_with_size(&size);
        assert_eq!(border.top(), 15);
        assert_eq!(border.right(), 30);
        assert_eq!(border.bottom(), 45);
        assert_eq!(border.left(), 30);

        let margins = Margins::from_str("10 20 30 40").unwrap();
        let border = margins.to_border_with_size(&size);
        assert_eq!(border.top(), 15);
        assert_eq!(border.right(), 30);
        assert_eq!(border.bottom(), 45);
        assert_eq!(border.left(), 60);
    }
}
