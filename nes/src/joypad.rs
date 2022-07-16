#[derive(Default)]
pub struct Joypad {
    pub status: u8,
    pub strobe: bool,
    pub index: u8,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Button {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right
}

impl From<Button> for u8 {
    fn from(button: Button) -> Self {
        match button {
            Button::A => 1 << 0,
            Button::B => 1 << 1,
            Button::Select => 1 << 2,
            Button::Start => 1 << 3,
            Button::Up => 1 << 4,
            Button::Down => 1 << 5,
            Button::Left => 1 << 6,
            Button::Right => 1 << 7,
        }
    }
}

impl Joypad {
    pub fn press(&mut self, btn: Button) {
        let btn = <Button as Into<u8>>::into(btn);
        self.status |= btn;
    }

    pub fn release(&mut self, btn: Button) {
        let btn = <Button as Into<u8>>::into(btn);
        self.status &= !btn;
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
