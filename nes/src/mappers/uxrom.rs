use anyhow::anyhow;
use anyhow::Result;

use super::Mapper;
use crate::Nes;

// Mapper 2
#[derive(Default)]
pub struct Uxrom {
    banksel: u8,
}

impl<S, A> Mapper<S, A> for Uxrom {
    fn read_prg(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        let mut mapped_addr = 0u64;
        match addr {
            0x8000..=0xbfff => {
                mapped_addr = (self.banksel as u64) * 0x4000 + (addr & 0x3fff) as u64;
            }
            0xc000..=0xffff => {
                let prgbank_size: u16 = nes.cartridge.prg_banks as u16 - 1;
                mapped_addr = (prgbank_size as u64) * 0x4000 + (addr & 0x3fff) as u64;
            }
            _ => Err(anyhow!("Cannot read at PRG address {:#x} for UXROM", addr))?,
        };
        Ok(nes.cartridge.prgmem[mapped_addr as usize])
    }

    fn write_prg(&mut self, _nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        match addr {
            0x8000..=0xffff => {
                self.banksel = data & 0x0f;
            }
            _ => log::warn!("Cannot write at PRG address {:#x} for UXROM", addr),
        }
        Ok(())
    }

    fn read_chr(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
        Ok(nes.cartridge.chrmem[addr as usize])
    }

    fn write_chr(&mut self, nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
        nes.cartridge.chrmem[addr as usize] = data;
        Ok(())
    }
}
