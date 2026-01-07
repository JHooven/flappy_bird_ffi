use stm32f49I_display_lib::{
    draw_rectangle_outline, init_display, DisplayConfig, Orientation, PixelFormat, Rect, Rgb565,
};

fn checksum(disp: &stm32f49I_display_lib::Display) -> u64 {
    let (w, h) = disp.size();
    let mut acc: u64 = 0;
    for y in 0..h { for x in 0..w { acc = acc.wrapping_add(disp.get_pixel(x, y).unwrap().0 as u64); } }
    acc
}

fn main() -> anyhow::Result<()> {
    let cfg = DisplayConfig { width: 16, height: 12, orientation: Orientation::Portrait, pixel_format: PixelFormat::Rgb565 };
    let mut disp = init_display(cfg)?;
    disp.clear(Rgb565::BLACK)?;
    let red = Rgb565::from_rgb888(220, 20, 60);
    draw_rectangle_outline(&mut disp, Rect { x: 2, y: 2, width: 10, height: 8 }, red, 2)?;
    println!("checksum={} size={:?}", checksum(&disp), disp.size());
    Ok(())
}
