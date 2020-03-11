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
        self.registers.out.write(|w| {
            w.ledr().bit(red);
            w.ledg().bit(green)
        });
    }

    pub fn off(&mut self) {
        unsafe {
            self.registers.out.write(|w| w.bits(0));
        }
    }

    pub fn on(&mut self) {
        unsafe {
            self.registers.out.write(|w| w.bits(3));
        }
    }

    pub fn toggle(&mut self) {
        self.toggle_mask(0xFFFF_FFFF);
    }

    pub fn toggle_mask(&mut self, mask: u32) {
        let val: u32 = self.registers.out.read().bits() ^ mask;
        unsafe {
            self.registers.out.write(|w| w.bits(val));
        }
    }
}