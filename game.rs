#![no_std]
#![feature(macro_rules)]
#![feature(globs)]

use base::prelude::*;

mod base;
mod gba;

#[start]
pub fn main(_: int, _: **u8) -> int {
    let mut col = 0u16;
    let mut key_state = gba::KeyState::new();
    gba::hw::write_dispcnt(1 << 8);
    gba::hw::write_bg0cnt(1 << 8);
    gba::hw::write_pal(15, 0x7fff);
    gba::hw::write_pal(31, 31 << 5);
    for i in range(1, 7) {
        gba::hw::write_vram16((i * 2) as u32, 0xfff0);
        gba::hw::write_vram16((i * 2 + 1) as u32, 0x0fff);
    }
    for i in range(0, 100) {
        gba::hw::write_vram16(0x400u32 + (i as u32), 1 << 12);
    }
    loop {
        key_state.update();
        if key_state.is_pressed(gba::KeyUp) { col += 1 }
        if key_state.is_pressed(gba::KeyDown) { col -= 1 }
        if key_state.is_triggered(gba::KeyA) { col += 0x2000 }
        gba::hw::write_pal(0, col);
        gba::wait_vblank();
    }
}
