# STM32F429I-DISC + ILI9341 (SPI) Setup

This guide wires the STM32F429I-DISC Discovery LCD (ILI9341 over SPI) and builds/flashes a minimal example using OpenOCD.

## Required Pin Mapping
Fill the table below from your board manual (Hardware layout, ~page 10) or the mbed platform page. Replace `TODO` with actual pins.

| Signal | MCU Pin | Notes |
|--------|---------|-------|
| SPI SCK | PF7 | SPI5 (example; confirm on your board) |
| SPI MOSI | PF9 | SPI5 |
| SPI MISO | PF8 | SPI5 (not used) |
| LCD DC (Data/Command) | PC2 | GPIO output |
| LCD CS (Chip Select) | — | Not used in example (NoCS interface) |
| LCD RST (Reset) | PD13 | GPIO output |
| LCD BL (Backlight) | TODO | GPIO (PWM optional) |

Orientation: Landscape configured via `MADCTL` (0x36) set to `0x28` (MV + BGR). Pixel format: RGB565.

Resolution: 320x240 (ILI9341) for STM32F429I-DISC; the hardware example uses width=320, height=240.

## Crates (planned)
- `stm32f4xx-hal` (stm32f429 + `rt`) for clocks, GPIO, SPI.
- `embedded-hal` for traits.
- `display-interface-spi` + an ILI9341 driver (or custom) to talk to the panel.
- `cortex-m`, `cortex-m-rt`, `panic-halt` for `no_std` runtime.

These will be added behind the `hw-ili9341` feature to keep desktop builds green.

## Build & Flash (OpenOCD)

Install tools on macOS:

```bash
brew install openocd
rustup target add thumbv7em-none-eabihf
```

Build the example:

```bash
cargo build --release --features hw-ili9341 --example f429i_rect --target thumbv7em-none-eabihf
```

Flash the binary with OpenOCD (adjust board config if needed):
Alternatively, using `probe-run` with ST-Link:

```bash
cargo install probe-run
PROBE_RUN_CHIPS=STM32F429ZI cargo run --features hw-ili9341 --example f429i_rect --target thumbv7em-none-eabihf
```


```bash
openocd -f board/stm32f429discovery.cfg \
  -c "program target/thumbv7em-none-eabihf/release/examples/f429i_rect verify reset exit"
```

## Next Steps
1. Provide exact pin mapping (SPI + DC/CS/RST/BL) and desired orientation.
2. I will wire the HAL setup, implement an `Ili9341Driver`, and update `examples/f429i_rect.rs` to draw to the LCD using the library's `draw_rectangle_outline()`.
3. We’ll add BMP rendering and more shapes next.
