use ::nes::nesaudio::NoAudio;
use anyhow::Result;

use crate::dk::DK_ROM;
use crate::nes::screen::Screen;

const NES_HEIGHT: usize = 240;
const NES_WIDTH: usize = 256;

pub struct Nes {
    nes: ::nes::Nes<Screen, NoAudio>,
}

impl Nes {
    pub fn new() -> Result<Self> {
        let screen = Screen::new()?;
        Ok(Self {
            nes: ::nes::Nes::new(screen, NoAudio),
        })
    }

    pub fn clock(&mut self) -> Result<()> {
        self.nes.clock()
    }

    pub fn reset(&mut self) -> Result<()> {
        self.nes.reset()
    }

    pub fn load(&mut self, _rom_bytes: &[u8]) -> Result<()> {
        self.nes.load(DK_ROM)
    }
}

pub enum NesEvent {
    Load(Vec<u8>),
    Reset,
}

pub mod screen;
