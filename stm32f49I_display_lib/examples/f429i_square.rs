#![cfg_attr(feature = "hw-ili9341", no_std)]
#![cfg_attr(feature = "hw-ili9341", no_main)]

// STM32F429I-DISC hardware square render using 8-bit parallel GC9A01A interface.
// Pins per provided mapping: CS=PA3, WR=PA15, RD=PB0, DC=PB1, RESET=PB8, BL=PA4,
// Data bus D0..D7 = PA10, PA9, PB15, PB14, PB13, PB12, PB11, PB10.

#[cfg(feature = "hw-ili9341")]
use panic_halt as _;

#[cfg(feature = "hw-ili9341")]
use cortex_m_rt::entry;

#[cfg(feature = "hw-ili9341")]
use stm32f4xx_hal as hal;

#[cfg(feature = "hw-ili9341")]
use hal::{
    gpio::{Output, PushPull},
    prelude::*,
};

#[cfg(feature = "hw-ili9341")]
use embedded_hal::digital::blocking::OutputPin as _;

#[cfg(feature = "hw-ili9341")]
use stm32f49I_display_lib::{draw_rectangle_outline, Rect, Rgb565};

#[cfg(feature = "hw-ili9341")]
use stm32f49I_display_lib::hw::ili9341::{Ili9341Display, Ili9341Driver};
#[cfg(feature = "hw-ili9341")]
use stm32f49I_display_lib::shapes::PixelSink;

#[cfg(feature = "hw-ili9341")]
struct Gc9a01Parallel {
    // Control
    cs: hal::gpio::gpioa::PA3<Output<PushPull>>,
    wr: hal::gpio::gpioa::PA15<Output<PushPull>>,
    rd: hal::gpio::gpiob::PB0<Output<PushPull>>,
    dc: hal::gpio::gpiob::PB1<Output<PushPull>>,
    rst: hal::gpio::gpiob::PB8<Output<PushPull>>,
    bl: hal::gpio::gpioa::PA4<Output<PushPull>>,
    // Data bus
    d0: hal::gpio::gpioa::PA10<Output<PushPull>>,
    d1: hal::gpio::gpioa::PA9<Output<PushPull>>,
    d2: hal::gpio::gpiob::PB15<Output<PushPull>>,
    d3: hal::gpio::gpiob::PB14<Output<PushPull>>,
    d4: hal::gpio::gpiob::PB13<Output<PushPull>>,
    d5: hal::gpio::gpiob::PB12<Output<PushPull>>,
    d6: hal::gpio::gpiob::PB11<Output<PushPull>>,
    d7: hal::gpio::gpiob::PB10<Output<PushPull>>,
}

#[cfg(feature = "hw-ili9341")]
impl Gc9a01Parallel {
    fn set_bus(&mut self, v: u8) {
        let _ = if v & 0x01 != 0 { self.d0.set_high() } else { self.d0.set_low() };
        let _ = if v & 0x02 != 0 { self.d1.set_high() } else { self.d1.set_low() };
        let _ = if v & 0x04 != 0 { self.d2.set_high() } else { self.d2.set_low() };
        let _ = if v & 0x08 != 0 { self.d3.set_high() } else { self.d3.set_low() };
        let _ = if v & 0x10 != 0 { self.d4.set_high() } else { self.d4.set_low() };
        let _ = if v & 0x20 != 0 { self.d5.set_high() } else { self.d5.set_low() };
        let _ = if v & 0x40 != 0 { self.d6.set_high() } else { self.d6.set_low() };
        let _ = if v & 0x80 != 0 { self.d7.set_high() } else { self.d7.set_low() };
    }

    fn pulse_wr(&mut self) {
        let _ = self.wr.set_low();
        for _ in 0..20 { cortex_m::asm::nop(); }
        let _ = self.wr.set_high();
    }

    fn write_cmd(&mut self, cmd: u8) {
        let _ = self.dc.set_low();
        self.set_bus(cmd);
        self.pulse_wr();
    }

    fn write_data_u8(&mut self, data: u8) {
        let _ = self.dc.set_high();
        self.set_bus(data);
        self.pulse_wr();
    }

    fn write_data_u16_be(&mut self, data: u16) {
        self.write_data_u8((data >> 8) as u8);
        self.write_data_u8((data & 0xFF) as u8);
    }

    fn begin_addr_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        // Column Address Set (CASET 0x2A)
        self.write_cmd(0x2A);
        self.write_data_u16_be(x0);
        self.write_data_u16_be(x1);
        // Page Address Set (PASET 0x2B)
        self.write_cmd(0x2B);
        self.write_data_u16_be(y0);
        self.write_data_u16_be(y1);
        // Memory Write (RAMWR 0x2C)
        self.write_cmd(0x2C);
    }

    fn fill_rect(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: Rgb565) {
        // Assumes CS is already low
        self.begin_addr_window(x0, y0, x1, y1);
        let px = ((x1 as u32 - x0 as u32 + 1) * (y1 as u32 - y0 as u32 + 1)) as u32;
        for _ in 0..px {
            self.write_data_u16_be(color.0);
        }
    }

    fn set_addr_window(&mut self, x: u16, y: u16) {
        // Column Address Set (CASET 0x2A)
        self.write_cmd(0x2A);
        self.write_data_u16_be(x);
        self.write_data_u16_be(x);
        // Page Address Set (PASET 0x2B)
        self.write_cmd(0x2B);
        self.write_data_u16_be(y);
        self.write_data_u16_be(y);
        // Memory Write (RAMWR 0x2C)
        self.write_cmd(0x2C);
    }

    fn init(&mut self) {
        let _ = self.cs.set_high();
        let _ = self.wr.set_high();
        let _ = self.rd.set_high();
        let _ = self.dc.set_high();
        let _ = self.bl.set_low();

        // Hardware reset with crude delays
        let _ = self.rst.set_low();
        for _ in 0..(72_000) { cortex_m::asm::nop(); } // ~1ms at 72MHz
        let _ = self.rst.set_high();
        for _ in 0..(8_640_000) { cortex_m::asm::nop(); } // ~120ms at 72MHz

        let _ = self.cs.set_low();

        // Sleep out
        self.write_cmd(0x11);
        for _ in 0..(8_640_000) { cortex_m::asm::nop(); } // ~120ms

        // Pixel format: 16-bit (RGB565)
        self.write_cmd(0x3A);
        self.write_data_u8(0x55);

        // Memory Access Control (orientation) - landscape (MV + BGR)
        self.write_cmd(0x36);
        self.write_data_u8(0x28);

        // Display ON
        self.write_cmd(0x29);
        let _ = self.bl.set_high();
    }
}

#[cfg(feature = "hw-ili9341")]
impl Ili9341Driver for Gc9a01Parallel {
    type Error = core::convert::Infallible;
    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) -> Result<(), Self::Error> {
        self.set_addr_window(x, y);
        self.write_data_u16_be(color.0);
        Ok(())
    }
}

#[cfg(feature = "hw-ili9341")]
#[entry]
fn main() -> ! {
    let dp = hal::pac::Peripherals::take().unwrap();

    // Configure clocks per provided spec (72 MHz)
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(72.MHz()).freeze();

    // Acquire GPIO ports
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    // Control pins
    let cs = gpioa.pa3.into_push_pull_output();
    let wr = gpioa.pa15.into_push_pull_output();
    let rd = gpiob.pb0.into_push_pull_output();
    let dc = gpiob.pb1.into_push_pull_output();
    let rst = gpiob.pb8.into_push_pull_output();
    let bl = gpioa.pa4.into_push_pull_output();

    // Data bus pins
    let d0 = gpioa.pa10.into_push_pull_output();
    let d1 = gpioa.pa9.into_push_pull_output();
    let d2 = gpiob.pb15.into_push_pull_output();
    let d3 = gpiob.pb14.into_push_pull_output();
    let d4 = gpiob.pb13.into_push_pull_output();
    let d5 = gpiob.pb12.into_push_pull_output();
    let d6 = gpiob.pb11.into_push_pull_output();
    let d7 = gpiob.pb10.into_push_pull_output();

    let mut panel = Gc9a01Parallel { cs, wr, rd, dc, rst, bl, d0, d1, d2, d3, d4, d5, d6, d7 };
    panel.init();

    // Wrap in our PixelSink adapter
    let mut lcd = Ili9341Display { drv: panel, width: 320, height: 240 };

    // Quick sanity: fill screen to verify wiring/backlight
    {
        let color = Rgb565::from_rgb888(0, 0, 255); // blue
        lcd.drv.fill_rect(0, 0, lcd.width - 1, lcd.height - 1, color);
    }

    // Compute a centered square and draw an outline
    let (w, h) = lcd.size();
    let side: u16 = 120;
    let x = ((w - side) / 2) as i32;
    let y = ((h - side) / 2) as i32;
    let square = Rect { x, y, width: side, height: side };
    let color = Rgb565::from_rgb888(0, 255, 0);
    draw_rectangle_outline(&mut lcd, square, color, 6).unwrap();

    loop { cortex_m::asm::wfi(); }
}

#[cfg(not(feature = "hw-ili9341"))]
fn main() {
    println!("Feature 'hw-ili9341' not enabled. Re-run with --features hw-ili9341.");
}
