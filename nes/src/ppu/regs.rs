#![allow(non_upper_case_globals)]

use bitflags::bitflags;

bitflags! {
    pub struct RegControl: u8 {
        const Nx = 1 << 0;   // Nametable select (x)
        const Ny = 1 << 1;   // Nametable select (y)
        const I = 1 << 2;     // Increment mode
        const S = 1 << 3;     // Sprite tile select
        const B = 1 << 4;     // Background tile select
        const H = 1 << 5;     // Sprite height
        const P = 1 << 6;     // PPU master/slave
        const V = 1 << 7;     // NMI enable
    }
}

impl Default for RegControl {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl RegControl {
    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }

    pub fn is_nmi_enabled(&mut self) -> bool {
        self.contains(RegControl::V)
    }

    pub fn is_inc_mode(&mut self) -> bool {
        self.contains(RegControl::I)
    }

    pub fn get_bg(&self) -> bool {
        self.contains(RegControl::B)
    }
}

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

#[derive(Default)]
pub struct RegScroll {
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub latch: bool,
}

impl RegScroll {
    pub fn write(&mut self, data: u8) {
        if !self.latch {
            self.scroll_x = data;
        } else {
            self.scroll_y = data;
        }
        self.latch = !self.latch;
    }
}
