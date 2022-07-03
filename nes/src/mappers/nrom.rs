use anyhow::Result;

use super::Mapper;
use crate::Nes;

// Mapper 0
pub struct Nrom;

impl Mapper for Nrom {
    fn read_prg(&mut self, nes: &mut Nes, addr: u16) -> Result<u8> {
        let mut mapped_addr = 0;
        if 0x8000 <= addr {
            if nes.cartridge.prg_banks == 2 {
                mapped_addr = addr & 0x7fff;
            }
            if nes.cartridge.prg_banks == 1 {
                mapped_addr = addr & 0x3fff;
            }
        }
        Ok(nes.cartridge.prgmem[mapped_addr as usize])
    }

    fn write_prg(&mut self, nes: &mut Nes, addr: u16, data: u8) -> Result<()> {
        let mut mapped_addr = 0;
        if 0x8000 <= addr {
            if nes.cartridge.prg_banks == 2 {
                mapped_addr = addr & 0x7fff;
            }
            if nes.cartridge.prg_banks == 1 {
                mapped_addr = addr & 0x3fff;
            }
        }
        nes.cartridge.prgmem[mapped_addr as usize] = data;
        Ok(())
    }

    fn read_chr(&mut self, nes: &mut Nes, addr: u16) -> Result<u8> {
        Ok(nes.cartridge.chrmem[addr as usize])
    }

    fn write_chr(&mut self, nes: &mut Nes, addr: u16, data: u8) -> Result<()> {
        nes.cartridge.chrmem[addr as usize] = data;
        Ok(())
    }
}
