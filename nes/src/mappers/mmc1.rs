use anyhow::anyhow;
use anyhow::Result;
use bitflags::bitflags;

use crate::cartridge::Mirroring;
use crate::mappers::Mapper;
use crate::Nes;

bitflags! {
    struct RegControl: u8 {
        // Mirroring: (0: one-screen, lower bank; 1: one-screen, upper bank, 2: vertical; 3: horizontal)
        const M0 = 1 << 0;
        const M1 = 1 << 1;
        /*
        P0- P1: PRG ROM bank mode (0, 1: switch 32 KB at $8000, ignoring low bit of bank number;
            2: fix first bank at $8000 and switch 16 KB bank at $C000;
            : fix last bank at $C000 and switch 16 KB bank at $8000)
        */
        const P0 = 1 << 2;
        const P1 = 1 << 3;
        // CHR ROM bank mode (0: switch 8 KB at a time; 1: switch two separate 4 KB banks)
        const C = 1 << 4;
    }
}

pub struct Mmc1 {
    reg_load: u8,
    load_count: usize,
    reg_control: RegControl,
    mirror_mode: Mirroring,
    wram: Vec<u8>,

    prg_bank_sel_16_lo: u8,
    prg_bank_sel_16_hi: u8,
    prg_bank_sel_32: u8,

    chr_bank_sel_4_lo: u8,
    chr_bank_sel_4_hi: u8,
    chr_bank_sel_8: u8,
}

impl Mmc1 {
    pub fn new() -> Self {
        Self {
            reg_load: 0x00,
            load_count: 0,
            reg_control: RegControl::from_bits_truncate(0),
            mirror_mode: Mirroring::HORIZONTAL,
            wram: vec![0; 32 * 1024],

            prg_bank_sel_16_lo: 0x00,
            prg_bank_sel_16_hi: 0x00,
            prg_bank_sel_32: 0x00,

            chr_bank_sel_4_lo: 0x00,
            chr_bank_sel_4_hi: 0x00,
            chr_bank_sel_8: 0x00,
        }
    }

    pub fn reset(&mut self) {
        self.reg_load = 0x00;
        self.load_count = 0;
        self.reg_control.bits = 0x1c;
    }
}

impl Default for Mmc1 {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, A> Mapper<S, A> for Mmc1 {
    fn read_prg(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        let mut mapped_addr = 0u16;
        match addr {
            0x6000..=0x7fff => Ok(self.wram[(addr & 0x1fff) as usize]),
            0x8000..=0xffff => {
                if self.reg_control.contains(RegControl::P1) {
                    match addr {
                        0x8000..=0xbfff => {
                            mapped_addr = self.prg_bank_sel_16_lo as u16 * 0x4000 + (addr & 0x3fff);
                        }
                        0xc000..=0xffff => {
                            mapped_addr = self.prg_bank_sel_16_hi as u16 * 0x4000 + (addr & 0x3fff);
                        }
                        _ => { /* Impossible */ }
                    };
                } else {
                    mapped_addr = self.prg_bank_sel_32 as u16 * 0x8000 + (addr & 0x7fff);
                }
                Ok(nes.cartridge.prgmem[mapped_addr as usize])
            }
            _ => {
                log::warn!("Cannot read at PRG address {:#x} for MMC1", addr);
                Ok(0)
            }
        }
    }

    fn write_prg(&mut self, nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        match addr {
            0x6000..=0x7fff => {
                self.wram[(addr & 0x1fff) as usize] = data;
            }
            0x8000..=0xffff => {
                if data & 0x80 != 0 {
                    self.reg_load = 0x00;
                    self.load_count = 0;
                    self.reg_control.bits |= 0xc0;
                } else {
                    self.reg_load >>= 1;
                    self.reg_load |= (data & 0x01) << 4;
                    self.load_count += 1;

                    if self.load_count == 5 {
                        match addr {
                            0x8000..=0x9fff => {
                                self.reg_control.bits = self.reg_load & 0x1f;
                                self.mirror_mode = match self.reg_control.bits & 0b11 {
                                    //0 => Mirroring::ONESCREEN_LO,
                                    //1 => Mirroring::ONESCREEN_HI,
                                    2 => Mirroring::VERTICAL,
                                    3 => Mirroring::HORIZONTAL,
                                    _ => Err(anyhow!("Mirroring not available for MMC1 yet..."))?,
                                }
                            }
                            0xa000..=0xbfff => {
                                if self.reg_control.contains(RegControl::C) {
                                    self.chr_bank_sel_4_lo = self.reg_load & 0x1f;
                                } else {
                                    self.chr_bank_sel_8 = self.reg_load & 0x1e;
                                }
                            }
                            0xc000..=0xdfff => {
                                if self.reg_control.contains(RegControl::C) {
                                    self.chr_bank_sel_4_hi = self.reg_load & 0x1f;
                                }
                            }
                            0xe000..=0xffff => {
                                let prgmode = (self.reg_control.bits >> 2) & 0x03;
                                match prgmode {
                                    0 | 1 => {
                                        self.prg_bank_sel_32 = (self.reg_load & 0x0e) >> 1;
                                    }
                                    2 => {
                                        self.prg_bank_sel_16_lo = 0;
                                        self.prg_bank_sel_16_hi = self.reg_load & 0x0f;
                                    }
                                    3 => {
                                        self.prg_bank_sel_16_lo = self.reg_load & 0x0f;
                                        self.prg_bank_sel_16_hi = nes.cartridge.prg_banks - 1;
                                    }
                                    _ => { /* IMPOSSIBLE */ }
                                }
                            }
                            _ => {}
                        }

                        self.reg_load = 0x00;
                        self.load_count = 0;
                    }
                }
            }
            _ => log::warn!("Cannot write at PRG address {:#x} for MMC1", addr),
        }
        Ok(())
    }

    fn read_chr(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        let mut mapped_addr = 0u16;
        match addr {
            0x0000..=0x1fff => {
                if nes.cartridge.chr_banks == 0 {
                    mapped_addr = addr;
                } else if self.reg_control.contains(RegControl::C) {
                    match addr {
                        0x0000..=0x0fff => {
                            mapped_addr = self.chr_bank_sel_4_lo as u16 * 0x1000 + (addr & 0x0fff);
                        }
                        0x1000..=0x1fff => {
                            mapped_addr = self.chr_bank_sel_4_hi as u16 * 0x1000 + (addr & 0x0fff);
                        }
                        _ => { /* Impossible */ }
                    };
                } else {
                    mapped_addr = self.chr_bank_sel_8 as u16 * 0x2000 + (addr & 0x1fff);
                }
                Ok(nes.cartridge.prgmem[mapped_addr as usize])
            }
            _ => Err(anyhow!("Cannot read at CHR address {:#x} for MMC1", addr)),
        }
    }

    fn write_chr(&mut self, nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        if addr < 0x2000 {
            nes.cartridge.chrmem[addr as usize] = data;
        }
        Ok(())
    }
}
