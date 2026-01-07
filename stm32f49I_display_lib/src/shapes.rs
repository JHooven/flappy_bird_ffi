use crate::display::Rgb565;

pub trait PixelSink {
    fn size(&self) -> (u16, u16);
    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565);
}

#[cfg(feature = "std")]
impl PixelSink for crate::display::Display {
    fn size(&self) -> (u16, u16) { self.size() }
    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) { self.set_pixel_unchecked(x, y, color) }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

/// Draw a rectangle outline inset by `thickness` pixels.
/// - If `width == 0` or `height == 0`, this is a no-op.
/// - If `thickness == 0`, this is a no-op.
/// - Drawing is clipped to the display bounds.
pub fn draw_rectangle_outline<S: PixelSink>(
    disp: &mut S,
    rect: Rect,
    color: Rgb565,
    thickness: u16,
) -> Result<(), crate::display::Error> {
    if rect.width == 0 || rect.height == 0 || thickness == 0 {
        return Ok(());
    }

    let (dw, dh) = disp.size();
    let dw = dw as i32;
    let dh = dh as i32;

    // Compute inner inset bounds (inset by thickness on all sides)
    let t = thickness as i32;
    let x0 = rect.x.max(0);
    let y0 = rect.y.max(0);
    let x1 = (rect.x + rect.width as i32).min(dw);
    let y1 = (rect.y + rect.height as i32).min(dh);

    if x0 >= x1 || y0 >= y1 {
        return Ok(()); // fully off-screen
    }

    // Top edge: y in [rect.y, rect.y + t)
    fill_span_rect(disp, rect.x, rect.y, rect.width, thickness, color);
    // Bottom edge
    if rect.height > thickness {
        fill_span_rect(
            disp,
            rect.x,
            rect.y + rect.height as i32 - t,
            rect.width,
            thickness,
            color,
        );
    } else {
        // If thickness >= height, top edge already covers whole area
    }

    // Left edge
    fill_span_rect(disp, rect.x, rect.y + t, thickness, rect.height.saturating_sub(thickness * 2), color);
    // Right edge
    if rect.width > thickness {
        fill_span_rect(
            disp,
            rect.x + rect.width as i32 - t,
            rect.y + t,
            thickness,
            rect.height.saturating_sub(thickness * 2),
            color,
        );
    }
    Ok(())
}

fn fill_span_rect<S: PixelSink>(
    disp: &mut S,
    x: i32,
    y: i32,
    w: u16,
    h: u16,
    color: Rgb565,
) {
    if w == 0 || h == 0 { return; }
    let (dw, dh) = disp.size();
    let dw = dw as i32;
    let dh = dh as i32;

    let x0 = x.max(0);
    let y0 = y.max(0);
    let x1 = (x + w as i32).min(dw);
    let y1 = (y + h as i32).min(dh);

    if x0 >= x1 || y0 >= y1 { return; }

    for yy in y0..y1 { for xx in x0..x1 { disp.set_pixel(xx as u16, yy as u16, color); } }
}

/// Draw a line between two points using integer Bresenham, clipped to display bounds.
fn draw_line<S: PixelSink>(disp: &mut S, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: Rgb565, thickness: u16) {
    if thickness == 0 { return; }
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        // Plot with thickness by drawing a small rectangle centered at (x0, y0)
        let t = thickness as i32;
        let half = t / 2;
        let bx = x0 - half;
        let by = y0 - half;
        let bw = (t.max(1)) as u16;
        let bh = (t.max(1)) as u16;
        fill_span_rect(disp, bx, by, bw, bh, color);

        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

/// Draw a triangle outline (three edges) using Bresenham lines.
/// - If all three points collapse to a single point, this is a no-op.
/// - `thickness == 0` is a no-op.
pub fn draw_triangle_outline<S: PixelSink>(
    disp: &mut S,
    tri: Triangle,
    color: Rgb565,
    thickness: u16,
) -> Result<(), crate::display::Error> {
    if thickness == 0 { return Ok(()); }
    let Triangle { a, b, c } = tri;
    // If degenerate (all same), no-op. If colinear, draw the three lines (which will overlap).
    draw_line(disp, a.x, a.y, b.x, b.y, color, thickness);
    draw_line(disp, b.x, b.y, c.x, c.y, color, thickness);
    draw_line(disp, c.x, c.y, a.x, a.y, color, thickness);
    Ok(())
}

