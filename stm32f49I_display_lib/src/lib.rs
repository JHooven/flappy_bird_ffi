#![cfg_attr(not(feature = "std"), no_std)]

pub mod display;
pub mod shapes;
pub mod hw;

#[cfg(feature = "std")]
pub use display::{init_display, Display};
pub use display::{DisplayConfig, Error, Orientation, PixelFormat, Rgb565};
pub use shapes::{draw_rectangle_outline, draw_triangle_outline, Point, Rect, Triangle};
