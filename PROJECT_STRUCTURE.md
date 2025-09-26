# Project Structure

This embedded Rust project follows a clean modular architecture:

```
src/
├── main.rs                    # Main application entry point
├── board.rs                   # Board-specific pin definitions (STM32F429I-DISCO)
├── mcu.rs                     # MCU hardware addresses and IRQ mappings
├── reg.rs                     # Low-level register manipulation functions
├── proc.rs                    # NVIC interrupt control
├── startup_stm32f429.rs       # Startup code and vector table
└── drivers/                   # Hardware drivers directory
    ├── mod.rs                 # Drivers module exports
    ├── gpio.rs                # GPIO configuration and control
    ├── led.rs                 # LED abstraction layer
    ├── button.rs              # Button with interrupt support
    ├── i2c.rs                 # I2C1 communication driver
    ├── exti.rs                # External interrupt controller
    ├── mpu6050.rs             # MPU6050 sensor (polling mode)
    └── mpu6050_interrupt.rs   # MPU6050 sensor (interrupt-driven)
```

## Architecture Layers

### 1. Hardware Abstraction Layer (HAL)
- **`reg.rs`**: Lowest level - volatile register read/write operations
- **`mcu.rs`**: Hardware memory map and interrupt vectors
- **`drivers/gpio.rs`**: GPIO configuration and pin control

### 2. Peripheral Drivers
- **`drivers/i2c.rs`**: I2C communication protocol implementation
- **`drivers/exti.rs`**: External interrupt configuration
- **`drivers/led.rs`**: LED control abstraction
- **`drivers/button.rs`**: Button input with interrupt support

### 3. Sensor Drivers
- **`drivers/mpu6050.rs`**: Basic MPU6050 sensor interface (polling)
- **`drivers/mpu6050_interrupt.rs`**: Advanced interrupt-driven MPU6050 interface

### 4. Board Support
- **`board.rs`**: STM32F429I-DISCO specific pin assignments
- **`startup_stm32f429.rs`**: Boot code and interrupt vector table

### 5. Application Layer
- **`main.rs`**: Application logic and main loop

## Key Design Principles

1. **Register-Level Control**: All hardware access goes through `reg.rs` functions
2. **Modular Design**: Each peripheral has its own driver module
3. **Interrupt Support**: Both polling and interrupt-driven operations
4. **Board Abstraction**: Hardware pins defined in `board.rs`
5. **No HAL Dependencies**: Custom bare-metal implementation following project patterns

## Usage

Import drivers in `main.rs`:
```rust
use drivers::*;  // Common exports
use drivers::button::Mode;  // Specific items
```

All driver modules follow the established register manipulation patterns and error handling conventions.