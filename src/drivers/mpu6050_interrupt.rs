use super::i2c::{i2c_read_register, i2c_write_register, I2CError};
use super::exti;
use super::gpio;
use crate::mcu;
use crate::proc;
use crate::board::{MPU6050_INT_PORT, MPU6050_INT_PIN};
use rtt_target::rprintln;
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
    rprintln!("MPU6050: Initializing interrupt-driven mode...");
    
    // Check WHO_AM_I register
    let who_am_i = i2c_read_register(MPU6050_ADDR, MPU6050_WHO_AM_I)?;
    rprintln!("MPU6050: WHO_AM_I = 0x{:02X}", who_am_i);
    
    if who_am_i != 0x68 {
        rprintln!("MPU6050: Device not found or incorrect WHO_AM_I");
        return Err(Mpu6050Error::DeviceNotFound);
    }
    
    // Reset the MPU6050 first (device reset)
    match i2c_write_register(MPU6050_ADDR, MPU6050_PWR_MGMT_1, 0x80) {
        Ok(()) => rprintln!("MPU6050: Device reset initiated"),
        Err(e) => {
            rprintln!("MPU6050: Failed to reset device: {:?}", e);
            return Err(Mpu6050Error::I2CError(e));
        }
    }
    
    // Wait for reset to complete (recommended 100ms)
    cortex_m::asm::delay(1_600_000); // ~100ms at 16MHz
    rprintln!("MPU6050: Reset wait completed");
    
    // Wake up the MPU6050 (clear sleep bit)
    match i2c_write_register(MPU6050_ADDR, MPU6050_PWR_MGMT_1, 0x00) {
        Ok(()) => rprintln!("MPU6050: Woke up device"),
        Err(e) => {
            rprintln!("MPU6050: Failed to wake up device: {:?}", e);
            return Err(Mpu6050Error::I2CError(e));
        }
    }
    
    // Wait for MPU6050 to stabilize after wake up (additional 50ms)
    cortex_m::asm::delay(800_000); // ~50ms at 16MHz
    rprintln!("MPU6050: Stabilization wait completed");
    
    // Set sample rate divider (1kHz / (1 + 19) = 50Hz for interrupt mode)
    match i2c_write_register(MPU6050_ADDR, MPU6050_SMPLRT_DIV, 0x13) {
        Ok(()) => rprintln!("MPU6050: Sample rate configured to 50Hz"),
        Err(e) => {
            rprintln!("MPU6050: Failed to set sample rate: {:?}", e);
            return Err(Mpu6050Error::I2CError(e));
        }
    }
    
    // Configure accelerometer (±2g full scale)
    match i2c_write_register(MPU6050_ADDR, MPU6050_ACCEL_CONFIG, 0x00) {
        Ok(()) => rprintln!("MPU6050: Accelerometer configured (±2g)"),
        Err(e) => {
            rprintln!("MPU6050: Failed to configure accelerometer: {:?}", e);
            return Err(Mpu6050Error::I2CError(e));
        }
    }
    
    // Configure gyroscope (±250°/s full scale)
    match i2c_write_register(MPU6050_ADDR, MPU6050_GYRO_CONFIG, 0x00) {
        Ok(()) => rprintln!("MPU6050: Gyroscope configured (±250°/s)"),
        Err(e) => {
            rprintln!("MPU6050: Failed to configure gyroscope: {:?}", e);
            return Err(Mpu6050Error::I2CError(e));
        }
    }
    
    // Set low pass filter (bandwidth 94Hz)
    match i2c_write_register(MPU6050_ADDR, MPU6050_CONFIG, 0x02) {
        Ok(()) => rprintln!("MPU6050: Low pass filter configured (94Hz)"),
        Err(e) => {
            rprintln!("MPU6050: Failed to configure low pass filter: {:?}", e);
            return Err(Mpu6050Error::I2CError(e));
        }
    }
    
    // Configure MPU6050 interrupt behavior (INT_CFG register)
    // Bit 7: INT_LEVEL=0 (active high), Bit 6: INT_OPEN=0 (push-pull), 
    // Bit 5: LATCH_INT_EN=1 (latch until cleared), Bit 4: INT_RD_CLEAR=1 (clear on status read)
    match i2c_write_register(MPU6050_ADDR, 0x37, 0x30) {
        Ok(()) => rprintln!("MPU6050: INT pin configured as active high, latched, clear on status read"),
        Err(e) => {
            rprintln!("MPU6050: Failed to configure INT pin: {:?}", e);
            return Err(Mpu6050Error::I2CError(e));
        }
    }
    
    // Enable data ready interrupt
    match i2c_write_register(MPU6050_ADDR, MPU6050_INT_ENABLE, 0x01) {
        Ok(()) => rprintln!("MPU6050: Data ready interrupt enabled"),
        Err(e) => {
            rprintln!("MPU6050: Failed to enable data ready interrupt: {:?}", e);
            return Err(Mpu6050Error::I2CError(e));
        }
    }
    
    // Configure interrupt pin on MCU
    setup_mpu6050_interrupt_pin()?;
    
    rprintln!("MPU6050: Interrupt-driven initialization complete");
    Ok(())
}

fn setup_mpu6050_interrupt_pin() -> Result<(), Mpu6050Error> {
    // Enable GPIOC clock
    gpio::enable_gpio_clock(MPU6050_INT_PORT);
    rprintln!("MPU6050: GPIOC clock enabled");
    
    // Configure PC13 as input with pull-down (since MPU6050 INT is active high)
    gpio::set_gpio_mode_input(MPU6050_INT_PORT, MPU6050_INT_PIN);
    gpio::set_gpio_pull_down(MPU6050_INT_PORT, MPU6050_INT_PIN);
    rprintln!("MPU6050: PC13 configured as input with pull-down");
    
    // Configure SYSCFG for EXTI13
    exti::gpio::configure_syscfg(MPU6050_INT_PORT, MPU6050_INT_PIN);
    rprintln!("MPU6050: SYSCFG configured for EXTI13");
    
    // Configure for rising edge (MPU6050 INT is active high)
    exti::gpio::set_edge(MPU6050_INT_PIN, exti::gpio::EdgeTrigger::Rising);
    rprintln!("MPU6050: EXTI13 configured for rising edge");
    
    // Enable EXTI13 interrupt
    if let Some(exti_line) = exti::ExtiLine::from_pin(MPU6050_INT_PIN) {
        exti::enable_interrupt(exti_line);
        rprintln!("MPU6050: EXTI13 interrupt enabled");
        
        // Enable NVIC for EXTI15_10 (covers EXTI13)
        if let Some(irq_num) = mcu::IRQn::from_pin(MPU6050_INT_PIN) {
            proc::enable_irq(irq_num);
            rprintln!("MPU6050: NVIC EXTI15_10 enabled (IRQ {})", irq_num);
        } else {
            rprintln!("MPU6050: ERROR - Could not map pin to IRQ number");
            return Err(Mpu6050Error::InitializationFailed);
        }
    } else {
        rprintln!("MPU6050: ERROR - Could not map pin to EXTI line");
        return Err(Mpu6050Error::InitializationFailed);
    }
    
    rprintln!("MPU6050: Interrupt pin setup complete on PC13");
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

// Conservative sensor read function with delays for I2C stability
pub fn mpu6050_read_all_burst() -> Result<Mpu6050Data, Mpu6050Error> {
    rprintln!("MPU6050: Starting conservative sensor read with delays...");
    
    // Read accelerometer data with small delays between register pairs
    let accel_x_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_XOUT_H)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    let accel_x_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_XOUT_L)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    
    // Small delay after reading X axis
    for _ in 0..1000 { cortex_m::asm::nop(); }
    
    let accel_y_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_YOUT_H)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    let accel_y_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_YOUT_L)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    
    // Small delay after reading Y axis
    for _ in 0..1000 { cortex_m::asm::nop(); }
    
    let accel_z_h = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_ZOUT_H)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    let accel_z_l = i2c_read_register(MPU6050_ADDR, MPU6050_ACCEL_ZOUT_L)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    
    // Longer delay before reading temperature (different register block)
    for _ in 0..2000 { cortex_m::asm::nop(); }
    
    // Read temperature data
    let temp_h = i2c_read_register(MPU6050_ADDR, MPU6050_TEMP_OUT_H)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    let temp_l = i2c_read_register(MPU6050_ADDR, MPU6050_TEMP_OUT_L)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    
    // Longer delay before reading gyroscope (different register block)
    for _ in 0..2000 { cortex_m::asm::nop(); }
    
    // Read gyroscope data with delays between axes
    let gyro_x_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_XOUT_H)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    let gyro_x_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_XOUT_L)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    
    for _ in 0..1000 { cortex_m::asm::nop(); }
    
    let gyro_y_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_YOUT_H)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    let gyro_y_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_YOUT_L)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    
    for _ in 0..1000 { cortex_m::asm::nop(); }
    
    let gyro_z_h = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_ZOUT_H)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    let gyro_z_l = i2c_read_register(MPU6050_ADDR, MPU6050_GYRO_ZOUT_L)
        .map_err(|e| Mpu6050Error::I2CError(e))?;
    
    rprintln!("MPU6050: Raw data - Accel: X({},{}) Y({},{}) Z({},{}) Temp({},{}) Gyro: X({},{}) Y({},{}) Z({},{})", 
        accel_x_h, accel_x_l, accel_y_h, accel_y_l, accel_z_h, accel_z_l,
        temp_h, temp_l, gyro_x_h, gyro_x_l, gyro_y_h, gyro_y_l, gyro_z_h, gyro_z_l);
    
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
    
    rprintln!("MPU6050: Full read complete - Accel({},{},{}) Gyro({},{},{}) Temp={}",
        accel.x, accel.y, accel.z, gyro.x, gyro.y, gyro.z, temperature);
    
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