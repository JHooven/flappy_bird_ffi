use stm32f49I_display_lib::{init_display, DisplayConfig, Orientation, PixelFormat, Rgb565};

fn main() {
    let cfg = DisplayConfig {
        width: 320,
        height: 240,
        orientation: Orientation::Landscape,
        pixel_format: PixelFormat::Rgb565,
    };
    let mut display = init_display(cfg).expect("init display");
    display.clear(Rgb565::from_rgb888(0x00, 0x20, 0x40)).expect("clear");
    println!("Initialized {}x{} display and cleared to color.", display.size().0, display.size().1);
}
