use anyhow::anyhow;
use anyhow::Result;

use super::Mapper;
use crate::Nes;

// Mapper 3
#[derive(Default)]
pub struct Cnrom {
    banksel: u8,
}

impl<S, A> Mapper<S, A> for Cnrom {
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

    fn write_prg(&mut self, _nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        match addr {
            0x8000..=0xffff => {
                self.banksel = (data & 0x03) as u8;
            }
            _ => log::warn!("Cannot write at PRG address {:#x} for CNROM", addr),
        }
        Ok(())
    }

    fn read_chr(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        match addr {
            0x0000..=0x1fff => {
                let mapped_addr = (self.banksel as u16) * 0x2000 + addr;
                Ok(nes.cartridge.chrmem[mapped_addr as usize])
            }
            _ => Err(anyhow!("Cannot read at CHR address {:#x} for CNROM", addr)),
        }
    }

    fn write_chr(&mut self, _nes: &mut Nes<S, A>, addr: u16, _data: u8) -> Result<()> {
        Err(anyhow!("Cannot write at CHR address {:#x} for CNROM", addr))
    }

    fn reset(&mut self, _nes: &mut Nes<S, A>) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn name(&self) -> &'static str {
        "CNROM"
    }
}
