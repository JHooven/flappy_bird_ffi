use std::io::{self, Write};
use std::path::PathBuf;
use stm32f49I_display_lib::{
    draw_rectangle_outline, init_display, DisplayConfig, Orientation, PixelFormat, Rect, Rgb565,
};

fn main() -> anyhow::Result<()> {
    let cfg = DisplayConfig { width: 160, height: 120, orientation: Orientation::Landscape, pixel_format: PixelFormat::Rgb565 };
    let mut disp = init_display(cfg)?;
    disp.clear(Rgb565::from_rgb888(16, 16, 16))?;

    let red = Rgb565::from_rgb888(220, 20, 60);
    draw_rectangle_outline(&mut disp, Rect { x: 10, y: 8, width: 120, height: 80 }, red, 3)?;

    let mut path = std::env::temp_dir();
    path.push("visual_rect.ppm");
    disp.save_ppm(&path)?;

    println!("Saved image to {}", path.display());
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&path).status();
    }

    println!("Please confirm the image shows a 3px red rectangle outline on a dark background. (y/N)");
    print!("Confirm [y/N]: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim().eq_ignore_ascii_case("y") {
        println!("✅ Confirmed by user.");
        Ok(())
    } else {
        anyhow::bail!("User did not confirm visual output");
    }
}
