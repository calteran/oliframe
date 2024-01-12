# oliframe

**oliframe** is a simple command line tool to add borders to images.  It can process a single image or multiple directories at a time.

## Installation

### For command-line usage
```bash
cargo install oliframe
```

### For library usage
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
oliframe = "0.1.0"
```

## Command-Line Usage

| Option                    | Description                                                                                                 |
|---------------------------|-------------------------------------------------------------------------------------------------------------|
| `-f`, `--file <FILENAME>` | One or more input files                                                                                     |
| `-d`, `--dir <DIR>`       | Director(y/ies) to search for input files                                                                   |
| `-R`, `--recursive`       | Recursively search for input files in the specified director(y/ies)                                         |
| `-x`, `--extension <FMT>` | File extension(s) of input files to accept (must be exact match, ie: "jpg" != "jpeg" != "JPG")              |
| `-o`, `--output <DEST>`   | Output destination                                                                                          |
| `-p`, `--prefix <PREFIX>` | Prefix to prepend to output files                                                                           |
| `-s`, `--suffix <SUFFIX>` | Suffix to append to output files                                                                            |
| `-C`, `--pct <%WIDTH>`    | Border width in percent of average dimension, ie: (width + height) / 2.0. [default: 5]                      |
| `-X`, `--px <PIXELS>`     | Border with in pixels, default is 0 (disabled)                                                              |
| `-c`, `--color <COLOR>`   | Border color, any legal CSS color value [default: white]                                                    |
| `-r`, `--radius <RADIUS>` | Border corner radius (in pixels; requires --px)                                                             |
| `-v`, `--verbose`         | Verbose output                                                                                              |
| `-q`, `--quiet`           | Quiet output -- suppresses all output except errors                                                         |
| `--dry-run`               | Dry run (don't actually create output files)                                                                |
| `-y`, `--overwrite`       | Overwrite existing files. Default is to ask for each image before overwriting if this flag is not specified |
| `-h`, `--help`            | Print help                                                                                                  |
| `-V`, `--version`         | Print version                                                                                               |


```bash
# Add a default white border to all images in the current directory
oliframe
````

```bash
# Add a 20% black border to a single image
oliframe -w 20 -c black image.jpg
```

```bash
# Add a 10% white border to all images in a directory
oliframe -w 10 -c white -d images
```

```bash
# Add a default border to all images in a directory and save them with a new suffix
oliframe -s _bordered -d images
```

## Library Usage

The library only exposes one function, `add_border`,
which expects an `&image::DynamicImage` as input and returns a new `image::DynamicImage` with the border applied.
Loading and saving images is left to the user.

```rust
use oliframe::*;
use image::prelude::*;

fn main() {
    let img = image::open("image.jpg").unwrap();
    let bordered_img = add_border(&img, BorderWidth::Percent(5), "white", None);
    bordered_img.save("image_bordered.jpg").unwrap();
}
```

## Contributing

Pull requests are welcome.  For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)

## AI-Assisted Development

This project has been developed with the assistance of AI-powered tools, including ChatGPT and GitHub Copilot.
These tools have contributed to various aspects of the code,
ranging from generating initial code snippets to offering suggestions for code optimization and bug fixes.
While these tools have aided in the development process,
the design and implementation of this project is the work of the author.
