use bitflags::bitflags;

bitflags! {
    pub struct RegStatus: u8 {
        const O = 1 << 5;     // sprite overflow
        const S = 1 << 6;     // sprite 0 hit
        const V = 1 << 7;     // vblank
    }
}

impl Default for RegStatus {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl RegStatus {
    pub fn new() -> Self {
        RegStatus::from_bits_truncate(0)
    }

    pub fn get_bits(&self) -> u8 {
        self.bits
    }

    pub fn set_vblank(&mut self, val: bool) {
        self.set(RegStatus::V, val);
    }
}
