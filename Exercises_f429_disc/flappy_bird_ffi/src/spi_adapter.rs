//use embedded_hal::blocking::spi::{Transfer, Write};
use stm32f4xx_hal::spi;

// Wrapper to unify HAL SPI errors under one type
#[derive(Debug)]
pub struct MappedSpiError(pub spi::Error);

// Trait for SPI abstraction with error mapping
pub trait SpiAdapterTrait {
    fn write(&mut self, data: &[u8]) -> Result<(), MappedSpiError>;
    fn transfer<'a>(&mut self, data: &'a mut [u8]) -> Result<&'a [u8], MappedSpiError>;
}

// Generic adapter over HAL SPI implementation
pub struct SpiAdapter<SPI> {
    pub inner: SPI,
}

impl<SPI> SpiAdapter<SPI> {
    // Create a new SPI adapter
    pub fn new(inner: SPI) -> Self {
        Self { inner }
    }
}

// impl<SPI> SpiAdapterTrait for SpiAdapter<SPI>
// where
//     SPI: Transfer<u8> + Write<u8>,
//     SPI::Error: Into<spi::Error>,

impl<SPI> SpiAdapter<SPI>
where
    SPI: embedded_hal::blocking::spi::Transfer<u8> + embedded_hal::blocking::spi::Write<u8>,
    <SPI as embedded_hal::blocking::spi::Transfer<u8>>::Error: Into<spi::Error>,
    <SPI as embedded_hal::blocking::spi::Write<u8>>::Error: Into<spi::Error>,
{
    fn write(&mut self, data: &[u8]) -> Result<(), MappedSpiError> {
        self.inner.write(data).map_err(|e| MappedSpiError(e.into()))
    }

    fn transfer<'a>(&mut self, data: &'a mut [u8]) -> Result<&'a [u8], MappedSpiError> {
        self.inner.transfer(data).map_err(|e| MappedSpiError(e.into()))
    }
}
