pub use csscolorparser::Color as Color;
pub use image;

use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

type Pixels = u32;

/// Border width can be specified in pixels or as a percentage of the average dimension
pub enum BorderWidth {
    /// Border width as a percentage of the average dimension ie: (width + height) / 2
    Percent(u32),
    /// Border width in pixels
    Pixels(u32),
}

impl BorderWidth {
    fn for_image(&self, img: &DynamicImage) -> Pixels {
        match self {
            BorderWidth::Percent(pct) => {
                let (width, height) = img.dimensions();
                let avg_dim = (width + height) / 2;
                avg_dim * pct / 100
            }
            BorderWidth::Pixels(px) => *px,
        }
    }
}

#[derive(Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x = (self.x - other.x).pow(2);
        let y = (self.y - other.y).pow(2);
        ((x + y) as f64).sqrt()
    }
}

/// Adds a border to the given image
///
/// # Arguments
///
/// * `img` - The image to add a border to
/// * `width` - The width of the border, either in pixels or as a percentage of the average dimension
/// * `color` - The color of the border
/// * `radius` - The radius of the border corners, in pixels
///
/// # Returns
///
/// A new image with the border added
///
/// # Examples
///
/// ```
/// use oliframe::{add_border, BorderWidth};
/// use oliframe::image::DynamicImage;
/// use oliframe::Color;
///
/// let img = DynamicImage::new_rgb8(100, 100);
/// let border = add_border(&img, BorderWidth::Pixels(10), &"red".parse::<Color>().unwrap(), None);
/// ```
pub fn add_border(
    img: &DynamicImage,
    width: BorderWidth,
    color: &Color,
    radius: Option<u32>,
) -> RgbaImage {
    let border = width.for_image(img);

    RgbaImage::from_fn(
        img.width() + border * 2,
        img.height() + border * 2,
        |x, y| {
            if in_border(x, y, img.width(), img.height(), border, radius) {
                Rgba(color.to_rgba8())
            } else {
                img.get_pixel(x - border, y - border)
            }
        },
    )
}

fn in_border(x: u32, y: u32, width: u32, height: u32, border: u32, radius: Option<u32>) -> bool {
    if x < border || x >= width + border || y < border || y >= height + border {
        return true;
    }

    if let Some(radius) = radius {
        return in_radius(x, y, width, height, border, radius);
    }

    false
}

// returns the nearest corner of the border where it intersects with the image
fn in_radius(x: u32, y: u32, width: u32, height: u32, border: u32, radius: u32) -> bool {
    let mut corner = Point::default();
    let mut center = Point::default();
    let pixel = Point {
        x: x as i64,
        y: y as i64,
    };
    let radius = radius as i64;

    if x < (width + border) / 2 {
        corner.x = border as i64;
        center.x = corner.x + radius;
    } else {
        corner.x = (width + border) as i64;
        center.x = corner.x - radius;
    }

    if y < (height + border) / 2 {
        corner.y = border as i64;
        center.y = corner.y + radius;
    } else {
        corner.y = (height + border) as i64;
        center.y = corner.y - radius;
    }

    let radius = radius as f64;
    let distance_to_center = pixel.distance(&center);
    let distance_to_corner = pixel.distance(&corner);

    distance_to_center > radius && distance_to_corner < radius
}
