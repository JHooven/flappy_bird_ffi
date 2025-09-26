// Hardware drivers module
//
// This module contains all hardware abstraction layer drivers
// for peripherals on the STM32F429I-DISCO board

pub mod gpio;
pub mod led; 
pub mod button;
pub mod i2c;
pub mod exti;
pub mod mpu6050;
pub mod mpu6050_interrupt;

// Re-export commonly used items for convenience
pub use led::*;