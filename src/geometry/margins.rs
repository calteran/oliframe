//! Margins module.
use crate::errors::OliframeError;
use crate::geometry::{Size, Unit};
use derive_getters::{Dissolve, Getters};
use std::str::FromStr;

/// Margins define the space around an image.
///
/// The user can specify the width of the margins on each side
/// of the image in pixels (px) or percentage (%).
#[derive(Clone, Debug, Dissolve, Getters)]
pub struct Margins {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
    unit: Unit,
}

impl Margins {
    pub fn for_size(&self, size: &Size) -> Margins {
        match self.unit() {
            Unit::Pixel => self.clone(),
            Unit::Percent => {
                let (width, height) = size.dimensions();
                let top = (self.top() as f32 / 100.0 * height as f32).round() as u32;
                let right = (self.right() as f32 / 100.0 * width as f32).round() as u32;
                let bottom = (self.bottom() as f32 / 100.0 * height as f32).round() as u32;
                let left = (self.left() as f32 / 100.0 * width as f32).round() as u32;
                Margins {
                    top,
                    right,
                    bottom,
                    left,
                    unit: Unit::Pixel,
                }
            }
        }
    }
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 5,
            right: 5,
            bottom: 5,
            left: 5,
            unit: Unit::Percent,
        }
    }
}

impl FromStr for Margins {
    type Err = OliframeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (values, unit) = super::parse_values_and_units(s).map_err(|e| match e {
            super::ParseError::InvalidValue => OliframeError::InvalidInput(
                "Please provide a positive whole number for the margin value.".to_string(),
            ),
            super::ParseError::InvalidUnit => OliframeError::InvalidInput(
                "Please provide a valid unit for the margin value.".to_string(),
            ),
            super::ParseError::UnitConflict => OliframeError::InvalidInput(
                "All margin values must have the same unit.".to_string(),
            ),
        })?;

        let (top, right, bottom, left) = match values.len() {
            1 => (values[0], values[0], values[0], values[0]),
            2 => (values[0], values[1], values[0], values[1]),
            3 => (values[0], values[1], values[2], values[1]),
            4 => (values[0], values[1], values[2], values[3]),
            _ => {
                return Err(OliframeError::InvalidInput(
                    "Please provide one, two, three, or four margin values.".to_string(),
                ))
            }
        };

        Ok(Self {
            top,
            right,
            bottom,
            left,
            unit: unit.unwrap_or(Unit::Percent),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pixel_values_from_string() {
        let one_value_margin = "10px".parse::<Margins>().unwrap();
        assert_eq!(one_value_margin.top, 10);
        assert_eq!(one_value_margin.right, 10);
        assert_eq!(one_value_margin.bottom, 10);
        assert_eq!(one_value_margin.left, 10);
        assert_eq!(one_value_margin.unit, Unit::Pixel);

        let two_value_margin = "10px,20px".parse::<Margins>().unwrap();
        assert_eq!(two_value_margin.top, 10);
        assert_eq!(two_value_margin.right, 20);
        assert_eq!(two_value_margin.bottom, 10);
        assert_eq!(two_value_margin.left, 20);
        assert_eq!(two_value_margin.unit, Unit::Pixel);

        let three_value_margin = "10px,20px,30px".parse::<Margins>().unwrap();
        assert_eq!(three_value_margin.top, 10);
        assert_eq!(three_value_margin.right, 20);
        assert_eq!(three_value_margin.bottom, 30);
        assert_eq!(three_value_margin.left, 20);
        assert_eq!(three_value_margin.unit, Unit::Pixel);

        let four_value_margin = "10px,20px,30px,40px".parse::<Margins>().unwrap();
        assert_eq!(four_value_margin.top, 10);
        assert_eq!(four_value_margin.right, 20);
        assert_eq!(four_value_margin.bottom, 30);
        assert_eq!(four_value_margin.left, 40);
        assert_eq!(four_value_margin.unit, Unit::Pixel);
    }

    #[test]
    fn parse_percent_values_from_string() {
        let one_value_margin = "10%".parse::<Margins>().unwrap();
        assert_eq!(one_value_margin.top, 10);
        assert_eq!(one_value_margin.right, 10);
        assert_eq!(one_value_margin.bottom, 10);
        assert_eq!(one_value_margin.left, 10);
        assert_eq!(one_value_margin.unit, Unit::Percent);

        let two_value_margin = "10%,20%".parse::<Margins>().unwrap();
        assert_eq!(two_value_margin.top, 10);
        assert_eq!(two_value_margin.right, 20);
        assert_eq!(two_value_margin.bottom, 10);
        assert_eq!(two_value_margin.left, 20);
        assert_eq!(two_value_margin.unit, Unit::Percent);

        let three_value_margin = "10%,20%,30%".parse::<Margins>().unwrap();
        assert_eq!(three_value_margin.top, 10);
        assert_eq!(three_value_margin.right, 20);
        assert_eq!(three_value_margin.bottom, 30);
        assert_eq!(three_value_margin.left, 20);
        assert_eq!(three_value_margin.unit, Unit::Percent);

        let four_value_margin = "10%,20%,30%,40%".parse::<Margins>().unwrap();
        assert_eq!(four_value_margin.top, 10);
        assert_eq!(four_value_margin.right, 20);
        assert_eq!(four_value_margin.bottom, 30);
        assert_eq!(four_value_margin.left, 40);
        assert_eq!(four_value_margin.unit, Unit::Percent);
    }

    #[test]
    fn parse_unitless_values_from_string_as_percent() {
        let one_value_margin = "10".parse::<Margins>().unwrap();
        assert_eq!(one_value_margin.top, 10);
        assert_eq!(one_value_margin.right, 10);
        assert_eq!(one_value_margin.bottom, 10);
        assert_eq!(one_value_margin.left, 10);
        assert_eq!(one_value_margin.unit, Unit::Percent);

        let two_value_margin = "10,20".parse::<Margins>().unwrap();
        assert_eq!(two_value_margin.top, 10);
        assert_eq!(two_value_margin.right, 20);
        assert_eq!(two_value_margin.bottom, 10);
        assert_eq!(two_value_margin.left, 20);
        assert_eq!(two_value_margin.unit, Unit::Percent);

        let three_value_margin = "10,20,30".parse::<Margins>().unwrap();
        assert_eq!(three_value_margin.top, 10);
        assert_eq!(three_value_margin.right, 20);
        assert_eq!(three_value_margin.bottom, 30);
        assert_eq!(three_value_margin.left, 20);
        assert_eq!(three_value_margin.unit, Unit::Percent);

        let four_value_margin = "10,20,30,40".parse::<Margins>().unwrap();
        assert_eq!(four_value_margin.top, 10);
        assert_eq!(four_value_margin.right, 20);
        assert_eq!(four_value_margin.bottom, 30);
        assert_eq!(four_value_margin.left, 40);
        assert_eq!(four_value_margin.unit, Unit::Percent);
    }

    #[test]
    fn mixed_unit_values_are_invalid() {
        let mixed_units = "10px,20%,30px".parse::<Margins>();
        assert!(mixed_units.is_err());
        assert_eq!(
            mixed_units.unwrap_err().to_string(),
            "Invalid input: All margin values must have the same unit."
        );
    }

    #[test]
    fn decimal_values_are_invalid() {
        let decimal_values = "10.5,20.5,30.5,40.5".parse::<Margins>();
        assert!(decimal_values.is_err());
        assert_eq!(
            decimal_values.unwrap_err().to_string(),
            "Invalid input: Please provide a positive whole number for the margin value."
        );
    }

    #[test]
    fn negative_values_are_invalid() {
        let negative_values = "-10,-20,-30,-40".parse::<Margins>();
        assert!(negative_values.is_err());
        assert_eq!(
            negative_values.unwrap_err().to_string(),
            "Invalid input: Please provide a positive whole number for the margin value."
        );
    }
}
