---
name: Feature — Display Initialization
about: Implement display initialization API and clear-screen behavior
title: "Feature: Display Initialization"
labels: [enhancement]
assignees: []
---

## Context
- MCU/Display: <STM32F4x9 series?>, controller <TBD>, bus <SPI/Parallel/DSI TBD>
- Pixel format & resolution: <RGB565>, <TBD width>x<TBD height>
- Constraints: `no_std`, no heap preferred; init ≤ 50 ms; allowed crates <e.g., embedded-hal, embedded-graphics?>

## Goal
Provide `init_display()` that configures the controller (orientation, pixel format, viewport) and exposes `clear(color)`.

## API Sketch
- `fn init_display(cfg: DisplayConfig) -> Result<Display, Error>`
- `impl Display { fn clear(&mut self, color: Rgb565) -> Result<(), Error> }`
- `struct DisplayConfig { width: u16, height: u16, orientation: Orientation, pixel_format: PixelFormat /* RGB565 */, /* bus specifics */ }`

## Constraints
- No dynamic allocation; retries on transient bus errors; clear should stream efficiently.

## Edge Cases
- Invalid dimensions/orientation; repeated init; bus timeouts; unsupported pixel format.

## Acceptance Criteria
- Unit tests for config validation and clear behavior (simulator framebuffer);
- Doc example compiles and demonstrates `init_display` + `clear`;
- Code structured into `src/lib.rs`, `src/display.rs`; example in `examples/init.rs`.

## Test Plan
- Host framebuffer simulator with golden-image diff for `clear` to a non-zero color.
- Property tests for dimension/orientation validation.

## Artifacts
- Create/update:
  - `src/lib.rs`, `src/display.rs`
  - `tests/display_init.rs`
  - `examples/init.rs`

## Open Questions
- Exact controller and bus; orientation defaults; error taxonomy; reset pin handling.