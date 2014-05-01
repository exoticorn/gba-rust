#![no_std]

mod gba;

#[start]
pub fn main(_: int, _: **u8) -> int {
    let mut col = 0u16;
    let mut key_state = gba::KeyState::new();
    gba::hw::write_dispcnt(0);
    loop {
        key_state.update();
        if key_state.is_pressed(gba::KeyUp) { col += 1 }
        if key_state.is_pressed(gba::KeyDown) { col -= 1 }
        if key_state.is_triggered(gba::KeyA) { col += 0x2000 }
        gba::hw::write_pal(0, col);
        gba::wait_vblank();
    }
}
