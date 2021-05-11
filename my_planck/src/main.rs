#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(const_generics, const_evaluatable_checked)]

extern crate avr_std_stub;

use atmega32u4_hal::pac::Peripherals;
use kbforge::board::planck_rev2::build_system;
use kbforge::keycode::qmk::*;
use kbforge::keycode::Keycode;

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn main() {
    let keymap: [[Keycode; 12]; 4] = [[KC_NO; 12]; 4];
    let peripherals = Peripherals::take().unwrap();
    let mut system = build_system(peripherals, keymap);
    loop {
        system.poll().map_err(|_| ()).unwrap();
    }
}
