# Bookmarks

## Conversation References

- "can you study the file stm32f429xx.pdf located in my documents folder and then tell me how to wire a "Pre-Soldered GY-521 MPU-6050 MPU6050 3 Axis Accelerometer Gyroscope Module 6 DOF 6-Axis Accelerometer Gyroscope Sensor Module 16 Bit AD Converter Data Output IIC I2C" to my board?"

## Useful Links

- [STM32F429I-DISCO Datasheet](https://www.st.com/resource/en/data_brief/32f429idiscovery.pdf)

## Project Notes

- Project uses register-level hardware abstraction (no HAL)
- RTT debugging via `rtt-target` crate
- Custom interrupt handling for button (EXTI0)

## MPU6050 Wiring (Interrupt-Driven)

Connect the GY-521 MPU6050 module to STM32F429I-DISCO as follows:

| MPU6050 Pin | STM32F429I-DISCO Pin | Description |
|-------------|---------------------|-------------|
| VCC         | 3.3V               | Power supply |
| GND         | GND                | Ground |
| SCL         | PB8                | I2C Serial Clock |
| SDA         | PB9                | I2C Serial Data |
| AD0         | GND                | Address select (0x68) |
| INT         | PC13               | **Interrupt output (EXTI13)** |

**Note**: 
- Connect AD0 to GND for I2C address 0x68, or to 3.3V for address 0x69.
- **INT pin connection is required** for interrupt-driven operation.
- The MPU6050 generates interrupts at 50Hz when new data is available.