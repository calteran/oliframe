//! Module containing helper functions for image processing.
use crate::config::FrameConfig;
use crate::errors::OliframeError;
use crate::geometry::*;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::path::PathBuf;

/// Load an image from the given file path.
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

/// Calculate the dimensions of the output image.
pub fn output_dimensions(input_size: &Size, border: &Border, config: &FrameConfig) -> Size {
    if let Some(aspect_ratio) = config.aspect_ratio() {
        size_with_ratio(input_size, aspect_ratio, border)
    } else {
        size_with_border(input_size, border)
    }
}

/// Calculate the position of the image within the output frame.
pub fn position(
    img_size: &Size,
    output_size: &Size,
    relative_position: &RelativePosition,
    border: &Border,
) -> Point {
    let x = match relative_position.horizontal() {
        HorizontalPosition::Left => border.left(),
        HorizontalPosition::Center => (output_size.width() - img_size.width()) / 2,
        HorizontalPosition::Right => output_size.width() - img_size.width() - border.right(),
    };

    let y = match relative_position.vertical() {
        VerticalPosition::Top => border.top(),
        VerticalPosition::Center => (output_size.height() - img_size.height()) / 2,
        VerticalPosition::Bottom => output_size.height() - img_size.height() - border.bottom(),
    };

    Point::new(x, y)
}

/// Calculate the size of the output image with a border.
pub fn size_with_border(img_size: &Size, border: &Border) -> Size {
    let new_width = img_size.width() + border.left() + border.right();
    let new_height = img_size.height() + border.top() + border.bottom();
    Size::from((new_width, new_height))
}

/// Calculate the size of the output image with a given aspect ratio.
pub fn size_with_ratio(img_size: &Size, aspect_ratio: &AspectRatio, border: &Border) -> Size {
    let (img_width, img_height) = img_size.dimensions();
    let frame_width = (img_width + border.left() + border.right()) as f32;
    let frame_height = (img_height + border.top() + border.bottom()) as f32;
    let frame_ar = frame_width / frame_height;

    if frame_ar < aspect_ratio.inner() {
        Size::from((
            (frame_height * aspect_ratio.inner()).round() as u32,
            frame_height as u32,
        ))
    } else {
        Size::from((
            frame_width as u32,
            (frame_width / aspect_ratio.inner()).round() as u32,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;
    use std::str::FromStr;
    use tempfile::Builder;

    #[test]
    fn load_known_image() {
        let image_file = PathBuf::from("samples/sample00.jpg");
        let result = load(&image_file);
        assert!(result.is_ok());
        let (_, fmt) = result.unwrap();
        assert_eq!(fmt, ImageFormat::Jpeg);
    }

    #[test]
    fn invalid_image_wont_load() {
        let not_an_image_file = PathBuf::from("Cargo.toml");
        let result = load(&not_an_image_file);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            OliframeError::ImageUnreadable(not_an_image_file)
        );
    }

    #[test]
    fn handle_unreadable_image() {
        let temp_img = Builder::new().suffix(".jpg").tempfile().unwrap();
        let result = load(&temp_img.path().to_path_buf());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            OliframeError::ImageUnreadable(temp_img.path().to_path_buf())
        );
    }

    #[test]
    fn output_dimensions_with_border() {
        let input_size = Size::from((100, 100));
        let border = Margins::from_str("10")
            .unwrap()
            .to_border_with_size(&input_size);
        let config = FrameConfig::default();
        let output_size = output_dimensions(&input_size, &border, &config);
        assert_eq!(output_size, Size::from((120, 120)));
    }

    #[test]
    fn output_dimensions_with_tall_ar() {
        let input_size = Size::from((100, 100));
        let margins = Margins::from_str("10").unwrap();
        let border = margins.to_border_with_size(&input_size);
        let config = FrameConfig::new(
            Some(AspectRatio::from_str("9:16").unwrap()),
            Rgba([0, 0, 0, 0]),
            None,
            margins,
            RelativePosition::default(),
        );
        let output_size = output_dimensions(&input_size, &border, &config);
        assert_eq!(output_size, Size::from((120, 213)));
    }

    #[test]
    fn output_dimensions_with_wide_ar() {
        let input_size = Size::from((100, 100));
        let margins = Margins::from_str("10").unwrap();
        let border = margins.to_border_with_size(&input_size);
        let config = FrameConfig::new(
            Some(AspectRatio::from_str("16:9").unwrap()),
            Rgba([0, 0, 0, 0]),
            None,
            margins,
            RelativePosition::default(),
        );
        let output_size = output_dimensions(&input_size, &border, &config);
        assert_eq!(output_size, Size::from((213, 120)));
    }

    #[test]
    fn position_in_frame() {
        let img_size = Size::from((100, 100));
        let output_size = Size::from((200, 200));
        let border = Margins::from_str("10")
            .unwrap()
            .to_border_with_size(&img_size);

        let relative_position = RelativePosition::from_str("center").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(50, 50));

        let relative_position = RelativePosition::from_str("left,top").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(10, 10));

        let relative_position = RelativePosition::from_str("right,bottom").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(90, 90));

        let relative_position = RelativePosition::from_str("left,bottom").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(10, 90));

        let relative_position = RelativePosition::from_str("right,top").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(90, 10));

        let relative_position = RelativePosition::from_str("center,bottom").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(50, 90));

        let relative_position = RelativePosition::from_str("center,top").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(50, 10));

        let relative_position = RelativePosition::from_str("left,center").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(10, 50));

        let relative_position = RelativePosition::from_str("right,center").unwrap();
        let pos = position(&img_size, &output_size, &relative_position, &border);
        assert_eq!(pos, Point::new(90, 50));
    }
}
