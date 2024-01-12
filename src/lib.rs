use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

pub enum BorderWidth {
    Percent(u32),
    Pixels(u32),
}

pub fn borderize(
    img: &DynamicImage,
    width: BorderWidth,
    
) -> RgbaImage {

}

pub fn add_border(
    img: &DynamicImage,
    border_width: u32,
    _width_is_pixels: bool,
    _corner_radius: Option<u32>,
    _verbose: bool,
) -> RgbaImage {
    let (width, height) = img.dimensions();
    let avg_dim = (width + height) / 2;
    let border_size = avg_dim * border_width / 100;

    let new_width = width + border_size * 2;
    let new_height = height + border_size * 2;

    let mut new_img = RgbaImage::new(new_width, new_height);

    // Fill with white color
    for (_, _, pixel) in new_img.enumerate_pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }
    // Copy the original image over the white background
    image::imageops::overlay(&mut new_img, img, border_size as i64, border_size as i64);

    new_img
}
