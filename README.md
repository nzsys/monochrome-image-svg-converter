# MonochromeImageToSVGConverter

MonochromeImageToSVGConverter is a command-line tool that converts a monochrome raster image (black and white) to an SVG file with contours.

## Features

- Converts monochrome images to SVG format.
- Extracts contours and draws them in the SVG.
- Utilizes a progress bar to indicate the conversion progress.

## Installation

Ensure you have Rust and Cargo installed. Clone the repository and run the following command:

```sh
cargo build --release
```

## Usage

```sh
bin/MonochromeImageToSVGConverter <image_path> <output_path>
```

To use the converter, run the following command:

```sh
cargo run -- <image_path> <output_path>
```

- `<image_path>`: Path to the input monochrome image.
- `<output_path>`: Path to the output SVG file.

## Dependencies

- `image`: For image processing.
- `imageproc`: For image processing algorithms.
- `svg`: For creating SVG files.
- `indicatif`: For displaying progress bars.

## Contributing

All contributions are welcome. Please fork the repository and create a pull request.

## License

This project is licensed under the MIT License.
