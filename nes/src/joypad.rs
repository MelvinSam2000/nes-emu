#[derive(Default)]
pub struct Joypad {
    pub status: u8,
    pub strobe: bool,
    pub index: u8,
}

#[allow(non_camel_case_types)]
pub enum Button {
    BTN_A = 1 << 0,
    BTN_B = 1 << 1,
    SELECT = 1 << 2,
    START = 1 << 3,
    UP = 1 << 4,
    DOWN = 1 << 5,
    LEFT = 1 << 6,
    RIGHT = 1 << 7,
}

impl Joypad {
    pub fn press(&mut self, btn: Button) {
        self.status |= btn as u8;
    }

    pub fn release(&mut self, btn: Button) {
        self.status &= !(btn as u8);
    }

    pub fn read(&mut self) -> u8 {
        if self.index > 7 {
            return 1;
        }
        let response = (self.status & (1 << self.index)) >> self.index;
        if !self.strobe && self.index <= 7 {
            self.index += 1;
        }
        response
    }

    pub fn write(&mut self, data: u8) {
        self.strobe = data & 0x01 != 0;
        if self.strobe {
            self.index = 0;
        }
    }
}
