#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use stm32f4xx_hal as hal;
use hal::{
    gpio::{Output, PushPull},
    prelude::*,
};
use embedded_hal::digital::blocking::OutputPin as _;

#[derive(Copy, Clone)]
struct Rgb565(u16);

impl Rgb565 {
    fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
        let r5 = (r as u16 >> 3) & 0x1F;
        let g6 = (g as u16 >> 2) & 0x3F;
        let b5 = (b as u16 >> 3) & 0x1F;
        Rgb565((r5 << 11) | (g6 << 5) | b5)
    }
}

struct Gc9a01Parallel {
    // Control
    cs: hal::gpio::gpioa::PA3<Output<PushPull>>,   // CS
    wr: hal::gpio::gpioa::PA15<Output<PushPull>>,  // WR
    rd: hal::gpio::gpiob::PB0<Output<PushPull>>,   // RD
    dc: hal::gpio::gpiob::PB1<Output<PushPull>>,   // DC
    rst: hal::gpio::gpiob::PB8<Output<PushPull>>,  // RESET
    bl: hal::gpio::gpioa::PA4<Output<PushPull>>,   // Backlight
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

    fn set_addr_window(&mut self, x: u16, y: u16) {
        self.write_cmd(0x2A); // CASET
        self.write_data_u16_be(x);
        self.write_data_u16_be(x);
        self.write_cmd(0x2B); // PASET
        self.write_data_u16_be(y);
        self.write_data_u16_be(y);
        self.write_cmd(0x2C); // RAMWR
    }

    fn init(&mut self) {
        let _ = self.cs.set_high();
        let _ = self.wr.set_high();
        let _ = self.rd.set_high();
        let _ = self.dc.set_high();
        let _ = self.bl.set_low();

        let _ = self.rst.set_low();
        for _ in 0..(72_000) { cortex_m::asm::nop(); }
        let _ = self.rst.set_high();
        for _ in 0..(8_640_000) { cortex_m::asm::nop(); }

        let _ = self.cs.set_low();
        self.write_cmd(0x11); // Sleep out
        for _ in 0..(8_640_000) { cortex_m::asm::nop(); }
        self.write_cmd(0x3A); // Pixel format
        self.write_data_u8(0x55); // 16-bit
        self.write_cmd(0x36); // MADCTL
        self.write_data_u8(0x28); // MV + BGR (landscape)
        self.write_cmd(0x29); // Display ON
        let _ = self.bl.set_high();
    }

    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) {
        self.set_addr_window(x, y);
        self.write_data_u16_be(color.0);
    }
}

fn draw_rect_outline(drv: &mut Gc9a01Parallel, x: u16, y: u16, w: u16, h: u16, thickness: u16, color: Rgb565) {
    if w == 0 || h == 0 { return; }
    let x2 = x + w - 1;
    let y2 = y + h - 1;
    let t = thickness;

    // Top and bottom borders
    for yy in y..=core::cmp::min(y + t - 1, y2) {
        for xx in x..=x2 { drv.set_pixel(xx, yy, color); }
    }
    for yy in y2.saturating_sub(t - 1)..=y2 {
        for xx in x..=x2 { drv.set_pixel(xx, yy, color); }
    }
    // Left and right borders
    for xx in x..=core::cmp::min(x + t - 1, x2) {
        for yy in y..=y2 { drv.set_pixel(xx, yy, color); }
    }
    for xx in x2.saturating_sub(t - 1)..=x2 {
        for yy in y..=y2 { drv.set_pixel(xx, yy, color); }
    }
}

#[entry]
fn main() -> ! {
    let dp = hal::pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(72.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let cs = gpioa.pa3.into_push_pull_output();
    let wr = gpioa.pa15.into_push_pull_output();
    let rd = gpiob.pb0.into_push_pull_output();
    let dc = gpiob.pb1.into_push_pull_output();
    let rst = gpiob.pb8.into_push_pull_output();
    let bl = gpioa.pa4.into_push_pull_output();

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

    // Simple rectangle to verify draw path works
    let color = Rgb565::from_rgb888(255, 0, 0);
    draw_rect_outline(&mut panel, 20, 30, 180, 120, 4, color);

    loop { cortex_m::asm::wfi(); }
}
