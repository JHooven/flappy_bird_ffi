# STM32F429I-DISCO Bare Metal Embedded Rust Project

This is a bare-metal embedded Rust project targeting the STM32F429I-DISCO development board with custom hardware abstraction layers.

## Architecture Overview

This project implements a **register-level hardware abstraction** without using HAL libraries like `stm32f4xx-hal` (which is only listed as a dependency but not actively used). The core architecture follows this clean modular approach:

- **Hardware Layer** (`src/reg.rs`): Low-level register read/write operations using volatile pointers
- **MCU Layer** (`src/mcu.rs`): Hardware addresses, pin definitions, and interrupt mappings 
- **Drivers Module** (`src/drivers/`): All peripheral drivers organized in dedicated directory
  - `gpio.rs`, `led.rs`, `button.rs`, `exti.rs`: Basic peripheral drivers
  - `i2c.rs`: I2C communication protocol
  - `mpu6050_interrupt.rs`: Advanced interrupt-driven sensor interface
- **Board Configuration** (`src/board.rs`): STM32F429I-DISCO pin mappings
- **Application** (`src/main.rs`): Main loop with RTT debugging and interrupt handling

## Critical Development Patterns

### Register Manipulation
All hardware access goes through `src/reg.rs` functions - **never write raw pointer operations**:
```rust
reg_set_bit(addr, pin, true);           // Set single bit
reg_set_bits(addr, value, pos, width);  // Set multiple bits
reg_set_val(addr, value);               // Set entire register
```

### GPIO Operations
GPIO functions in `src/drivers/gpio.rs` follow this pattern:
1. Enable peripheral clock via `enable_gpio_clock(port)`
2. Configure mode with `set_gpio_mode_output/input(port, pin)`
3. Set additional properties (output type, etc.)
4. Control pin state with `PinState` enum (High/Low/Toggle)

### Interrupt Configuration
Button interrupts require coordinating multiple subsystems (`src/drivers/button.rs`):
1. GPIO configuration (clock, input mode)
2. SYSCFG mapping (`drivers::exti::gpio::configure_syscfg`)
3. EXTI edge configuration (`drivers::exti::gpio::set_edge`)
4. NVIC enable (`proc::enable_irq`)
5. Implement handler (e.g., `EXTI0_Handler`)

### Module Organization
All drivers are in `src/drivers/` directory:
- Use `use drivers::*;` for common exports
- Use `use drivers::module::specific_item;` for specific items
- Follow established register manipulation patterns

## Build & Debug Workflow

### Building
Use the predefined VS Code tasks or run:
```bash
cargo build --target thumbv7em-none-eabihf
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/rtt_prints target/thumbv7em-none-eabihf/debug/rtt_prints.bin
```

### Debugging
This project uses **RTT (Real-Time Transfer)** for debugging, not ITM:
```rust
use rtt_target::{rprintln, rtt_init_print};
rtt_init_print!();
rprintln!("Debug message: {}", value);
```

### Memory Layout
- Flash: 0x08000000, 2048K
- RAM: 0x20000000, 192K  
- Custom linker script: `link.x` (use this over `memory.x`)

## Project-Specific Conventions

### Hardware Addresses
All register base addresses are defined in `src/mcu.rs` as constants:
- Use `GPIOA_BASE`, `GPIOG_BASE` for GPIO ports
- Interrupt numbers in `IRQn` enum with `from_pin()` mapping

### Pin Definitions
Board-specific pins in `src/board.rs`:
- Green LED: GPIOG Pin 13
- Red LED: GPIOG Pin 14  
- User Button: GPIOA Pin 0 (EXTI0 interrupt)

### Critical Sections
Always wrap RTT prints in interrupt handlers with `cortex_m::interrupt::free()` to prevent data races.

### Error Handling
This is a `no_std` environment - use `panic_handler` for unrecoverable errors. The current handler just loops infinitely.

## Key Files to Understand

- `src/reg.rs`: Core register manipulation - understand before modifying any hardware access
- `src/mcu.rs`: Hardware memory map and interrupt vectors  
- `src/drivers/`: All peripheral drivers organized by functionality
  - `gpio.rs`: GPIO configuration and control
  - `exti.rs`: External interrupt controller - critical for button handling
  - `i2c.rs`: I2C communication implementation
  - `mpu6050_interrupt.rs`: Advanced interrupt-driven sensor interface
- `src/proc.rs`: NVIC interrupt control
- `link.x`: Memory layout and sections

When adding new peripherals, follow the established pattern: create a new driver in `src/drivers/`, add exports to `drivers/mod.rs`, add constants to `mcu.rs`, and coordinate with existing interrupt/clock systems.