//! The Pixel Source module is responsible
//! for determining the source of a pixel during the frame drawing process.
use crate::geometry::{Point, Size};

/// The source of a pixel during the frame drawing process.
#[derive(Debug, PartialEq)]
pub enum PixelSource {
    /// The pixel comes from the input image.
    Image,
    /// The image is a blend of the input image and the background.
    Blend(f32),
    /// The pixel comes from the background color.
    Background,
}

impl PixelSource {
    /// Determine the source of a pixel based on its position and the image's corner radius.
    pub fn at(
        pixel: Point,
        img_position: &Point,
        img_size: &Size,
        corner_radius: &Option<u32>,
    ) -> Self {
        if is_border_pixel(&pixel, img_position, img_size) {
            return PixelSource::Background;
        } else if corner_radius.is_none() {
            return PixelSource::Image;
        }

        let radius = corner_radius.unwrap() as i32;
        let x = (pixel.x() - img_position.x()) as i32;
        let y = (pixel.y() - img_position.y()) as i32;
        let width = img_size.width() as i32;
        let height = img_size.height() as i32;

        let (dx, dy) = if x < radius && y < radius {
            // Top Left
            (radius - x, radius - y)
        } else if x < radius && y >= height - radius {
            // Bottom Left
            (radius - x, y - height + radius + 1)
        } else if x >= width - radius && y < radius {
            // Top Right
            (x - width + radius + 1, radius - y)
        } else if x >= width - radius && y >= height - radius {
            // Bottom Right
            (x - width + radius + 1, y - height + radius + 1)
        } else {
            return PixelSource::Image;
        };

        let distance_squared = dx * dx + dy * dy;
        let radius_squared = radius * radius;
        if distance_squared > radius_squared {
            PixelSource::Background
        } else if distance_squared <= radius_squared - radius {
            PixelSource::Image
        } else {
            let distance = (distance_squared as f32).sqrt();
            let blend = 1.0 - (distance / radius as f32);
            PixelSource::Blend(blend)
        }
    }
}

/// Determine if the given pixel is outside the image's border.
fn is_border_pixel(pixel: &Point, img_position: &Point, img_size: &Size) -> bool {
    if pixel.x() < img_position.x() || pixel.x() >= img_position.x() + img_size.width() {
        return true;
    }
    if pixel.y() < img_position.y() || pixel.y() >= img_position.y() + img_size.height() {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_pixels() {
        let img_position = Point::new(10, 10);
        let img_size = Size::new(100, 100);
        let corner_radius = Some(10);

        let mid_image_pixel = Point::new(60, 60);
        assert_eq!(
            PixelSource::at(mid_image_pixel, &img_position, &img_size, &corner_radius),
            PixelSource::Image
        );

        let top_left_in_corner_radius_pixel = Point::new(15, 15);
        assert_eq!(
            PixelSource::at(
                top_left_in_corner_radius_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Image
        );

        let top_right_in_corner_radius_pixel = Point::new(105, 15);
        assert_eq!(
            PixelSource::at(
                top_right_in_corner_radius_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Image
        );

        let bottom_left_in_corner_radius_pixel = Point::new(15, 105);
        assert_eq!(
            PixelSource::at(
                bottom_left_in_corner_radius_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Image
        );

        let bottom_right_in_corner_radius_pixel = Point::new(105, 105);
        assert_eq!(
            PixelSource::at(
                bottom_right_in_corner_radius_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Image
        );

        let corner_radius = None;

        let top_left_pixel = Point::new(10, 10);
        assert_eq!(
            PixelSource::at(top_left_pixel, &img_position, &img_size, &corner_radius),
            PixelSource::Image
        );

        let top_right_pixel = Point::new(109, 10);
        assert_eq!(
            PixelSource::at(top_right_pixel, &img_position, &img_size, &corner_radius),
            PixelSource::Image
        );

        let bottom_left_pixel = Point::new(10, 109);
        assert_eq!(
            PixelSource::at(bottom_left_pixel, &img_position, &img_size, &corner_radius),
            PixelSource::Image
        );

        let bottom_right_pixel = Point::new(109, 109);
        assert_eq!(
            PixelSource::at(bottom_right_pixel, &img_position, &img_size, &corner_radius),
            PixelSource::Image
        );
    }

    #[test]
    fn border_pixels() {
        let img_position = Point::new(10, 10);
        let img_size = Size::new(100, 100);
        let corner_radius = Some(10);

        let left_border_pixel = Point::new(5, 60);
        assert_eq!(
            PixelSource::at(left_border_pixel, &img_position, &img_size, &corner_radius),
            PixelSource::Background
        );

        let right_border_pixel = Point::new(115, 60);
        assert_eq!(
            PixelSource::at(right_border_pixel, &img_position, &img_size, &corner_radius),
            PixelSource::Background
        );

        let top_border_pixel = Point::new(60, 5);
        assert_eq!(
            PixelSource::at(top_border_pixel, &img_position, &img_size, &corner_radius),
            PixelSource::Background
        );

        let bottom_border_pixel = Point::new(60, 115);
        assert_eq!(
            PixelSource::at(
                bottom_border_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );

        let top_left_corner_pixel = Point::new(5, 5);
        assert_eq!(
            PixelSource::at(
                top_left_corner_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );

        let top_right_corner_pixel = Point::new(115, 5);
        assert_eq!(
            PixelSource::at(
                top_right_corner_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );

        let bottom_left_corner_pixel = Point::new(5, 115);
        assert_eq!(
            PixelSource::at(
                bottom_left_corner_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );

        let bottom_right_corner_pixel = Point::new(115, 115);
        assert_eq!(
            PixelSource::at(
                bottom_right_corner_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );

        let top_left_outside_corner_radius_pixel = Point::new(12, 12);
        assert_eq!(
            PixelSource::at(
                top_left_outside_corner_radius_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );

        let top_right_outside_corner_radius_pixel = Point::new(108, 12);
        assert_eq!(
            PixelSource::at(
                top_right_outside_corner_radius_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );

        let bottom_left_outside_corner_radius_pixel = Point::new(12, 108);
        assert_eq!(
            PixelSource::at(
                bottom_left_outside_corner_radius_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );

        let bottom_right_outside_corner_radius_pixel = Point::new(108, 108);
        assert_eq!(
            PixelSource::at(
                bottom_right_outside_corner_radius_pixel,
                &img_position,
                &img_size,
                &corner_radius
            ),
            PixelSource::Background
        );
    }

    #[test]
    fn blended_pixels() {
        let img_position = Point::new(10, 10);
        let img_size = Size::new(100, 100);
        let corner_radius = Some(10);

        let top_left_corner_pixel = Point::new(13, 13);
        let pixel_source = PixelSource::at(
            top_left_corner_pixel,
            &img_position,
            &img_size,
            &corner_radius,
        );
        match pixel_source {
            PixelSource::Blend(blend) => {
                assert!(blend > 0.01);
                assert!(blend < 0.02);
            }
            _ => panic!("Expected PixelSource::Blend"),
        }

        let top_right_corner_pixel = Point::new(106, 13);
        let pixel_source = PixelSource::at(
            top_right_corner_pixel,
            &img_position,
            &img_size,
            &corner_radius,
        );
        match pixel_source {
            PixelSource::Blend(blend) => {
                assert!(blend > 0.01);
                assert!(blend < 0.02);
            }
            _ => panic!("Expected PixelSource::Blend"),
        }

        let bottom_left_corner_pixel = Point::new(13, 106);
        let pixel_source = PixelSource::at(
            bottom_left_corner_pixel,
            &img_position,
            &img_size,
            &corner_radius,
        );
        match pixel_source {
            PixelSource::Blend(blend) => {
                assert!(blend > 0.01);
                assert!(blend < 0.02);
            }
            _ => panic!("Expected PixelSource::Blend"),
        }

        let bottom_right_corner_pixel = Point::new(106, 106);
        let pixel_source = PixelSource::at(
            bottom_right_corner_pixel,
            &img_position,
            &img_size,
            &corner_radius,
        );
        match pixel_source {
            PixelSource::Blend(blend) => {
                assert!(blend > 0.01);
                assert!(blend < 0.02);
            }
            _ => panic!("Expected PixelSource::Blend"),
        }
    }
}
