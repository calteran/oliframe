//! A module for parsing and storing size values.
use crate::errors::OliframeError;
use crate::geometry::Unit;
use derive_getters::Getters;
use std::str::FromStr;

/// A struct for storing size values.
#[derive(Debug, Getters)]
pub struct Size {
    width: u32,
    height: u32,
    unit: Unit,
}

impl FromStr for Size {
    type Err = OliframeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (values, unit) = super::parse_values_and_units(input).map_err(|e| match e {
            super::ParseError::InvalidValue => OliframeError::InvalidInput(
                "Please provide a positive whole number for the size value.".to_string(),
            ),
            super::ParseError::InvalidUnit => OliframeError::InvalidInput(
                "Please provide a valid unit for the size value.".to_string(),
            ),
            super::ParseError::UnitConflict => {
                OliframeError::InvalidInput("All size values must have the same unit.".to_string())
            }
        })?;

        let unit = unit.unwrap_or(Unit::Pixel);

        match values.len() {
            1 => Ok(Self {
                width: values[0],
                height: values[0],
                unit,
            }),
            2 => Ok(Self {
                width: values[0],
                height: values[1],
                unit,
            }),
            _ => Err(OliframeError::InvalidInput(
                "Please provide one or two size values.".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pixel_values_from_string() {
        let size = "200px".parse::<Size>().unwrap();
        assert_eq!(size.width, 200);
        assert_eq!(size.height, 200);
        assert_eq!(size.unit, Unit::Pixel);

        let size = "200px,100px".parse::<Size>().unwrap();
        assert_eq!(size.width, 200);
        assert_eq!(size.height, 100);
        assert_eq!(size.unit, Unit::Pixel);

        let size = "200,100px".parse::<Size>().unwrap();
        assert_eq!(size.width, 200);
        assert_eq!(size.height, 100);
        assert_eq!(size.unit, Unit::Pixel);
    }

    #[test]
    fn parse_percent_values_from_string() {
        let size = "50%".parse::<Size>().unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 50);
        assert_eq!(size.unit, Unit::Percent);

        let size = "50%,25%".parse::<Size>().unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 25);
        assert_eq!(size.unit, Unit::Percent);

        let size = "50,25%".parse::<Size>().unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 25);
        assert_eq!(size.unit, Unit::Percent);
    }

    #[test]
    fn parse_unitless_values_as_pixels() {
        let size = "200".parse::<Size>().unwrap();
        assert_eq!(size.width, 200);
        assert_eq!(size.height, 200);
        assert_eq!(size.unit, Unit::Pixel);

        let size = "200,100".parse::<Size>().unwrap();
        assert_eq!(size.width, 200);
        assert_eq!(size.height, 100);
        assert_eq!(size.unit, Unit::Pixel);
    }

    #[test]
    fn conflicting_units_are_err() {
        let size = "200px,100%".parse::<Size>();
        assert!(size.is_err());
        assert_eq!(
            size.unwrap_err().to_string(),
            "Invalid input: All size values must have the same unit."
        );
    }

    #[test]
    fn more_than_two_values_are_err() {
        let size = "200,100,50".parse::<Size>();
        assert!(size.is_err());
        assert_eq!(
            size.unwrap_err().to_string(),
            "Invalid input: Please provide one or two size values."
        );
    }

    #[test]
    fn decimal_values_are_err() {
        let size = "200.5,100.5".parse::<Size>();
        assert!(size.is_err());
        assert_eq!(
            size.unwrap_err().to_string(),
            "Invalid input: Please provide a positive whole number for the size value."
        );
    }
}
