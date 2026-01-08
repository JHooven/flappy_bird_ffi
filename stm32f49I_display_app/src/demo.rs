use crate::hw::{Gc9a01Parallel, Rgb565};

pub fn f429_rect(panel: &mut Gc9a01Parallel) {
    let x0 = 40u16;
    let y0 = 60u16;
    let w = 160u16;
    let h = 80u16;

    let white = Rgb565::from_rgb888(255, 255, 255);

    for dx in 0..w {
        panel.set_pixel(x0 + dx, y0, white);
        panel.set_pixel(x0 + dx, y0 + h - 1, white);
    }

    for dy in 0..h {
        panel.set_pixel(x0, y0 + dy, white);
        panel.set_pixel(x0 + w - 1, y0 + dy, white);
    }
}
