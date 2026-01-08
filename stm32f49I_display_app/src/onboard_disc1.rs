#![allow(dead_code)]

use stm32f4xx_hal as hal;
use hal::pac;
use hal::prelude::*;
use hal::gpio::Speed;

pub struct Ili9341Spi;

impl Ili9341Spi {
    #[inline(always)]
    fn delay_cycles(n: u32) { for _ in 0..n { unsafe { cortex_m::asm::nop() } } }

    #[inline(always)]
    fn spi5() -> &'static pac::spi5::RegisterBlock {
        unsafe { &*pac::SPI5::ptr() }
    }

    #[inline(always)]
    fn cs_low() { unsafe { (&*pac::GPIOC::ptr()).bsrr.write(|w| w.br2().set_bit()); } }
    #[inline(always)]
    fn cs_high() { unsafe { (&*pac::GPIOC::ptr()).bsrr.write(|w| w.bs2().set_bit()); } }
    #[inline(always)]
    fn dc_low() { unsafe { (&*pac::GPIOD::ptr()).bsrr.write(|w| w.br13().set_bit()); } }
    #[inline(always)]
    fn dc_high() { unsafe { (&*pac::GPIOD::ptr()).bsrr.write(|w| w.bs13().set_bit()); } }

    fn spi_tx(byte: u8) {
        let spi = Self::spi5();
        // Wait TXE
        while spi.sr.read().txe().bit_is_clear() {}
        unsafe { spi.dr.write(|w| w.dr().bits(byte as u16)); }
        // Wait BSY cleared (end of transfer)
        while spi.sr.read().bsy().bit_is_set() {}
        // Read and clear OVR if any by reading DR then SR (not strictly needed for pure TX)
        let _ = spi.dr.read().dr().bits();
        let _ = spi.sr.read().bits();
    }

    fn wr_cmd(cmd: u8) {
        Self::cs_low();
        Self::dc_low();
        Self::spi_tx(cmd);
        Self::cs_high();
    }

    fn wr_data8(data: u8) {
        Self::cs_low();
        Self::dc_high();
        Self::spi_tx(data);
        Self::cs_high();
    }

    fn wr_data16(data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        Self::cs_low();
        Self::dc_high();
        Self::spi_tx(hi);
        Self::spi_tx(lo);
        Self::cs_high();
    }

    fn write_stream_16(color_565: u16, count: u32) {
        let hi = (color_565 >> 8) as u8;
        let lo = (color_565 & 0xFF) as u8;
        Self::cs_low();
        Self::dc_high();
        for _ in 0..count {
            Self::spi_tx(hi);
            Self::spi_tx(lo);
        }
        Self::cs_high();
    }

    pub fn init_and_take() -> Self {
        let dp = pac::Peripherals::take().unwrap();

        // Clocks (keep modest core clock for now)
        let rcc = dp.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(84.MHz()).hclk(84.MHz()).freeze();

        // Enable GPIOC/D/E/F clocks and SPI5 clock
        let rcc_pac = unsafe { &*pac::RCC::ptr() };
        rcc_pac.ahb1enr.modify(|_, w| w.gpiocen().enabled().gpioden().enabled().gpioeen().enabled().gpiofen().enabled());
        rcc_pac.apb2enr.modify(|_, w| w.spi5en().enabled());

        // Configure SPI5 pins: PF7=SCK, PF8=MISO, PF9=MOSI (AF5)
        let gpiof = dp.GPIOF.split();
        let mut pf7 = gpiof.pf7.into_alternate::<5>(); pf7.set_speed(Speed::VeryHigh);
        let mut pf8 = gpiof.pf8.into_alternate::<5>(); pf8.set_speed(Speed::VeryHigh);
        let mut pf9 = gpiof.pf9.into_alternate::<5>(); pf9.set_speed(Speed::VeryHigh);
        let _ = (pf7, pf8, pf9); // keep locals alive until after SPI enabled

        // Control pins: PC2=CS, PD13=DC, PD12=RST. Also try PB1 as BL (if present)
        let gpioe = dp.GPIOE.split();
        let gpioc = dp.GPIOC.split();
        let gpiod = dp.GPIOD.split();
        let gpiob = dp.GPIOB.split();
        let mut pc2 = gpioc.pc2.into_push_pull_output(); pc2.set_speed(Speed::VeryHigh);
        let mut pd13 = gpiod.pd13.into_push_pull_output(); pd13.set_speed(Speed::VeryHigh);
        let mut pd12 = gpiod.pd12.into_push_pull_output(); pd12.set_speed(Speed::VeryHigh);
        let mut pb1 = gpiob.pb1.into_push_pull_output(); pb1.set_speed(Speed::VeryHigh);
        let mut pe6 = gpioe.pe6.into_push_pull_output(); pe6.set_speed(Speed::VeryHigh);
        let _ = pc2.set_high(); // CS idle high
        let _ = pd13.set_low(); // DC default low
        let _ = pb1.set_high(); // Try enable backlight on PB1 (harmless if NC)
        let _ = pe6.set_high(); // Also try enable BL on PE6 (alternate revisions)

        // SPI5 basic init: master, 8-bit, CPOL=0 CPHA=0, software NSS, fPCLK/8
        let spi = Self::spi5();
        // Disable SPI before configuring
        spi.cr1.modify(|_, w| w.spe().clear_bit());
        // CR1 fields
        spi.cr1.modify(|_, w| {
            w.cpha().clear_bit(); // CPHA=0
            w.cpol().clear_bit(); // CPOL=0
            w.mstr().set_bit();   // Master
            unsafe { w.br().bits(0b010) }; // fPCLK/8
            w.lsbfirst().clear_bit();
            w.ssm().set_bit();    // Software NSS
            w.ssi().set_bit();    // Internal NSS high
            w.rxonly().clear_bit();
            w.dff().clear_bit();  // 8-bit
            w
        });
        // Enable SPI
        spi.cr1.modify(|_, w| w.spe().set_bit());

        // Reset pulse
        let _ = pd12.set_low();
        Self::delay_cycles(8_000_0);
        let _ = pd12.set_high();
        Self::delay_cycles(8_000_0);

        // ILI9341 init sequence
        Self::wr_cmd(0x01); // Software reset
        Self::delay_cycles(8_000_00);
        Self::wr_cmd(0x11); // Sleep out
        Self::delay_cycles(8_000_00);

        Self::wr_cmd(0x3A); // Pixel format
        Self::wr_data8(0x55); // 16-bit

        Self::wr_cmd(0x36); // MADCTL
        // MX=0, MY=1 (top-left origin), MV=0; BGR=1
        Self::wr_data8(0x48);

        // Address window 240x320 (portrait)
        Self::wr_cmd(0x2A); // CASET
        Self::wr_data16(0);
        Self::wr_data16(239);
        Self::wr_cmd(0x2B); // PASET
        Self::wr_data16(0);
        Self::wr_data16(319);

        Self::wr_cmd(0x29); // Display on
        Self::delay_cycles(200_000);

        Ili9341Spi
    }

    pub fn set_window(x0: u16, y0: u16, x1: u16, y1: u16) {
        Self::wr_cmd(0x2A);
        Self::wr_data16(x0);
        Self::wr_data16(x1);
        Self::wr_cmd(0x2B);
        Self::wr_data16(y0);
        Self::wr_data16(y1);
        Self::wr_cmd(0x2C);
    }

    pub fn set_pixel(x: u16, y: u16, color_565: u16) {
        Self::set_window(x, y, x, y);
        Self::wr_data16(color_565);
    }

    pub fn fill_rect(x0: u16, y0: u16, w: u16, h: u16, color_565: u16) {
        let x1 = x0.saturating_add(w.saturating_sub(1));
        let y1 = y0.saturating_add(h.saturating_sub(1));
        Self::set_window(x0, y0, x1, y1);
        let count = (w as u32) * (h as u32);
        Self::write_stream_16(color_565, count);
    }
}

pub fn draw_rect_outline(x: u16, y: u16, w: u16, h: u16, color_565: u16) {
    let x2 = x.saturating_add(w.saturating_sub(1));
    let y2 = y.saturating_add(h.saturating_sub(1));

    // Top and bottom
    Ili9341Spi::set_window(x, y, x2, y);
    Ili9341Spi::write_stream_16(color_565, (x2 - x + 1) as u32);
    Ili9341Spi::set_window(x, y2, x2, y2);
    Ili9341Spi::write_stream_16(color_565, (x2 - x + 1) as u32);

    // Left and right
    for yy in y..=y2 {
        Ili9341Spi::set_window(x, yy, x, yy);
        Ili9341Spi::wr_data16(color_565);
        Ili9341Spi::set_window(x2, yy, x2, yy);
        Ili9341Spi::wr_data16(color_565);
    }
}
