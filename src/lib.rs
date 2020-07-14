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
// #![deny(missing_docs)]
#![deny(unsafe_code)]
#![allow(warnings)]

// #![deny(warnings)]
#![no_std]

// the board configuration
pub mod board;


