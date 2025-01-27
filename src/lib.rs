use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const BLACK: Self = Self(0, 0, 0);
    pub const WHITE: Self = Self(255, 255, 255);
}

pub struct Pixels {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

impl Pixels {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Color::BLACK; width * height],
            width,
            height,
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            pixels: self,
            index: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        let idx = self.idx(x, y);
        self.data[idx]
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        let idx = self.idx(x, y);
        self.data[idx] = color;
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        let idx = y * self.width + x;
        if idx >= self.data.len() {
            panic!(
                "x={x} y={y} is out of bounds of image with dimensions w={} h={}",
                self.width, self.height
            );
        }
        idx
    }

    pub fn read(path: impl AsRef<Path>) -> Self {
        let file = File::open(path).expect("failed to open file");
        let mut reader = BufReader::new(file);

        let mut buf = String::new();
        _ = reader
            .read_line(&mut buf)
            .expect("failed to read magic value line in header");

        // read_line yields the trailing new line, so throughout this function we must explicitly strip it away
        if &buf[..buf.len() - 1] != "P3" {
            panic!("magic value does not indicate that this file is an ASCII Portable PixMap file");
        }

        buf.clear();
        _ = reader
            .read_line(&mut buf)
            .expect("failed to read image size line in header");

        let tokens: Vec<_> = buf[..buf.len() - 1].split(' ').collect();
        if tokens.len() != 2 {
            panic!("image dimensions in header were in the wrong format");
        }

        let width: usize = tokens[0].parse().expect("failed to parse width");
        let height: usize = tokens[1].parse().expect("failed to parse height");

        buf.clear();
        _ = reader
            .read_line(&mut buf)
            .expect("failed to read color channel max value in header");

        if &buf[..buf.len() - 1] != "255" {
            panic!("this library only supports color channel max values of 255 (u8)");
        }

        let pixel_count = width * height;
        let mut data = Vec::with_capacity(pixel_count);

        for (idx, line) in reader.lines().enumerate() {
            if idx >= pixel_count {
                panic!("more pixels in body than indicated in header");
            }

            let line = line.expect("failed to read pixel line");
            let tokens: Vec<_> = line.split(' ').collect();

            if tokens.len() != 3 {
                panic!("pixel line {idx} was in the wrong format");
            }

            let r: u8 = tokens[0].parse().expect("failed to parse red channel");
            let g: u8 = tokens[1].parse().expect("failed to parse green channel");
            let b: u8 = tokens[2].parse().expect("failed to parse blue channel");

            data.push(Color(r, g, b));
        }

        if data.len() < pixel_count {
            panic!("less pixels in body than indicated in header");
        }

        Self {
            data,
            width,
            height,
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) {
        let mut temp_path = PathBuf::from(path.as_ref());
        let extension = temp_path.extension().unwrap().to_str().unwrap().to_owned();
        let extension = format!("{extension}.tmp");
        temp_path.set_extension(extension);

        let mut file = File::create(&temp_path).expect("failed to create file");

        // Magic value to indicate that this file is written using the ASCII Portable PixMap representation.
        file.write_all(b"P3\n")
            .expect("failed to write magic value to header");

        // Space-delimited width and height of the PixMap.
        file.write_all(format!("{} {}\n", self.width, self.height).as_bytes())
            .expect("failed to write image dimensions to header");

        // Each color channel is represented by a u8, which inherently has a maximum value of 255.
        file.write_all(b"255\n")
            .expect("failed to write color channel max value to header");

        for (idx, color) in self.data.iter().enumerate() {
            if let Err(error) =
                file.write_all(format!("{} {} {}\n", color.0, color.1, color.2).as_bytes())
            {
                let x = idx % self.width;
                let y = idx / self.width;
                panic!("failed to write pixel at x={x} y={y}: {error}");
            }
        }

        std::fs::rename(temp_path, path).expect("failed to swap temp file");
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

pub struct Iter<'a> {
    pixels: &'a Pixels,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (usize, usize, Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.pixels.width * self.pixels.height {
            return None;
        }
        let color = self.pixels.data[self.index];
        let x = self.index % self.pixels.width;
        let y = self.index / self.pixels.width;
        self.index += 1;
        Some((x, y, color))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read() {
        let pixels = Pixels::read("./test-fixtures/out.ppm");
        assert_eq!(pixels.width(), 91);
        assert_eq!(pixels.height(), 91);
    }

    #[test]
    fn taste_the_rainbow() {
        const RED: Color = Color(255, 0, 0);
        const ORANGE: Color = Color(255, 127, 0);
        const YELLOW: Color = Color(255, 255, 0);
        const GREEN: Color = Color(0, 255, 0);
        const BLUE: Color = Color(0, 0, 255);
        const INDIGO: Color = Color(75, 0, 130);
        const VIOLET: Color = Color(238, 130, 238);

        const WIDTH: usize = 300;
        const HEIGHT: usize = 7 * COLOR_HEIGHT;
        const COLOR_HEIGHT: usize = 50;

        let mut pixels = Pixels::new(WIDTH, HEIGHT);

        for (idx, color) in [RED, ORANGE, YELLOW, GREEN, BLUE, INDIGO, VIOLET]
            .into_iter()
            .enumerate()
        {
            let base_y = idx * COLOR_HEIGHT;
            for y in base_y..base_y + COLOR_HEIGHT {
                for x in 0..WIDTH {
                    pixels.set(x, y, color);
                }
            }
        }

        pixels.save("test.ppm");
    }
}
