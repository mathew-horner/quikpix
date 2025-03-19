# `quikpix`

`quikpix` is a library that provides a simple pixel grid that can read from or save to [Netpbm Portable PixMap](https://en.wikipedia.org/wiki/Netpbm) files.

## What is a ppm image file?

`ppm` is an image file format used by the Netpbm graphics suite.

The two main reasons I like to use it (in certain situations) are:

1. It is a very simple format that is easy to understand for both humans and computers.
2. You can encode your image data in plain text, which makes it easy to edit simple images with a text editor.

## How does `quikpix` work?

`quikpix` provides a `Pixels` struct with a few important APIs:

- `Pixels::read` - Read from a `ppm` file.
- `Pixels::write` - Write to a `ppm` file.
- `Pixels::get` - Get the color of a given pixel.
- `Pixels::set` - Set the color of a given pixel.

It's important to note that `quikpix` currently only works with *ASCII* `ppm` files, not binary.
