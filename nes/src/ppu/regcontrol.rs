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
