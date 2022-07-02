use anyhow::Result;

use crate::Nes;

pub trait Mapper {
    fn read_prg(&mut self, nes: &mut Nes, addr: u16) -> Result<u8>;
    fn write_prg(&mut self, nes: &mut Nes, addr: u16, data: u8) -> Result<()>;
    fn read_chr(&mut self, nes: &mut Nes, addr: u16) -> Result<u8>;
    fn write_chr(&mut self, nes: &mut Nes, addr: u16, data: u8) -> Result<()>;
}

pub mod nrom;
