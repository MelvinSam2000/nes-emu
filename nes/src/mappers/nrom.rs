use anyhow::Result;

use super::Mapper;
use crate::Nes;

// Mapper 0
pub struct Nrom;

impl<S, A> Mapper<S, A> for Nrom {
    fn read_prg(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        let mapped_addr = if 0x8000 <= addr {
            match nes.cartridge.prg_banks {
                1 => addr & 0x3fff,
                2 => addr & 0x7fff,
                _ => 0,
            }
        } else {
            0
        };
        Ok(nes.cartridge.prgmem[mapped_addr as usize])
    }

    fn write_prg(&mut self, nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        let mapped_addr = if 0x8000 <= addr {
            match nes.cartridge.prg_banks {
                1 => addr & 0x3fff,
                2 => addr & 0x7fff,
                _ => 0,
            }
        } else {
            0
        };
        nes.cartridge.prgmem[mapped_addr as usize] = data;
        Ok(())
    }

    fn read_chr(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        Ok(nes.cartridge.chrmem[addr as usize])
    }

    fn write_chr(&mut self, nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        nes.cartridge.chrmem[addr as usize] = data;
        Ok(())
    }

    fn reset(&mut self, _nes: &mut Nes<S, A>) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn name(&self) -> &'static str {
        "NROM"
    }
}
