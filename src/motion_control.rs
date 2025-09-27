//! Motion Control System for Flappy Bird Game
//! 
//! This module provides various motion-based input methods using the MPU6050 6-axis IMU:
//! 1. Tilt Control - Bird follows board tilt angle (X/Y acceleration)
//! 2. Tap Control - Quick Z-axis acceleration spikes trigger flap
//! 3. Rotation Control - Angular velocity controls bird movement
//! 4. Motion Gestures - Complex motion patterns for special actions
//! 5. Calibrated Control - Auto-calibrating motion detection

use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use rtt_target::rprintln;

use crate::{ACCEL_X, ACCEL_Y, ACCEL_Z, GYRO_X, GYRO_Y, GYRO_Z, TEMPERATURE};

// Motion control state
static CONTROL_MODE: AtomicU32 = AtomicU32::new(0);
static CALIBRATION_SAMPLES: AtomicU32 = AtomicU32::new(0);

// Calibration data (neutral position)
static ACCEL_X_NEUTRAL: AtomicI32 = AtomicI32::new(0);
static ACCEL_Y_NEUTRAL: AtomicI32 = AtomicI32::new(0);
static ACCEL_Z_NEUTRAL: AtomicI32 = AtomicI32::new(-16384); // -1g when flat
static GYRO_X_NEUTRAL: AtomicI32 = AtomicI32::new(0);
static GYRO_Y_NEUTRAL: AtomicI32 = AtomicI32::new(0);
static GYRO_Z_NEUTRAL: AtomicI32 = AtomicI32::new(0);

// Motion detection thresholds
const TAP_THRESHOLD: i32 = 8000;    // Z-axis acceleration spike
const TILT_THRESHOLD: i32 = 4000;   // X/Y tilt sensitivity  
const ROTATION_THRESHOLD: i32 = 5000; // Angular velocity threshold
const CALIBRATION_SAMPLES_NEEDED: u32 = 50;

#[derive(Debug, Copy, Clone)]
pub enum MotionControlMode {
    Disabled = 0,
    TiltControl = 1,      // Bird follows board tilt
    TapControl = 2,       // Tap to flap
    RotationControl = 3,  // Gyroscope steering
    GestureControl = 4,   // Complex gestures
    CalibratedTilt = 5,   // Auto-calibrating tilt control
}

#[derive(Debug, Copy, Clone)]
pub struct MotionState {
    pub flap_trigger: bool,      // Should bird flap?
    pub vertical_input: i32,     // Up/down movement (-100 to +100)
    pub horizontal_input: i32,   // Left/right movement (-100 to +100) 
    pub special_action: bool,    // Special game action triggered
    pub calibrating: bool,       // Currently calibrating
}

#[derive(Debug, Copy, Clone)]
pub struct SensorReading {
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub temperature: i16,
}

impl MotionControlMode {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::TiltControl,
            2 => Self::TapControl,
            3 => Self::RotationControl,
            4 => Self::GestureControl,
            5 => Self::CalibratedTilt,
            _ => Self::Disabled,
        }
    }
}

/// Initialize motion control system
pub fn motion_init() {
    CONTROL_MODE.store(MotionControlMode::Disabled as u32, Ordering::Relaxed);
    CALIBRATION_SAMPLES.store(0, Ordering::Relaxed);
    rprintln!("Motion Control: Initialized");
}

/// Set the current motion control mode
pub fn motion_set_mode(mode: MotionControlMode) {
    CONTROL_MODE.store(mode as u32, Ordering::Relaxed);
    
    // Reset calibration when switching modes
    if matches!(mode, MotionControlMode::CalibratedTilt) {
        CALIBRATION_SAMPLES.store(0, Ordering::Relaxed);
        rprintln!("Motion Control: Starting calibration for mode {:?}", mode);
    } else {
        rprintln!("Motion Control: Set mode to {:?}", mode);
    }
}

/// Get current sensor readings from atomic storage
pub fn motion_get_sensors() -> SensorReading {
    SensorReading {
        accel_x: ACCEL_X.load(Ordering::Relaxed) as i16,
        accel_y: ACCEL_Y.load(Ordering::Relaxed) as i16,
        accel_z: ACCEL_Z.load(Ordering::Relaxed) as i16,
        gyro_x: GYRO_X.load(Ordering::Relaxed) as i16,
        gyro_y: GYRO_Y.load(Ordering::Relaxed) as i16,
        gyro_z: GYRO_Z.load(Ordering::Relaxed) as i16,
        temperature: TEMPERATURE.load(Ordering::Relaxed) as i16,
    }
}

/// Process motion input and return game control state
pub fn motion_process() -> MotionState {
    let mode = MotionControlMode::from_u32(CONTROL_MODE.load(Ordering::Relaxed));
    let sensors = motion_get_sensors();
    
    match mode {
        MotionControlMode::Disabled => MotionState {
            flap_trigger: false,
            vertical_input: 0,
            horizontal_input: 0,
            special_action: false,
            calibrating: false,
        },
        
        MotionControlMode::TiltControl => process_tilt_control(sensors),
        MotionControlMode::TapControl => process_tap_control(sensors),
        MotionControlMode::RotationControl => process_rotation_control(sensors),
        MotionControlMode::GestureControl => process_gesture_control(sensors),
        MotionControlMode::CalibratedTilt => process_calibrated_tilt_control(sensors),
    }
}

/// Tilt Control: Bird follows board tilt angle
fn process_tilt_control(sensors: SensorReading) -> MotionState {
    // Use X-axis for vertical movement (pitch)
    // Use Y-axis for horizontal movement (roll)
    
    let vertical_raw = sensors.accel_x as i32;
    let horizontal_raw = sensors.accel_y as i32;
    
    // Scale to -100 to +100 range
    let vertical_input = (vertical_raw * 100) / 16384; // ±2g range
    let horizontal_input = (horizontal_raw * 100) / 16384;
    
    // Clamp to valid range
    let vertical_input = vertical_input.max(-100).min(100);
    let horizontal_input = horizontal_input.max(-100).min(100);
    
    // Flap trigger on strong upward tilt
    let flap_trigger = vertical_input > 50;
    
    MotionState {
        flap_trigger,
        vertical_input,
        horizontal_input,
        special_action: false,
        calibrating: false,
    }
}

/// Tap Control: Quick Z-axis acceleration spikes trigger flap
fn process_tap_control(sensors: SensorReading) -> MotionState {
    // Static variables to track previous readings
    static mut PREV_ACCEL_Z: i16 = 0;
    static mut TAP_COOLDOWN: u32 = 0;
    
    unsafe {
        // Decrease cooldown
        if TAP_COOLDOWN > 0 {
            TAP_COOLDOWN -= 1;
        }
        
        // Calculate acceleration change (derivative)
        let accel_change = (sensors.accel_z as i32) - (PREV_ACCEL_Z as i32);
        PREV_ACCEL_Z = sensors.accel_z;
        
        // Detect tap (sudden acceleration spike)
        let flap_trigger = accel_change.abs() > TAP_THRESHOLD && TAP_COOLDOWN == 0;
        
        if flap_trigger {
            TAP_COOLDOWN = 10; // Prevent multiple triggers
        }
        
        // Use tilt for horizontal movement
        let horizontal_input = ((sensors.accel_y as i32) * 100) / 16384;
        let horizontal_input = horizontal_input.max(-100).min(100);
        
        MotionState {
            flap_trigger,
            vertical_input: 0,
            horizontal_input,
            special_action: false,
            calibrating: false,
        }
    }
}

/// Rotation Control: Angular velocity controls bird movement
fn process_rotation_control(sensors: SensorReading) -> MotionState {
    // Use gyroscope for control
    let vertical_raw = sensors.gyro_x as i32;   // Pitch rate
    let horizontal_raw = sensors.gyro_y as i32; // Roll rate
    let rotation_raw = sensors.gyro_z as i32;   // Yaw rate
    
    // Scale gyroscope readings (±250°/s range for default config)
    let vertical_input = (vertical_raw * 100) / 32768;
    let horizontal_input = (horizontal_raw * 100) / 32768;
    
    // Clamp to valid range
    let vertical_input = vertical_input.max(-100).min(100);
    let horizontal_input = horizontal_input.max(-100).min(100);
    
    // Flap trigger on strong rotation
    let flap_trigger = rotation_raw.abs() > ROTATION_THRESHOLD;
    
    // Special action on very fast rotation
    let special_action = rotation_raw.abs() > ROTATION_THRESHOLD * 2;
    
    MotionState {
        flap_trigger,
        vertical_input,
        horizontal_input,
        special_action,
        calibrating: false,
    }
}

/// Gesture Control: Complex motion patterns
fn process_gesture_control(sensors: SensorReading) -> MotionState {
    // Static variables for gesture detection
    static mut GESTURE_BUFFER: [i16; 8] = [0; 8]; // Circular buffer for accel_z
    static mut BUFFER_INDEX: usize = 0;
    
    unsafe {
        // Update circular buffer
        GESTURE_BUFFER[BUFFER_INDEX] = sensors.accel_z;
        BUFFER_INDEX = (BUFFER_INDEX + 1) % 8;
        
        // Detect wave gesture (up-down-up pattern) - pass by value instead of reference
        let gesture_buffer_copy = GESTURE_BUFFER;
        let wave_detected = detect_wave_gesture(gesture_buffer_copy);
        
        // Detect shake gesture (rapid back-and-forth) - pass by value instead of reference
        let shake_detected = detect_shake_gesture(gesture_buffer_copy);
        
        // Use basic tilt for movement
        let vertical_input = ((sensors.accel_x as i32) * 100) / 16384;
        let horizontal_input = ((sensors.accel_y as i32) * 100) / 16384;
        let vertical_input = vertical_input.max(-100).min(100);
        let horizontal_input = horizontal_input.max(-100).min(100);
        
        MotionState {
            flap_trigger: wave_detected,
            vertical_input,
            horizontal_input,
            special_action: shake_detected,
            calibrating: false,
        }
    }
}

/// Calibrated Tilt Control: Auto-calibrating motion detection
fn process_calibrated_tilt_control(sensors: SensorReading) -> MotionState {
    let samples = CALIBRATION_SAMPLES.load(Ordering::Relaxed);
    
    if samples < CALIBRATION_SAMPLES_NEEDED {
        // Calibration phase - collect neutral position
        let new_samples = samples + 1;
        CALIBRATION_SAMPLES.store(new_samples, Ordering::Relaxed);
        
        // Running average for calibration
        let current_x = ACCEL_X_NEUTRAL.load(Ordering::Relaxed);
        let current_y = ACCEL_Y_NEUTRAL.load(Ordering::Relaxed);
        let current_z = ACCEL_Z_NEUTRAL.load(Ordering::Relaxed);
        
        let new_x = ((current_x * samples as i32) + sensors.accel_x as i32) / new_samples as i32;
        let new_y = ((current_y * samples as i32) + sensors.accel_y as i32) / new_samples as i32;
        let new_z = ((current_z * samples as i32) + sensors.accel_z as i32) / new_samples as i32;
        
        ACCEL_X_NEUTRAL.store(new_x, Ordering::Relaxed);
        ACCEL_Y_NEUTRAL.store(new_y, Ordering::Relaxed);
        ACCEL_Z_NEUTRAL.store(new_z, Ordering::Relaxed);
        
        if new_samples == CALIBRATION_SAMPLES_NEEDED {
            rprintln!("Motion Control: Calibration complete - Neutral: X:{} Y:{} Z:{}", new_x, new_y, new_z);
        }
        
        return MotionState {
            flap_trigger: false,
            vertical_input: 0,
            horizontal_input: 0,
            special_action: false,
            calibrating: true,
        };
    }
    
    // Normal operation with calibration offset
    let neutral_x = ACCEL_X_NEUTRAL.load(Ordering::Relaxed);
    let neutral_y = ACCEL_Y_NEUTRAL.load(Ordering::Relaxed);
    
    let vertical_offset = (sensors.accel_x as i32) - neutral_x;
    let horizontal_offset = (sensors.accel_y as i32) - neutral_y;
    
    // Scale to -100 to +100 range with increased sensitivity
    let vertical_input = (vertical_offset * 100) / 8192;  // More sensitive than basic tilt
    let horizontal_input = (horizontal_offset * 100) / 8192;
    
    // Clamp to valid range  
    let vertical_input = vertical_input.max(-100).min(100);
    let horizontal_input = horizontal_input.max(-100).min(100);
    
    // Flap trigger on significant upward tilt from neutral
    let flap_trigger = vertical_offset > TILT_THRESHOLD;
    
    MotionState {
        flap_trigger,
        vertical_input,
        horizontal_input,
        special_action: false,
        calibrating: false,
    }
}

/// Detect wave gesture (up-down-up pattern in Z-axis)
fn detect_wave_gesture(buffer: [i16; 8]) -> bool {
    // Simple pattern detection: look for peak-valley-peak
    let mut peaks = 0;
    let mut valleys = 0;
    
    for i in 1..7 {
        if buffer[i] > buffer[i-1] && buffer[i] > buffer[i+1] && buffer[i] > 8000 {
            peaks += 1;
        }
        if buffer[i] < buffer[i-1] && buffer[i] < buffer[i+1] && buffer[i] < -8000 {
            valleys += 1;
        }
    }
    
    peaks >= 2 && valleys >= 1
}

/// Detect shake gesture (rapid oscillation)
fn detect_shake_gesture(buffer: [i16; 8]) -> bool {
    // Count direction changes
    let mut direction_changes = 0;
    
    for i in 1..7 {
        let prev_slope = buffer[i] - buffer[i-1];
        let curr_slope = buffer[i+1] - buffer[i];
        
        // Sign change indicates direction change
        if (prev_slope > 0 && curr_slope < 0) || (prev_slope < 0 && curr_slope > 0) {
            if prev_slope.abs() > 1000 || curr_slope.abs() > 1000 {
                direction_changes += 1;
            }
        }
    }
    
    direction_changes >= 3 // Rapid back-and-forth motion
}

/// Get current motion control mode
pub fn motion_get_mode() -> MotionControlMode {
    MotionControlMode::from_u32(CONTROL_MODE.load(Ordering::Relaxed))
}

/// Check if system is currently calibrating
pub fn motion_is_calibrating() -> bool {
    let mode = motion_get_mode();
    matches!(mode, MotionControlMode::CalibratedTilt) && 
    CALIBRATION_SAMPLES.load(Ordering::Relaxed) < CALIBRATION_SAMPLES_NEEDED
}

/// Get calibration progress (0-100)
pub fn motion_get_calibration_progress() -> u32 {
    let samples = CALIBRATION_SAMPLES.load(Ordering::Relaxed);
    (samples * 100) / CALIBRATION_SAMPLES_NEEDED
}

/// Force recalibration for calibrated modes
pub fn motion_recalibrate() {
    CALIBRATION_SAMPLES.store(0, Ordering::Relaxed);
    rprintln!("Motion Control: Recalibration started");
}