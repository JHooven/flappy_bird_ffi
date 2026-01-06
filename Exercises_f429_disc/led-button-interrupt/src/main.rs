#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]

//use board::*;
use button::*;
use core::panic::PanicInfo;
use led::*;

use crate::board::{
    BLUE_LED_PIN, BLUE_LED_PORT, RED_LED_PIN, RED_LED_PORT, USER_BTN_PIN, USER_BTN_PORT,
};

//use crate::{button::{button_configure_interrupt, button_init}, led::{led_init, led_off}};

mod board;
mod button;
mod gpio;
mod led;
mod mcu;
mod reg;
mod proc;
mod startup_stm32f429;
mod exti;

#[unsafe(no_mangle)]
fn main() {
    led_init(BLUE_LED_PORT, BLUE_LED_PIN);
    led_init(RED_LED_PORT, RED_LED_PIN);

    led_on(BLUE_LED_PORT, BLUE_LED_PIN);
    led_on(RED_LED_PORT, RED_LED_PIN);

    button::button_init(
        USER_BTN_PORT,
        USER_BTN_PIN,
        Mode::Interrupt(Trigger::FallingEdge),
    );

    //button_configure_interrupt(BUTTON_PIN);

    loop {} 
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

//button interrupt handler
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn EXTI0_Handler() {
    led_toggle(RED_LED_PORT, RED_LED_PIN);
    button::button_clear_interrupt(USER_BTN_PIN);
}
