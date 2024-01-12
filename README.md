# oliframe

**oliframe** is a simple command line tool to add borders to images.  It can process a single image or a directory of images.  It optionally accepts an integer proportion of the image to use as the border width and the color of the border.  The default is white and 10% of the image width.

## Installation

```bash
cargo install oliframe
```

## Usage


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
