use core::ffi;
use crate::display::{DisplayDriver, FontDef};
use rtt_target::rprintln;
// Allow using FontDef in a static; raw pointers are assumed valid for FFI.
unsafe impl Sync for FontDef {}

// Provide Rust-defined extern "C" stubs so linking succeeds without C.
#[no_mangle]
pub extern "C" fn display_register_driver(driver: *const DisplayDriver) {
    rprintln!("display_register_driver: {:p}", driver);
}

#[no_mangle]
pub extern "C" fn display_init() {}

#[no_mangle]
pub extern "C" fn display_draw_image(x: u16, w: u16, y: u16, h: u16, img_data: *const u16) {
    rprintln!("display_draw_image: x={}, y={}, w={}, h={}, data={:p}", x, y, w, h, img_data);
}

#[no_mangle]
pub extern "C" fn display_fill_screen(color: u16) {
    rprintln!("display_fill_screen: color=0x{:04X}", color);
}

#[no_mangle]
pub extern "C" fn display_fill_rect(x: u16, w: u16, y: u16, h: u16, color: u16) {
    rprintln!("display_fill_rect: x={}, y={}, w={}, h={}, color=0x{:04X}", x, y, w, h, color);
}

#[no_mangle]
pub extern "C" fn display_write_string(
    x: u16,
    y: u16,
    str_ptr: *const ffi::c_char,
    font: FontDef,
    color: u16,
    bgcolor: u16,
) {
    rprintln!("display_write_string: x={}, y={}, color=0x{:04X}, bg=0x{:04X}", x, y, color, bgcolor);
}


// Safe wrappers used by display.rs
pub fn register_driver(driver: &DisplayDriver) {
    display_register_driver(driver as *const DisplayDriver);
}

pub fn init() {
    display_init();
}

pub fn draw_image_u16(x: u16, w: u16, y: u16, h: u16, image_data: &[u16]) {
    display_draw_image(x, w, y, h, image_data.as_ptr());
}

pub fn fill_screen(color: u16) {
    display_fill_screen(color);
}

pub fn fill_rect(x: u16, w: u16, y: u16, h: u16, color: u16) {
    display_fill_rect(x, w, y, h, color);
}

pub fn write_string_u16(x: u16, y: u16, c_str: &ffi::CStr, color: u16, bgcolor: u16) {
    extern "C" {
        static Font_16x26: crate::display::FontDef;
    }
    // Use our Rust-defined symbol to satisfy the extern reference.
    display_write_string(x, y, c_str.as_ptr(), unsafe { Font_16x26 }, color, bgcolor);
}

// Define the `Font_16x26` symbol using the constructor in display.rs.
#[no_mangle]
pub static Font_16x26: crate::display::FontDef = crate::display::font_16x26();
