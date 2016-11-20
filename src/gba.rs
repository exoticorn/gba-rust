#![allow(dead_code)]

pub mod hw {
    use core::ptr::{read_volatile, write_volatile};

    unsafe fn read16(addr: u32) -> u16 {
        read_volatile(addr as *const u16)
    }

    unsafe fn write16(addr: u32, value: u16) {
        write_volatile(addr as *mut u16, value);
    }

    macro_rules! hw_reg {
        (rw $addr: expr, $read:ident, $write: ident) => {
            #[allow(dead_code)]
            pub fn $read() -> u16 {
                unsafe { read16($addr) }
            }

            #[allow(dead_code)]
            pub fn $write(value: u16) {
                unsafe { write16($addr, value) }
            }
        };
        (r $addr: expr, $read: ident) => {
            #[allow(dead_code)]
            pub fn $read() -> u16 {
                unsafe { read16($addr) }
            }
        };
        (w $addr: expr, $write: ident) => {
            #[allow(dead_code)]
            pub fn $write(value: u16) {
                unsafe { write16($addr, value) }
            }
        };
    }

    hw_reg!(rw 0x4000000, read_dispcnt, write_dispcnt);
    hw_reg!(rw 0x4000004, read_dispstat, write_dispstat);
    hw_reg!(rw 0x4000008, read_bg0cnt, write_bg0cnt);
    hw_reg!(rw 0x400000a, read_bg1cnt, write_bg1cnt);
    hw_reg!(rw 0x400000c, read_bg2cnt, write_bg2cnt);
    hw_reg!(rw 0x400000e, read_bg3cnt, write_bg3cnt);
    hw_reg!(w 0x4000010, write_bg0hofs);
    hw_reg!(w 0x4000012, write_bg0vofs);
    hw_reg!(w 0x4000014, write_bg1hofs);
    hw_reg!(w 0x4000016, write_bg1vofs);
    hw_reg!(w 0x4000018, write_bg2hofs);
    hw_reg!(w 0x400001a, write_bg2vofs);
    hw_reg!(w 0x400001c, write_bg3hofs);
    hw_reg!(w 0x400001e, write_bg3vofs);
    hw_reg!(r 0x4000130, read_keyinput);

    pub fn write_pal(index: u32, col: u16) {
        if index < 512 {
            unsafe { write16(0x5000000u32 + (index * 2) as u32, col) }
        }
    }

    pub fn write_vram16(offset: u32, data: u16) {
        if offset < 0xc000 {
            unsafe { write16(0x6000000u32 + offset * 2, data) }
        }
    }
}

pub struct KeyState {
    state: u32,
}
pub enum Key {
    A = 1,
    B = 2,
    Select = 4,
    Start = 8,
    Right = 16,
    Left = 32,
    Up = 64,
    Down = 128,
    R = 256,
    L = 512,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState { state: 0 }
    }
    pub fn update(&mut self) {
        let pressed = hw::read_keyinput() ^ 0xffffu16;
        let triggered = pressed & !self.get_pressed();
        self.state = (pressed as u32) | ((triggered as u32) << 16);
    }
    fn get_pressed(&self) -> u16 {
        self.state as u16
    }
    fn get_triggered(&self) -> u16 {
        (self.state >> 16) as u16
    }
    #[allow(dead_code)]
    pub fn is_pressed(&self, key: Key) -> bool {
        self.get_pressed() & (key as u16) != 0
    }
    #[allow(dead_code)]
    pub fn is_triggered(&self, key: Key) -> bool {
        self.get_triggered() & (key as u16) != 0
    }
}

pub fn wait_vblank() {
    while hw::read_dispstat() & 1 != 0 {}
    while hw::read_dispstat() & 1 == 0 {}
}
