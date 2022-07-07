use anyhow::Result;

use crate::Nes;

pub trait Mapper<S, A> {
    fn read_prg(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8>;
    fn write_prg(&mut self, nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()>;
    fn read_chr(&mut self, nes: &mut Nes<S, A>, addr: u16) -> Result<u8>;
    fn write_chr(&mut self, nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()>;
}

pub mod cnrom;
pub mod gxrom;
pub mod mmc1;
pub mod nrom;
pub mod uxrom;
