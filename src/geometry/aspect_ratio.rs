//! Aspect ratio representation and parsing.
use crate::errors::OliframeError;
use derive_getters::Getters;
use std::str::FromStr;

/// Final image aspect ratio.
///
/// The aspect ratio is a floating point number that represents the ratio of the width to the height of the image.
/// The user can provide the aspect ratio in the following formats:
/// - Decimal: `1.5`
/// - Fraction with colon: `16:9`
/// - Fraction with slash: `16/9`
///
#[derive(Debug, Getters, PartialEq)]
pub struct AspectRatio {
    /// The aspect ratio value.
    inner: f32,
}

impl FromStr for AspectRatio {
    type Err = OliframeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(inner) = input.parse::<f32>() {
            return Ok(Self { inner });
        }

        let delimiters = [':', '/'];
        for delimiter in &delimiters {
            if let Some((left, right)) = input.split_once(*delimiter) {
                let numerator = left.trim().parse::<f32>();
                let denominator = right.trim().parse::<f32>();

                match (numerator, denominator) {
                    (Ok(numerator), Ok(denominator)) if denominator != 0.0 => {
                        if numerator < 0.0 || denominator < 0.0 {
                            return Err(OliframeError::InvalidInput(
                                "Aspect ratio values cannot be negative".to_string(),
                            ));
                        }
                        return Ok(Self {
                            inner: numerator / denominator,
                        });
                    }
                    (Ok(_), Ok(0.0)) => {
                        return Err(OliframeError::InvalidInput(format!(
                            "Denominator cannot be zero: {}",
                            input
                        )));
                    }
                    _ => continue,
                }
            }
        }

        Err(OliframeError::InvalidInput(format!(
            "Invalid ratio: {}",
            input
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal() {
        let ratio = "1.5".parse::<AspectRatio>().unwrap();
        assert_eq!(ratio.inner, 1.5);
    }

    #[test]
    fn parse_fraction_with_colon() {
        let ratio = "16:9".parse::<AspectRatio>().unwrap();
        assert_eq!(ratio.inner, 16.0 / 9.0);
    }

    #[test]
    fn parse_fraction_with_slash() {
        let ratio = "16/9".parse::<AspectRatio>().unwrap();
        assert_eq!(ratio.inner, 16.0 / 9.0);
    }

    #[test]
    fn divide_by_zero_is_err() {
        let ratio = "16:0".parse::<AspectRatio>();
        assert!(ratio.is_err());
        assert_eq!(
            ratio.unwrap_err().to_string(),
            "Invalid input: Denominator cannot be zero: 16:0"
        );
    }

    #[test]
    fn more_than_one_delimiter_is_err() {
        let ratio = "16:9:8".parse::<AspectRatio>();
        assert!(ratio.is_err());
        assert_eq!(
            ratio.unwrap_err().to_string(),
            "Invalid input: Invalid ratio: 16:9:8"
        );
    }

    #[test]
    fn negative_values_are_err() {
        let ratio = "-16:9".parse::<AspectRatio>();
        assert!(ratio.is_err());
        assert_eq!(
            ratio.unwrap_err().to_string(),
            "Invalid input: Aspect ratio values cannot be negative"
        );
    }

    #[test]
    fn non_numeric_values_are_err() {
        let ratio = "16:9a".parse::<AspectRatio>();
        assert!(ratio.is_err());
        assert_eq!(
            ratio.unwrap_err().to_string(),
            "Invalid input: Invalid ratio: 16:9a"
        );
    }
}
