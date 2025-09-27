# Motion-Controlled Flappy Bird Demo

This embedded Rust project now implements comprehensive motion sensing capabilities using the MPU6050 6-axis IMU sensor. Here's how to use the different motion control modes:

## Motion Control Modes

### 1. **Tilt Control** (Default Mode)
- **How it works**: Tilt the board to control the bird
- **Controls**:
  - Tilt forward (X-axis): Bird moves up
  - Tilt backward: Bird moves down  
  - Tilt left/right (Y-axis): Bird moves horizontally
  - Strong upward tilt: Triggers flap action
- **Best for**: Smooth, analog control like a flight simulator

### 2. **Tap Control**
- **How it works**: Tap or shake the board to make the bird flap
- **Controls**:
  - Quick Z-axis acceleration spike: Flap trigger
  - Tilt left/right: Horizontal movement
  - Tap detection has cooldown to prevent multiple triggers
- **Best for**: Classic flappy bird gameplay with physical tapping

### 3. **Rotation Control**
- **How it works**: Rotate the board using gyroscope input
- **Controls**:
  - Pitch rotation (X-axis): Vertical movement
  - Roll rotation (Y-axis): Horizontal movement
  - Yaw rotation (Z-axis): Flap trigger
  - Very fast rotation: Special action (resets bird position)
- **Best for**: Advanced users who want gyroscopic control

### 4. **Gesture Control**
- **How it works**: Complex motion patterns trigger actions
- **Controls**:
  - Wave gesture (up-down-up): Flap trigger
  - Shake gesture (rapid oscillation): Special action
  - Basic tilt: Movement control
- **Best for**: Fun party mode with gesture recognition

### 5. **Calibrated Tilt Control**
- **How it works**: Auto-calibrating tilt control that learns neutral position
- **Controls**:
  - First 50 samples: Calibration phase (hold board steady)
  - After calibration: Highly sensitive tilt control from neutral
  - Tilt from calibrated neutral: Movement and flap triggers
- **Best for**: Precise control adapted to your preferred holding position

### 6. **Disabled Mode**
- Motion control turned off for traditional button-only gameplay

## Game Controls

### **Button Controls**:
- Press the blue user button to cycle through motion control modes
- Button press cycles: Disabled → Tilt → Tap → Rotation → Gesture → Calibrated → Disabled...

### **Game Physics**:
- **Gravity**: Constant downward force on the bird
- **Flap Strength**: Upward velocity applied when flap is triggered
- **Velocity Damping**: Bird slows down over time (95% velocity retained per frame)
- **Position Bounds**: Bird position clamped to -100 to +100 range
- **Scoring**: Points awarded for flaps and special actions

## RTT Debug Output

The system provides real-time debug information via RTT:

```
Motion Control: Switched to TiltControl
GAME | Mode:TiltControl Bird:(5,-23) Vel:8 Score:15 Flap:false
A:1247,-3891,15823 G:145,-67,234 T:8456
```

### Output Explanation:
- **Mode**: Current motion control mode
- **Bird**: (X,Y) position coordinates  
- **Vel**: Current vertical velocity
- **Score**: Game score (flaps + special actions)
- **Flap**: Whether flap is currently triggered
- **A**: Raw accelerometer readings (X,Y,Z)
- **G**: Raw gyroscope readings (X,Y,Z) 
- **T**: Raw temperature reading

### Calibration Output:
```
Motion Control: Starting calibration for mode CalibratedTilt
CALIBRATING CalibratedTilt: 67% complete
Motion Control: Calibration complete - Neutral: X:1205 Y:-156 Z:-16234
```

## Performance Metrics

- **Sensor Reading Speed**: 105μs per register (78x improvement from original)
- **Complete 6-Axis Data**: Every 1.365ms (13 registers × 105μs)
- **Effective Sampling Rate**: ~730Hz for motion control
- **Motion Processing**: Every main loop iteration for responsive gaming
- **Game Physics Update**: Real-time with motion input integration

## Hardware Setup

1. **STM32F429I-DISCO** development board
2. **MPU6050** connected via I2C1:
   - SDA: PB9 (I2C1_SDA)  
   - SCL: PB8 (I2C1_SCL)
   - INT: PC13 (External interrupt, optional)
   - VCC: 3.3V
   - GND: Ground

## Development Features

- **Bare-metal Rust**: `no_std` environment with direct register access
- **Atomic Operations**: Thread-safe sensor data sharing between interrupt and main loop
- **RTT Debugging**: Real-time debug output without JTAG
- **High-Performance I2C**: Optimized for minimal latency
- **Modular Design**: Motion control separated into dedicated module
- **Multiple Control Schemes**: 6 different interaction modes for various gameplay styles

## Next Steps for Game Development

This motion sensing foundation is ready for:

1. **Obstacle Generation**: Add pipes or obstacles that the bird must navigate
2. **Collision Detection**: Implement boundaries and obstacle collision
3. **Visual Display**: Add LCD/OLED display for visual feedback
4. **Sound Effects**: Add audio feedback for flaps and collisions  
5. **High Score System**: Persistent score storage in flash memory
6. **Multiplayer**: Multiple birds controlled by different motion modes
7. **Level Progression**: Increasing difficulty and new motion challenges

The motion control system provides a solid foundation for creating engaging, physically interactive embedded games!