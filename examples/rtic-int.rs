#![deny(warnings)]
#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use asm_delay::bitrate::*;
use asm_delay::AsmDelay;
use embedded_hal::blocking::delay::DelayMs;
use rtic::app;
use stm32f3::stm32f302;

#[app(device = stm32f3::stm32f302, peripherals = true)]
const APP: () = {
    struct Resources {
        device: stm32f302::Peripherals,
        delay: AsmDelay,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let device: stm32f302::Peripherals = ctx.device;
        let delay = AsmDelay::new(32u32.mhz());

        // pb13 -- led
        device.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());
        device.GPIOB.moder.modify(|_, w| w.moder13().output());
        device.GPIOB.bsrr.write(|w| w.bs13().clear_bit());

        // pc13 -- interrupt (button)
        device.RCC.ahbenr.modify(|_, w| w.iopcen().set_bit());
        device.GPIOC.moder.modify(|_, w| w.moder13().input());
        device.GPIOC
              .pupdr
              .modify(|_, w| unsafe { w.pupdr13().bits(0b01) });

        device.RCC.apb2enr.write(|w| w.syscfgen().enabled());
        device.SYSCFG
              .exticr4
              .modify(|_, w| unsafe { w.exti13().bits(0b010) });

        device.EXTI.imr1.modify(|_, w| w.mr13().set_bit());
        device.EXTI.emr1.modify(|_, w| w.mr13().set_bit());
        device.EXTI.rtsr1.modify(|_, w| w.tr13().set_bit());

        init::LateResources { device, delay }
    }

    #[task(binds=EXTI15_10, resources = [device, delay])]
    fn int(ctx: int::Context) {
        for _ in 1..3 {
            ctx.resources.device.GPIOB.bsrr.write(|w| w.bs13().set_bit());
            ctx.resources.delay.delay_ms(100u32);
            ctx.resources.device.GPIOB.brr.write(|w| w.br13().set_bit());
            ctx.resources.delay.delay_ms(100u32);
        }
        ctx.resources
           .device
           .EXTI
           .pr1
           .modify(|_, w| w.pr13().set_bit());
    }
};
