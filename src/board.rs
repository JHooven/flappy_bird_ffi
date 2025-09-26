use crate::mcu::*;

pub const GREEN_LED_PIN: u32 = GPIO_PIN_13;
pub const GREEN_LED_PORT: u32 = GPIOG_BASE;

pub const RED_LED_PIN: u32 = GPIO_PIN_14;
pub const RED_LED_PORT: u32 = GPIOG_BASE;

pub const USER_BTN_PORT: u32 = GPIOA_BASE;
pub const USER_BTN_PIN: u32 = GPIO_PIN_0;

// I2C1 pins for MPU6050 connection
// Connect MPU6050 SCL to PB8, SDA to PB9
pub const I2C_SCL_PORT: u32 = GPIOB_BASE;
pub const I2C_SCL_PIN: u32 = 8;
pub const I2C_SDA_PORT: u32 = GPIOB_BASE;
pub const I2C_SDA_PIN: u32 = 9;

// MPU6050 interrupt pin
// Connect MPU6050 INT to PC13 (EXTI13)
pub const MPU6050_INT_PORT: u32 = GPIOC_BASE;
pub const MPU6050_INT_PIN: u32 = GPIO_PIN_13;
