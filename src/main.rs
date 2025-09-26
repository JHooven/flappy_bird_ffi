#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]

// use cortex_m::peripheral::syst::SystClkSource; // No longer needed
//use board::*;
use drivers::*;
use cortex_m::delay::Delay;
use core::panic::PanicInfo;
use rtt_target::{rprintln, rtt_init_print};

use crate::board::{
    GREEN_LED_PIN, GREEN_LED_PORT, RED_LED_PIN, RED_LED_PORT, USER_BTN_PIN, USER_BTN_PORT,
};

// Removed static PERIPHERALS to avoid Sync error
//use crate::{button::{button_configure_interrupt, button_init}, led::{led_init, led_off}};

// ITM debug module removed - not supported on STM32F429I-DISCO
// mod itm_debug;
mod board;
mod drivers;
mod mcu;
mod reg;
mod proc;
mod startup_stm32f429;

// Macro to initialize cortex-m peripherals
macro_rules! init_cortex_m_peripherals {
    () => {
        cortex_m::Peripherals::take().unwrap()
    };
}

// Note: ITM macros removed - RTT is used instead for STM32F429I-DISCO

#[unsafe(no_mangle)]
fn main() {
    led_init(GREEN_LED_PORT, GREEN_LED_PIN);
    led_init(RED_LED_PORT, RED_LED_PIN);

    led_off(GREEN_LED_PORT, GREEN_LED_PIN);
    led_on(RED_LED_PORT, RED_LED_PIN);


    drivers::button::button_init(
        USER_BTN_PORT,
        USER_BTN_PIN,
        drivers::button::Mode::Interrupt(drivers::button::Trigger::FallingEdge),
    );

    let cp = init_cortex_m_peripherals!();

    // Initialize RTT for debug output
    rtt_init_print!();
    
    rprintln!("RTT Debug: Starting flappy_bird_ffi on STM32F429I-DISCO");

    // Initialize I2C for MPU6050 communication
    drivers::i2c::i2c_init();
    rprintln!("RTT Debug: I2C initialized");
    
    // Small delay to let I2C settle
    cortex_m::asm::delay(1_000_000);
    
    // Initialize MPU6050 in interrupt-driven mode
    match drivers::mpu6050_interrupt::mpu6050_init_interrupt_driven() {
        Ok(()) => {
            rprintln!("RTT Debug: MPU6050 interrupt mode initialized successfully");
            led_on(GREEN_LED_PORT, GREEN_LED_PIN);
        }
        Err(e) => {
            rprintln!("RTT Debug: MPU6050 initialization failed: {:?}", e);
            led_off(GREEN_LED_PORT, GREEN_LED_PIN);
        }
    }

    // Create a delay instance - assuming 16MHz system clock
    let mut delay = Delay::new(cp.SYST, 16_000_000);
    rprintln!("RTT Debug: Delay initialized");
    
    let mut counter = 0;
    loop {
        // Blink LED to show we're alive - delay for 1000ms
        delay.delay_ms(1000u32);
        led_toggle(RED_LED_PORT, RED_LED_PIN);
        
        counter += 1;
        rprintln!("RTT Debug: Loop iteration {}, LED toggled", counter);
        
        // Check if MPU6050 has new data (interrupt-driven)
        if drivers::mpu6050_interrupt::mpu6050_data_ready() {
            match drivers::mpu6050_interrupt::mpu6050_read_all() {
                Ok(data) => {
                    let (ax, ay, az) = data.accel.to_g();
                    let (gx, gy, gz) = data.gyro.to_dps();
                    let temp = drivers::mpu6050_interrupt::temperature_to_celsius(data.temperature);
                    
                    rprintln!("MPU6050 INT [{}]: Accel(g): X={:.2}, Y={:.2}, Z={:.2} | Gyro(°/s): X={:.1}, Y={:.1}, Z={:.1} | Temp: {:.1}°C", 
                        counter, ax, ay, az, gx, gy, gz, temp);
                }
                Err(e) => {
                    rprintln!("RTT Debug: Failed to read MPU6050: {:?}", e);
                }
            }
            // Clear the data ready flag
            drivers::mpu6050_interrupt::mpu6050_clear_data_ready();
        }
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
    
    }
}

// Commented out since Delay::new() handles SysTick configuration
// fn systick_init(cp: &mut cortex_m::Peripherals) {
//     cp.SYST.set_clock_source(SystClkSource::Core);
//     cp.SYST.set_reload(4_000_000); // 1/4 second at 16 MHz
//     cp.SYST.enable_counter();
//     cp.SYST.enable_interrupt();
// }

//button interrupt handler
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn EXTI0_Handler() {
    
    cortex_m::interrupt::free(|_| {
        rprintln!("RTT Debug: Button pressed!"); 
        led_toggle(RED_LED_PORT, RED_LED_PIN);
        led_toggle(GREEN_LED_PORT, GREEN_LED_PIN);
    });

    drivers::button::button_clear_interrupt(USER_BTN_PIN);
}

// MPU6050 interrupt handler (EXTI13 via EXTI15_10)
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn EXTI15_10_Handler() {
    cortex_m::interrupt::free(|_| {
        rprintln!("RTT Debug: MPU6050 data ready interrupt!"); 
        
        // Set the data ready flag
        drivers::mpu6050_interrupt::mpu6050_set_data_ready();
        
        // Clear MPU6050 interrupt (read status register)
        match drivers::mpu6050_interrupt::mpu6050_clear_interrupt() {
            Ok(()) => {},
            Err(_) => rprintln!("RTT Debug: Failed to clear MPU6050 interrupt"),
        }
    });

    // Clear EXTI13 pending interrupt
    if let Some(exti_line) = drivers::exti::ExtiLine::from_pin(board::MPU6050_INT_PIN) {
        drivers::exti::clear_pending_interrupt(exti_line);
    }
}
