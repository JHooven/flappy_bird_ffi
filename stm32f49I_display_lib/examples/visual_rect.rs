use std::io::{self, Write};
use stm32f49I_display_lib::{
    draw_rectangle_outline, init_display, DisplayConfig, Orientation, PixelFormat, Rect, Rgb565,
};

fn main() {
    let cfg = DisplayConfig { width: 160, height: 120, orientation: Orientation::Landscape, pixel_format: PixelFormat::Rgb565 };
    let mut disp = init_display(cfg).expect("init display");
    disp.clear(Rgb565::from_rgb888(16, 16, 16)).expect("clear");

    let red = Rgb565::from_rgb888(220, 20, 60);
    draw_rectangle_outline(&mut disp, Rect { x: 10, y: 8, width: 120, height: 80 }, red, 3).expect("draw");

    let mut path = std::env::temp_dir();
    path.push("visual_rect.ppm");
    disp.save_ppm(&path).expect("save ppm");

    println!("Saved image to {}", path.display());
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&path).status();
    }

    println!("Please confirm the image shows a 3px red rectangle outline on a dark background. (y/N)");
    print!("Confirm [y/N]: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read input");
        std::process::exit(1);
    }
    if input.trim().eq_ignore_ascii_case("y") {
        println!("✅ Confirmed by user.");
        return;
    } else {
        eprintln!("User did not confirm visual output");
        std::process::exit(1);
    }
}
