
use anyhow::Result;
use ::nes::nesaudio::NoAudio;
use ::nes::nesscreen::NesScreen;

pub struct Nes(pub ::nes::Nes);

impl Default for Nes {
    fn default() -> Self {
        Self(::nes::Nes::new(Box::new(NesCanvasScreen), Box::new(NoAudio)))
    }
}

struct NesCanvasScreen;

impl NesScreen for NesCanvasScreen {
    fn draw_pixel(&mut self, _x: u8, _y: u8, _rgb: (u8, u8, u8)) -> Result<()> {
        todo!()
    }
}