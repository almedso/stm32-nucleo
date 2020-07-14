#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused)]
use panic_halt;

use cortex_m;

use cortex_m_rt::{entry, exception, ExceptionFrame};

use hal::{
    delay::Delay,
    gpio::{PullNone, PushPull},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let device = hal::pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();
    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .pclk2(32.mhz())
        .freeze(&mut flash.acr);
    let mut delay = Delay::new(core.SYST, clocks);
    let gpiob = device.GPIOB.split(&mut rcc.ahb);

    // configure the pin PB13 as an output
    let mut led = gpiob
        .pb13
        .pull_type(PullNone)
        .output()
        .output_type(PushPull);

    loop {
        led.set_low().unwrap();
        delay.delay_ms(250_u16);

        led.set_high().unwrap();
        delay.delay_ms(250_u16);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
