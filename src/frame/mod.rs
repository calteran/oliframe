mod build;
mod pixel_source;

use crate::config::FrameConfig;
use crate::errors::OliframeError;
use crate::file_collector::FilePair;
use crate::frame::pixel_source::PixelSource;
use crate::geometry::*;
use derive_getters::Getters;
use image::buffer::ConvertBuffer;
use image::{DynamicImage, GenericImageView, ImageFormat, Pixel, RgbaImage};
use std::path::PathBuf;

#[derive(Debug, Getters)]
pub struct Frame {
    img: DynamicImage,
    fmt: ImageFormat,
    input_size: Size,
    output_path: PathBuf,
    output_size: Size,
    position: Point,
}

impl Frame {
    pub fn process(
        file_pair: FilePair,
        config: &FrameConfig,
        dry_run: bool,
    ) -> Result<(), OliframeError> {
        log::debug!("Processing image: {:?}", file_pair.input_path());
        Self::build(file_pair, config)?.draw(config).save(dry_run)
    }

    fn build(file_pair: FilePair, config: &FrameConfig) -> Result<Self, OliframeError> {
        let (img, fmt) = build::load(file_pair.input_path())?;
        let input_size = Size::from(img.dimensions());
        let border = config.margins().to_border_with_size(&input_size);
        let (_, output_path) = file_pair.into_parts();
        let output_size = build::output_dimensions(&input_size, &border, config);
        let position = build::position(&input_size, &output_size, config.position(), &border);

        Ok(Self {
            img,
            fmt,
            input_size,
            output_path,
            output_size,
            position,
        })
    }

    pub fn draw(mut self, config: &FrameConfig) -> Self {
        let output = RgbaImage::from_fn(
            self.output_size.width(),
            self.output_size.height(),
            |x, y| {
                let pixel = Point::new(x, y);
                match PixelSource::at(
                    pixel,
                    self.position(),
                    self.input_size(),
                    config.corner_radius(),
                ) {
                    PixelSource::Image => self
                        .img
                        .get_pixel(x - self.position.x(), y - self.position.y()),
                    PixelSource::Background => *config.color(),
                    PixelSource::Blend(alpha) => self
                        .img
                        .get_pixel(x - self.position.x(), y - self.position.y())
                        .map2(config.color(), |img, bkg| {
                            (img as f32 * alpha + bkg as f32 * (1.0 - alpha)) as u8
                        }),
                }
            },
        );

        self.img = match self.fmt {
            ImageFormat::Jpeg => DynamicImage::ImageRgb8(output.convert()),
            _ => DynamicImage::ImageRgba8(output),
        };

        self
    }

    fn save(&self, dry_run: bool) -> Result<(), OliframeError> {
        if dry_run {
            log::info!("Dry run: Would save image to {:?}", self.output_path());
            return Ok(());
        }

        self.img
            .save(self.output_path())
            .map_err(|e| OliframeError::SaveError(self.output_path().clone(), e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TEST_FS_PREFIX;
    use image::Rgba;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use tempfile::TempDir;
    use xxhash_rust::xxh3::Xxh3;

    #[test]
    fn process_image_with_default_frame() {
        let temp_dir = TempDir::with_prefix(TEST_FS_PREFIX).unwrap();
        let config = FrameConfig::default();
        let file_pair = FilePair::new(
            PathBuf::from("samples/sample00.jpg"),
            temp_dir.path().join("sample00.jpg"),
        );

        let result = Frame::process(file_pair, &config, false);
        assert!(result.is_ok());
        assert_eq!(
            "f751f264598cb8bd".to_string(),
            hash_file(&temp_dir.path().join("sample00.jpg"))
        );
    }

    #[test]
    fn process_image_with_corner_radius() {
        let temp_dir = TempDir::with_prefix(TEST_FS_PREFIX).unwrap();
        let config = FrameConfig::new(
            None,
            Rgba([255, 255, 255, 255]),
            Some(10),
            Margins::default(),
            RelativePosition::default(),
        );
        let file_pair = FilePair::new(
            PathBuf::from("samples/sample00.jpg"),
            temp_dir.path().join("sample00.jpg"),
        );

        let result = Frame::process(file_pair, &config, false);
        assert!(result.is_ok());
        assert_eq!(
            "fbcaa7f5aaaee1a9".to_string(),
            hash_file(&temp_dir.path().join("sample00.jpg"))
        );
    }

    #[test]
    fn process_png() {
        let temp_dir = TempDir::with_prefix(TEST_FS_PREFIX).unwrap();
        let config = FrameConfig::default();
        let file_pair = FilePair::new(
            PathBuf::from("samples/sample24.png"),
            temp_dir.path().join("sample24.png"),
        );

        let result = Frame::process(file_pair, &config, false);
        assert!(result.is_ok());
        assert_eq!(
            "3d835adc93318aab".to_string(),
            hash_file(&temp_dir.path().join("sample24.png"))
        );
    }

    #[test]
    fn process_dry_run() {
        let temp_dir = TempDir::with_prefix(TEST_FS_PREFIX).unwrap();
        let config = FrameConfig::default();
        let file_pair = FilePair::new(
            PathBuf::from("samples/sample00.jpg"),
            temp_dir.path().join("sample00.jpg"),
        );

        let result = Frame::process(file_pair, &config, true);
        assert!(result.is_ok());
        assert!(!temp_dir.path().join("sample00.jpg").exists());
    }

    fn hash_file(filepath: &PathBuf) -> String {
        let file = File::open(filepath).unwrap();
        let mut reader = BufReader::new(file);
        let mut hasher = Xxh3::default();

        let mut buffer = [0u8; 4096];
        loop {
            let bytes_read = reader.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        format!("{:x}", hasher.digest())
    }
}
