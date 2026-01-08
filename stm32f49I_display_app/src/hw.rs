use stm32f4xx_hal as hal;
use hal::{
    gpio::{Output, PushPull},
    prelude::*,
};

#[derive(Copy, Clone)]
pub struct Rgb565(pub u16);

impl Rgb565 {
    pub fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
        let r5 = (r as u16 >> 3) & 0x1F;
        let g6 = (g as u16 >> 2) & 0x3F;
        let b5 = (b as u16 >> 3) & 0x1F;
        Rgb565((r5 << 11) | (g6 << 5) | b5)
    }

    pub const BLACK: Rgb565 = Rgb565(0x0000);
    pub const WHITE: Rgb565 = Rgb565(0xFFFF);
    pub const RED: Rgb565 = Rgb565(0xF800);
    pub const GREEN: Rgb565 = Rgb565(0x07E0);
    pub const BLUE: Rgb565 = Rgb565(0x001F);
    pub const YELLOW: Rgb565 = Rgb565(0xFFE0);
    pub const CYAN: Rgb565 = Rgb565(0x07FF);
    pub const MAGENTA: Rgb565 = Rgb565(0xF81F);
    pub const GRAY: Rgb565 = Rgb565(0x8410);
}

pub struct Gc9a01Parallel {
    pub cs: hal::gpio::gpioa::PA3<Output<PushPull>>,   // CS
    pub wr: hal::gpio::gpioa::PA15<Output<PushPull>>,  // WR
    pub rd: hal::gpio::gpiob::PB0<Output<PushPull>>,   // RD
    pub dc: hal::gpio::gpiob::PB1<Output<PushPull>>,   // DC
    pub rst: hal::gpio::gpiob::PB8<Output<PushPull>>,  // RESET
    pub bl: hal::gpio::gpioa::PA4<Output<PushPull>>,   // Backlight
    pub d0: hal::gpio::gpioa::PA10<Output<PushPull>>,
    pub d1: hal::gpio::gpioa::PA9<Output<PushPull>>,
    pub d2: hal::gpio::gpiob::PB15<Output<PushPull>>,
    pub d3: hal::gpio::gpiob::PB14<Output<PushPull>>,
    pub d4: hal::gpio::gpiob::PB13<Output<PushPull>>,
    pub d5: hal::gpio::gpiob::PB12<Output<PushPull>>,
    pub d6: hal::gpio::gpiob::PB11<Output<PushPull>>,
    pub d7: hal::gpio::gpiob::PB10<Output<PushPull>>,
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

    pub fn init(&mut self) {
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

    pub fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) {
        self.set_addr_window(x, y);
        self.write_data_u16_be(color.0);
    }
}

pub fn init() -> Gc9a01Parallel {
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

    panel
}
