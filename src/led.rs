//! On-board user LEDs on NUCLEO-xx
//! NUCLEO-STM32F302R8 has just one (green LED)
//! attached to PB13 (Arduino connector D13)


use hal::prelude::*;

use hal::gpio::gpiob::{self, PBx };
use hal::gpio::{Output, PushPull};

///  Green user LED
pub type USER_LED = PB13<Output<PushPull>>;

/// One of the on-board user LEDs
pub struct Led {
    px: PBx<Output<PushPull>>,
}


impl UserLed {

    /// Constructor
    pub fn new() -> UserLed {
       UserLed {
           px: gpiob.pb13.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper).into()
       }
    }

    /// Turns the LED off
    pub fn off(&mut self) {
        self.px.set_low()
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.px.set_high()
    }
}
