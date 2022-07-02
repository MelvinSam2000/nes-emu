use anyhow::Result;
use buscpu::BusCpu;
use cartridge::Cartridge;

use crate::cpu::Cpu;

#[derive(Default)]
pub struct Nes {
    cpu: Cpu,
    bus_cpu: BusCpu,
    cartridge: Cartridge,
}

impl Nes {
    pub fn reset(&mut self) -> Result<()> {
        cpu::reset(self)
    }

    pub fn clock(&mut self) -> Result<()> {
        cpu::clock(self)?;
        Ok(())
    }

    pub fn load(&mut self, rom_bytes: &[u8]) -> Result<()> {
        cartridge::load_cartridge(self, rom_bytes)
    }
}

pub mod buscpu;
pub mod cartridge;
pub mod cpu;
pub mod mappers;

#[cfg(test)]
mod tests {
    mod cpu;
}
