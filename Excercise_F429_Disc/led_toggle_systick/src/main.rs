#![no_std]
#![no_main]
#![allow(dead_code)]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt:: {
    entry,
    exception
};

use panic_halt as _;
use cortex_m::peripheral::Peripherals;
//use cortex_m::peripheral::syst;


mod led;
mod gpio;
mod mcu;
mod reg;
mod board;


#[entry]
#[allow(clippy::empty_loop)]
fn main() -> ! {
    led::led_init(board::BLUE_LED_PORT, board::BLUE_LED_PIN);
    led::led_on(board::BLUE_LED_PORT, board::BLUE_LED_PIN);



    let mut peripherals= Peripherals::take().unwrap();
    let systick = &mut peripherals.SYST;
    systick.set_clock_source(SystClkSource::Core); 
    systick.set_reload(4_000_000 - 1);
    systick.clear_current();
    systick.enable_interrupt();
    systick.enable_counter();

    loop{

    }
}




#[exception]
fn SysTick() {
    led::led_toggle(board::BLUE_LED_PORT, board::BLUE_LED_PIN);

}