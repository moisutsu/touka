# Touka

[日本語](./README.ja.md)

This is a commandline tool for making image backgrounds transparent.

## Installation

```bash
cargo install touka
```

## Usage

You can make an image with a transparent background by passing the image and output path as command line arguments of `touka`.

```bash
touka input/image/path -o output/image/path
```

Use `-t` to control the background detection threshold.

```bash
touka input/image/path -o output/image/path -t 250
```

When all three RGB colour values are above the threshold, that pixel will be
made transparent. The default threshold is 230.

Only for macOS, you can directly exchange images with the clipboard. When you don't specify an input image, the clipboard image is transparent, and when you don't specify an output path, the transparent image is saved to the clipboard.

```bash
# The clipboard image's background is transparent and saved to the clipboard.
touka
```
