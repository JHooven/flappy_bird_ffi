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
use core::sync::atomic::{AtomicU32, AtomicI32, Ordering};

use crate::board::{
    GREEN_LED_PIN, GREEN_LED_PORT, RED_LED_PIN, RED_LED_PORT, USER_BTN_PIN, USER_BTN_PORT,
};

// Removed static PERIPHERALS to avoid Sync error
//use crate::{button::{button_configure_interrupt, button_init}, led::{led_init, led_off}};

// Interrupt throttling counter - only process every Nth interrupt
static MPU6050_INTERRUPT_COUNT: AtomicU32 = AtomicU32::new(0);
const INTERRUPT_SKIP_COUNT: u32 = 10; // Process every 10th interrupt

// Rotating sensor read system
static SENSOR_ROTATION: AtomicU32 = AtomicU32::new(0);

// Sensor data storage - using AtomicI32 for thread safety (storing i16 values)
static ACCEL_X: AtomicI32 = AtomicI32::new(0);
static ACCEL_Y: AtomicI32 = AtomicI32::new(0);
static ACCEL_Z: AtomicI32 = AtomicI32::new(0);
static GYRO_X: AtomicI32 = AtomicI32::new(0);
static GYRO_Y: AtomicI32 = AtomicI32::new(0);
static GYRO_Z: AtomicI32 = AtomicI32::new(0);
static TEMPERATURE: AtomicI32 = AtomicI32::new(0);

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
    
    // Much longer delay to let I2C bus fully stabilize after power-on
    cortex_m::asm::delay(10_000_000); // ~55ms at 180MHz
    
    rprintln!("RTT Debug: I2C initialized, attempting MPU6050 init...");
    
    // Try simple MPU6050 initialization - just wake it up
    rprintln!("RTT Debug: Attempting simple MPU6050 wake-up...");
    
    match drivers::i2c::i2c_write_register(0x68, 0x6B, 0x00) { // PWR_MGMT_1: wake up
        Ok(()) => {
            led_on(GREEN_LED_PORT, GREEN_LED_PIN);
            rprintln!("RTT Debug: MPU6050 wake-up successful");
            cortex_m::asm::delay(5_000_000); // ~28ms at 180MHz
        }
        Err(_) => {
            led_off(GREEN_LED_PORT, GREEN_LED_PIN);
            rprintln!("RTT Debug: MPU6050 wake-up FAILED");
            cortex_m::asm::delay(5_000_000);
        }
    }

    // Enable DWT cycle counter for precise timing measurements
    let dwt = unsafe { &*cortex_m::peripheral::DWT::PTR };
    unsafe {
        // Enable DWT cycle counter
        dwt.ctrl.modify(|r| r | 1);
        dwt.cyccnt.write(0);
    }
    
    // Timing measurement array: [before_time, after_time] for 10 measurements
    let mut timing_array = [[0u32; 2]; 10];
    let mut timing_index = 0;
    let mut measurements_complete = false;
    
    rprintln!("RTT Debug: Testing basic I2C communication...");
    
    // Test basic I2C communication before starting main loop
    match drivers::i2c::i2c_read_register(0x68, 0x75) {  // WHO_AM_I register
        Ok(who_am_i) => {
            rprintln!("RTT Debug: MPU6050 WHO_AM_I = 0x{:02X} (expected 0x68)", who_am_i);
        }
        Err(_) => {
            rprintln!("RTT Debug: Failed to read WHO_AM_I register - I2C communication failed");
        }
    }
    
    // Test reading a simple data register
    match drivers::i2c::i2c_read_register(0x68, 0x3B) {  // ACCEL_XOUT_H register
        Ok(accel_x_h) => {
            rprintln!("RTT Debug: ACCEL_XOUT_H = 0x{:02X}", accel_x_h);
        }
        Err(_) => {
            rprintln!("RTT Debug: Failed to read ACCEL_XOUT_H register");
        }
    }
    
    rprintln!("RTT Debug: Interrupt-driven mode with throttling initialized (skip every {} interrupts)", INTERRUPT_SKIP_COUNT);
    
    loop {
        // Faster loop since we're throttling interrupts
        cortex_m::asm::delay(900_000); // ~50ms at 180MHz - faster with interrupt throttling
        
        // Toggle LED every loop (every 100ms)
        led_toggle(RED_LED_PORT, RED_LED_PIN);
        
        // Simple polling approach - read sensor every few loops
        static MAIN_LOOP_COUNT: AtomicU32 = AtomicU32::new(0);
        let loop_count = MAIN_LOOP_COUNT.fetch_add(1, Ordering::Relaxed);
        
        // Check interrupt status for debug
        let interrupt_count = MPU6050_INTERRUPT_COUNT.load(Ordering::Relaxed);
        
        if loop_count % 20 == 0 {
            rprintln!("MAIN: Loop {}, interrupts: {} (polling mode)", loop_count, interrupt_count);
        }
        
        // Read sensor data every 5th loop (every ~250ms) - no I2C status register polling
        if loop_count % 5 == 0 {
            
            // Measure timing if we haven't completed 10 measurements yet
            if !measurements_complete && timing_index < 10 {
                // Record time BEFORE simple register read
                let before_time = dwt.cyccnt.read();
                
                // Rotating sensor read system for maximum speed
                let rotation_index = SENSOR_ROTATION.fetch_add(1, Ordering::Relaxed);
                
                let (register_addr, sensor_name) = match rotation_index % 13 {
                    0 => (0x3B, "ACCEL_X_H"),  // Accelerometer X high byte
                    1 => (0x3C, "ACCEL_X_L"),  // Accelerometer X low byte  
                    2 => (0x3D, "ACCEL_Y_H"),  // Accelerometer Y high byte
                    3 => (0x3E, "ACCEL_Y_L"),  // Accelerometer Y low byte
                    4 => (0x3F, "ACCEL_Z_H"),  // Accelerometer Z high byte
                    5 => (0x40, "ACCEL_Z_L"),  // Accelerometer Z low byte
                    6 => (0x41, "TEMP_H"),     // Temperature high byte
                    7 => (0x43, "GYRO_X_H"),   // Gyroscope X high byte
                    8 => (0x44, "GYRO_X_L"),   // Gyroscope X low byte
                    9 => (0x45, "GYRO_Y_H"),   // Gyroscope Y high byte
                    10 => (0x46, "GYRO_Y_L"),  // Gyroscope Y low byte
                    11 => (0x47, "GYRO_Z_H"),  // Gyroscope Z high byte
                    12 => (0x48, "GYRO_Z_L"),  // Gyroscope Z low byte
                    _ => (0x3B, "ACCEL_X_H"),  // Fallback
                };
                
                match drivers::i2c::i2c_read_register(0x68, register_addr) {
                    Ok(register_value) => {
                        // Record time AFTER register read
                        let after_time = dwt.cyccnt.read();
                        
                        // Store timing measurement
                        timing_array[timing_index][0] = before_time;
                        timing_array[timing_index][1] = after_time;
                        timing_index += 1;
                        
                        // Update the appropriate sensor data storage
                        match rotation_index % 13 {
                            0 => {
                                // Update high byte, preserve low byte for ACCEL X
                                let current_low = (ACCEL_X.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                ACCEL_X.store(new_value as i32, Ordering::Relaxed);
                            }
                            1 => {
                                // Update low byte, preserve high byte for ACCEL X  
                                let current_high = (ACCEL_X.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                ACCEL_X.store(new_value as i32, Ordering::Relaxed);
                            }
                            2 => {
                                // Update high byte for ACCEL Y
                                let current_low = (ACCEL_Y.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                ACCEL_Y.store(new_value as i32, Ordering::Relaxed);
                            }
                            3 => {
                                // Update low byte for ACCEL Y
                                let current_high = (ACCEL_Y.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                ACCEL_Y.store(new_value as i32, Ordering::Relaxed);
                            }
                            4 => {
                                // Update high byte for ACCEL Z
                                let current_low = (ACCEL_Z.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                ACCEL_Z.store(new_value as i32, Ordering::Relaxed);
                            }
                            5 => {
                                // Update low byte for ACCEL Z
                                let current_high = (ACCEL_Z.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                ACCEL_Z.store(new_value as i32, Ordering::Relaxed);
                            }
                            6 => {
                                // Update temperature high byte
                                TEMPERATURE.store((register_value as i32) << 8, Ordering::Relaxed);
                            }
                            7 => {
                                // Update high byte for GYRO X
                                let current_low = (GYRO_X.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                GYRO_X.store(new_value as i32, Ordering::Relaxed);
                            }
                            8 => {
                                // Update low byte for GYRO X
                                let current_high = (GYRO_X.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                GYRO_X.store(new_value as i32, Ordering::Relaxed);
                            }
                            9 => {
                                // Update high byte for GYRO Y
                                let current_low = (GYRO_Y.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                GYRO_Y.store(new_value as i32, Ordering::Relaxed);
                            }
                            10 => {
                                // Update low byte for GYRO Y
                                let current_high = (GYRO_Y.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                GYRO_Y.store(new_value as i32, Ordering::Relaxed);
                            }
                            11 => {
                                // Update high byte for GYRO Z
                                let current_low = (GYRO_Z.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                GYRO_Z.store(new_value as i32, Ordering::Relaxed);
                            }
                            12 => {
                                // Update low byte for GYRO Z
                                let current_high = (GYRO_Z.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                GYRO_Z.store(new_value as i32, Ordering::Relaxed);
                            }
                            _ => {}
                        }
                        
                        // Process sensor data in critical section
                        cortex_m::interrupt::free(|_| {
                            // Output current register reading with timing info
                            let cycles = after_time.wrapping_sub(before_time);
                            let microseconds = cycles / 180; // 180MHz = 180 cycles per microsecond
                            rprintln!("{}: 0x{:02X} Time:{}μs", sensor_name, register_value, microseconds);
                        });
                        
                        // Check if we've completed 10 measurements
                        if timing_index >= 10 {
                            measurements_complete = true;
                            
                            // Calculate average timing
                            let mut total_cycles = 0u32;
                            for i in 0..10 {
                                let diff = timing_array[i][1].wrapping_sub(timing_array[i][0]);
                                total_cycles += diff;
                            }
                            let avg_cycles = total_cycles / 10;
                            let avg_microseconds = avg_cycles / 180;
                            
                            // Get interrupt statistics
                            let total_interrupts = MPU6050_INTERRUPT_COUNT.load(Ordering::Relaxed);
                            
                            rprintln!("=== TIMING ANALYSIS COMPLETE ===");
                            rprintln!("Average mpu6050_read_all() time: {}μs ({} cycles)", avg_microseconds, avg_cycles);
                            rprintln!("Interrupt throttling: {} total interrupts, processing every {}th", total_interrupts, INTERRUPT_SKIP_COUNT);
                            rprintln!("Individual measurements (μs):");
                            for i in 0..10 {
                                let diff = timing_array[i][1].wrapping_sub(timing_array[i][0]);
                                let us = diff / 180;
                                rprintln!("  Measurement {}: {}μs", i + 1, us);
                            }
                        }
                    }
                    _ => {
                        // Any I2C error in the tuple - try recovery
                        drivers::i2c::i2c_bus_recovery();
                    }
                }
            } else {
                // Normal operation with rotating sensor reads for full data
                let rotation_index = SENSOR_ROTATION.fetch_add(1, Ordering::Relaxed);
                
                let register_addr = match rotation_index % 13 {
                    0 => 0x3B,  // ACCEL_X_H
                    1 => 0x3C,  // ACCEL_X_L
                    2 => 0x3D,  // ACCEL_Y_H
                    3 => 0x3E,  // ACCEL_Y_L
                    4 => 0x3F,  // ACCEL_Z_H
                    5 => 0x40,  // ACCEL_Z_L
                    6 => 0x41,  // TEMP_H
                    7 => 0x43,  // GYRO_X_H
                    8 => 0x44,  // GYRO_X_L
                    9 => 0x45,  // GYRO_Y_H
                    10 => 0x46, // GYRO_Y_L
                    11 => 0x47, // GYRO_Z_H
                    12 => 0x48, // GYRO_Z_L
                    _ => 0x3B,  // ACCEL_X_H
                };
                
                match drivers::i2c::i2c_read_register(0x68, register_addr) {
                    Ok(register_value) => {
                        // Update sensor data and show complete 6-axis + temperature data
                        match rotation_index % 13 {
                            0 => {
                                let current_low = (ACCEL_X.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                ACCEL_X.store(new_value as i32, Ordering::Relaxed);
                            }
                            1 => {
                                let current_high = (ACCEL_X.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                ACCEL_X.store(new_value as i32, Ordering::Relaxed);
                            }
                            2 => {
                                let current_low = (ACCEL_Y.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                ACCEL_Y.store(new_value as i32, Ordering::Relaxed);
                            }
                            3 => {
                                let current_high = (ACCEL_Y.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                ACCEL_Y.store(new_value as i32, Ordering::Relaxed);
                            }
                            4 => {
                                let current_low = (ACCEL_Z.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                ACCEL_Z.store(new_value as i32, Ordering::Relaxed);
                            }
                            5 => {
                                let current_high = (ACCEL_Z.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                ACCEL_Z.store(new_value as i32, Ordering::Relaxed);
                            }
                            6 => {
                                TEMPERATURE.store((register_value as i32) << 8, Ordering::Relaxed);
                            }
                            7 => {
                                let current_low = (GYRO_X.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                GYRO_X.store(new_value as i32, Ordering::Relaxed);
                            }
                            8 => {
                                let current_high = (GYRO_X.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                GYRO_X.store(new_value as i32, Ordering::Relaxed);
                            }
                            9 => {
                                let current_low = (GYRO_Y.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                GYRO_Y.store(new_value as i32, Ordering::Relaxed);
                            }
                            10 => {
                                let current_high = (GYRO_Y.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                GYRO_Y.store(new_value as i32, Ordering::Relaxed);
                            }
                            11 => {
                                let current_low = (GYRO_Z.load(Ordering::Relaxed) & 0xFF) as i16;
                                let new_value = ((register_value as i16) << 8) | current_low;
                                GYRO_Z.store(new_value as i32, Ordering::Relaxed);
                            }
                            12 => {
                                let current_high = (GYRO_Z.load(Ordering::Relaxed) & 0xFF00) as i16;
                                let new_value = current_high | (register_value as i16);
                                GYRO_Z.store(new_value as i32, Ordering::Relaxed);
                                
                                // Every time we complete the full rotation cycle, show all sensor data
                                let ax = ACCEL_X.load(Ordering::Relaxed) as i16;
                                let ay = ACCEL_Y.load(Ordering::Relaxed) as i16;
                                let az = ACCEL_Z.load(Ordering::Relaxed) as i16;
                                let gx = GYRO_X.load(Ordering::Relaxed) as i16;
                                let gy = GYRO_Y.load(Ordering::Relaxed) as i16;
                                let gz = GYRO_Z.load(Ordering::Relaxed) as i16;
                                let temp = TEMPERATURE.load(Ordering::Relaxed) as i16;
                                
                                cortex_m::interrupt::free(|_| {
                                    rprintln!("A:{},{},{} G:{},{},{} T:{}", ax, ay, az, gx, gy, gz, temp);
                                });
                            }
                            _ => {}
                        }
                    }
                    Err(_e) => {
                        // Simple I2C bus recovery on error
                        drivers::i2c::i2c_bus_recovery();
                    }
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

// MPU6050 interrupt handler (EXTI13 via EXTI15_10) with throttling
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn EXTI15_10_Handler() {
    // Increment interrupt counter atomically
    let count = MPU6050_INTERRUPT_COUNT.fetch_add(1, Ordering::Relaxed);
    
    // Debug: Show interrupt activity (only every 50th to avoid spam)
    if count % 50 == 0 {
        cortex_m::interrupt::free(|_| {
            rprintln!("INT: {} interrupts received", count);
        });
    }
    
    // Only process every Nth interrupt to throttle frequency
    if count % INTERRUPT_SKIP_COUNT == 0 {
        // Set the data ready flag (atomic operation, should be safe)
        drivers::mpu6050_interrupt::mpu6050_set_data_ready();
        
        cortex_m::interrupt::free(|_| {
            rprintln!("INT: Setting data ready flag (count: {})", count);
        });
    }
    
    // Always clear EXTI13 pending interrupt to prevent handler re-entry
    if let Some(exti_line) = drivers::exti::ExtiLine::from_pin(board::MPU6050_INT_PIN) {
        drivers::exti::clear_pending_interrupt(exti_line);
    }
    
    // Don't do I2C operations in interrupt handler - do them in main loop
    // This prevents potential stack overflow and timing issues
}
