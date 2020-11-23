use std::io::{self, Write};
use std::path::Path;
use std::convert::TryInto;
use std::fs::File;
use std::fmt::Debug;

pub type Rgb = [u8;3];

pub struct PPM {
    width: usize,
    height: usize,
    pixels: Vec<Rgb>,
}

impl PPM {
    pub fn new<N: TryInto<usize>>(width: N, height: N) -> PPM
        where <N as TryInto<usize>>::Error: Debug
    {
        let width: usize = width.try_into().unwrap();
        let height: usize = height.try_into().unwrap();
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize_with(width * height, Default::default);
        PPM {
            width,
            height,
            pixels,
        }
    }

    pub fn put_pixel<N: TryInto<usize>>(&mut self, x: N, y: N, v: Rgb)
        where <N as TryInto<usize>>::Error: Debug
    {
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
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
