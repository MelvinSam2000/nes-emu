use anyhow::anyhow;
use anyhow::Result;

use super::Mapper;
use crate::Nes;

// Mapper 66
#[derive(Default)]
pub struct Gxrom {
    prg_banksel: u8,
    chr_banksel: u8,
}

impl<S, A> Mapper<S, A> for Gxrom {
    fn read_prg(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        match addr {
            0x8000..=0xffff => {
                let mapped_addr = (self.prg_banksel as usize * 0x8000) + (addr as usize & 0x7fff);
                Ok(nes.cartridge.prgmem[mapped_addr])
            }
            _ => {
                log::warn!("Cannot read at PRG address {:#x} for GXROM", addr);
                Ok(0)
            }
        }
    }

    fn write_prg(&mut self, _nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        match addr {
            0x8000..=0xffff => {
                self.prg_banksel = (data & 0x30) >> 4;
                self.chr_banksel = data & 0x03;
                Ok(())
            }
            _ => Err(anyhow!("Cannot write at PRG address {:#x} for GXROM", addr)),
        }
    }

    fn read_chr(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        let mapped_addr = (self.chr_banksel as usize * 0x2000) + (addr as usize & 0x1fff);
        Ok(nes.cartridge.chrmem[mapped_addr])
    }

    fn write_chr(&mut self, _nes: &mut Nes<S, A>, addr: u16, _data: u8) -> Result<()> {
        Err(anyhow!("Cannot write at CHR address {:#x} for GXROM", addr))
    }

    fn reset(&mut self, _nes: &mut Nes<S, A>) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn name(&self) -> &'static str {
        "GxROM"
    }
}
