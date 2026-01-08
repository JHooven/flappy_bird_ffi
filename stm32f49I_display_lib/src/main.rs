#[cfg(feature = "std")]
fn main() {
    use stm32f49I_display_lib as lib;

    // Initialize a host display (framebuffer) and draw a centered square outline.
    let mut disp = lib::init_display(lib::DisplayConfig {
        width: 320,
        height: 240,
        orientation: lib::Orientation::Landscape,
        pixel_format: lib::PixelFormat::Rgb565,
    }).expect("init_display failed");

    let _ = disp.clear(lib::Rgb565::BLACK);

    let (w, h) = disp.size();
    let side: u16 = 120;
    let x = ((w - side) / 2) as i32;
    let y = ((h - side) / 2) as i32;
    let square = lib::Rect { x, y, width: side, height: side };
    let color = lib::Rgb565::from_rgb888(0, 255, 0);
    let _ = lib::draw_rectangle_outline(&mut disp, square, color, 6);

    disp.save_ppm("square.ppm").expect("write square.ppm");
    println!("square.ppm written ({w}x{h}), centered {side}x{side} square");
}

