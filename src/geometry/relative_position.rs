//! Relative position of an element in the final frame.
use crate::errors::OliframeError;
use derive_getters::Getters;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

/// Relative position of an image in the final frame.
///
/// The relative position is defined by two values:
/// - Horizontal position: left, center, right
/// - Vertical position: top, center, bottom
///
#[derive(Debug, Default, Getters, PartialEq)]
pub struct RelativePosition {
    /// Horizontal position of the image.
    horizontal: HorizontalPosition,
    /// Vertical position of the image.
    vertical: VerticalPosition,
}

impl FromStr for RelativePosition {
    type Err = OliframeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let horizontal = extract_position(s).ok_or(OliframeError::InvalidInput(
            "Please provide only one horizontal position.".to_string(),
        ))?;

        let vertical = extract_position(s).ok_or(OliframeError::InvalidInput(
            "Please provide only one vertical position.".to_string(),
        ))?;

        Ok(Self {
            horizontal,
            vertical,
        })
    }
}

/// Extract the position from the given string.
fn extract_position<P>(s: &str) -> Option<P>
where
    P: FromStr + IntoEnumIterator + AsRef<str> + Default + PartialEq,
    P::Err: std::fmt::Debug,
{
    P::iter()
        .filter(|pos| s.contains(pos.as_ref()))
        .try_fold(P::default(), |acc, pos| match (acc, pos) {
            (acc, pos) if acc == P::default() => Ok(pos),
            (acc, pos) if pos == P::default() => Ok(acc),
            _ => Err(()),
        })
        .ok()
}

/// Horizontal position of an element in the final frame.
#[derive(AsRefStr, Debug, Default, EnumIter, EnumString, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum HorizontalPosition {
    /// The element is positioned on the left side of the frame.
    Left,
    /// The element is positioned in the center of the frame.
    #[default]
    Center,
    /// The element is positioned on the right side of the frame.
    Right,
}

/// Vertical position of an element in the final frame.
#[derive(AsRefStr, Debug, Default, EnumIter, EnumString, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum VerticalPosition {
    /// The element is positioned at the top of the frame.
    Top,
    #[default]
    /// The element is positioned in the center of the frame.
    Center,
    /// The element is positioned at the bottom of the frame.
    Bottom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_horizontal_position_from_string() {
        let position = "left".parse::<HorizontalPosition>().unwrap();
        assert_eq!(position, HorizontalPosition::Left);

        let position = "center".parse::<HorizontalPosition>().unwrap();
        assert_eq!(position, HorizontalPosition::Center);

        let position = "right".parse::<HorizontalPosition>().unwrap();
        assert_eq!(position, HorizontalPosition::Right);
    }

    #[test]
    fn parse_vertical_position_from_string() {
        let position = "top".parse::<VerticalPosition>().unwrap();
        assert_eq!(position, VerticalPosition::Top);

        let position = "center".parse::<VerticalPosition>().unwrap();
        assert_eq!(position, VerticalPosition::Center);

        let position = "bottom".parse::<VerticalPosition>().unwrap();
        assert_eq!(position, VerticalPosition::Bottom);
    }

    #[test]
    fn parse_relative_position_from_string() {
        let position = "left,top".parse::<RelativePosition>().unwrap();
        assert_eq!(position.horizontal, HorizontalPosition::Left);
        assert_eq!(position.vertical, VerticalPosition::Top);

        let position = "center,bottom".parse::<RelativePosition>().unwrap();
        assert_eq!(position.horizontal, HorizontalPosition::Center);
        assert_eq!(position.vertical, VerticalPosition::Bottom);

        let position = "right,center".parse::<RelativePosition>().unwrap();
        assert_eq!(position.horizontal, HorizontalPosition::Right);
        assert_eq!(position.vertical, VerticalPosition::Center);

        let position = "center,center".parse::<RelativePosition>().unwrap();
        assert_eq!(position.horizontal, HorizontalPosition::Center);
        assert_eq!(position.vertical, VerticalPosition::Center);

        let position = "center".parse::<RelativePosition>().unwrap();
        assert_eq!(position.horizontal, HorizontalPosition::Center);
        assert_eq!(position.vertical, VerticalPosition::Center);
    }

    #[test]
    fn parse_invalid_relative_position_from_string() {
        let position = "left,right,bottom".parse::<RelativePosition>();
        assert!(position.is_err());
        assert_eq!(
            position.unwrap_err().to_string(),
            "Invalid input: Please provide only one horizontal position."
        );

        let position = "center,bottom,top".parse::<RelativePosition>();
        assert!(position.is_err());
        assert_eq!(
            position.unwrap_err().to_string(),
            "Invalid input: Please provide only one vertical position."
        );
    }
}
