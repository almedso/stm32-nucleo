//! Board support crate for the NUCLEO STM32F302R8
//!
//! # Usage
//!
//! - Trying out the examples
//!
//! ``` text
//! # on another terminal
//! $ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
//!
//! # flash and debug the "Hello, world" example
//! $ cd f3
//! $ rustup target add thumbv7em-none-eabihf
//! $ cargo run --example hello
//! ```
//!
//! You'll need to have both OpenOCD and arm-none-eabi-gcc installed.
//!
//! - Building an application that depends on this crate
//!
//! To build applications (binary crates) using this crate follow [cortex-m-quickstart] instructions
//! and add this crate as a dependency in step number 6 and make sure you enable the "rt" Cargo
//! feature of this crate. Also, instead of step number 4 remove *both* the build.rs and memory.x
//! files.
//!
//! [cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/~0.3
//!
//! # Examples
//!
//! See the [examples] module.
//!
//! [examples]: examples/index.html

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

// Panic handler
extern crate panic_semihosting;
// extern crate panic_halt;

