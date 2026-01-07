use stm32f49I_display_lib::{
    draw_rectangle_outline, init_display, DisplayConfig, Orientation, PixelFormat, Rect, Rgb565,
};

fn mk_display(w: u16, h: u16) -> stm32f49I_display_lib::Display {
    let cfg = DisplayConfig { width: w, height: h, orientation: Orientation::Portrait, pixel_format: PixelFormat::Rgb565 };
    init_display(cfg).unwrap()
}

fn count_color(disp: &stm32f49I_display_lib::Display, color: Rgb565) -> usize {
    let (w, h) = disp.size();
    let mut n = 0usize;
    for y in 0..h { for x in 0..w { if disp.get_pixel(x, y) == Some(color) { n += 1; } } }
    n
}

#[test]
fn zero_size_is_noop() {
    let mut d = mk_display(10, 10);
    let red = Rgb565::from_rgb888(255, 0, 0);
    draw_rectangle_outline(&mut d, Rect { x: 2, y: 2, width: 0, height: 5 }, red, 2).unwrap();
    assert_eq!(count_color(&d, red), 0);
}

#[test]
fn thickness_zero_is_noop() {
    let mut d = mk_display(10, 10);
    let red = Rgb565::from_rgb888(255, 0, 0);
    draw_rectangle_outline(&mut d, Rect { x: 1, y: 1, width: 8, height: 8 }, red, 0).unwrap();
    assert_eq!(count_color(&d, red), 0);
}

#[test]
fn simple_outline_thickness_one() {
    let mut d = mk_display(8, 6);
    let red = Rgb565::from_rgb888(255, 0, 0);
    draw_rectangle_outline(&mut d, Rect { x: 1, y: 1, width: 6, height: 4 }, red, 1).unwrap();

    // Expected perimeter: 2*(w+h)-4 = 2*(6+4)-4 = 16
    assert_eq!(count_color(&d, red), 16);
}

#[test]
fn thick_outline_insets_and_overlaps() {
    let mut d = mk_display(10, 10);
    let blue = Rgb565::from_rgb888(0, 0, 255);
    draw_rectangle_outline(&mut d, Rect { x: 2, y: 2, width: 6, height: 6 }, blue, 3).unwrap();
    // With thickness >= side/2, it should fill the entire area: 6*6 = 36
    assert_eq!(count_color(&d, blue), 36);
}

#[test]
fn partially_offscreen_is_clipped() {
    let mut d = mk_display(10, 10);
    let green = Rgb565::from_rgb888(0, 255, 0);
    draw_rectangle_outline(&mut d, Rect { x: -5, y: 3, width: 8, height: 4 }, green, 2).unwrap();
    // It should draw only within x in [0,3], y in [3,6). Rough count check: > 0 and < full perimeter
    let n = count_color(&d, green);
    assert!(n > 0 && n < 2 * (8 + 4) as usize);
}

#[test]
fn fully_offscreen_draws_nothing() {
    let mut d = mk_display(10, 10);
    let c = Rgb565::from_rgb888(200, 200, 0);
    draw_rectangle_outline(&mut d, Rect { x: 15, y: 15, width: 3, height: 3 }, c, 1).unwrap();
    assert_eq!(count_color(&d, c), 0);
}
