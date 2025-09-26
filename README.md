# STM32F429I-DISCO Bare Metal Embedded Rust Project

A bare-metal embedded Rust project for the STM32F429I-DISCO development board featuring custom hardware abstraction layers and MPU6050 sensor integration.

## 🎯 Features

- **Bare Metal Implementation**: Direct register manipulation without HAL dependencies
- **MPU6050 Sensor Support**: Both polling and interrupt-driven implementations
- **Real-Time Debugging**: RTT (Real-Time Transfer) for live debug output
- **Modular Architecture**: Clean separation of drivers and application logic
- **Interrupt Handling**: External interrupts for buttons and sensors
- **I2C Communication**: Custom I2C driver for sensor interfacing

## 🏗️ Architecture

This project implements a **register-level hardware abstraction** following a clean modular approach:

```
src/
├── main.rs              # Main application with interrupt handlers
├── board.rs            # STM32F429I-DISCO pin mappings
├── mcu.rs              # Hardware addresses & interrupt definitions
├── reg.rs              # Low-level register read/write operations
├── proc.rs             # NVIC interrupt control
├── startup_stm32f429.rs # System startup code
└── drivers/            # Hardware abstraction layer
    ├── mod.rs          # Driver module exports
    ├── gpio.rs         # GPIO configuration & control
    ├── led.rs          # LED driver (Green/Red LEDs)
    ├── button.rs       # User button with interrupt support
    ├── exti.rs         # External interrupt controller
    ├── i2c.rs          # I2C communication protocol
    ├── mpu6050.rs      # MPU6050 polling implementation
    └── mpu6050_interrupt.rs # MPU6050 interrupt-driven implementation
```

## 🎛️ Hardware Support

### STM32F429I-DISCO Board
- **MCU**: STM32F429ZIT6 (ARM Cortex-M4, 180MHz)
- **Flash**: 2048KB
- **RAM**: 256KB (192KB available)
- **Package**: LQFP144

### Peripherals
- **GPIO**: Full port control with configurable modes
- **LEDs**: Green (PG13) and Red (PG14) LEDs
- **Button**: User button (PA0) with EXTI0 interrupt
- **I2C1**: Hardware I2C for sensor communication
- **MPU6050**: 6-axis motion sensor (I2C interface)

## 🚀 Quick Start

### Prerequisites
- Rust with `thumbv7em-none-eabihf` target
- ARM GCC toolchain (`arm-none-eabi-gcc`)
- OpenOCD or ST-Link tools for flashing
- RTT viewer for debugging

### Building
```bash
# Build the project
cargo build --target thumbv7em-none-eabihf

# Generate binary for flashing
arm-none-eabi-objcopy -O binary \
  target/thumbv7em-none-eabihf/debug/flappy_bird_ffi \
  target/thumbv7em-none-eabihf/debug/flappy_bird_ffi.bin
```

### VS Code Tasks
Use the predefined build task:
- **Ctrl+Shift+P** → "Tasks: Run Task" → "Build flappy_bird_ffi project and generate .bin"

## 🔧 Usage Examples

### Basic LED Control
```rust
use drivers::led::*;

// Initialize LED
led_init();

// Control LEDs
set_led(LedColor::Green, PinState::High);   // Turn on green LED
set_led(LedColor::Red, PinState::Toggle);   // Toggle red LED
```

### Button with Interrupt
```rust
use drivers::button::*;

// Initialize button with interrupt
button_init();

// Interrupt handler (defined in main.rs)
#[no_mangle]
unsafe extern "C" fn EXTI0_Handler() {
    // Handle button press
    button_clear_interrupt_flag();
}
```

### MPU6050 Sensor (Polling)
```rust
use drivers::mpu6050::*;

// Initialize I2C and MPU6050
i2c_init();
mpu6050_init();

// Read sensor data
if let Some((ax, ay, az, gx, gy, gz)) = mpu6050_read_all() {
    rprintln!("Accel: {} {} {}, Gyro: {} {} {}", ax, ay, az, gx, gy, gz);
}
```

### MPU6050 Sensor (Interrupt-Driven)
```rust
use drivers::mpu6050_interrupt::*;

// Initialize with interrupt support
mpu6050_interrupt_init();

// In main loop
if mpu6050_interrupt::is_data_ready() {
    if let Some((ax, ay, az, gx, gy, gz)) = mpu6050_interrupt::read_sensor_data() {
        // Process sensor data
    }
}
```

## 🐛 Debugging

### RTT (Real-Time Transfer)
```rust
use rtt_target::{rprintln, rtt_init_print};

rtt_init_print!();
rprintln!("Debug message: {}", value);
```

### Memory Layout
- **Flash**: 0x08000000 (2048KB)
- **RAM**: 0x20000000 (192KB available)
- **Linker Script**: `link.x`

## 🔌 Hardware Connections

### MPU6050 Wiring
| MPU6050 Pin | STM32F429I-DISCO Pin | Description |
|-------------|---------------------|-------------|
| VCC         | 3.3V               | Power supply |
| GND         | GND                | Ground |
| SCL         | PB8 (I2C1_SCL)     | I2C Clock |
| SDA         | PB9 (I2C1_SDA)     | I2C Data |
| INT         | PB13               | Data Ready Interrupt |

## 📋 Development Patterns

### Register Manipulation
All hardware access uses safe register operations:
```rust
use crate::reg::*;

reg_set_bit(addr, pin, true);           // Set single bit
reg_set_bits(addr, value, pos, width);  // Set bit field
reg_set_val(addr, value);               // Set entire register
```

### Interrupt Safety
Wrap RTT prints in interrupt handlers:
```rust
cortex_m::interrupt::free(|_| {
    rprintln!("Safe debug output in interrupt");
});
```

## 🧪 Testing

The project includes comprehensive sensor integration tests:
- Basic I2C communication verification
- MPU6050 WHO_AM_I register check
- Interrupt-driven data acquisition
- Real-time sensor data monitoring

## 📚 Documentation

- **AI Coding Guidelines**: `.github/copilot-instructions.md`
- **Project Structure**: `PROJECT_STRUCTURE.md`
- **Hardware Reference**: STM32F429 datasheet and reference manual

## 🤝 Contributing

1. Follow the established register manipulation patterns
2. Add new drivers to `src/drivers/` directory
3. Update module exports in `drivers/mod.rs`
4. Maintain compatibility with `no_std` environment
5. Use RTT for debugging output

## 📄 License

This project is provided as-is for educational and development purposes.

## 🔗 Resources

- [STM32F429 Reference Manual](https://www.st.com/resource/en/reference_manual/dm00031020-stm32f405-415-stm32f407-417-stm32f427-437-and-stm32f429-439-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf)
- [MPU6050 Datasheet](https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-6000-Datasheet1.pdf)
- [Embedded Rust Documentation](https://docs.rust-embedded.org/)
- [RTT Documentation](https://docs.rs/rtt-target/)

---

**Built with ❤️ using Rust for embedded systems**