use ::nes::nesaudio::NoAudio;
use anyhow::Result;
use nes::joypad::Button;

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

    pub fn load(&mut self, rom_bytes: &[u8]) -> Result<()> {
        self.nes.load(rom_bytes)
    }

    pub fn press_btn(&mut self, btn: Button) -> Result<()> {
        self.nes.press_btn(btn, true)
    }

    pub fn release_btn(&mut self, btn: Button) -> Result<()> {
        self.nes.release_btn(btn, true)
    }
}

pub mod screen;
