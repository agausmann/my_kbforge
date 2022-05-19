#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(generic_const_exprs)]

extern crate avr_std_stub;

use atmega_hal::entry;
use atmega_hal::pac::Peripherals;
use kbforge::board::planck_rev2::build_system;
use kbforge::keycode::qmk::*;
use kbforge::keycode::Keycode;
use kbforge::keymap::Layered;

// TODO activate setup if both are held
const CK_LOWR: Keycode = MO(LAYER_LOWER);
const CK_RAIS: Keycode = MO(LAYER_RAISE);

const LAYER_LOWER: u8 = 1;
const LAYER_RAISE: u8 = 2;
const LAYER_SETUP: u8 = 3;

#[rustfmt::skip]
static LAYERS: [[[Keycode; 12]; 4]; 4] = [
    // 0: Default/Base
    [
        [KC_TAB , KC_Q   , KC_W   , KC_E   , KC_R   , KC_T   , KC_Y   , KC_U   , KC_I   , KC_O   , KC_P   , KC_BSPC],
        [KC_CLCK, KC_A   , KC_S   , KC_D   , KC_F   , KC_G   , KC_H   , KC_J   , KC_K   , KC_L   , KC_SCLN, KC_QUOT],
        [KC_LSFT, KC_Z   , KC_X   , KC_C   , KC_V   , KC_B   , KC_N   , KC_M   , KC_COMM, KC_DOT , KC_SLSH, KC_RSFT],
        [KC_LCTL, KC_LGUI, KC_LALT, XXXXXXX, CK_LOWR, KC_ENT , KC_SPC , CK_RAIS, XXXXXXX, KC_RALT, KC_RGUI, KC_RCTL],
    ],
    // 1: Lower
    [
        [KC_ESC , KC_F1  , KC_F2  , KC_F3  , KC_F4  , _______, KC_HOME, KC_PGDN, KC_PGUP, KC_END , KC_INS , _______],
        [_______, KC_F5  , KC_F6  , KC_F7  , KC_F8  , _______, KC_LEFT, KC_DOWN, KC_UP  , KC_RGHT, KC_DEL , _______],
        [_______, KC_F9  , KC_F10 , KC_F11 , KC_F12 , _______, _______, KC_PAUS, KC_PSCR, KC_SLCK, _______, _______],
        [_______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______],
    ],
    // 2: Raise
    [
        [KC_GRV , KC_1   , KC_2   , KC_3   , KC_4   , KC_5   , KC_6   , KC_7   , KC_8   , KC_9   , KC_0   , _______],
        [_______, _______, _______, _______, _______, _______, _______, KC_MINS, KC_EQL , KC_LBRC, KC_RBRC, KC_BSLS],
        [_______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______],
    ],
    // 3: Setup
    [
        [RESET  , _______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______, _______],
    ],
];

#[entry]
fn main() -> ! {
    let keymap = Layered {
        layers: &LAYERS,
        layer_mask: 0x00000001,
    };

    let peripherals = Peripherals::take().unwrap();
    let mut system = build_system(peripherals, keymap);
    loop {
        system.poll().ok();
    }
}
