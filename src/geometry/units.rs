//! Units for geometry values.
use strum_macros::EnumString;

/// Units for geometry values.
#[derive(Clone, Debug, EnumString, PartialEq)]
pub enum Unit {
    /// Pixels.
    #[strum(serialize = "px")]
    Pixel,
    /// Percentages.
    #[strum(serialize = "%")]
    Percent,
}
