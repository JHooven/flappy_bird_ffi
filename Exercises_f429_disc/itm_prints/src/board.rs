use crate::mcu::*;

pub const BLUE_LED_PIN: u32 = GPIO_PIN_13;
pub const BLUE_LED_PORT: u32 = GPIOG_BASE;

pub const RED_LED_PIN: u32 = GPIO_PIN_14;
pub const RED_LED_PORT: u32 = GPIOG_BASE;

pub const USER_BTN_PORT: u32 = GPIOA_BASE;
pub const USER_BTN_PIN: u32 = GPIO_PIN_0;
