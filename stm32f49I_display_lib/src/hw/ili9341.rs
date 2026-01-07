#![cfg(feature = "hw-ili9341")]
use crate::display::{Orientation, PixelFormat, Rgb565};
use crate::shapes::PixelSink;

/// Wrapper type around an ILI9341 driver that implements `PixelSink`.
/// This is a skeleton; wire it up with your HAL SPI/DC/RST types.
pub struct Ili9341Display<D> {
    pub drv: D,
    pub width: u16,
    pub height: u16,
}

impl<D> PixelSink for Ili9341Display<D>
where
    D: Ili9341Driver,
{
    fn size(&self) -> (u16, u16) { (self.width, self.height) }
    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) {
        let _ = self.drv.set_pixel(x, y, color);
    }
}

/// Minimal driver trait expected by this wrapper. Implement for your chosen driver.
pub trait Ili9341Driver {
    type Error;
    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) -> Result<(), Self::Error>;
}
