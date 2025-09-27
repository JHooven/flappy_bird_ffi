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
            
            // Small delay then check if sensor is generating data
            cortex_m::asm::delay(5_000_000); // Give sensor time to start
            
            // Try manual read to see if sensor is working
            match drivers::mpu6050_interrupt::mpu6050_read_all() {
                Ok(_data) => {
                    rprintln!("RTT Debug: Initial sensor test successful - data available");
                }
                Err(e) => {
                    rprintln!("RTT Debug: Initial sensor test failed: {:?}", e);
                }
            }
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
    let mut recovery_count = 0;
    
    loop {
        // Blink LED to show we're alive - delay for 1000ms
        delay.delay_ms(1000u32);
        led_toggle(RED_LED_PORT, RED_LED_PIN);
        
        counter += 1;
        rprintln!("RTT Debug: Loop iteration {}, LED toggled", counter);
        
        // Debug: Every 10 loops, check interrupt pin state and MPU6050 status
        if counter % 10 == 0 {
            // Read PC13 pin state
            let pin_state = drivers::gpio::get_gpio_pin_state(board::MPU6050_INT_PORT, board::MPU6050_INT_PIN);
            
            // Try to read MPU6050 INT_STATUS register
            match drivers::i2c::i2c_read_register(0x68, 0x3A) {
                Ok(int_status) => {
                    rprintln!("RTT Debug: PC13={}, MPU6050_INT_STATUS=0x{:02X}", pin_state, int_status);
                }
                Err(_) => {
                    rprintln!("RTT Debug: PC13={}, Failed to read MPU6050 status", pin_state);
                }
            }
        }
        
        // Check if MPU6050 has new data (interrupt-driven) 
        // Read more frequently after recovery (every 3 loops) or normally every 10 loops
        let read_interval = if recovery_count > 0 && (counter - recovery_count) < 10 { 3 } else { 10 };
        
        if counter % read_interval == 0 && drivers::mpu6050_interrupt::mpu6050_data_ready() {
            rprintln!("RTT Debug: About to read MPU6050 data...");
            
            // Add small delay before I2C operation to avoid bus conflicts
            cortex_m::asm::delay(10_000); // ~0.6ms delay
            
            match drivers::mpu6050_interrupt::mpu6050_read_all() {
                Ok(data) => {
                    rprintln!("RTT Debug: Data read successful, converting...");
                    
                    // Use critical section to prevent interrupt during data processing
                    cortex_m::interrupt::free(|_| {
                        // Use integer versions to avoid floating-point hardfaults
                        let (ax, ay, az) = data.accel.to_g(); // Returns millig (1/1000 g)
                        let (gx, gy, gz) = data.gyro.to_dps(); // Returns milli-degrees/sec
                        let temp_raw = data.temperature;
                        
                        // Split the large rprintln into smaller ones to avoid stack issues
                        // Display in millig and milli-degrees/sec to avoid floating point
                        rprintln!("MPU6050 [{}] Accel(mg): X={}, Y={}, Z={}", counter, ax, ay, az);
                        rprintln!("MPU6050 [{}] Gyro(mdps): X={}, Y={}, Z={}", counter, gx, gy, gz);
                        rprintln!("MPU6050 [{}] Temp_raw: {}", counter, temp_raw);
                    });
                }
                Err(e) => {
                    rprintln!("RTT Debug: Failed to read MPU6050: {:?}", e);
                    
                    // If we get I2C errors, try to recover the bus
                    match e {
                        drivers::mpu6050_interrupt::Mpu6050Error::I2CError(drivers::i2c::I2CError::AddressNack) |
                        drivers::mpu6050_interrupt::Mpu6050Error::I2CError(drivers::i2c::I2CError::DataNack) => {
                            rprintln!("RTT Debug: I2C error detected, attempting bus recovery...");
                            drivers::i2c::i2c_bus_recovery();
                            recovery_count = counter; // Mark when recovery happened
                            
                            // Wait after recovery and test with a simple WHO_AM_I read
                            cortex_m::asm::delay(100_000); // Longer delay after recovery
                            
                            match drivers::i2c::i2c_read_register(0x68, 0x75) {
                                Ok(who_am_i) => {
                                    rprintln!("RTT Debug: I2C recovery test successful, WHO_AM_I = 0x{:02X}", who_am_i);
                                    rprintln!("RTT Debug: Will attempt sensor reads every 3 loops for the next 10 loops");
                                }
                                Err(recovery_err) => {
                                    rprintln!("RTT Debug: I2C recovery test failed: {:?}", recovery_err);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            // Clear the data ready flag
            drivers::mpu6050_interrupt::mpu6050_clear_data_ready();
            rprintln!("RTT Debug: MPU6050 processing complete");
        } else if drivers::mpu6050_interrupt::mpu6050_data_ready() {
            // If we're not reading this loop, still clear the flag to prevent overflow
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
