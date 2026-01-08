# STM32F429 Rectangle App (Standalone)

This is a self-contained embedded app for STM32F429ZI that drives an 8-bit parallel display and draws a rectangle outline. It has no dependency on the workspace root library.

## Prerequisites
- Rust target installed: `thumbv7em-none-eabihf`
- ARM toolchain: `arm-none-eabi-gdb`, OpenOCD or ST-Link tools
- macOS Homebrew OpenOCD scripts path (adjust if different): `/opt/homebrew/share/openocd/scripts`

## Build
```bash
# From this folder
cargo build --target thumbv7em-none-eabihf
```

## Flash (OpenOCD)
```bash
openocd -s /opt/homebrew/share/openocd/scripts \
  -f interface/stlink.cfg -f target/stm32f4x.cfg \
  -c "init; reset halt; program target/thumbv7em-none-eabihf/debug/f429i_rect_app verify; reset run; shutdown"
```

## Flash (ST-Link)
```bash
# Create a binary and flash to 0x08000000
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/f429i_rect_app target/thumbv7em-none-eabihf/debug/f429i_rect_app.bin
st-flash --reset write target/thumbv7em-none-eabihf/debug/f429i_rect_app.bin 0x08000000
```

## Debug (VS Code)
You can use the app-local `.vscode` configs in this folder if you open this folder as the workspace root in VS Code.
- Use “Debug: app f429i_rect (OpenOCD)” or “Debug: app f429i_rect (ST-Link)”.
- Breaks on `main`. On faults, custom handlers `HardFault`, `DefaultHandler`, and `SysTick` trigger a BKPT to stop in GDB.

## Debug (CLI, OpenOCD + GDB)
```bash
# Terminal 1: OpenOCD
openocd -s /opt/homebrew/share/openocd/scripts -f interface/stlink.cfg -f target/stm32f4x.cfg

# Terminal 2: GDB
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/f429i_rect_app -ex "target extended-remote :3333" -ex "monitor reset halt" -ex "load" -ex "break main" -ex "continue"
```

## Notes
- Pin mapping and panel protocol live entirely in `src/main.rs`.
- Memory map is defined in `memory.x`.
- If OpenOCD scripts live elsewhere, change the `-s` argument accordingly.
