use crate::config::*;
use core::convert::TryInto;
use core::ffi;
// Color constants not used in this module; remove to silence warnings.
use crate::display_ffi;

extern "C" {
    fn display_register_driver(driver: *const DisplayDriver);
    fn display_init();
    fn display_draw_image(x: u16, w: u16, y: u16, h: u16, img_data: *const u16);
    fn display_fill_screen(color: u16);
    fn display_fill_rect(x: u16, w: u16, y: u16, h: u16, color: u16);
    fn display_write_string(
        x: u16,
        y: u16,
        str_ptr: *const ffi::c_char,
        font: FontDef,
        color: u16,
        bgcolor: u16,
    );

    static Font_16x26: FontDef;

    fn display_game_title();
    fn ili9341_write_string(
        x: i32,
        y: i32,
        text: *const u8,
        font: FontDef,
        fg_color: u16,
        bg_color: u16,
    );
}

#[repr(C)]
pub struct DisplayDriver {
    __private: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FontDef {
    width: ffi::c_uchar,
    height: ffi::c_uchar,
    data: *const u16,
}

// Provide a const constructor so other modules can define the `Font_16x26` symbol.
pub const fn font_16x26() -> FontDef {
    FontDef { width: 16, height: 26, data: core::ptr::null() }
}

pub fn register_driver(driver: &DisplayDriver) {
    display_ffi::register_driver(driver);
}

pub fn init() {
    display_ffi::init();
}

pub fn draw_image(x: Coord, w: u32, y: Coord, h: u32, image_data: &[u16]) {
    let x: u16 = x.try_into().expect("X co-ordinate is out of range");
    let y: u16 = y.try_into().expect("y co-ordinate is out of range");
    let w: u16 = w.try_into().expect("width out of range");
    let h: u16 = h.try_into().expect("height out of range");
    display_ffi::draw_image_u16(x, w, y, h, image_data);
}

pub fn set_background_color(bg_color: u16) {
    display_ffi::fill_screen(bg_color);
}

pub fn draw_rect_angle(x: Coord, w: u32, y: Coord, h: u32, color: u16) {
    let x: u16 = x.try_into().expect("X co-ordinate is out of range");
    let y: u16 = y.try_into().expect("y co-ordinate is out of range");
    let w: u16 = w.try_into().expect("width out of range");
    let h: u16 = h.try_into().expect("height out of range");
    display_ffi::fill_rect(x, w, y, h, color);
}

pub fn write_string(x: Coord, y: Coord, c_str: &ffi::CStr, color: u16, bgcolor: u16) {
    let x: u16 = x.try_into().expect("X co-ordinate is out of range");
    let y: u16 = y.try_into().expect("y co-ordinate is out of range");
    display_ffi::write_string_u16(x, y, c_str, color, bgcolor);
}