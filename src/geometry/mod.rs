//! Geometry module.
mod aspect_ratio;
mod margins;
mod relative_position;
mod size;
mod units;

pub use aspect_ratio::AspectRatio;
pub use margins::Margins;
use regex::Regex;
pub use relative_position::RelativePosition;
pub use size::Size;
use std::str::FromStr;
pub use units::Unit;

fn parse_values_and_units<T: FromStr>(input: &str) -> Result<(Vec<T>, Option<Unit>), ParseError> {
    // Future improvement: consider using once_cell::sync::Lazy to compile the regex once.
    let re = Regex::new(r"(?P<value>-?\d*\.?\d+)(?P<unit>px|%)?").expect("Invalid regex");
    let mut values = Vec::new();
    let mut unit = None;

    for caps in re.captures_iter(input) {
        values.push(
            caps.name("value")
                .expect("Expected value not found.")
                .as_str()
                .parse::<T>()
                .map_err(|_| ParseError::InvalidValue)?,
        );

        let next_unit = caps
            .name("unit")
            .map(|u| u.as_str().parse::<Unit>())
            .transpose()
            .map_err(|_| ParseError::InvalidUnit)?;

        match (&unit, next_unit) {
            (None, Some(u)) => unit = Some(u),
            (Some(u1), Some(u2)) if u1 != &u2 => return Err(ParseError::UnitConflict),
            _ => (),
        }
    }

    Ok((values, unit))
}

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Failed to parse value")]
    InvalidValue,
    #[error("Failed to parse unit")]
    InvalidUnit,
    #[error("Unit conflict: all values must have the same unit")]
    UnitConflict,
}
