use std::io::{self, Write};
use std::path::Path;
use std::fs::File;

pub type Rgb = [u8;3];

pub struct PPM {
    width: usize,
    height: usize,
    pixels: Vec<Rgb>,
}

impl PPM {
    pub fn new(width: usize, height: usize) -> PPM {
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize_with(width * height, Default::default);
        PPM {
            width,
            height,
            pixels,
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, v: Rgb) {
        let i = self.width * x + y;
        self.pixels[i] = v;
    }

    pub fn save<P: AsRef<Path>>(&self, filename: P) -> io::Result<()> {
        let mut f = File::create(filename)?;
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.width * x + y;
                writeln!(
                    f,
                    "{} {} {}",
                    self.pixels[i][0],
                    self.pixels[i][1],
                    self.pixels[i][2],
                )?;
            }
        }
        Ok(())
    }
}
