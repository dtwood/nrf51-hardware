#![no_std]
#![feature(asm)]

#[macro_use]
extern crate lazy_static;
extern crate nrf51_atomic;
extern crate spin;
extern crate volatile_register;

pub mod busy_loop;
pub mod gpio;
pub mod pins;
#[macro_use]
pub mod serial;
mod svd;
