use crate::args::Args;
use oliframe::{add_border, BorderWidth};
use std::fmt::Display;
use std::path::PathBuf;

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
        Self {
            input_path,
            output_path,
        }
    }

    pub fn add_border(&self, args: &Args) -> Result<(), String> {
        let img = image::open(&self.input_path)
            .map_err(|e| format!("Unable to open {}: {}", self.input_path.display(), e))?;
        let width = match args.pixels {
            Some(px) => BorderWidth::Pixels(px),
            None => BorderWidth::Percent(args.percent.unwrap()),
        };

        let new_img = add_border(&img, width, &args.color, args.radius);
        if args.dry_run {
            println!(
                "Would write {} to {}",
                self.input_path.display(),
                self.output_path.display()
            );
        } else {
            new_img
                .save(&self.output_path)
                .map_err(|e| format!("Unable to save {}: {}", self.output_path.display(), e))?;
        }

        Ok(())
    }
}
