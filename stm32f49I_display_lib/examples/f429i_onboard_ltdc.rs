#![cfg_attr(feature = "hw-onboard-lcd", no_std)]
#![cfg_attr(feature = "hw-onboard-lcd", no_main)]

#[cfg(feature = "hw-onboard-lcd")]
use panic_halt as _;

#[cfg(feature = "hw-onboard-lcd")]
use cortex_m_rt::entry;

#[cfg(feature = "hw-onboard-lcd")]
use stm32f4xx_hal as hal;

#[cfg(feature = "hw-onboard-lcd")]
use hal::prelude::*;

#[cfg(feature = "hw-onboard-lcd")]
use stm32f49I_display_lib::{draw_rectangle_outline, Rect, Rgb565};

#[cfg(feature = "hw-onboard-lcd")]
use stm32f49I_display_lib::shapes::PixelSink;

#[cfg(feature = "hw-onboard-lcd")]
const WIDTH: usize = 320;
#[cfg(feature = "hw-onboard-lcd")]
const HEIGHT: usize = 240;

#[cfg(feature = "hw-onboard-lcd")]
#[unsafe(link_section = ".bss.fb")]
static mut FRAMEBUFFER: [u16; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[cfg(feature = "hw-onboard-lcd")]
struct FbDisplay;

#[cfg(feature = "hw-onboard-lcd")]
impl PixelSink for FbDisplay {
    fn size(&self) -> (u16, u16) { (WIDTH as u16, HEIGHT as u16) }
    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) {
        let xi = x as usize;
        let yi = y as usize;
        if xi < WIDTH && yi < HEIGHT {
            let idx = yi * WIDTH + xi;
            unsafe { FRAMEBUFFER[idx] = color.0; }
        }
    }
}

#[cfg(feature = "hw-onboard-lcd")]
fn ltdc_init(dp: &mut hal::pac::Peripherals) {
    use hal::pac::{RCC, LTDC};
    let rcc: &RCC = &dp.RCC;
    let ltdc: &LTDC = &dp.LTDC;

    // 1) Enable HSE/PLL system clock as usual
    // Done by HAL clock freeze below (entry)

    // 2) Configure PLLSAI for LTDC pixel clock (example values; may require tuning)
    // PLLSAI VCO = HSE * (PLLSAIN / PLLM)
    // Pixel clock ≈ VCO / PLLSAIR / PLLSAIDivR
    // Example from common configs: N=192, M=8, R=4, DivR=8 → ~6 MHz @ 8 MHz HSE
    unsafe {
        // Enable power interface clock if needed for voltage scaling
        // rcc.apb1enr.modify(|_, w| w.pwren().set_bit());

        // Set PLLSAI factors
        rcc.pllsaicfgr.write(|w| {
            w.pllsain().bits(192)
             .pllsair().bits(4)
        });
        // Set DivR = 8
        rcc.dckcfgr.modify(|_, w| w.pllsaidivr().bits(0b11));
        // Enable PLLSAI
        rcc.cr.modify(|_, w| w.pllsaion().set_bit());
        while rcc.cr.read().pllsairdy().bit_is_clear() {}
    }

    // 3) Enable LTDC clock
    unsafe {
        rcc.apb2enr.modify(|_, w| w.ltdcen().set_bit());
    }

    // 4) Configure GPIOs to LTDC AF (AF14) for required RGB pins, HSYNC, VSYNC, DE, CLK
    // NOTE: This is board-specific and must match the STM32F429I-DISCO schematics.
    // TODO: Configure all required pins to AF14 with suitable speeds.

    // 5) Configure LTDC timings for 320x240 (example values)
    let hsync: u16 = 10;
    let hbp: u16 = 20;
    let hfp: u16 = 10;
    let vsync: u16 = 2;
    let vbp: u16 = 2;
    let vfp: u16 = 4;
    let width: u16 = WIDTH as u16;
    let height: u16 = HEIGHT as u16;

    unsafe {
        // Synchronization size configuration
        ltdc.sscr.write(|w| w.hsw().bits(hsync - 1).vsh().bits(vsync - 1));
        // Back porch configuration (accumulated)
        ltdc.bpcr.write(|w| {
            w.ahbp().bits(hsync + hbp - 1)
             .avbp().bits(vsync + vbp - 1)
        });
        // Active width and height (accumulated)
        ltdc.awcr.write(|w| {
            w.aaw().bits(hsync + hbp + width - 1)
             .aah().bits(vsync + vbp + height - 1)
        });
        // Total width/height (including front porch)
        ltdc.twcr.write(|w| {
            w.totalw().bits(hsync + hbp + width + hfp - 1)
             .totalh().bits(vsync + vbp + height + vfp - 1)
        });
        // Background color (black) – writes to BCCR register
        // Some PACs name fields differently; set full register to 0.
        ltdc.bccr.write(|w| unsafe { w.bits(0) });

        // Layer 1 config
        // Windowing
        let hstart = (hsync + hbp) as u16 + 1;
        let hend = hstart + width - 1;
        let vstart = (vsync + vbp) as u16 + 1;
        let vend = vstart + height - 1;
        let l1 = &ltdc.layer1;
        l1.whpcr.write(|w| w.whstpos().bits(hstart).whsppos().bits(hend));
        l1.wvpcr.write(|w| w.wvstpos().bits(vstart).wvsppos().bits(vend));

        // Pixel format RGB565 (PF=2)
        l1.pfcr.write(|w| w.pf().bits(2));

        // Default constant alpha and blending factors
        l1.cacr.write(|w| w.consta().bits(255));
        l1.bfcr.write(|w| w.bf1().bits(0x06).bf2().bits(0x07));

        // Line length and pitch
        let pitch_bytes: u16 = (WIDTH as u16) * 2;
        let line_length: u16 = pitch_bytes + 3; // per RM0090
        l1.cfblr.write(|w| w.cfbll().bits(line_length).cfbp().bits(pitch_bytes));
        l1.cfblnr.write(|w| w.cfblnbr().bits(height));

        // Framebuffer address
        let fb_addr = core::ptr::addr_of_mut!(FRAMEBUFFER) as *mut u16 as u32;
        l1.cfbar.write(|w| w.cfbadd().bits(fb_addr));

        // Enable layer 1
        l1.cr.modify(|_, w| w.len().set_bit());

        // Immediate reload of shadow regs
        ltdc.srcr.write(|w| w.imr().set_bit());

        // Enable LTDC
        ltdc.gcr.modify(|_, w| w.ltdcen().set_bit());
    }
}

#[cfg(feature = "hw-onboard-lcd")]
#[entry]
fn main() -> ! {
    let mut dp = hal::pac::Peripherals::take().unwrap();

    // Initialize LTDC to point at our framebuffer
    ltdc_init(&mut dp);

    // Clear framebuffer to blue
    unsafe {
        let base = core::ptr::addr_of_mut!(FRAMEBUFFER) as *mut u16;
        let color = Rgb565::from_rgb888(0, 0, 255).0;
        for i in 0..(WIDTH * HEIGHT) {
            core::ptr::write_volatile(base.add(i), color);
        }
    }

    // Draw a centered green square outline to exercise thickness + color
    let mut fb = FbDisplay;
    let side: u16 = 120;
    let x = ((WIDTH as u16 - side) / 2) as i32;
    let y = ((HEIGHT as u16 - side) / 2) as i32;
    let square = Rect { x, y, width: side, height: side };
    let green = Rgb565::from_rgb888(0, 255, 0);
    let _ = draw_rectangle_outline(&mut fb, square, green, 6);

    loop { cortex_m::asm::wfi(); }
}

#[cfg(not(feature = "hw-onboard-lcd"))]
fn main() {
    println!("Feature 'hw-onboard-lcd' not enabled. Re-run with --features hw-onboard-lcd.");
}
