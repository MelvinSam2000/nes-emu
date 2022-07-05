use anyhow::anyhow;
use anyhow::Result;

use super::Mapper;
use crate::Nes;

// Mapper 66
#[derive(Default)]
pub struct Gxrom {
    banksel: (u8, u8),
}

impl<S, A> Mapper<S, A> for Gxrom {
    fn read_prg(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        match addr {
            0x8000..=0xffff => {
                let mapped_addr = (self.banksel.0 as u16) * 0x8000 + (addr & 0x7fff);
                Ok(nes.cartridge.prgmem[mapped_addr as usize])
            }
            _ => Err(anyhow!("Cannot read at PRG address {:#x} for GXROM", addr)),
        }
    }

    fn write_prg(&mut self, _nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        match addr {
            0x8000..=0xffff => {
                self.banksel = (data & 0x03, (data & 0x30) >> 4);
                Ok(())
            }
            _ => Err(anyhow!("Cannot write at PRG address {:#x} for GXROM", addr)),
        }
    }

    fn read_chr(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        match addr {
            0x0000..=0x1fff => {
                let mapped_addr = (self.banksel.1 as u16) * 0x2000 + addr;
                Ok(nes.cartridge.chrmem[mapped_addr as usize])
            }
            _ => Err(anyhow!("Cannot read at CHR address {:#x} for GXROM", addr)),
        }
    }

    fn write_chr(&mut self, _nes: &mut Nes<S, A>, addr: u16, _data: u8) -> Result<()> {
        Err(anyhow!("Cannot write at CHR address {:#x} for GXROM", addr))
    }
}
