pub mod hw {
    extern "rust-intrinsic" {
        fn volatile_load<T>(src: *T) -> T;
        fn volatile_store<T>(dst: *mut T, val: T);
    }
    
    unsafe fn read16(addr: u32) -> u16 {
        volatile_load(addr as *u16)
    }

    unsafe fn write16(addr: u32, value: u16) {
        volatile_store(addr as *mut u16, value);
    }

    #[allow(dead_code)]
    pub fn read_dispcnt() -> u16 {
        unsafe { read16(0x4000000) }
    }

    pub fn write_dispcnt(value: u16) {
        unsafe { write16(0x4000000, value) }
    }

    pub fn read_dispstat() -> u16 {
        unsafe { read16(0x4000004) }
    }

    #[allow(dead_code)]
    pub fn write_dispstat(value: u16) {
        unsafe { write16(0x4000004, value) }
    }

    pub fn write_pal(index: uint, col: u16) {
        if index < 512 {
            unsafe { write16(0x5000000u32 + (index * 2) as u32, col) }
        }
    }
}

pub fn wait_vblank() {
    while hw::read_dispstat() & 1 != 0 {}
    while hw::read_dispstat() & 1 == 0 {}
}
