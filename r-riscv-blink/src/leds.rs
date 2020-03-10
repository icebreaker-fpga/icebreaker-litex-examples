use icebesoc_pac::LEDS;

pub struct Leds {
    registers: LEDS,
}

#[allow(dead_code)]
impl Leds {
    pub fn new(registers: LEDS) -> Self {
        Self { registers }
    }

    pub fn set(&mut self, red: bool, green: bool) {
        let val: u8 = if red {0} else {1} | if green {0} else {2};
        unsafe {
            self.registers.out.write(|w| w.bits(val));
        }
    }

    pub fn off(&mut self) {
        unsafe {
            self.registers.out.write(|w| w.bits(3));
        }
    }

    pub fn on(&mut self) {
        unsafe {
            self.registers.out.write(|w| w.bits(0));
        }
    }

    pub fn toggle(&mut self) {
        self.toggle_mask(0xFF);
    }

    pub fn toggle_mask(&mut self, mask: u8) {
        let val: u8 = self.registers.out.read().bits() ^ mask;
        unsafe {
            self.registers.out.write(|w| w.bits(val));
        }
    }
}