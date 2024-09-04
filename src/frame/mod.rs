mod build;
mod draw;

use crate::config::FrameConfig;
use crate::errors::OliframeError;
use crate::file_collector::FilePair;
use crate::frame::draw::PixelSource;
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
