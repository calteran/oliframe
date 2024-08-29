//! CLI options related to the style of the frame around the image.
use clap::Args;

/// CLI options related to the style of the frame around the image.
#[derive(Args, Debug)]
pub struct FrameOptions {
    /// Choose the framing style: "border" to add an equal border on all sides, or
    /// "background" to position the image within a larger canvas.
    #[arg(short = 'S', long, value_name = "STYLE", default_value = "border")]
    style: String,

    /// Margin around the image, in pixels (px) or percentage (%).
    /// Provide one, two, or four values to specify different widths for each side.
    #[arg(short = 'm', long, value_name = "SIZE(S)")]
    margin: Option<String>,

    /// Add rounded corners to the image.  
    /// Specify the radius in pixels (px) or percentage (%).
    #[arg(short = 'r', long = "radius", value_name = "RADIUS")]
    corner_radius: Option<u32>,

    /// Color of the border/background.  Specify any valid CSS color.
    #[arg(short = 'c', long, value_name = "COLOR", default_value = "white")]
    color: String,

    /// Fix the final aspect ratio of the output image.
    /// Specify the ratio as a fraction (e.g.: "16:9") or a decimal (e.g.: "1.777").
    #[arg(
        long = "ar",
        value_name = "RATIO",
        help_heading = "Framing Options",
        conflicts_with = "upscale"
    )]
    aspect_ratio: Option<String>,

    /// Expand the background so the final image is this proportion of the original image.
    /// E.g.: "200%", "200x200", "200x", "x200"
    #[arg(
        short = 'u',
        long,
        value_name = "SIZE",
        conflicts_with = "aspect_ratio"
    )]
    upscale: Option<String>,

    /// Relative position of the input image within the output image.
    /// Horizontal values: "left", "center", "right"
    /// Vertical values: "top", "center", "bottom"
    #[arg(short = 'P', long, value_name = "POSITION")]
    position: Option<String>,
}
