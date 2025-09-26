//! # Module Title
//!
//! This module provides [a brief overview of what the module does].
//!
//! It contains functions, structs, and enums to [explain key features or functionalities].
//!
//! ---
//!
//! The following items are included in this module:
//! - `EnumName`: Describes the different states or categories.
//! - `StructName`: Contains relevant data and methods.
//! - Functions like `function_name`: Describe what each function does.
//!

use crate::mcu::*;
use crate::reg::*;

pub fn enable_gpio_clock(port: u32) {
    let rcc_ahb1enr_addr = (RCC_BASE + 0x30) as *mut u32;

    match port {
        GPIOA_BASE => {
            //enable the 0th bit of rcc_ahb1enr_addr
            reg_set_bit(rcc_ahb1enr_addr, 0, true);
        }

        GPIOB_BASE => {
            //enable the 1st bit of rcc_ahb1enr_addr
            reg_set_bit(rcc_ahb1enr_addr, 1, true);
        }

        GPIOC_BASE => {
            //enable the 2nd bit of rcc_ahb1enr_addr
            reg_set_bit(rcc_ahb1enr_addr, 2, true);
        }

        GPIOG_BASE => {
            //enable the 6th bit of rcc_ahb1enr_addr
            reg_set_bit(rcc_ahb1enr_addr, 6, true);
        }

        _ => {} //catch all pattern, do nothing for values other than GPIOA_BASE
    }
}

pub fn set_gpio_mode_output(port: u32, pin: u32) {
    let set_gpio_mode_addr = (port + 0x00) as *mut u32;
    let bit_position = pin * 2;
    let mode_value = 0x1;

    reg_set_bits(set_gpio_mode_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_mode_input(port: u32, pin: u32) {
    let gpio_mode_reg_addr = (port + 0x00) as *mut u32;
    let bit_position = pin * 2;
    let mode_value = 0;

    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}


pub fn set_gpio_output_type_push_pull(port: u32, pin: u32) {
    let gpio_op_type_reg_addr = (port + 0x04) as *mut u32;
    let bit_position = pin;
    let bit_value = 0;

    reg_set_bits(gpio_op_type_reg_addr, bit_value, bit_position, 1);
}

/// Represents the state of a GPIO pin.
///
/// This enum is used to indicate whether the GPIO pin should be set to a high,
/// low, or toggled state.
pub enum PinState {
    /// The pin is set to a high state (logic 1).
    High,
    /// The pin is set to a low state (logic 0).
    Low,
    /// The pin state is toggled (from high to low, or from low to high)
    Toggle,
}

pub fn set_gpio_pin_state(port: u32, pin: u32, pin_state: PinState) {
    let gpio_bsrr_addr = (port + 0x18) as *mut u32;

    match pin_state {
        PinState::High => {
            reg_set_val(gpio_bsrr_addr, 1 << pin);
        }

        PinState::Low => {
            reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
        }

        PinState::Toggle => {
            let gpio_odr_addr = (port + 0x14) as *mut u32;
            if reg_read_bit(gpio_odr_addr, pin) {
                reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
            } else {
                reg_set_val(gpio_bsrr_addr, 1 << pin);
            }
        }
    }
}

pub fn get_gpio_pin_state(port: u32, pin: u32) -> bool {
    let gpio_idr_addr = (port + 0x10) as *mut u32;
    reg_read_bit(gpio_idr_addr, pin)
}

pub fn set_gpio_mode_alternate_function(port: u32, pin: u32) {
    let gpio_mode_reg_addr = (port + 0x00) as *mut u32;
    let bit_position = pin * 2;
    let mode_value = 0x2; // 10 = Alternate function mode
    
    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_alternate_function(port: u32, pin: u32, af_number: u32) {
    let gpio_afr_offset = if pin < 8 { 0x20 } else { 0x24 }; // AFRL or AFRH
    let gpio_afr_addr = (port + gpio_afr_offset) as *mut u32;
    let bit_position = (pin % 8) * 4;
    
    reg_set_bits(gpio_afr_addr, af_number, bit_position, 4);
}

pub fn set_gpio_output_type_open_drain(port: u32, pin: u32) {
    let gpio_otyper_addr = (port + 0x04) as *mut u32;
    reg_set_bit(gpio_otyper_addr, pin, true); // 1 = Open drain
}

pub fn set_gpio_pull_up(port: u32, pin: u32) {
    let gpio_pupdr_addr = (port + 0x0C) as *mut u32;
    let bit_position = pin * 2;
    let pupdr_value = 0x1; // 01 = Pull-up
    
    reg_set_bits(gpio_pupdr_addr, pupdr_value, bit_position, 2);
}

pub fn set_gpio_speed_high(port: u32, pin: u32) {
    let gpio_ospeedr_addr = (port + 0x08) as *mut u32;
    let bit_position = pin * 2;
    let speed_value = 0x2; // 10 = High speed
    
    reg_set_bits(gpio_ospeedr_addr, speed_value, bit_position, 2);
}
