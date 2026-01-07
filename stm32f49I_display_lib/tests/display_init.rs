use stm32f49I_display_lib::{init_display, DisplayConfig, Orientation, PixelFormat, Rgb565};

#[test]
fn init_rejects_zero_dimensions() {
    let cfg = DisplayConfig { width: 0, height: 240, orientation: Orientation::Portrait, pixel_format: PixelFormat::Rgb565 };
    let err = init_display(cfg).unwrap_err();
    assert!(format!("{}", err).contains("invalid dimensions"));
}

#[test]
fn clear_fills_framebuffer() {
    let cfg = DisplayConfig { width: 4, height: 3, orientation: Orientation::Landscape, pixel_format: PixelFormat::Rgb565 };
    let mut disp = init_display(cfg).expect("init ok");
    let color = Rgb565::from_rgb888(0xAA, 0x55, 0x00);
    disp.clear(color).unwrap();

    let (w, h) = disp.size();
    for y in 0..h {
        for x in 0..w {
            assert_eq!(disp.get_pixel(x, y), Some(color));
        }
    }
}
