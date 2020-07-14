//! # STM32F302R8 NUCLEO board
//!
//! * Real Time Interrupt Controlled (RTIC)
//! * Embedded HAL: stm32f3xx-hal
//!
//! ## Board Characteristics:
//!
//! * clock rate: 32 MHz
//! * 64 kbyte Flash ROM
//! * 16 kbyte RAM
//! * cortex-m4 / armv7-m hf
//!
//! Hardware abstraction into board.rs
//!
//! Motor Driver Schield: X-Nucleo12A1
//! * STSPIN240 Driver chip
//! * PWM-A at  CN9.6 PD-2
//! * PWM-B at CN9.5 PD-3
//! * PH-A at CN9.7 PD-1
//! * PH-B at CN9.8 PD-0
//! * FN at CN9.3 - PD
//! * RST at CN5.2
//!
#![deny(unsafe_code)]
#![allow(warnings)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;
#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use stm32f3::stm32f302;

use cortex_m::peripheral::DWT;

use rtic::app;
use rtic::cyccnt::{Instant, U32Ext};
use rtic::export::wfi;

use stm32f302r8_nucleo::board::{board_init, Board, Button, Led, TimerInterrupt};

use embedded_hal::digital::v2::OutputPin;

use debounced_pin::prelude::*;
use debounced_pin::ActiveHigh;

const CORE_CLOCK_FREQUENCY: u32 = 32_000_000; // Hz

#[app(device = stm32f3::stm32f302, peripherals = true,  monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        // timer: TimerInterrupt,
        button: Button,
        led: Led,
    }

    #[init(schedule = [toggle])]
    fn init(mut ctx: init::Context) -> init::LateResources {
        #[cfg(debug_assertions)]
        hprintln!("** Init started").unwrap();

        ctx.core.DWT.enable_cycle_counter();
        ctx.core.DCB.enable_trace(); // needed so the DWT cycle counter doesn't get disabled when no debugger is connected

        let b = board_init(Some(ctx.device));

        // schedule the LED toggle task for immediate execution
        ctx.schedule.toggle(Instant::now()).unwrap();

        #[cfg(debug_assertions)]
        hprintln!("** Init ended").unwrap();
        // late resources
        init::LateResources {
            button: b.button,
            led: b.led,
        }
    }

    // this task requires access to led_pin to toggle the LED and
    // re-schedules itself toggle_interval_seconds later
    #[task(resources = [led,
        //toggle_interval_seconds
        ], schedule = [toggle], spawn = [toggle_led])]
    fn toggle(ctx: toggle::Context) {
        // static mut STATE: bool = false;

        #[cfg(debug_assertions)]
        hprintln!(
            "foo(scheduled = {:?}, now = {:?})",
            ctx.scheduled,
            Instant::now()
        )
        .unwrap();

        ctx.spawn.toggle_led().unwrap();

        // re-schedule toggle()
        ctx.schedule
            .toggle(ctx.scheduled + CORE_CLOCK_FREQUENCY.cycles())
            .unwrap()
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        #[cfg(debug_assertions)]
        hprintln!("** Idle").unwrap();
        loop {
            // This puts the processor to sleep until there's a task to service
            wfi();
        }
    }

    #[task(binds = TIM2, resources = [button], spawn = [toggle_led])]
    fn TIM2(ctx: TIM2::Context) {
        static mut LAST_STATE: bool = false;
        #[cfg(debug_assertions)]
        hprintln!("** Timer Event").unwrap();

        match ctx.resources.button.update().unwrap() {
            // Pin is not active.
            DebounceState::NotActive => *LAST_STATE = false,
            // Pin was reset or is not active in general.
            DebounceState::Reset => return,
            // Pin is active but still debouncing.
            DebounceState::Debouncing => return,
            // Pin is active and debounced.
            DebounceState::Active => {
                if !*LAST_STATE {
                    *LAST_STATE = true;
                    #[cfg(debug_assertions)]
                    hprintln!("** Button pressed").unwrap();
                    ctx.spawn.toggle_led().unwrap();
                }
            }
        }
    }

    // it is an separated task since it maintains own static resource
    #[task(resources = [led])]
    fn toggle_led(ctx: toggle_led::Context) {
        static mut STATE: bool = false;
        #[cfg(debug_assertions)]
        hprintln!("Toggle LED").unwrap();
        if *STATE {
            ctx.resources.led.set_low().unwrap();
        } else {
            ctx.resources.led.set_high().unwrap();
        }
        *STATE = !*STATE;
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART2_EXTI26();
    }
};
