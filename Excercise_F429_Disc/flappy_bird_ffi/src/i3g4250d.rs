// // Reads gyroscope sensor data from I3G4250D over SPI and maps movement to screen Y position.
// //
// // Steps:
// // 1. Reads raw gyro data
// // 2. Computes angular roll using atan2 from Y and Z axis
// // 3. Maps roll to y-coordinate between y_min and y_max

// use crate::{config::Coord, game::InputDevice};
// use rtt_target::{rtt_init_print, rprintln};

// use embedded_hal::digital::v2::OutputPin;
// use embedded_hal::blocking::spi::Transfer;

// //  Import the driver from the external crate
// //use crate::i3g4250d::I3g4250d;

// pub struct SensorInput<SPI, CS>
// where
//     SPI: Transfer<u8>,
//     CS: OutputPin,
// {
//     pub gyro: I3g4250d<SPI, CS>,
// }

// impl<SPI, CS> SensorInput<SPI, CS>
// where
//     SPI: Transfer<u8>,
//     CS: OutputPin,
// {
//     pub fn new(spi: SPI, cs: CS) -> Self {
//         Self {
//             gyro: I3g4250d::new(spi, cs),
//         }
//     }
// }

// impl<SPI, CS> InputDevice for SensorInput<SPI, CS>
// where
//     SPI: Transfer<u8>,
//     CS: OutputPin,
// {
//     type Error = SPI::Error;

//     fn init(&mut self) -> Result<(), Self::Error> {
//         #[cfg(feature = "log")]
//         rtt_init_print!();

//         self.gyro.init().map_err(|_| {
//             // You can replace this with a more graceful error conversion if needed
//             panic!("Gyroscope initialization failed")
//         })?;

//         Ok(())
//     }

//     #[cfg(feature = "log")]
//     fn log_data(&mut self) {
//         if let Ok(gyro) = self.gyro.read_gyro_raw() {
//             rprintln!("gyro: X={}, Y={}, Z={}", gyro[0], gyro[1], gyro[2]);
//         }
//     }

//     fn is_tap(&mut self, y_min: Coord, y_max: Coord) -> Result<(Coord, bool), Self::Error> {
//         let gyro_data = self.gyro.read_gyro_raw().map_err(|_| {
//             panic!("Gyro read failed")
//         })?;

//         let roll_angle = get_roll_angle(gyro_data[1] as f32, gyro_data[2] as f32);

//         let roll_min = -200.0;
//         let roll_max = 200.0;

//         let new_y = map_roll_angle_to_y(roll_angle, roll_min, roll_max, y_min, y_max);

//         Ok((new_y as Coord, true))
//     }
// }

// pub fn get_roll_angle(y: f32, z: f32) -> f32 {
//     libm::atan2f(y, z) * (180.0_f32 / core::f32::consts::PI)
// }

// fn map_roll_angle_to_y(
//     roll: f32,
//     roll_min: f32,
//     roll_max: f32,
//     y_min: Coord,
//     y_max: Coord,
// ) -> Coord {
//     let clamped_roll = roll.clamp(roll_min, roll_max);
//     let normalized = (clamped_roll - roll_min) / (roll_max - roll_min);
//     let y_range = (y_max - y_min) as f32;
//     let mapped = y_min as f32 + normalized * y_range;
//     (mapped + 0.5) as Coord
// }
