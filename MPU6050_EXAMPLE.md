# MPU6050 Integration Example - Interrupt-Driven

This example demonstrates how to read accelerometer and gyroscope data from an MPU6050 sensor using **interrupt-driven** I2C communication with the STM32F429I-DISCO board.

## Hardware Setup

1. **Connect the GY-521 MPU6050 module:**
   - VCC → 3.3V
   - GND → GND  
   - SCL → PB8 (I2C1_SCL)
   - SDA → PB9 (I2C1_SDA)
   - AD0 → GND (for address 0x68)
   - **INT → PC13 (EXTI13)** ⚠️ **Required for interrupt operation**

## Code Features

### I2C Driver (`src/i2c.rs`)
- Custom register-level I2C implementation
- Standard mode (100kHz) communication
- Error handling for timeouts and NACK conditions
- Functions for register read/write operations

### MPU6050 Interrupt Driver (`src/mpu6050_interrupt.rs`)
- **Interrupt-driven** device initialization and configuration
- Accelerometer reading (±2g range)  
- Gyroscope reading (±250°/s range)
- Temperature sensor reading
- **Atomic flag-based** data ready signaling
- Unit conversion functions (raw → g-force, degrees/second, Celsius)
- **50Hz interrupt rate** for responsive motion detection

### Main Application (`src/main.rs`)
- Initializes I2C and MPU6050 in interrupt mode
- **Event-driven** sensor data reading (only when new data available)
- EXTI15_10 interrupt handler for MPU6050 data ready
- Outputs formatted sensor readings via RTT
- LED status indication (Green = MPU6050 OK, Red = heartbeat)

## Sample Output

```
RTT Debug: Starting flappy_bird_ffi on STM32F429I-DISCO
RTT Debug: I2C initialized
MPU6050: Initializing interrupt-driven mode...
MPU6050: WHO_AM_I = 0x68
MPU6050: Woke up device
MPU6050: Interrupt pin configured on PC13
MPU6050: Interrupt-driven initialization complete
RTT Debug: MPU6050 interrupt mode initialized successfully
RTT Debug: Delay initialized
RTT Debug: Loop iteration 1, LED toggled
RTT Debug: MPU6050 data ready interrupt!
MPU6050 INT [1]: Accel(g): X=0.02, Y=-0.15, Z=0.98 | Gyro(°/s): X=-1.2, Y=0.8, Z=0.1 | Temp: 26.4°C
RTT Debug: MPU6050 data ready interrupt!
MPU6050 INT [1]: Accel(g): X=0.01, Y=-0.14, Z=0.99 | Gyro(°/s): X=-0.9, Y=1.1, Z=-0.2 | Temp: 26.5°C
```

## Usage Notes

1. **I2C Configuration:** Uses I2C1 at 100kHz with internal pull-ups
2. **Interrupt-Driven:** 50Hz data ready interrupts via EXTI13 (PC13 pin)
3. **Atomic Operations:** Thread-safe data ready signaling using `AtomicBool`
4. **Error Handling:** Comprehensive error types for I2C and MPU6050 failures  
5. **Register Access:** All hardware access follows the project's register-level abstraction
6. **Debugging:** All operations logged via RTT for troubleshooting
7. **Efficiency:** CPU only processes data when new measurements are available

## Extending the Code

- Add interrupt-driven data acquisition using the INT pin
- Implement motion detection algorithms
- Add support for other I2C sensors on the same bus
- Create a sensor fusion algorithm combining accelerometer and gyroscope data