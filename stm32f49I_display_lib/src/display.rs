#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
    InvertedPortrait,
    InvertedLandscape,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb565,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rgb565(pub u16);

impl Rgb565 {
    pub const BLACK: Self = Self(0x0000);
    pub const WHITE: Self = Self(0xFFFF);

    pub fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
        let r5 = (r as u16 >> 3) & 0x1F;
        let g6 = (g as u16 >> 2) & 0x3F;
        let b5 = (b as u16 >> 3) & 0x1F;
        Self((r5 << 11) | (g6 << 5) | b5)
    }
}

#[derive(Clone, Debug)]
pub struct DisplayConfig {
    pub width: u16,
    pub height: u16,
    pub orientation: Orientation,
    pub pixel_format: PixelFormat,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Layer {
    Layer1,
    Layer2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidDimensions,
    UnsupportedPixelFormat,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidDimensions => f.write_str("invalid dimensions: width and height must be > 0"),
            Error::UnsupportedPixelFormat => f.write_str("unsupported pixel format"),
        }
    }
}

/// A simple host-side display backed by a framebuffer for testing/examples.
/// In embedded builds, this is not compiled.
#[cfg(feature = "std")]
#[derive(Clone, Debug)]
pub struct Display {
    width: u16,
    height: u16,
    orientation: Orientation,
    pixel_format: PixelFormat,
    framebuffer_l1: std::vec::Vec<u16>, // RGB565 pixels - Layer1
    framebuffer_l2: std::vec::Vec<u16>, // RGB565 pixels - Layer2
}

#[cfg(feature = "std")]
pub fn init_display(cfg: DisplayConfig) -> Result<Display, Error> {
    if cfg.width == 0 || cfg.height == 0 {
        return Err(Error::InvalidDimensions);
    }
    match cfg.pixel_format {
        PixelFormat::Rgb565 => {}
    }

    let len = cfg.width as usize * cfg.height as usize;
    let framebuffer_l1 = vec![Rgb565::BLACK.0; len];
    let framebuffer_l2 = vec![Rgb565::BLACK.0; len];
    Ok(Display {
        width: cfg.width,
        height: cfg.height,
        orientation: cfg.orientation,
        pixel_format: cfg.pixel_format,
        framebuffer_l1,
        framebuffer_l2,
    })
}

#[cfg(feature = "std")]
impl Display {
    pub fn size(&self) -> (u16, u16) { (self.width, self.height) }
    pub fn orientation(&self) -> Orientation { self.orientation }
    pub fn pixel_format(&self) -> PixelFormat { self.pixel_format }

    pub fn clear(&mut self, color: Rgb565) -> Result<(), Error> {
        // For backward-compat, clear both layers
        self.framebuffer_l1.fill(color.0);
        self.framebuffer_l2.fill(color.0);
        Ok(())
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> Option<Rgb565> {
        if x >= self.width || y >= self.height { return None; }
        let idx = y as usize * self.width as usize + x as usize;
        self.framebuffer_l1.get(idx).copied().map(Rgb565)
    }

    pub(crate) fn set_pixel_unchecked(&mut self, x: u16, y: u16, color: Rgb565) {
        let idx = y as usize * self.width as usize + x as usize;
        if let Some(px) = self.framebuffer_l1.get_mut(idx) {
            *px = color.0;
        }
    }

    pub fn clear_layer(&mut self, layer: Layer, color: Rgb565) {
        match layer {
            Layer::Layer1 => self.framebuffer_l1.fill(color.0),
            Layer::Layer2 => self.framebuffer_l2.fill(color.0),
        }
    }

    pub fn get_pixel_from_layer(&self, layer: Layer, x: u16, y: u16) -> Option<Rgb565> {
        if x >= self.width || y >= self.height { return None; }
        let idx = y as usize * self.width as usize + x as usize;
        match layer {
            Layer::Layer1 => self.framebuffer_l1.get(idx).copied().map(Rgb565),
            Layer::Layer2 => self.framebuffer_l2.get(idx).copied().map(Rgb565),
        }
    }

    pub(crate) fn set_pixel_unchecked_in_layer(&mut self, layer: Layer, x: u16, y: u16, color: Rgb565) {
        let idx = y as usize * self.width as usize + x as usize;
        match layer {
            Layer::Layer1 => {
                if let Some(px) = self.framebuffer_l1.get_mut(idx) { *px = color.0; }
            }
            Layer::Layer2 => {
                if let Some(px) = self.framebuffer_l2.get_mut(idx) { *px = color.0; }
            }
        }
    }

    #[cfg(feature = "std")]
    pub fn save_ppm<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), std::io::Error> {
        use std::fs::File;
        use std::io::{BufWriter, Write};
        let (w, h) = self.size();
        let mut f = BufWriter::new(File::create(path)?);
        // P6 binary PPM header
        writeln!(f, "P6\n{} {}\n255", w, h)?;
        for y in 0..h {
            for x in 0..w {
                if let Some(Rgb565(px)) = self.get_pixel(x, y) {
                    let r5 = ((px >> 11) & 0x1F) as u8;
                    let g6 = ((px >> 5) & 0x3F) as u8;
                    let b5 = (px & 0x1F) as u8;
                    // Expand to 8-bit
                    let r8 = (r5 << 3) | (r5 >> 2);
                    let g8 = (g6 << 2) | (g6 >> 4);
                    let b8 = (b5 << 3) | (b5 >> 2);
                    f.write_all(&[r8, g8, b8])?;
                } else {
                    f.write_all(&[0, 0, 0])?;
                }
            }
        }
        Ok(())
    }
}
