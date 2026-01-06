#![no_std]
#![no_main]
#![allow(dead_code)]

use core::cell::RefCell;

//use cortex_m::peripheral;

use cortex_m_rt::{entry,exception};
//use cortex_m::peripheral::syst;
use cortex_m::peripheral::syst::SystClkSource;

use cortex_m::peripheral::Peripherals;
use cortex_m::interrupt::Mutex;
use panic_halt as _;

mod led;
mod gpio;
mod mcu;
mod reg;
mod board;
mod itm_debug;



//static mut PERIPHERALS: Option<Peripherals> = None;
static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None)); 

#[entry]
fn main() -> ! {
    led::led_init(board::BLUE_LED_PORT, board::BLUE_LED_PIN);
    led::led_on(board::BLUE_LED_PORT, board::BLUE_LED_PIN);

    let mut peripherals= Peripherals::take().unwrap();
    
    itm_debug::itm_init(&mut peripherals);
    itm_debug::itm_print(&mut peripherals, "Hello from Main");
    
    systick_init(&mut peripherals.SYST);

    cortex_m::interrupt::free(|cs| {
        *PERIPHERALS.borrow(cs).borrow_mut() = Some(peripherals);
    });
    

    loop {
        cortex_m::interrupt::free(|cs: &cortex_m::interrupt::CriticalSection|
  
                if let Some(peripherals) = &mut *PERIPHERALS.borrow(cs).borrow_mut() {
                    itm_debug::itm_print(peripherals, "Message from Main loop");
                }
             
        );      
    }
}

fn systick_init(systick: &mut cortex_m::peripheral::SYST) {
    systick.set_clock_source(SystClkSource::Core); 
    systick.set_reload(84_000_000 - 1);
    systick.clear_current();
    systick.enable_interrupt();
    systick.enable_counter();
}




#[exception]
fn SysTick() {

    cortex_m::interrupt::free(|cs| 
            if let Some(peripherals) = &mut *PERIPHERALS.borrow(cs).borrow_mut() {
                itm_debug::itm_print(peripherals, "Hello from SysTick");
            }
           
    );
    led::led_toggle(board::BLUE_LED_PORT, board::BLUE_LED_PIN);

}