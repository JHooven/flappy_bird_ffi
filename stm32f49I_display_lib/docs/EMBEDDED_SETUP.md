# STM32F429I-DISC LCD Options

This repo supports two distinct hardware paths. Make sure you follow the one that matches your setup:

1) External panel (8-bit GPIO, ILI9341/GC9A01A-like)
  - Uses discrete GPIO pins for CS/WR/RD/DC/RESET/BL and D0..D7.
  - Examples: `examples/f429i_rect.rs`, `examples/f429i_square.rs`.
  - Good for breakout modules wired to the Discovery headers.

2) Onboard LCD on STM32F429I-DISC/‑DISC1
  - The on‑board 2.4" TFT (ILI9341) is wired in 8080‑style parallel via FSMC/FMC, not LTDC RGB.
  - It requires FMC (FSMC) timing config, address/data bus GPIO AF setup, and ILI9341 register init.
  - The current GPIO bit‑banged examples will not light up the onboard screen.

## External Panel: Required Pin Mapping
Wire your panel per the mapping below (adjust for your hardware). Replace `TODO` with actual pins if needed.

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

Resolution: 320x240 (ILI9341). The hardware example uses width=320, height=240.

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

Build the external‑panel example:

```bash
cargo build --release --no-default-features --features hw-ili9341 --example f429i_rect --target thumbv7em-none-eabihf
```

Flash the binary with OpenOCD (adjust board config if needed):
Alternatively, using `probe-run` with ST-Link:

```bash
cargo install probe-run
PROBE_RUN_CHIPS=STM32F429ZI cargo run --no-default-features --features hw-ili9341 --example f429i_rect --target thumbv7em-none-eabihf
```


```bash
openocd -f board/stm32f429discovery.cfg \
  -c "program target/thumbv7em-none-eabihf/release/examples/f429i_rect verify reset exit"

## Onboard LCD (FMC 8080): Plan

To drive the on‑board display, we’ll provide a separate example that configures FMC and talks to ILI9341 over the 8080 bus:

- Configure FMC Bank for 8080‑style accesses with appropriate write timings.
- Configure GPIOs (ports D/E, etc.) to FMC alternate functions for A/D lines and control.
- Map LCD registers/GRAM to a memory region and perform ILI9341 init (sleep out, pixel format, MADCTL, display on).
- Implement an `Ili9341Driver` that uses FMC writes for `set_pixel()` and block fills.
- Optionally add a small framebuffer and a DMA strategy later.

Status: Not yet implemented in this repo. If you’re targeting the on‑board LCD, we will add `examples/f429i_onboard_fmc.rs` behind a `hw-onboard-lcd` feature.
```

## Next Steps
1. Provide exact pin mapping (SPI + DC/CS/RST/BL) and desired orientation.
2. I will wire the HAL setup, implement an `Ili9341Driver`, and update `examples/f429i_rect.rs` to draw to the LCD using the library's `draw_rectangle_outline()`.
3. We’ll add BMP rendering and more shapes next.
