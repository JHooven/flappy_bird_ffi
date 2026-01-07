use stm32f49I_display_lib::{
    init_display, DisplayConfig, Orientation, PixelFormat, Rgb565, Point, Triangle, draw_triangle_outline,
};

fn main() {
    let mut disp = init_display(DisplayConfig {
        width: 128,
        height: 128,
        orientation: Orientation::Portrait,
        pixel_format: PixelFormat::Rgb565,
    }).expect("init display");
    disp.clear(Rgb565::from_rgb888(20, 20, 20)).expect("clear");

    let tri = Triangle {
        a: Point { x: 20, y: 20 },
        b: Point { x: 100, y: 30 },
        c: Point { x: 64, y: 110 },
    };
    let cyan = Rgb565::from_rgb888(0, 200, 200);
    draw_triangle_outline(&mut disp, tri, cyan, 2).expect("draw triangle");

    // Save a quick visual for manual inspection
    disp.save_ppm("triangle.ppm").expect("save ppm");
}
