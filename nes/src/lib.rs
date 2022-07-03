use anyhow::Result;

use crate::apu::Apu;
use crate::buscpu::BusCpu;
use crate::busppu::BusPpu;
use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::joypad::Joypad;
use crate::nesaudio::NesAudio;
use crate::nesscreen::NesScreen;
use crate::ppu::Ppu;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    apu: Apu,
    bus_cpu: BusCpu,
    bus_ppu: BusPpu,
    cartridge: Cartridge,
    joypad: Joypad,
    screen: Box<dyn NesScreen>,
    audio: Box<dyn NesAudio>,
}

impl Nes {
    pub fn new(screen: Box<dyn NesScreen>, audio: Box<dyn NesAudio>) -> Self {
        Self {
            cpu: Cpu::default(),
            ppu: Ppu::default(),
            apu: Apu::default(),
            bus_cpu: BusCpu::default(),
            bus_ppu: BusPpu::default(),
            cartridge: Cartridge::default(),
            joypad: Joypad::default(),
            screen,
            audio,
        }
    }

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

pub mod apu;
pub mod buscpu;
pub mod busppu;
pub mod cartridge;
pub mod cpu;
pub mod joypad;
pub mod mappers;
pub mod nesaudio;
pub mod nesscreen;
pub mod ppu;

#[cfg(test)]
mod tests {
    mod cpu;
}
