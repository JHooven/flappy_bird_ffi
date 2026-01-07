//! Manual visual check (ignored by default). Run with `cargo test --test visual_manual -- --ignored`.
use std::io::{self, Write};
use stm32f49I_display_lib::{
    draw_rectangle_outline, init_display, DisplayConfig, Orientation, PixelFormat, Rect, Rgb565,
};

#[test]
#[ignore]
fn rectangle_visual_check() {
    if std::env::var("RUN_VISUAL_TEST").ok().as_deref() != Some("1") {
        eprintln!("Skipping: set RUN_VISUAL_TEST=1 to run this interactive test.");
        return;
    }

    let cfg = DisplayConfig { width: 200, height: 150, orientation: Orientation::Landscape, pixel_format: PixelFormat::Rgb565 };
    let mut disp = init_display(cfg).expect("init");
    disp.clear(Rgb565::from_rgb888(8, 8, 8)).unwrap();
    let cyan = Rgb565::from_rgb888(0, 200, 200);
    draw_rectangle_outline(&mut disp, Rect { x: 20, y: 15, width: 160, height: 110 }, cyan, 4).unwrap();

    let mut path = std::env::temp_dir();
    path.push("visual_manual_rect.ppm");
    disp.save_ppm(&path).unwrap();
    println!("Saved image to {}", path.display());

    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&path).status();
    }
    println!("Does the image show a 4px cyan rectangle outline on a dark background? (y/N)");
    print!("Confirm [y/N]: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    assert!(input.trim().eq_ignore_ascii_case("y"), "User did not confirm visual output");
}
