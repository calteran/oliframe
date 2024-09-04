use crate::geometry::{Point, Size};

#[derive(Debug)]
pub enum PixelSource {
    Image,
    Blend(f32),
    Background,
}

impl PixelSource {
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

fn is_border_pixel(pixel: &Point, img_position: &Point, img_size: &Size) -> bool {
    if pixel.x() < img_position.x() || pixel.x() >= img_position.x() + img_size.width() {
        return true;
    }
    if pixel.y() < img_position.y() || pixel.y() >= img_position.y() + img_size.height() {
        return true;
    }
    false
}
