use stm32f49I_display_lib::{
    draw_triangle_outline, init_display, DisplayConfig, Orientation, PixelFormat, Point, Rgb565, Triangle,
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
fn thickness_zero_is_noop() {
    let mut d = mk_display(20, 20);
    let red = Rgb565::from_rgb888(255, 0, 0);
    let tri = Triangle { a: Point { x: 2, y: 2 }, b: Point { x: 10, y: 2 }, c: Point { x: 6, y: 12 } };
    draw_triangle_outline(&mut d, tri, red, 0).unwrap();
    assert_eq!(count_color(&d, red), 0);
}

#[test]
fn simple_triangle_thickness_one_draws_edges() {
    let mut d = mk_display(16, 16);
    let c = Rgb565::from_rgb888(0, 200, 200);
    let tri = Triangle { a: Point { x: 2, y: 2 }, b: Point { x: 12, y: 3 }, c: Point { x: 6, y: 12 } };
    draw_triangle_outline(&mut d, tri, c, 1).unwrap();
    // Expect at least the vertices to be painted and some additional edge pixels.
    assert!(count_color(&d, c) >= 3);
    assert_eq!(d.get_pixel(2, 2), Some(c));
    assert_eq!(d.get_pixel(12, 3), Some(c));
    assert_eq!(d.get_pixel(6, 12), Some(c));
}

#[test]
fn partially_offscreen_is_clipped() {
    let mut d = mk_display(10, 10);
    let c = Rgb565::from_rgb888(100, 100, 255);
    let tri = Triangle { a: Point { x: -3, y: 1 }, b: Point { x: 8, y: -2 }, c: Point { x: 6, y: 8 } };
    draw_triangle_outline(&mut d, tri, c, 2).unwrap();
    let n = count_color(&d, c);
    assert!(n > 0 && n < 60);
}

#[test]
fn fully_offscreen_draws_nothing() {
    let mut d = mk_display(10, 10);
    let c = Rgb565::from_rgb888(200, 0, 200);
    let tri = Triangle { a: Point { x: 20, y: 20 }, b: Point { x: 25, y: 25 }, c: Point { x: 22, y: 28 } };
    draw_triangle_outline(&mut d, tri, c, 1).unwrap();
    assert_eq!(count_color(&d, c), 0);
}

#[test]
fn degenerate_same_points_is_noop() {
    let mut d = mk_display(16, 16);
    let c = Rgb565::from_rgb888(255, 50, 50);
    let p = Point { x: 8, y: 8 };
    let tri = Triangle { a: p, b: p, c: p };
    draw_triangle_outline(&mut d, tri, c, 3).unwrap();
    assert_eq!(count_color(&d, c), 0);
}

#[test]
fn colinear_points_draw_overlapping_line() {
    let mut d = mk_display(20, 20);
    let c = Rgb565::from_rgb888(10, 200, 10);
    let tri = Triangle { a: Point { x: 2, y: 2 }, b: Point { x: 10, y: 10 }, c: Point { x: 18, y: 18 } };
    draw_triangle_outline(&mut d, tri, c, 2).unwrap();
    let n = count_color(&d, c);
    assert!(n > 0);
}

#[test]
fn very_thick_triangle_is_clipped() {
    let mut d = mk_display(15, 10);
    let c = Rgb565::from_rgb888(0, 0, 255);
    let tri = Triangle { a: Point { x: 5, y: 1 }, b: Point { x: 14, y: 5 }, c: Point { x: 1, y: 8 } };
    draw_triangle_outline(&mut d, tri, c, 50).unwrap();
    let (w, h) = d.size();
    let total = (w as usize) * (h as usize);
    let n = count_color(&d, c);
    assert!(n > 0 && n <= total);
}
