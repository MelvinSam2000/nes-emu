use anyhow::anyhow;
use anyhow::Result;

use super::Mapper;
use crate::Nes;

// Mapper 3
#[derive(Default)]
pub struct Cnrom {
    banksel: u8,
}

impl Mapper for Cnrom {
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

    fn write_prg(&mut self, _nes: &mut Nes, addr: u16, data: u8) -> Result<()> {
        match addr {
            0x8000..=0xffff => {
                self.banksel = (data & 0x03) as u8;
                Ok(())
            }
            _ => Err(anyhow!(
                "Cannot write at PRG address {:#x} for CNROM",
                addr
            )),
        }
    }

    fn read_chr(&mut self, nes: &mut Nes, addr: u16) -> Result<u8> {
        match addr {
            0x0000..=0x1fff => {
                let mapped_addr = (self.banksel as u16) * 0x2000 + addr;
                Ok(nes.cartridge.chrmem[mapped_addr as usize])
            }
            _ => Err(anyhow!("Cannot read at CHR address {:#x} for CNROM", addr)),
        }
    }

    fn write_chr(&mut self, _nes: &mut Nes, addr: u16, _data: u8) -> Result<()> {
        Err(anyhow!("Cannot write at CHR address {:#x} for CNROM", addr))
    }
}
