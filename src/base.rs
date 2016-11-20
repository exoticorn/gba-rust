#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
pub extern "C" fn rust_begin_panic(_msg: ::core::fmt::Arguments,
                                   _file: &'static str,
                                   _line: u32)
                                   -> ! {
    loop {}
}

#[allow(dead_code)]
pub mod rand {
    pub struct Rand {
        state: u32,
    }

    impl Rand {
        pub fn new(seed: u32) -> Rand {
            Rand { state: seed }
        }
        pub fn next_bool(&mut self) -> bool {
            self.state = self.state * 1664525u32 + 1013904223u32;
            self.state & 0x80000000u32 != 0
        }
        pub fn next_u8(&mut self) -> u8 {
            let mut result = 0u8;
            for i in 0..8 {
                result |= (self.next_bool() as u8) << i;
            }
            result
        }
    }
}
