#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]

// use cortex_m::peripheral::syst::SystClkSource; // No longer needed
//use board::*;
use drivers::*;
// use cortex_m::delay::Delay; // Not needed in LUDICROUS SPEED MODE!
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

    let _cp = init_cortex_m_peripherals!(); // Still needed for some initialization

    // Initialize RTT for debug output
    rtt_init_print!();
    
    // Initialize I2C for MPU6050 communication
    drivers::i2c::i2c_init();
    
    // Small delay to let I2C settle
    cortex_m::asm::delay(1_000_000);
    
    // Initialize MPU6050 in interrupt-driven mode
    match drivers::mpu6050_interrupt::mpu6050_init_interrupt_driven() {
        Ok(()) => {
            led_on(GREEN_LED_PORT, GREEN_LED_PIN);
            // Allow sensor time to initialize
            cortex_m::asm::delay(5_000_000);
        }
        Err(_e) => {
            led_off(GREEN_LED_PORT, GREEN_LED_PIN);
        }
    }

    // 🚀 LUDICROUS SPEED MODE - No delay needed!
    rprintln!("RTT Debug: Maximum speed mode initialized");
    
    loop {
        // 🚀 LUDICROUS SPEED MODE - NO DELAY!
        // Running at maximum CPU speed limited only by I2C and RTT bandwidth
        
        // Toggle LED very infrequently at maximum speed
        static mut LED_COUNTER: u32 = 0;
        unsafe {
            LED_COUNTER += 1;
            if LED_COUNTER % 10000 == 0 {  // Toggle every ~10k loops 
                led_toggle(RED_LED_PORT, RED_LED_PIN);
            }
        }
        
        // Check MPU6050 interrupt status register directly (polling approach)
        let mut has_new_data = false;
        match drivers::i2c::i2c_read_register(0x68, 0x3A) {
            Ok(int_status) => {
                if int_status & 0x01 != 0 {  // DATA_RDY_INT bit
                    has_new_data = true;
                    // Clear the interrupt by reading the status (per MPU6050 datasheet)
                    let _ = drivers::i2c::i2c_read_register(0x68, 0x3A);
                }
            }
            Err(_) => {
                // I2C error - try recovery
                drivers::i2c::i2c_bus_recovery();
            }
        }
        
        // Check if MPU6050 has new data available (either via interrupt or polling)
        if drivers::mpu6050_interrupt::mpu6050_data_ready() || has_new_data {
            match drivers::mpu6050_interrupt::mpu6050_read_all() {
                Ok(data) => {
                    // Process sensor data in critical section
                    cortex_m::interrupt::free(|_| {
                        let (ax, ay, az) = data.accel.to_g(); // Returns millig (1/1000 g)
                        let (gx, gy, gz) = data.gyro.to_dps(); // Returns milli-degrees/sec
                        let temp_raw = data.temperature;
                        
                        // Display sensor data
                        rprintln!("Accel(mg): X={}, Y={}, Z={}", ax, ay, az);
                        rprintln!("Gyro(mdps): X={}, Y={}, Z={}", gx, gy, gz);
                        rprintln!("Temp: {}", temp_raw);
                    });
                }
                Err(_e) => {
                    // Simple I2C bus recovery on error
                    drivers::i2c::i2c_bus_recovery();
                }
            }
            // Clear the data ready flag
            drivers::mpu6050_interrupt::mpu6050_clear_data_ready();
        }
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    cortex_m::interrupt::free(|_| {
        if let Some(location) = info.location() {
            rprintln!("PANIC at {}:{} - {}", location.file(), location.line(), info.message());
        } else {
            rprintln!("PANIC occurred but no location info available");
        }
    });
    
    loop {
        // Flash red LED to indicate panic
        led_toggle(RED_LED_PORT, RED_LED_PIN);
        cortex_m::asm::delay(1_000_000);
    }
}

// Add a custom hardfault handler for better debugging
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn HardFault() -> ! {
    cortex_m::interrupt::free(|_| {
        rprintln!("HARDFAULT occurred! Check for stack overflow or invalid memory access.");
    });
    
    loop {
        // Flash both LEDs to indicate hardfault
        led_toggle(RED_LED_PORT, RED_LED_PIN);
        led_toggle(GREEN_LED_PORT, GREEN_LED_PIN);
        cortex_m::asm::delay(500_000);
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
    // Simplified interrupt handler to avoid potential issues
    // Set the data ready flag (atomic operation, should be safe)
    drivers::mpu6050_interrupt::mpu6050_set_data_ready();
    
    // Clear EXTI13 pending interrupt first
    if let Some(exti_line) = drivers::exti::ExtiLine::from_pin(board::MPU6050_INT_PIN) {
        drivers::exti::clear_pending_interrupt(exti_line);
    }
    
    // Don't do I2C operations in interrupt handler - do them in main loop
    // This prevents potential stack overflow and timing issues
}
