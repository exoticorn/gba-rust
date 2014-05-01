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

    pub fn read_keyinput() -> u16 {
        unsafe { read16(0x4000130) }
    }

    pub fn write_pal(index: uint, col: u16) {
        if index < 512 {
            unsafe { write16(0x5000000u32 + (index * 2) as u32, col) }
        }
    }
}

pub struct KeyState { state: u32 }
pub enum Key {
    KeyA = 1u16,
    KeyB = 2u16,
    KeySelect = 4u16,
    KeyStart = 8u16,
    KeyRight = 16u16,
    KeyLeft = 32u16,
    KeyUp = 64u16,
    KeyDown = 128u16,
    KeyR = 256u16,
    KeyL = 512u16
}

impl KeyState {
    pub fn new() -> KeyState { KeyState { state: 0 } }
    pub fn update(&mut self) {
        let pressed = hw::read_keyinput() ^ 0xffffu16;
        let triggered = pressed & !self.get_pressed();
        self.state = (pressed as u32) | ((triggered as u32) << 16);
    }
    fn get_pressed(&self) -> u16 { self.state as u16 }
    fn get_triggered(&self) -> u16 { (self.state >> 16) as u16 }
    pub fn is_pressed(&self, key: Key) -> bool { self.get_pressed() & (key as u16) != 0 }
    pub fn is_triggered(&self, key: Key) -> bool { self.get_triggered() & (key as u16) != 0 }
}

pub fn wait_vblank() {
    while hw::read_dispstat() & 1 != 0 {}
    while hw::read_dispstat() & 1 == 0 {}
}
