#![deny(unsafe_code)]
#![no_std]
#![no_main]

/*
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}*/
/*
extern crate panic_semihosting;
extern crate psoc6_hal;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;*/


use cortex_m_rt::entry;
use panic_halt as _;
use cortex_m_semihosting::hprintln;

/*
use cortex_m::peripheral::{syst, Peripherals};
use cortex_m_rt::entry;
use panic_halt as _;*/


/*#[psoc6_hal::entry]*/
#[entry]
fn main() -> ! {
    hprintln!("Hello, world! Rust").unwrap();
    loop{}
}