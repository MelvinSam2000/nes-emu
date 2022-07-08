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
            3: fix last bank at $C000 and switch 16 KB bank at $8000)
        */
        const P0 = 1 << 2;
        const P1 = 1 << 3;
        // CHR ROM bank mode (0: switch 8 KB at a time; 1: switch two separate 4 KB banks)
        const C = 1 << 4;
    }
}

impl RegControl {
    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }

    pub fn mirroring(&self) -> Mirroring {
        match self.bits & 0b00011 {
            //0 => Mirroring::ONESCREEN_LO,
            //1 => Mirroring::ONESCREEN_HI,
            2 => Mirroring::VERTICAL,
            3 => Mirroring::HORIZONTAL,
            _ => panic!("Mirroring not available for MMC1 yet..."),
        }
    }

    pub fn prg_bank_mode(&self) -> u8 {
        (self.bits & 0b01100) >> 2
    }

    pub fn chr_bank_mode(&self) -> bool {
        self.contains(RegControl::C)
    }
}

// Mapper 1
pub struct Mmc1 {
    reg_load: u8,
    load_count: usize,
    reg_control: RegControl,
    wram: [u8; 0x2000],

    prg_bank_sel_16: (u8, u8),
    prg_bank_sel_32: u8,

    chr_bank_sel_4: (u8, u8),
    chr_bank_sel_8: u8,
}

impl Mmc1 {
    pub fn new() -> Self {
        Self {
            reg_load: 0x00,
            load_count: 0,
            reg_control: RegControl::from_bits_truncate(0x1c),
            wram: [0; 0x2000],

            prg_bank_sel_16: (0x00, 0x00),
            prg_bank_sel_32: 0x00,

            chr_bank_sel_4: (0x00, 0x00),
            chr_bank_sel_8: 0x00,
        }
    }
}

impl Default for Mmc1 {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, A> Mapper<S, A> for Mmc1 {
    fn read_prg(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        match addr {
            0x6000..=0x7fff => Ok(self.wram[(addr & 0x1fff) as usize]),
            0x8000..=0xffff => {
                let mapped_addr = if self.reg_control.bits & 0b01000 != 0 {
                    match addr {
                        0x8000..=0xbfff => {
                            self.prg_bank_sel_16.0 as usize * 0x4000 + (addr as usize & 0x3fff)
                        }
                        0xc000..=0xffff => {
                            self.prg_bank_sel_16.1 as usize * 0x4000 + (addr as usize & 0x3fff)
                        }
                        _ => unreachable!(),
                    }
                } else {
                    self.prg_bank_sel_32 as usize * 0x8000 + (addr as usize & 0x7fff)
                };
                Ok(nes.cartridge.prgmem[mapped_addr])
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
                    self.reg_control.update(self.reg_control.bits | 0x0c);
                } else {
                    self.reg_load >>= 1;
                    self.reg_load |= (data & 0x01) << 4;
                    self.load_count += 1;
                    if self.load_count == 5 {
                        match addr {
                            0x8000..=0x9fff => {
                                self.reg_control.update(self.reg_load & 0x1f);
                                nes.cartridge.mirroring = self.reg_control.mirroring();
                            }
                            0xa000..=0xbfff => {
                                if self.reg_control.chr_bank_mode() {
                                    self.chr_bank_sel_4.0 = self.reg_load & 0x1f;
                                } else {
                                    self.chr_bank_sel_8 = self.reg_load & 0x1e;
                                }
                            }
                            0xc000..=0xdfff => {
                                if self.reg_control.chr_bank_mode() {
                                    self.chr_bank_sel_4.1 = self.reg_load & 0x1f;
                                }
                            }
                            0xe000..=0xffff => match self.reg_control.prg_bank_mode() {
                                0 | 1 => {
                                    self.prg_bank_sel_32 = (self.reg_load & 0x0e) >> 1;
                                }
                                2 => {
                                    self.prg_bank_sel_16.0 = 0;
                                    self.prg_bank_sel_16.1 = self.reg_load & 0x0f;
                                }
                                3 => {
                                    self.prg_bank_sel_16.0 = self.reg_load & 0x0f;
                                    self.prg_bank_sel_16.1 = nes.cartridge.prg_banks - 1;
                                }
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
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
        let mapped_addr = if nes.cartridge.chr_banks == 0 {
            addr as usize
        } else if self.reg_control.chr_bank_mode() {
            match addr {
                0x0000..=0x0fff => {
                    self.chr_bank_sel_4.0 as usize * 0x1000 + (addr as usize & 0x0fff)
                }
                0x1000..=0x1fff => {
                    self.chr_bank_sel_4.1 as usize * 0x1000 + (addr as usize & 0x0fff)
                }
                _ => unreachable!(),
            }
        } else {
            self.chr_bank_sel_8 as usize * 0x1000 + (addr as usize & 0x1fff)
        };
        Ok(nes.cartridge.chrmem[mapped_addr])
    }

    fn write_chr(&mut self, nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        nes.cartridge.chrmem[addr as usize] = data;
        Ok(())
    }

    fn reset(&mut self, nes: &mut Nes<S, A>) -> Result<()> {
        self.reg_load = 0x00;
        self.load_count = 0;
        self.reg_control.update(0x1c);
        self.chr_bank_sel_4 = (0x00, 0x00);
        self.chr_bank_sel_8 = 0x00;
        self.prg_bank_sel_16.0 = 0x00;
        self.prg_bank_sel_16.1 = nes.cartridge.prg_banks - 1;
        self.prg_bank_sel_32 = 0;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "MMC1"
    }
}
