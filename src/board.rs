#![deny(unsafe_code)]
#![allow(warnings)]

use hal::{
    delay::Delay,
    gpio::{Input, LowSpeed, Output, PullNone, PullUp, PushPull},
    prelude::*,
};

use debounced_pin::{prelude::*, ActiveHigh};

#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;

pub type TimerInterrupt = hal::timer::tim4::Timer<hal::pac::TIM4>;
pub type Button = DebouncedInputPin<hal::gpio::PC13<PullUp, Input>, ActiveHigh>;
pub type Led = hal::gpio::PB13<PullNone, Output<PushPull, LowSpeed>>;

// pub type I2cSda = gpiob::PB7<Analog>;
// pub type I2cScl = gpiob::PB6<Analog>;

// pub type UsartRx = gpiob::PB10<Analog>;
// pub type UsartTx = gpiob::PB11<Analog>;

pub struct Board {
    // pub timer: TimerInterrupt,
    pub button: Button,
    pub led: Led,
}

pub fn board_init(device: Option<hal::pac::Peripherals>) -> Board {
    #[cfg(debug_assertions)]
    hprintln!("** Start board config").unwrap();

    let device = match (device) {
        None => hal::pac::Peripherals::take().unwrap(),
        Some(device) => device,
    };

    // Configure the clock.
    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    // Configure the timer.
    let mut timer = hal::timer::tim4::Timer::new(device.TIM4, 1.khz(), clocks);
    timer.enable();
    // timer.listen();

    // Configure the button
    let gpioc = device.GPIOC.split(&mut rcc.ahb);
    let button = gpioc.pc13.input().pull_type(PullUp);
    let button = DebouncedInputPin::new(button, ActiveHigh);

    // Configure the user LED's
    let gpiob = device.GPIOB.split(&mut rcc.ahb);
    let led: Led = gpiob.pb13.output().pull_type(PullNone);

    #[cfg(debug_assertions)]
    hprintln!("** Low level board configuration done").unwrap();

    Board {
        // timer,
        button,
        led,
    }
}
