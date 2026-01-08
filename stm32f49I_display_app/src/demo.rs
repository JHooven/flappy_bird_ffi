use crate::hw::{Gc9a01Parallel, Rgb565};

pub fn f429_rect(panel: &mut Gc9a01Parallel, color: Rgb565) {
    let x0 = 40u16;
    let y0 = 60u16;
    let w = 160u16;
    let h = 80u16;

    for dx in 0..w {
        panel.set_pixel(x0 + dx, y0, color);
        panel.set_pixel(x0 + dx, y0 + h - 1, color);
    }

    for dy in 0..h {
        panel.set_pixel(x0, y0 + dy, color);
        panel.set_pixel(x0 + w - 1, y0 + dy, color);
    }
}
