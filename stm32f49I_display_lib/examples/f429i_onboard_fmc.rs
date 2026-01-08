#![cfg_attr(feature = "hw-onboard-lcd", no_std)]
#![cfg_attr(feature = "hw-onboard-lcd", no_main)]

#[cfg(feature = "hw-onboard-lcd")]
use panic_halt as _;

#[cfg(feature = "hw-onboard-lcd")]
use cortex_m_rt::entry;

#[cfg(feature = "hw-onboard-lcd")]
use stm32f4xx_hal as hal;
use hal::gpio::Speed;
use hal::prelude::*;

#[cfg(feature = "hw-onboard-lcd")]
use stm32f49I_display_lib::{draw_rectangle_outline, Rect, Rgb565};
#[cfg(feature = "hw-onboard-lcd")]
use stm32f49I_display_lib::shapes::PixelSink;

#[cfg(feature = "hw-onboard-lcd")]
const WIDTH: usize = 320;
#[cfg(feature = "hw-onboard-lcd")]
const HEIGHT: usize = 240;

// FMC RS address line selection via features (default A16)
#[cfg(all(feature = "rs-a16", any(feature = "rs-a17", feature = "rs-a18")))]
compile_error!("Select only one of rs-a16, rs-a17, rs-a18");
#[cfg(all(feature = "rs-a17", feature = "rs-a18"))]
compile_error!("Select only one of rs-a16, rs-a17, rs-a18");

const FMC_BASE: u32 = 0x6000_0000;
#[cfg(feature = "rs-a18")] const FMC_DATA_OFFSET: u32 = 0x0008_0000; // A18
#[cfg(feature = "rs-a17")] const FMC_DATA_OFFSET: u32 = 0x0004_0000; // A17
#[cfg(any(feature = "rs-a16", not(any(feature = "rs-a17", feature = "rs-a18"))))]
const FMC_DATA_OFFSET: u32 = 0x0002_0000; // A16 (default)

// Software framebuffer in SRAM for PixelSink drawing
#[cfg(feature = "hw-onboard-lcd")]
#[unsafe(link_section = ".bss.fb")]
static mut FRAMEBUFFER: [u16; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

// Raw pointer to FB to avoid Rust 2024 references to static mut
#[cfg(feature = "hw-onboard-lcd")]
static mut FB_PTR: *mut u16 = core::ptr::null_mut();

#[cfg(feature = "hw-onboard-lcd")]
struct FbDisplay;

#[cfg(feature = "hw-onboard-lcd")]
impl PixelSink for FbDisplay {
    fn size(&self) -> (u16, u16) { (WIDTH as u16, HEIGHT as u16) }
    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) {
        let xi = x as usize; let yi = y as usize;
        if xi < WIDTH && yi < HEIGHT {
            let idx = yi * WIDTH + xi;
            unsafe { core::ptr::write_volatile(FB_PTR.add(idx), color.0); }
        }
    }
}

// ILI9341 via FMC 8080 (Bank1 NE4) common mapping: CMD at 0x6000_0000, DATA at 0x6002_0000 (A18 as RS)
#[cfg(feature = "hw-onboard-lcd")]
struct Ili9341Fmc {
    cmd: *mut u16,
    data: *mut u16,
}

#[cfg(feature = "hw-onboard-lcd")]
impl Ili9341Fmc {
    const fn new() -> Self {
        Self { cmd: FMC_BASE as *mut u16, data: (FMC_BASE + FMC_DATA_OFFSET) as *mut u16 }
    }
    #[inline]
    fn write_cmd(&mut self, c: u8) {
        unsafe { core::ptr::write_volatile(self.cmd, c as u16); }
    }
    #[inline]
    fn write_data8(&mut self, d: u8) {
        unsafe { core::ptr::write_volatile(self.data, d as u16); }
    }
    #[inline]
    fn write_data16(&mut self, d: u16) {
        unsafe { core::ptr::write_volatile(self.data, d); }
    }
    #[inline]
    fn read_data8(&mut self) -> u8 {
        unsafe { core::ptr::read_volatile(self.data) as u8 }
    }
    fn write_data16_be(&mut self, d: u16) {
        self.write_data8((d >> 8) as u8);
        self.write_data8((d & 0xFF) as u8);
    }
    fn read_id4(&mut self) -> [u8; 4] {
        // Read ID4 (D3h): typically returns 0x00, 0x93, 0x41, 0x?? for ILI9341
        self.write_cmd(0xD3);
        let b0 = self.read_data8();
        let b1 = self.read_data8();
        let b2 = self.read_data8();
        let b3 = self.read_data8();
        [b0, b1, b2, b3]
    }
    fn set_addr_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        self.write_cmd(0x2A);
        self.write_data16_be(x0);
        self.write_data16_be(x1);
        self.write_cmd(0x2B);
        self.write_data16_be(y0);
        self.write_data16_be(y1);
        self.write_cmd(0x2C);
    }
    fn init(&mut self) {
        // Basic ILI9341 init for 8080 mode
        self.write_cmd(0x11); // Sleep out
        delay_cycles(8_000_000); // ~100ms
        self.write_cmd(0x3A); // Pixel format
        self.write_data8(0x55); // 16bpp
        self.write_cmd(0x36); // MADCTL landscape + BGR
        self.write_data8(0x28);
        self.write_cmd(0x29); // Display ON
    }
    fn flush_full(&mut self) {
        self.set_addr_window(0, 0, (WIDTH as u16) - 1, (HEIGHT as u16) - 1);
        unsafe {
            for i in 0..(WIDTH * HEIGHT) {
                let px = core::ptr::read_volatile(FB_PTR.add(i));
                self.write_data16_be(px);
            }
        }
    }
}

#[cfg(feature = "hw-onboard-lcd")]
fn delay_cycles(mut n: u32) { while n > 0 { cortex_m::asm::nop(); n -= 1; } }

#[cfg(feature = "hw-onboard-lcd")]
fn fmc_enable(dp: &mut hal::pac::Peripherals) {
    // Minimal step: enable FMC clock. Full GPIO AF/timing config is board-specific (TODO).
    unsafe { dp.RCC.ahb3enr.modify(|_, w| w.fmcen().set_bit()); }
}

#[cfg(feature = "hw-onboard-lcd")]
fn fmc_gpio_setup(gpiod_dev: hal::pac::GPIOD, gpioe_dev: hal::pac::GPIOE, gpiog_dev: hal::pac::GPIOG) {
    // Configure GPIOs for FMC (AF12) per typical F429I-DISCO mapping
    let gpiod = gpiod_dev.split();
    let gpioe = gpioe_dev.split();
    let gpiog = gpiog_dev.split();

    macro_rules! af12_vhs {
        ($pin:expr) => {{ let mut p = $pin.into_alternate::<12>(); p.set_speed(Speed::VeryHigh); }};
    }

    // GPIOD: D0..D3 (PD14,PD15,PD0,PD1), NOE (PD4), NWE (PD5), D13..D15 (PD8,PD9,PD10), A16..A18 (PD11,PD12,PD13)
    let mut _pd14 = af12_vhs!(gpiod.pd14); // D0
    let mut _pd15 = af12_vhs!(gpiod.pd15); // D1
    let mut _pd0  = af12_vhs!(gpiod.pd0);  // D2
    let mut _pd1  = af12_vhs!(gpiod.pd1);  // D3
    let mut _pd4  = af12_vhs!(gpiod.pd4);  // NOE
    let mut _pd5  = af12_vhs!(gpiod.pd5);  // NWE
    let mut _pd8  = af12_vhs!(gpiod.pd8);  // D13
    let mut _pd9  = af12_vhs!(gpiod.pd9);  // D14
    let mut _pd10 = af12_vhs!(gpiod.pd10); // D15
    let mut _pd11 = af12_vhs!(gpiod.pd11); // A16
    let mut _pd12 = af12_vhs!(gpiod.pd12); // A17
    let mut _pd13 = af12_vhs!(gpiod.pd13); // A18 (used as RS)

    // GPIOE: D4..D12 (PE7..PE15)
    let mut _pe7  = af12_vhs!(gpioe.pe7);
    let mut _pe8  = af12_vhs!(gpioe.pe8);
    let mut _pe9  = af12_vhs!(gpioe.pe9);
    let mut _pe10 = af12_vhs!(gpioe.pe10);
    let mut _pe11 = af12_vhs!(gpioe.pe11);
    let mut _pe12 = af12_vhs!(gpioe.pe12);
    let mut _pe13 = af12_vhs!(gpioe.pe13);
    let mut _pe14 = af12_vhs!(gpioe.pe14);
    let mut _pe15 = af12_vhs!(gpioe.pe15);

    // GPIOG: NE4 (PG12)
    let mut _pg12 = af12_vhs!(gpiog.pg12);
}

// Try enabling LCD backlight. Many STM32F429I-DISCO revisions route BL to PB1 (PWM-capable).
// We start simple: drive PB1 high as push-pull output.
#[cfg(feature = "hw-onboard-lcd")]
fn backlight_enable(gpiob_dev: hal::pac::GPIOB) {
    let gpiob = gpiob_dev.split();
    #[cfg(any(feature = "bl-pb1", not(feature = "bl-pb5")))]
    {
        let mut bl = gpiob.pb1.into_push_pull_output();
        bl.set_high();
    }
    #[cfg(feature = "bl-pb5")]
    {
        let mut bl = gpiob.pb5.into_push_pull_output();
        bl.set_high();
    }
}

#[cfg(feature = "hw-onboard-lcd")]
fn fmc_timings(dp: &mut hal::pac::Peripherals) {
    let fmc = &dp.FMC;
    unsafe {
        // Disable bank 4 while configuring
        fmc.bcr4.modify(|_, w| w.mbken().clear_bit());
        // BCR4: SRAM, 16-bit, write enable
        // MBKEN(0)=0, MUXEN(1)=0 (no address/data mux), MTYP(3:2)=0b00, MWID(5:4)=0b01 (16-bit), WREN(12)=1
        let bcr4 = (1 << 12) | (0b01 << 4);
        fmc.bcr4.write(|w| w.bits(bcr4));

        // Conservative timings to start: ADDSET=5 HCLKs, DATAST=12 HCLKs
        let btr4 = (5 & 0xF) | ((12 & 0xFF) << 8);
        fmc.btr4.write(|w| w.bits(btr4));

        // Enable bank 4
        fmc.bcr4.modify(|r, w| w.bits(r.bits() | 1));
    }
}

#[cfg(feature = "hw-onboard-lcd")]
#[entry]
fn main() -> ! {
    let mut dp = hal::pac::Peripherals::take().unwrap();

    // Enable FMC clock and set timings first
    fmc_enable(&mut dp);
    fmc_timings(&mut dp);
    // Move out GPIO peripherals for configuration (dp cannot be used after this)
    let gpiob_dev = dp.GPIOB;
    let gpiod_dev = dp.GPIOD;
    let gpioe_dev = dp.GPIOE;
    let gpiog_dev = dp.GPIOG;
    fmc_gpio_setup(gpiod_dev, gpioe_dev, gpiog_dev);
    // Enable backlight early so we can see output
    backlight_enable(gpiob_dev);

    // Initialize raw pointer to framebuffer
    unsafe { FB_PTR = core::ptr::addr_of_mut!(FRAMEBUFFER) as *mut u16; }

    // Clear the software framebuffer to blue
    unsafe {
        let color = Rgb565::from_rgb888(0, 0, 255).0;
        for i in 0..(WIDTH * HEIGHT) { core::ptr::write_volatile(FB_PTR.add(i), color); }
    }

    // Init controller and show framebuffer, then draw green square via PixelSink and flush again
    let mut lcd = Ili9341Fmc::new();
    lcd.init();
    lcd.flush_full();

    // Probe and visualize ILI9341 ID4 bytes as four horizontal stripes (grayscale)
    let id = lcd.read_id4();
    unsafe {
        let stripe_h = (HEIGHT / 4) as usize;
        for s in 0..4 {
            let v = id[s] as u16;
            let color = Rgb565::from_rgb888(v as u8, v as u8, v as u8).0;
            for y in (s * stripe_h)..((s + 1) * stripe_h).min(HEIGHT) {
                let row = y * WIDTH;
                for x in 0..WIDTH {
                    core::ptr::write_volatile(FB_PTR.add(row + x), color);
                }
            }
        }
    }
    lcd.flush_full();
    delay_cycles(12_000_000);

    // Diagnostic: cycle solid colors to verify bus writes
    unsafe {
        for &(r,g,b) in &[(255,0,0),(0,255,0),(0,0,255)] {
            let color = Rgb565::from_rgb888(r, g, b).0;
            for i in 0..(WIDTH * HEIGHT) { core::ptr::write_volatile(FB_PTR.add(i), color); }
            lcd.flush_full();
            delay_cycles(12_000_000);
        }
    }

    let mut fb = FbDisplay;
    let side: u16 = 120;
    let x = ((WIDTH as u16 - side) / 2) as i32;
    let y = ((HEIGHT as u16 - side) / 2) as i32;
    let square = Rect { x, y, width: side, height: side };
    let green = Rgb565::from_rgb888(0, 255, 0);
    let _ = draw_rectangle_outline(&mut fb, square, green, 6);
    lcd.flush_full();

    loop { cortex_m::asm::wfi(); }
}

#[cfg(not(feature = "hw-onboard-lcd"))]
fn main() { println!("Enable --features hw-onboard-lcd to run this example"); }
