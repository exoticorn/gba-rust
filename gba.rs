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

    macro_rules! hw_reg(
        ($addr: expr rw, $read:ident, $write: ident) => {
            #[allow(dead_code)]
            pub fn $read() -> u16 {
                unsafe { read16($addr) }
            }

            #[allow(dead_code)]
            pub fn $write(value: u16) {
                unsafe { write16($addr, value) }
            }
        };
        ($addr: expr r, $read: ident) => {
            #[allow(dead_code)]
            pub fn $read() -> u16 {
                unsafe { read16($addr) }
            }
        };
        ($addr: expr w, $write: ident) => {
            #[allow(dead_code)]
            pub fn $write(value: u16) {
                unsafe { write16($addr, value) }
            }
        };
    )

    hw_reg!(0x4000000 rw, read_dispcnt, write_dispcnt)
    hw_reg!(0x4000004 rw, read_dispstat, write_dispstat)
    hw_reg!(0x4000008 rw, read_bg0cnt, write_bg0cnt)
    hw_reg!(0x400000a rw, read_bg1cnt, write_bg1cnt)
    hw_reg!(0x400000c rw, read_bg2cnt, write_bg2cnt)
    hw_reg!(0x400000e rw, read_bg3cnt, write_bg3cnt)
    hw_reg!(0x4000010 w, write_bg0hofs)
    hw_reg!(0x4000012 w, write_bg0vofs)
    hw_reg!(0x4000014 w, write_bg1hofs)
    hw_reg!(0x4000016 w, write_bg1vofs)
    hw_reg!(0x4000018 w, write_bg2hofs)
    hw_reg!(0x400001a w, write_bg2vofs)
    hw_reg!(0x400001c w, write_bg3hofs)
    hw_reg!(0x400001e w, write_bg3vofs)
    hw_reg!(0x4000130 r, read_keyinput)

    pub fn write_pal(index: uint, col: u16) {
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
