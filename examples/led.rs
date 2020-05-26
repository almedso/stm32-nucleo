
#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_extern_crates)]

use cortex_m_rt::entry;
// stm32f30x-hal crate was generated using `svd2rust`
pub extern crate stm32f30x_hal as hal;
use hal::{delay::Delay, prelude::*, stm32f30x};

use hal::gpio::gpiob::{PB13};
use hal::gpio::{Output, PushPull};

#[entry]
fn main() -> ! {

    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    // splits the GPIOA peripheral into 16 independent pins + registers
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (all clocks run at 16 MHz)
    // let clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr);

    // configure the pin PB13 as an output
    let mut led: PB13<Output<PushPull>> = gpiob
        .pb13
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
        .into();

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high();
        delay.delay_ms(1_000_u16);
        led.set_low();
        delay.delay_ms(1_000_u16);
    }
}
