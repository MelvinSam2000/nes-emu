#![allow(non_upper_case_globals)]

use bitflags::bitflags;

bitflags! {

    pub struct RegMask: u8 {

        const Gr = 1 << 0;    // Greyscale
        const m = 1 << 1;     // Background left column enable
        const M = 1 << 2;     // Sprite left column enable
        const b = 1 << 3;     // Background enable
        const s = 1 << 4;     // Sprite enable
        const R = 1 << 5;     // Color emphasis
        const G = 1 << 6;     // Color emphasis
        const B = 1 << 7;     // Color emphasis
    }
}

impl Default for RegMask {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl RegMask {
    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
    /*
    pub fn render_bg_enabled(&self) -> bool {
        // TODO: change later
        return true;//self.get_flag(Flag::b);
    }
    pub fn render_spr_enabled(&self) -> bool {
        return self.get_flag(Flag::s);
    }
    pub fn get_color_emphasis(&self) -> (bool, bool, bool) {
        return (
            self.get_flag(Flag::R),
            self.get_flag(Flag::G),
            self.get_flag(Flag::B),
        );
    }
    fn set_flag(&mut self, flag: Flag, val: bool) {
        if val {
            self.reg |= flag as u8;
        } else {
            self.reg &= !(flag as u8);
        }
    }
    fn get_flag(&self, flag: Flag) -> bool {
        return flag as u8 & self.reg != 0x00;
    }
    */
}
