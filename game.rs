#![no_std]

#[start]
pub fn main(_: int, _: **u8) -> int {
    unsafe {
        *(0x5000000 as *mut u16) = 31;
    }
    loop {}
}
