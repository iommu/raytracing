use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::{interval::Interval, vec3::Color};

pub trait Exporter {
    fn set_dims(&mut self, width: i32, height: i32);
    fn write_header(&mut self) -> io::Result<()>;
    fn write_pixel(&mut self, color: Color) -> io::Result<()>;
    fn flush(&mut self) -> io::Result<()>;
}

#[derive(Debug)]
pub struct PPMExporter {
    file: BufWriter<File>,
    width: i32,
    height: i32,
}

impl PPMExporter {
    #[allow(dead_code)]
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            file: BufWriter::new(file),
            width: 0,
            height: 0,
        })
    }
}

impl Exporter for PPMExporter {
    fn set_dims(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    fn write_header(&mut self) -> io::Result<()> {
        let file = &mut self.file;

        // Header
        file.write_fmt(format_args!("P3\n{} {}\n255\n", self.width, self.height))?;

        Ok(())
    }

    fn write_pixel(&mut self, color: Color) -> io::Result<()> {
        let file = &mut self.file;

        // Translate the [0,1] component values to the byte range [0,255]
        let intensity = Interval::new(0.0, 0.999);
        let ir = (255.999 * intensity.clamp(color.x)) as i64;
        let ig = (255.999 * intensity.clamp(color.y)) as i64;
        let ib = (255.999 * intensity.clamp(color.z)) as i64;

        // Write out the pixel color components
        file.write_fmt(format_args!("{ir} {ig} {ib}\n"))?;

        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct BMPExporter {
    file: BufWriter<File>,
    width: i32,
    height: i32,
}

impl BMPExporter {
    #[allow(dead_code)]
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            file: BufWriter::new(file),
            width: 0,
            height: 0,
        })
    }
}

impl Exporter for BMPExporter {
    fn set_dims(&mut self, width: i32, height: i32) {
        self.width = width;
        // Note : negative height because BMPs are usually bottom up data structures and we render top down
        self.height = -height;
    }

    fn write_header(&mut self) -> io::Result<()> {
        let file = &mut self.file;

        // Header
        file.write_all(&[66, 77])?; // signature
        file.write_u32::<LittleEndian>(0x38)?; // file size
        file.write_u32::<LittleEndian>(0x00)?; // reserved
        file.write_u32::<LittleEndian>(0x36)?; // data offset

        // Info Header
        file.write_u32::<LittleEndian>(0x28)?; // size of info header
        file.write_i32::<LittleEndian>(self.width)?; // width
        file.write_i32::<LittleEndian>(self.height)?; // height
        file.write_u16::<LittleEndian>(1)?; // num planes
        file.write_u16::<LittleEndian>(24)?; // bits per pixel 
        file.write_u32::<LittleEndian>(0)?; // compression // none
        file.write_u32::<LittleEndian>(0)?; // image size // 0 for no compression
        file.write_u32::<LittleEndian>(0x002e23)?; // horizontal resolution
        file.write_u32::<LittleEndian>(0x002e23)?; // vertical resolution
        file.write_u32::<LittleEndian>(0)?; // number of colors used
        file.write_u32::<LittleEndian>(0)?; // important colors

        Ok(())
    }

    fn write_pixel(&mut self, color: Color) -> io::Result<()> {
        let file = &mut self.file;

        // Translate the [0,1] component values to the byte range [0,255]
        let intensity = Interval::new(0.0, 0.999);
        let r = (255.999 * intensity.clamp(color.x)) as u8;
        let g = (255.999 * intensity.clamp(color.y)) as u8;
        let b = (255.999 * intensity.clamp(color.z)) as u8;

        // Write out the pixel color components
        file.write_u8(r)?;
        file.write_u8(g)?;
        file.write_u8(b)?;

        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()?;
        Ok(())
    }
}
