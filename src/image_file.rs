use std::fmt::Display;
use std::path::PathBuf;
use oliframe::add_border;
use crate::args::Args;

pub struct ImageFile {
    input_path: PathBuf,
    output_path: PathBuf,
}

impl Display for ImageFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.input_path.display())
    }
}

impl ImageFile {
    pub fn new(input_path: PathBuf, output_path: PathBuf) -> Self {
        Self { input_path, output_path }
    }

    pub fn add_border(&self, args: &Args) -> Result<(), String> {
        let img = image::open(&self.input_path).map_err(|e| format!("Unable to open {}: {}", self.input_path.display(), e))?;
        let (width, is_pixels) = if let Some(percent) = args.percent {
            (percent, false)
        } else if let Some(pixels) = args.pixels {
            (pixels, true)
        } else {
            (5, false)
        };

        let new_img = add_border(&img, width, is_pixels, args.radius, args.verbose);
        if args.dry_run {
            println!("Would write {} to {}", self.input_path.display(), self.output_path.display());
        } else {
            new_img.save(&self.output_path).map_err(|e| format!("Unable to save {}: {}", self.output_path.display(), e))?;
        }

        Ok(())
    }
}