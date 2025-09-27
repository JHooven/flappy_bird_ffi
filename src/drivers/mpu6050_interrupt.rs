use super::i2c::{i2c_read_register, i2c_write_register, I2CError};
use super::exti;
use super::gpio;
use crate::mcu;
use crate::proc;
use crate::board::{MPU6050_INT_PORT, MPU6050_INT_PIN};

use core::sync::atomic::{AtomicBool, Ordering};

// Global flag for data ready interrupt
static MPU6050_DATA_READY: AtomicBool = AtomicBool::new(false);

// MPU6050 I2C address (AD0 pin low)
pub const MPU6050_ADDR: u8 = 0x68;

// MPU6050 Register addresses
const MPU6050_WHO_AM_I: u8 = 0x75;
const MPU6050_PWR_MGMT_1: u8 = 0x6B;
const MPU6050_PWR_MGMT_2: u8 = 0x6C;
const MPU6050_GYRO_CONFIG: u8 = 0x1B;
const MPU6050_ACCEL_CONFIG: u8 = 0x1C;
const MPU6050_SMPLRT_DIV: u8 = 0x19;
const MPU6050_CONFIG: u8 = 0x1A;
const MPU6050_INT_ENABLE: u8 = 0x38;
const MPU6050_INT_STATUS: u8 = 0x3A;

// Data registers
const MPU6050_ACCEL_XOUT_H: u8 = 0x3B;
const MPU6050_ACCEL_XOUT_L: u8 = 0x3C;
const MPU6050_ACCEL_YOUT_H: u8 = 0x3D;
const MPU6050_ACCEL_YOUT_L: u8 = 0x3E;
const MPU6050_ACCEL_ZOUT_H: u8 = 0x3F;
const MPU6050_ACCEL_ZOUT_L: u8 = 0x40;

const MPU6050_TEMP_OUT_H: u8 = 0x41;
const MPU6050_TEMP_OUT_L: u8 = 0x42;

const MPU6050_GYRO_XOUT_H: u8 = 0x43;
const MPU6050_GYRO_XOUT_L: u8 = 0x44;
const MPU6050_GYRO_YOUT_H: u8 = 0x45;
const MPU6050_GYRO_YOUT_L: u8 = 0x46;
const MPU6050_GYRO_ZOUT_H: u8 = 0x47;
const MPU6050_GYRO_ZOUT_L: u8 = 0x48;

#[derive(Debug, Copy, Clone)]
pub struct AccelData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Debug, Copy, Clone)]
pub struct GyroData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Debug, Copy, Clone)]
pub struct Mpu6050Data {
    pub accel: AccelData,
    pub gyro: GyroData,
    pub temperature: i16,
}

#[derive(Debug)]
pub enum Mpu6050Error {
    I2CError(I2CError),
    DeviceNotFound,
    InitializationFailed,
}

impl From<I2CError> for Mpu6050Error {
    fn from(error: I2CError) -> Self {
        Mpu6050Error::I2CError(error)
    }
}

pub fn mpu6050_init_interrupt_driven() -> Result<(), Mpu6050Error> {
    // Initialize MPU6050 in interrupt-driven mode
    
    // Check WHO_AM_I register
    let who_am_i = i2c_read_register(MPU6050_ADDR, MPU6050_WHO_AM_I)?;

    
    if who_am_i != 0x68 {
        return Err(Mpu6050Error::DeviceNotFound);
    }
    
    // Reset the MPU6050 first (device reset)
    i2c_write_register(MPU6050_ADDR, MPU6050_PWR_MGMT_1, 0x80)?;
    
    // Wait for reset to complete (recommended 100ms)
    cortex_m::asm::delay(18_000_000); // ~100ms at 180MHz
    
    // Wake up the MPU6050 (clear sleep bit)
    i2c_write_register(MPU6050_ADDR, MPU6050_PWR_MGMT_1, 0x00)?;
    
    // Wait for MPU6050 to stabilize after wake up (additional 50ms)
    cortex_m::asm::delay(9_000_000); // ~50ms at 180MHz
    
    // Set sample rate divider (1kHz / (1 + 19) = 50Hz for interrupt mode)
    i2c_write_register(MPU6050_ADDR, MPU6050_SMPLRT_DIV, 0x13)?;
    
    // Configure accelerometer (±2g full scale)
    i2c_write_register(MPU6050_ADDR, MPU6050_ACCEL_CONFIG, 0x00)?;
    
    // Configure gyroscope (±250°/s full scale)
    i2c_write_register(MPU6050_ADDR, MPU6050_GYRO_CONFIG, 0x00)?;
    
    // Set low pass filter (bandwidth 94Hz)
    i2c_write_register(MPU6050_ADDR, MPU6050_CONFIG, 0x02)?;
    
    // Configure MPU6050 interrupt pin behavior (INT_PIN_CFG register 0x37)
    // Bit 7: INT_LEVEL=0 (active high), Bit 6: INT_OPEN=0 (push-pull), 
    // Bit 5: LATCH_INT_EN=1 (latch until cleared), Bit 4: INT_RD_CLEAR=1 (clear on status read)
    // Bit 2: FSYNC_INT_LEVEL=0, Bit 1: FSYNC_INT_EN=0, Bit 0: I2C_BYPASS_EN=0
    i2c_write_register(MPU6050_ADDR, 0x37, 0x30)?;
    
    // Also try alternative configuration - some MPU6050s need different settings
    // Let's try with LATCH disabled first to see if that helps
    i2c_write_register(MPU6050_ADDR, 0x37, 0x10)?;  // Only INT_RD_CLEAR=1
    
    // Enable data ready interrupt
    i2c_write_register(MPU6050_ADDR, MPU6050_INT_ENABLE, 0x01)?;
    
    // Configure interrupt pin on MCU
    setup_mpu6050_interrupt_pin()?;
    
    Ok(())
}

fn setup_mpu6050_interrupt_pin() -> Result<(), Mpu6050Error> {
    // Enable GPIOC clock
    gpio::enable_gpio_clock(MPU6050_INT_PORT);
    
    // Configure PC13 as input with pull-down (since MPU6050 INT is active high)
    gpio::set_gpio_mode_input(MPU6050_INT_PORT, MPU6050_INT_PIN);
    gpio::set_gpio_pull_down(MPU6050_INT_PORT, MPU6050_INT_PIN);
    
    // Configure SYSCFG for EXTI13
    exti::gpio::configure_syscfg(MPU6050_INT_PORT, MPU6050_INT_PIN);
    
    // Configure for rising edge (MPU6050 INT is active high)
    exti::gpio::set_edge(MPU6050_INT_PIN, exti::gpio::EdgeTrigger::Rising);
    
    // Enable EXTI13 interrupt
    if let Some(exti_line) = exti::ExtiLine::from_pin(MPU6050_INT_PIN) {
        exti::enable_interrupt(exti_line);
        
        // Enable NVIC for EXTI15_10 (covers EXTI13)
        if let Some(irq_num) = mcu::IRQn::from_pin(MPU6050_INT_PIN) {
            proc::enable_irq(irq_num);
        } else {
            return Err(Mpu6050Error::InitializationFailed);
        }
    } else {
        return Err(Mpu6050Error::InitializationFailed);
    }
    
    Ok(())
}

pub fn mpu6050_read_accel() -> Result<AccelData, Mpu6050Error> {
    let x_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_XOUT_H)? as i16;
    let x_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_XOUT_L)? as i16;
    let x = (x_h << 8) | x_l;
    
    let y_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_YOUT_H)? as i16;
    let y_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_YOUT_L)? as i16;
    let y = (y_h << 8) | y_l;
    
    let z_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_ZOUT_H)? as i16;
    let z_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_ZOUT_L)? as i16;
    let z = (z_h << 8) | z_l;
    
    Ok(AccelData { x, y, z })
}

pub fn mpu6050_read_gyro() -> Result<GyroData, Mpu6050Error> {
    let x_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_XOUT_H)? as i16;
    let x_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_XOUT_L)? as i16;
    let x = (x_h << 8) | x_l;
    
    let y_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_YOUT_H)? as i16;
    let y_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_YOUT_L)? as i16;
    let y = (y_h << 8) | y_l;
    
    let z_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_ZOUT_H)? as i16;
    let z_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_ZOUT_L)? as i16;
    let z = (z_h << 8) | z_l;
    
    Ok(GyroData { x, y, z })
}

pub fn mpu6050_read_temperature() -> Result<i16, Mpu6050Error> {
    let temp_h = i2c_read_register(MPU6050_ADDR, MPU6050_TEMP_OUT_H)? as i16;
    let temp_l = i2c_read_register(MPU6050_ADDR, MPU6050_TEMP_OUT_L)? as i16;
    let temp_raw = (temp_h << 8) | temp_l;
    
    Ok(temp_raw)
}

pub fn mpu6050_read_all() -> Result<Mpu6050Data, Mpu6050Error> {
    // Use burst read to get all sensor data in one I2C transaction
    // This is safer than multiple individual reads
    mpu6050_read_all_burst()
}

// Conservative sensor read function - original working delays for I2C stability
pub fn mpu6050_read_all_burst() -> Result<Mpu6050Data, Mpu6050Error> {
    // Read all accelerometer data with conservative delays
    let accel_x_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_XOUT_H)?;
    let accel_x_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_XOUT_L)?;
    
    // Conservative delay between axis pairs (1000 NOPs = ~5.6μs)
    for _ in 0..1000 { cortex_m::asm::nop(); }
    
    let accel_y_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_YOUT_H)?;
    let accel_y_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_YOUT_L)?;
    
    for _ in 0..1000 { cortex_m::asm::nop(); }
    
    let accel_z_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_ZOUT_H)?;
    let accel_z_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_ZOUT_L)?;
    
    // Longer delay between sensor blocks (2000 NOPs = ~11μs)
    for _ in 0..2000 { cortex_m::asm::nop(); }
    
    // Read temperature data
    let temp_h = i2c_read_register(MPU6050_ADDR, MPU6050_TEMP_OUT_H)?;
    let temp_l = i2c_read_register(MPU6050_ADDR, MPU6050_TEMP_OUT_L)?;
    
    for _ in 0..2000 { cortex_m::asm::nop(); }
    
    // Read all gyroscope data
    let gyro_x_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_XOUT_H)?;
    let gyro_x_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_XOUT_L)?;
    
    for _ in 0..1000 { cortex_m::asm::nop(); }
    
    let gyro_y_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_YOUT_H)?;
    let gyro_y_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_YOUT_L)?;
    
    for _ in 0..1000 { cortex_m::asm::nop(); }
    
    let gyro_z_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_ZOUT_H)?;
    let gyro_z_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_ZOUT_L)?;
    
    // Convert raw bytes to 16-bit signed values
    let accel = AccelData {
        x: ((accel_x_h as i16) << 8) | (accel_x_l as i16),
        y: ((accel_y_h as i16) << 8) | (accel_y_l as i16),
        z: ((accel_z_h as i16) << 8) | (accel_z_l as i16),
    };
    
    let gyro = GyroData {
        x: ((gyro_x_h as i16) << 8) | (gyro_x_l as i16),
        y: ((gyro_y_h as i16) << 8) | (gyro_y_l as i16),
        z: ((gyro_z_h as i16) << 8) | (gyro_z_l as i16),
    };
    
    let temperature = ((temp_h as i16) << 8) | (temp_l as i16);
    
    Ok(Mpu6050Data {
        accel,
        gyro,
        temperature,
    })
}

// Check if new data is available (called from main loop)
pub fn mpu6050_data_ready() -> bool {
    MPU6050_DATA_READY.load(Ordering::Relaxed)
}

// Clear the data ready flag after reading
pub fn mpu6050_clear_data_ready() {
    MPU6050_DATA_READY.store(false, Ordering::Relaxed);
}

// Set the data ready flag (called from interrupt handler)
pub fn mpu6050_set_data_ready() {
    MPU6050_DATA_READY.store(true, Ordering::Relaxed);
}

// Clear MPU6050 interrupt status
pub fn mpu6050_clear_interrupt() -> Result<(), Mpu6050Error> {
    // Read interrupt status register to clear the interrupt
    let _status = i2c_read_register(MPU6050_ADDR, MPU6050_INT_STATUS)?;
    Ok(())
}

// Helper functions to convert raw values to physical units
impl AccelData {
    /// Convert raw accelerometer values to g-force (assuming ±2g range)
    /// Using integer arithmetic to avoid floating-point issues
    pub fn to_g(&self) -> (i32, i32, i32) {
        // Convert to millig (1/1000 g) to avoid floating point
        // ±2g range, 16-bit: 2000mg / 32768 = ~0.061mg per LSB
        // Multiply by 61 then divide by 1000 to get millig
        (
            (self.x as i32 * 61) / 1000,
            (self.y as i32 * 61) / 1000,
            (self.z as i32 * 61) / 1000,
        )
    }
    
    /// Convert raw accelerometer values to floating point (safer version)
    pub fn to_g_float(&self) -> (f32, f32, f32) {
        const ACCEL_SCALE: f32 = 2.0 / 32768.0; // ±2g range, 16-bit
        (
            self.x as f32 * ACCEL_SCALE,
            self.y as f32 * ACCEL_SCALE,
            self.z as f32 * ACCEL_SCALE,
        )
    }
}

impl GyroData {
    /// Convert raw gyroscope values to degrees per second (integer version)
    /// Using integer arithmetic to avoid floating-point issues  
    pub fn to_dps(&self) -> (i32, i32, i32) {
        // Convert to milli-degrees per second to avoid floating point
        // ±250°/s range, 16-bit: 250000 mdps / 32768 = ~7.6 mdps per LSB
        // Multiply by 76 then divide by 10 to get mdps
        (
            (self.x as i32 * 76) / 10,
            (self.y as i32 * 76) / 10, 
            (self.z as i32 * 76) / 10,
        )
    }
    
    /// Convert raw gyroscope values to floating point (safer version)
    pub fn to_dps_float(&self) -> (f32, f32, f32) {
        const GYRO_SCALE: f32 = 250.0 / 32768.0; // ±250°/s range, 16-bit
        (
            self.x as f32 * GYRO_SCALE,
            self.y as f32 * GYRO_SCALE,
            self.z as f32 * GYRO_SCALE,
        )
    }
}

/// Convert raw temperature to Celsius
pub fn temperature_to_celsius(temp_raw: i16) -> f32 {
    (temp_raw as f32 / 340.0) + 36.53
}