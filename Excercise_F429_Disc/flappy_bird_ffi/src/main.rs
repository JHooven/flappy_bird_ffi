// #![no_std]
// #![no_main]
// #![allow(dead_code)]

// mod game;
// mod display;
// mod assets;
// mod config;
// mod color;
// mod obstacle;
// mod player;
// mod spi_adapter;
// //mod i3g4250d;

// //use dummy_input::DummyInputDevice;
// use core::panic::PanicInfo;
// use game::Game;
// use crate::config::Coord;
// use crate::game::InputDevice;

// unsafe extern "C" {
//     fn c_main();
//     static ili9341_display_driver: display::DisplayDriver; 
// }

// pub struct DummyInputDevice;

// impl DummyInputDevice {
//     pub fn new() -> Self {
//         DummyInputDevice
//     }
// }

// impl InputDevice for DummyInputDevice {
//     type Error = ();

//     fn init(&mut self) -> Result<(), Self::Error> {
//         Ok(())
//     }

//     fn is_tap(&mut self, _y_min: Coord, _y_max: Coord) -> Result<(Coord, bool), Self::Error> {
//         Ok((100, false)) // Always return no tap
//     }
// }


// #[unsafe(no_mangle)]
// pub extern "C" fn main() -> ! {
//     unsafe {
//         c_main();
//        display::register_driver(&ili9341_display_driver);
//     }

//     display::init();
// //let input_device = MyInputDevice::new(); // your input device constructor
// //let mut game = Game::init(input_device).expect("Failed to init game");
// //let mut game = Game::init().expect("Failed to init game");
//     // let sensor_input = mpu6050::SensorInput::new(MPU6050_DEV_ADDR, i2c);

// let input = DummyInputDevice::new();
// let mut game_instance = Game::init(input).unwrap();

//     // let mut game = game_init(sensor_input).expect("Game init failed");

//     //let mut game = Game::init();

//     loop {
//         game_instance.update();
//     }
// }



// #[panic_handler]
// fn panic(_: &PanicInfo) -> ! {
//     loop {}
// }



#![no_std]
#![no_main]
#![allow(dead_code)]

mod game;
mod display;
mod assets;
mod config;
mod color;
mod obstacle;
mod player;
mod spi_adapter;
// mod i3g4250d;

use core::panic::PanicInfo;
use crate::config::Coord;
use crate::game::InputDevice;
use game::Game;

// External symbols defined in C
extern "C" {
    fn c_main();
    static ili9341_display_driver: display::DisplayDriver;
}

// Dummy input device for testing
pub struct DummyInputDevice;

impl DummyInputDevice {
    pub fn new() -> Self {
        DummyInputDevice
    }
}

impl InputDevice for DummyInputDevice {
    type Error = ();

    fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn is_tap(&mut self, _y_min: Coord, _y_max: Coord) -> Result<(Coord, bool), Self::Error> {
        // Always return "no tap" and dummy X-coordinate
        Ok((100, false))
    }
}

// Entry point for Rust (called after `c_main`)
#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Call low-level board initialization from C (clocks, GPIO, etc.)
        c_main();
        // Register your ILI9341 display driver provided by the C side
        display::register_driver(&ili9341_display_driver);
    }

    display::init(); // Initialize display module

    let input = DummyInputDevice::new();
    let mut game_instance = Game::init(input).expect("Failed to initialize game");

    // Game loop
    loop {
        game_instance.update();
    }
}

// Panic handler required for `#![no_std]`
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
