use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;
use nes::joypad::Button;
use web_sys::AudioContext;

use self::audio::NesAudio;
use crate::nes::screen::Screen;

const NES_HEIGHT: usize = 240;
const NES_WIDTH: usize = 256;

pub struct Nes {
    nes: ::nes::Nes<Screen, NesAudio>,
    pub audio_ctx: Rc<AudioContext>,
}

impl Nes {
    pub fn new() -> Result<Self> {
        let screen = Screen::new()?;
        let audio =
            NesAudio::new().map_err(|err| anyhow!("Error initializing audio: {:?}", err))?;
        let audio_ctx = audio.get_audio_ctx();
        Ok(Self {
            nes: ::nes::Nes::new(screen, audio),
            audio_ctx,
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

pub mod audio;
pub mod screen;
