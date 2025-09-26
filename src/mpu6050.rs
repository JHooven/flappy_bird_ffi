use crate::i2c::{i2c_read_register, i2c_write_register, I2CError};
use rtt_target::rprintln;

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

pub fn mpu6050_init() -> Result<(), Mpu6050Error> {
    rprintln!("MPU6050: Initializing...");
    
    // Check WHO_AM_I register
    let who_am_i = i2c_read_register(MPU6050_ADDR, MPU6050_WHO_AM_I)?;
    rprintln!("MPU6050: WHO_AM_I = 0x{:02X}", who_am_i);
    
    if who_am_i != 0x68 {
        rprintln!("MPU6050: Device not found or incorrect WHO_AM_I");
        return Err(Mpu6050Error::DeviceNotFound);
    }
    
    // Wake up the MPU6050 (clear sleep bit)
    i2c_write_register(MPU6050_ADDR, MPU6050_PWR_MGMT_1, 0x00)?;
    rprintln!("MPU6050: Woke up device");
    
    // Set sample rate divider (1kHz / (1 + 7) = 125Hz)
    i2c_write_register(MPU6050_ADDR, MPU6050_SMPLRT_DIV, 0x07)?;
    
    // Configure accelerometer (±2g full scale)
    i2c_write_register(MPU6050_ADDR, MPU6050_ACCEL_CONFIG, 0x00)?;
    
    // Configure gyroscope (±250°/s full scale)
    i2c_write_register(MPU6050_ADDR, MPU6050_GYRO_CONFIG, 0x00)?;
    
    // Set low pass filter (bandwidth 94Hz)
    i2c_write_register(MPU6050_ADDR, MPU6050_CONFIG, 0x02)?;
    
    rprintln!("MPU6050: Initialization complete");
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
    let accel = mpu6050_read_accel()?;
    let gyro = mpu6050_read_gyro()?;
    let temperature = mpu6050_read_temperature()?;
    
    Ok(Mpu6050Data {
        accel,
        gyro,
        temperature,
    })
}

// Helper functions to convert raw values to physical units
impl AccelData {
    /// Convert raw accelerometer values to g-force (assuming ±2g range)
    pub fn to_g(&self) -> (f32, f32, f32) {
        const ACCEL_SCALE: f32 = 2.0 / 32768.0; // ±2g range, 16-bit
        (
            self.x as f32 * ACCEL_SCALE,
            self.y as f32 * ACCEL_SCALE,
            self.z as f32 * ACCEL_SCALE,
        )
    }
}

impl GyroData {
    /// Convert raw gyroscope values to degrees per second (assuming ±250°/s range)
    pub fn to_dps(&self) -> (f32, f32, f32) {
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