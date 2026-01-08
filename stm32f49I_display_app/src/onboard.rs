#![allow(dead_code)]

use stm32f4xx_hal as hal;
use hal::pac;
use hal::prelude::*;
use hal::gpio::Speed;

// Selected working addresses (initialized to common default)
static mut CMD_ADDR: *mut u16 = 0x6000_0000 as *mut u16;
static mut DAT_ADDR: *mut u16 = 0x6002_0000 as *mut u16;

pub struct Ili9341Fmc;

impl Ili9341Fmc {
    // Candidate command/data address pairs across NE1/NE4 and A16/A17/A18 RS lines.
    // For 16-bit bus, address line A16 maps to offset 0x20000, A17 to 0x40000, A18 to 0x80000.
    const ADDR_PAIRS: &[(u32, u32)] = &[
        (0x6000_0000, 0x6002_0000),
        (0x6000_0000, 0x6004_0000),
        (0x6000_0000, 0x6008_0000),
        (0x6C00_0000, 0x6C02_0000),
        (0x6C00_0000, 0x6C04_0000),
        (0x6C00_0000, 0x6C08_0000),
    ];


    #[inline(always)]
    fn wr_cmd(cmd: u8) { unsafe { core::ptr::write_volatile(CMD_ADDR, cmd as u16) } }

    #[inline(always)]
    fn wr_data8(data: u8) { unsafe { core::ptr::write_volatile(DAT_ADDR, data as u16) } }

    #[inline(always)]
    fn wr_data16(data: u16) { unsafe { core::ptr::write_volatile(DAT_ADDR, data) } }

    #[inline(always)]
    fn rd_data8() -> u8 { unsafe { core::ptr::read_volatile(DAT_ADDR) as u8 } }

    fn delay_cycles(n: u32) {
        for _ in 0..n { unsafe { cortex_m::asm::nop() } }
    }

    pub fn init_and_take() -> Self {
        let dp = pac::Peripherals::take().unwrap();

        // Use HAL RCC to set a known HCLK; keep it modest to ease FMC timings.
        let rcc = dp.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(84.MHz()).hclk(84.MHz()).freeze();

        // Enable required GPIO clocks and FMC clock via PAC to ensure FMCEN is set.
        let rcc_pac = unsafe { &*pac::RCC::ptr() };
        // GPIOD/E/F/G clocks (common FMC pins live here); some boards also need GPIOI/J/K for other peripherals.
        rcc_pac.ahb1enr.modify(|_, w| w
            .gpioden().enabled()
            .gpioeen().enabled()
            .gpiofen().enabled()
            .gpiogen().enabled()
        );
        // FMC clock on AHB3
        rcc_pac.ahb3enr.modify(|_, w| w.fmcen().enabled());

        // Configure FMC alternate functions on the pins used by Bank1 16-bit NOR/SRAM (8080-like):
        // This mapping follows the common F4 FMC layout (AF12). It matches many F429 examples.
        // D0..D3: PD14, PD15, PD0, PD1
        // D4..D12: PE7..PE15
        // D13..D15: PD8, PD9, PD10
        // NOE (RD): PD4, NWE (WR): PD5, NE1: PD7, A16 (RS): PD11
        let gpiod = dp.GPIOD.split();
        let gpioe = dp.GPIOE.split();
        let gpiog = dp.GPIOG.split();
        let gpiob = dp.GPIOB.split();

        // Data lines
        let mut _pd0 = gpiod.pd0.into_alternate::<12>(); _pd0.set_speed(Speed::VeryHigh);
        let mut _pd1 = gpiod.pd1.into_alternate::<12>(); _pd1.set_speed(Speed::VeryHigh);
        let mut _pd8 = gpiod.pd8.into_alternate::<12>(); _pd8.set_speed(Speed::VeryHigh);
        let mut _pd9 = gpiod.pd9.into_alternate::<12>(); _pd9.set_speed(Speed::VeryHigh);
        let mut _pd10 = gpiod.pd10.into_alternate::<12>(); _pd10.set_speed(Speed::VeryHigh);
        let mut _pd14 = gpiod.pd14.into_alternate::<12>(); _pd14.set_speed(Speed::VeryHigh);
        let mut _pd15 = gpiod.pd15.into_alternate::<12>(); _pd15.set_speed(Speed::VeryHigh);

        let mut _pe7 = gpioe.pe7.into_alternate::<12>(); _pe7.set_speed(Speed::VeryHigh);
        let mut _pe8 = gpioe.pe8.into_alternate::<12>(); _pe8.set_speed(Speed::VeryHigh);
        let mut _pe9 = gpioe.pe9.into_alternate::<12>(); _pe9.set_speed(Speed::VeryHigh);
        let mut _pe10 = gpioe.pe10.into_alternate::<12>(); _pe10.set_speed(Speed::VeryHigh);
        let mut _pe11 = gpioe.pe11.into_alternate::<12>(); _pe11.set_speed(Speed::VeryHigh);
        let mut _pe12 = gpioe.pe12.into_alternate::<12>(); _pe12.set_speed(Speed::VeryHigh);
        let mut _pe13 = gpioe.pe13.into_alternate::<12>(); _pe13.set_speed(Speed::VeryHigh);
        let mut _pe14 = gpioe.pe14.into_alternate::<12>(); _pe14.set_speed(Speed::VeryHigh);
        let mut _pe15 = gpioe.pe15.into_alternate::<12>(); _pe15.set_speed(Speed::VeryHigh);

        // Control signals
        let mut _pd4 = gpiod.pd4.into_alternate::<12>(); _pd4.set_speed(Speed::VeryHigh); // NOE (RD)
        let mut _pd5 = gpiod.pd5.into_alternate::<12>(); _pd5.set_speed(Speed::VeryHigh); // NWE (WR)
        let mut _pd7 = gpiod.pd7.into_alternate::<12>(); _pd7.set_speed(Speed::VeryHigh); // NE1
        let mut _pg12 = gpiog.pg12.into_alternate::<12>(); _pg12.set_speed(Speed::VeryHigh); // NE4
        let mut _pd11 = gpiod.pd11.into_alternate::<12>(); _pd11.set_speed(Speed::VeryHigh); // A16 used as RS (cmd/data)

        // Try enabling backlight via common F429I-DISCO pins
        let mut _pb1_bl = gpiob.pb1.into_push_pull_output();
        let _ = _pb1_bl.set_high();
        let mut _pd13_bl = gpiod.pd13.into_push_pull_output();
        let _ = _pd13_bl.set_high();

        // Configure FMC Bank1 NOR/SRAM region timing (BCR1/BTR1) for 16-bit, read/write enabled.
        let fmc = unsafe { &*pac::FMC::ptr() };

        // Disable bank while configuring.
        fmc.bcr1.modify(|_, w| w.mbken().disabled());

        // BCR1: SRAM, 16-bit, write enabled
        fmc.bcr1.modify(|_, w| {
            w.muxen().disabled(); // no address/data multiplex
            w.mtyp().sram();
            unsafe { w.mwid().bits(0b01) }; // 16-bit
            w.bursten().disabled();
            w.waitpol().clear_bit();
            w.wrapmod().clear_bit();
            w.waitcfg().clear_bit();
            w.wren().set_bit();
            w.waiten().clear_bit();
            w.extmod().clear_bit();
            w.asyncwait().clear_bit();
            w
        });

        // BTR1: conservative timings at 84MHz HCLK
        // ADDSET=2, DATAST=9 (~> 11 HCLK cycles address setup + 10+11 data cycles total)
        fmc.btr1.modify(|_, w| unsafe {
            w.addset().bits(2);
            w.datast().bits(9);
            w.busturn().bits(1);
            w.accmod().bits(0); // Mode A
            w
        });

        // Enable NE1 bank
        fmc.bcr1.modify(|_, w| w.mbken().enabled());

        // Configure NE4 bank (BCR4/BTR4) similarly
        fmc.bcr4.modify(|_, w| {
            w.muxen().disabled();
            w.mtyp().sram();
            unsafe { w.mwid().bits(0b01) }; // 16-bit
            w.bursten().disabled();
            w.wren().set_bit();
            w.extmod().clear_bit();
            w
        });
        fmc.btr4.modify(|_, w| unsafe {
            w.addset().bits(2);
            w.datast().bits(9);
            w.busturn().bits(1);
            w.accmod().bits(0);
            w
        });
        fmc.bcr4.modify(|_, w| w.mbken().enabled());

        // Probe address pairs by reading ILI9341 ID (0xD3: returns 00h, 93h, 41h)
        let mut found = false;
        for (cmd, dat) in Self::ADDR_PAIRS {
            unsafe { CMD_ADDR = (*cmd) as *mut u16; DAT_ADDR = (*dat) as *mut u16; }
            // Soft reset and sleep out for this mapping
            Self::wr_cmd(0x01);
            Self::delay_cycles(2_000_000);
            Self::wr_cmd(0x11);
            Self::delay_cycles(2_000_000);

            // Read ID
            Self::wr_cmd(0xD3);
            let _dummy = Self::rd_data8();
            let id1 = Self::rd_data8();
            let id2 = Self::rd_data8();
            let id3 = Self::rd_data8();
            if id1 == 0x00 && id2 == 0x93 && id3 == 0x41 {
                found = true;
                break;
            }
        }

        if !found {
            // If no match, leave defaults and trigger a breakpoint for inspection
            cortex_m::asm::bkpt();
        }

        // Basic ILI9341 init (software reset, RGB565, memory access control, display on)
        Self::wr_cmd(0x01); // Software reset
        Self::delay_cycles(8_000_000);
        Self::wr_cmd(0x11); // Sleep out
        Self::delay_cycles(8_000_000);

        Self::wr_cmd(0x3A); // Pixel format
        Self::wr_data8(0x55); // 16-bit

        Self::wr_cmd(0x36); // MADCTL
        // MX=0, MY=1 (top-left origin), MV=0; BGR=1
        Self::wr_data8(0x48);

        // Set full address window 240x320 (portrait) default of onboard panel
        Self::wr_cmd(0x2A); // CASET
        Self::wr_data16(0);
        Self::wr_data16(239);
        Self::wr_cmd(0x2B); // PASET
        Self::wr_data16(0);
        Self::wr_data16(319);

        Self::wr_cmd(0x29); // Display on
        Self::delay_cycles(100_000);

        Ili9341Fmc
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
        for _ in 0..count { Self::wr_data16(color_565); }
    }
}

pub fn draw_rect_outline(x: u16, y: u16, w: u16, h: u16, color_565: u16) {
    let x2 = x.saturating_add(w.saturating_sub(1));
    let y2 = y.saturating_add(h.saturating_sub(1));

    // Top and bottom
    Ili9341Fmc::set_window(x, y, x2, y);
    for _ in x..=x2 { Ili9341Fmc::wr_data16(color_565); }
    Ili9341Fmc::set_window(x, y2, x2, y2);
    for _ in x..=x2 { Ili9341Fmc::wr_data16(color_565); }

    // Left and right
    for yy in y..=y2 {
        Ili9341Fmc::set_window(x, yy, x, yy);
        Ili9341Fmc::wr_data16(color_565);
        Ili9341Fmc::set_window(x2, yy, x2, yy);
        Ili9341Fmc::wr_data16(color_565);
    }
}
