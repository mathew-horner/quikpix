use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Copy)]
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

    pub fn save(&self, path: impl AsRef<Path>) {
        let mut file = File::create(path).expect("failed to create file");

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
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
