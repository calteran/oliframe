# qwikborder

**qwikborder** is a simple command line tool to add borders to images.  It can process a single image or a directory of images.  It optionally accepts an integer proportion of the image to use as the border width and the color of the border.  The default is white and 10% of the image width.

## Installation

```bash
cargo install qwikborder
```

## Usage


```bash
# Add a default white border to all images in the current directory
qwikborder
````

```bash
# Add a 20% black border to a single image
qwikborder -w 20 -c black image.jpg
```

```bash
# Add a 10% white border to all images in a directory
qwikborder -w 10 -c white -d images
```

```bash
# Add a default border to all images in a directory and save them with a new suffix
qwikborder -s _bordered -d images
```

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Contributing

Pull requests are welcome.  For major changes, please open an issue first to discuss what you would like to change.