use std::cell::RefCell;
use std::rc::Rc;

use ::nes::joypad::Button;
use anyhow::Result;
use minifb::Key;
use minifb::Window;

use crate::audio::NesAudio;
use crate::dbg::chrscreen::ChrScreen;
use crate::dbg::palettescreen::PaletteScreen;
use crate::dbg::vramscreen::Corner;
use crate::dbg::vramscreen::VramScreen;
use crate::screen::NesScreen;

pub struct Nes {
    nes: ::nes::Nes<NesScreen, NesAudio>,
    window: Rc<RefCell<Window>>,
    dbg_chr: Option<[ChrScreen; 2]>,
    dbg_vram: Option<[VramScreen; 4]>,
    dbg_palette: Option<PaletteScreen>,
    clock: u16,
}

impl Nes {
    pub fn new(window: Rc<RefCell<Window>>) -> Result<Self> {
        window.try_borrow_mut()?.set_position(20, 20);

        let (dbg_chr, dbg_vram, dbg_palette) = if cfg!(feature = "screens") {
            let dbg_chr = Some([ChrScreen::new(true)?, ChrScreen::new(false)?]);
            let dbg_vram = Some([
                VramScreen::new(Corner::TopLeft)?,
                VramScreen::new(Corner::TopRight)?,
                VramScreen::new(Corner::BottomLeft)?,
                VramScreen::new(Corner::BottomRight)?,
            ]);
            let dbg_palette = Some(PaletteScreen::new()?);
            (dbg_chr, dbg_vram, dbg_palette)
        } else {
            (None, None, None)
        };

        Ok(Self {
            nes: ::nes::Nes::new(NesScreen::new(window.clone()), NesAudio::default()),
            window,
            dbg_chr,
            dbg_vram,
            dbg_palette,
            clock: 0,
        })
    }

    pub fn clock(&mut self) -> Result<()> {
        if cfg!(feature = "step") {
            let inst = self.nes.step()?;
            println!("{inst}");
        } else {
            self.nes.clock()?;
        }
        self.clock = self.clock.wrapping_add(1);
        if self.clock == 0 && cfg!(feature = "screens") {
            ::nes::ppu::draw_chr(&mut self.nes, 0, &mut self.dbg_chr.as_mut().unwrap()[0])?;
            ::nes::ppu::draw_chr(&mut self.nes, 1, &mut self.dbg_chr.as_mut().unwrap()[1])?;
            ::nes::ppu::draw_vram(&mut self.nes, 0, &mut self.dbg_vram.as_mut().unwrap()[0])?;
            ::nes::ppu::draw_vram(&mut self.nes, 1, &mut self.dbg_vram.as_mut().unwrap()[1])?;
            ::nes::ppu::draw_vram(&mut self.nes, 2, &mut self.dbg_vram.as_mut().unwrap()[2])?;
            ::nes::ppu::draw_vram(&mut self.nes, 3, &mut self.dbg_vram.as_mut().unwrap()[3])?;
            ::nes::ppu::draw_palette(&mut self.nes, self.dbg_palette.as_mut().unwrap())?;
        }
        Ok(())
    }

    pub fn load(&mut self, rom_bytes: &[u8]) -> Result<()> {
        self.nes.load(rom_bytes)?;
        self.nes.reset()?;
        Ok(())
    }

    pub fn poll_key_press(&mut self) -> Result<()> {
        let window = self.window.try_borrow();
        if let Ok(window) = window {
            if window.is_key_down(Key::Up) {
                self.nes.press_btn(Button::UP, true)?;
            } else {
                self.nes.release_btn(Button::UP, true)?;
            }
            if window.is_key_down(Key::Down) {
                self.nes.press_btn(Button::DOWN, true)?;
            } else {
                self.nes.release_btn(Button::DOWN, true)?;
            }
            if window.is_key_down(Key::Right) {
                self.nes.press_btn(Button::RIGHT, true)?;
            } else {
                self.nes.release_btn(Button::RIGHT, true)?;
            }
            if window.is_key_down(Key::Left) {
                self.nes.press_btn(Button::LEFT, true)?;
            } else {
                self.nes.release_btn(Button::LEFT, true)?;
            }
            if window.is_key_down(Key::A) {
                self.nes.press_btn(Button::BTN_A, true)?;
            } else {
                self.nes.release_btn(Button::BTN_A, true)?;
            }
            if window.is_key_down(Key::S) {
                self.nes.press_btn(Button::BTN_B, true)?;
            } else {
                self.nes.release_btn(Button::BTN_B, true)?;
            }
            if window.is_key_down(Key::Enter) {
                self.nes.press_btn(Button::START, true)?;
            } else {
                self.nes.release_btn(Button::START, true)?;
            }
            if window.is_key_down(Key::Space) {
                self.nes.press_btn(Button::SELECT, true)?;
            } else {
                self.nes.release_btn(Button::SELECT, true)?;
            }
        }
        Ok(())
    }
}
