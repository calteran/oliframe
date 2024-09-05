//! CLI options related to the style of the frame around the image.

use crate::config::FrameConfig;
use crate::errors::OliframeError;
use crate::geometry::{AspectRatio, Margins, RelativePosition};
use clap::Args;
use csscolorparser::Color;
use image::Rgba;
use std::str::FromStr;

/// CLI options related to the style of the frame around the image.
#[derive(Args, Debug)]
pub struct FrameOptions {
    /// Fix the final aspect ratio of the output image.
    /// Specify the ratio as a fraction (e.g.: "16:9") or a decimal (e.g.: "1.777").
    #[arg(long = "ar", value_name = "RATIO", help_heading = "Framing Options")]
    aspect_ratio: Option<String>,

    /// Color of the border/background.  Specify any valid CSS color.
    #[arg(short = 'c', long, value_name = "COLOR", default_value = "white")]
    color: String,

    /// Add rounded corners to the image.
    /// Specify the radius in pixels (px) or percentage (%).
    #[arg(short = 'r', long = "radius", value_name = "RADIUS")]
    corner_radius: Option<u32>,

    /// Relative margin around the image.
    /// Provide one, two, three, or four values to specify different widths for each side.
    #[arg(short = 'm', long, value_name = "SIZE(S)")]
    margins: Option<String>,

    /// Relative position of the input image within the output image.
    /// Horizontal values: "left", "center", "right"
    /// Vertical values: "top", "center", "bottom"
    #[arg(short = 'P', long, value_name = "POSITION")]
    position: Option<String>,
}

impl TryFrom<FrameOptions> for FrameConfig {
    type Error = OliframeError;

    fn try_from(opts: FrameOptions) -> Result<Self, Self::Error> {
        let aspect_ratio = opts
            .aspect_ratio
            .map(|ar| AspectRatio::from_str(&ar))
            .transpose()?;

        let color = opts
            .color
            .parse::<Color>()
            .map(|c| Rgba(c.to_rgba8()))
            .map_err(|_| OliframeError::InvalidInput("Invalid color.".to_string()))?;

        let margins = opts
            .margins
            .map(|m| Margins::from_str(&m))
            .transpose()?
            .unwrap_or_default();

        let position = opts
            .position
            .map(|p| RelativePosition::from_str(&p))
            .transpose()?
            .unwrap_or_default();

        Ok(FrameConfig::new(
            aspect_ratio,
            color,
            opts.corner_radius,
            margins,
            position,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_frame_options() {
        let opts = FrameOptions {
            aspect_ratio: Some("16:9".to_string()),
            color: "black".to_string(),
            corner_radius: Some(5),
            margins: Some("10".to_string()),
            position: Some("center".to_string()),
        };

        let config = FrameConfig::try_from(opts).unwrap();
        assert_eq!(config.aspect_ratio().as_ref().unwrap().inner(), 16.0 / 9.0);
        assert_eq!(config.color(), &Rgba([0, 0, 0, 255]));
        assert_eq!(config.corner_radius(), &Some(5));
        assert_eq!(config.margins().values(), &[0.1]);
        assert_eq!(config.position(), &RelativePosition::default());
    }

    #[test]
    fn parse_frame_options_with_defaults() {
        let opts = FrameOptions {
            aspect_ratio: None,
            color: "white".to_string(),
            corner_radius: None,
            margins: None,
            position: None,
        };

        let config = FrameConfig::try_from(opts).unwrap();
        assert!(config.aspect_ratio().is_none());
        assert_eq!(config.color(), &Rgba([255, 255, 255, 255]));
        assert!(config.corner_radius().is_none());
        assert_eq!(config.margins().values(), &[0.05]);
        assert_eq!(config.position(), &RelativePosition::default());
    }

    #[test]
    fn parsing_with_invalid_color_fails() {
        let opts = FrameOptions {
            aspect_ratio: None,
            color: "invalid".to_string(),
            corner_radius: None,
            margins: None,
            position: None,
        };

        let result = FrameConfig::try_from(opts);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid input: Invalid color."
        );
    }
}
