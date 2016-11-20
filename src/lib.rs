#![no_std]
#![feature(lang_items)]

use core::ptr::write_volatile;

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        write_volatile(0x4000000 as *mut u16, 0);
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments,
                                   _file: &'static str,
                                   _line: u32)
                                   -> ! {
    loop {}
}
