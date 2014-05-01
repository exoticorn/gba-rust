#![no_std]

mod gba;

#[start]
pub fn main(_: int, _: **u8) -> int {
    let mut col = 0u16;
    gba::hw::write_dispcnt(0);
    loop {
        gba::hw::write_pal(0, col);
        col += 1;
        gba::wait_vblank();
    }
}
