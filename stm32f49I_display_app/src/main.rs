#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use core::mem::MaybeUninit;
// Avoid forming & references to MMIO; use volatile reads of SCB registers.

mod hw;
mod demo;
mod onboard_disc1;

#[entry]
fn main() -> ! {
    // STM32F429I-DISC1: onboard ILI9341 is on SPI5
    let _lcd = onboard_disc1::Ili9341Spi::init_and_take();
    // Fill background for visibility, then draw a red rectangle border
    onboard_disc1::Ili9341Spi::fill_rect(0, 0, 240, 320, hw::Rgb565::WHITE.0);
    onboard_disc1::draw_rect_outline(10, 10, 220, 300, hw::Rgb565::RED.0);

    loop {
        cortex_m::asm::bkpt();
    }
}

#[allow(non_snake_case)]
#[cortex_m_rt::exception]
unsafe fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    record_fault(_ef);
    cortex_m::asm::bkpt();
    loop {}
}

#[allow(non_snake_case)]
#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_irqn: i16) {
    cortex_m::asm::bkpt();
}

#[cortex_m_rt::exception]
fn SysTick() {
    cortex_m::asm::bkpt();
}

#[repr(C)]
pub struct FaultInfo {
    pub cfsr: u32,
    pub hfsr: u32,
    pub mmfar: u32,
    pub bfar: u32,
    pub lr: u32,
    pub pc: u32,
    pub xpsr: u32,
}

#[unsafe(no_mangle)]
static mut FAULT_INFO: MaybeUninit<FaultInfo> = MaybeUninit::uninit();

#[inline(never)]
fn record_fault(ef: &cortex_m_rt::ExceptionFrame) {
    // SAFETY: HardFault context
    let cfsr = unsafe { core::ptr::read_volatile(0xE000_ED28 as *const u32) };
    let hfsr = unsafe { core::ptr::read_volatile(0xE000_ED2C as *const u32) };
    let mmfar = unsafe { core::ptr::read_volatile(0xE000_ED38 as *const u32) };
    let bfar = unsafe { core::ptr::read_volatile(0xE000_ED3C as *const u32) };
    let info = FaultInfo {
        cfsr,
        hfsr,
        mmfar,
        bfar,
        lr: ef.lr(),
        pc: ef.pc(),
        xpsr: ef.xpsr(),
    };
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(FAULT_INFO), MaybeUninit::new(info)); }
}
