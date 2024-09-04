use crate::config::FrameConfig;
use crate::errors::OliframeError;
use crate::geometry::*;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::path::PathBuf;

pub fn load(image_file: &PathBuf) -> Result<(DynamicImage, ImageFormat), OliframeError> {
    let img = ImageReader::open(image_file)
        .map_err(|_| OliframeError::ImageUnreadable(image_file.to_path_buf()))?;
    let fmt = img
        .format()
        .ok_or_else(|| OliframeError::ImageUnreadable(image_file.to_path_buf()))?;
    let img = img
        .decode()
        .map_err(|_| OliframeError::ImageUnreadable(image_file.to_path_buf()))?;

    Ok((img, fmt))
}

pub fn output_dimensions(input_size: &Size, img_margins: &Margins, config: &FrameConfig) -> Size {
    if let Some(aspect_ratio) = config.aspect_ratio() {
        size_with_ratio(input_size, aspect_ratio, img_margins)
    } else {
        size_with_margins(input_size, img_margins)
    }
}

pub fn position(
    img_size: &Size,
    output_size: &Size,
    relative_position: &RelativePosition,
    img_margins: &Margins,
) -> Point {
    let x = match relative_position.horizontal() {
        HorizontalPosition::Left => img_margins.left(),
        HorizontalPosition::Center => (output_size.width() - img_size.width()) / 2,
        HorizontalPosition::Right => output_size.width() - img_size.width() - img_margins.right(),
    };

    let y = match relative_position.vertical() {
        VerticalPosition::Top => img_margins.top(),
        VerticalPosition::Center => (output_size.height() - img_size.height()) / 2,
        VerticalPosition::Bottom => output_size.height() - img_size.height() - img_margins.bottom(),
    };

    Point::new(x, y)
}

pub fn size_with_margins(img_size: &Size, img_margins: &Margins) -> Size {
    let new_width = img_size.width() + img_margins.left() + img_margins.right();
    let new_height = img_size.height() + img_margins.top() + img_margins.bottom();
    Size::from((new_width, new_height))
}

pub fn size_with_ratio(img_size: &Size, aspect_ratio: &AspectRatio, img_margins: &Margins) -> Size {
    let (width, height) = img_size.dimensions();
    let img_ar = width as f32 / height as f32;

    if img_ar > aspect_ratio.inner() {
        let new_height = height + img_margins.top() + img_margins.bottom();
        let new_width = (new_height as f32 * aspect_ratio.inner()).round() as u32;
        Size::from((new_width, new_height))
    } else {
        let new_width = width + img_margins.left() + img_margins.right();
        let new_height = (new_width as f32 / aspect_ratio.inner()).round() as u32;
        Size::from((new_width, new_height))
    }
}
