use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use anyhow::Result;
use minifb::Key;
use minifb::Window;
use nes::joypad::Button;

use crate::audio::NesAudio;
use crate::screen::NesScreen;

pub struct Nes {
    nes: ::nes::Nes,
    window: Rc<RefCell<Window>>,
}

impl Nes {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        window
            .borrow_mut()
            .limit_update_rate(Some(Duration::from_micros(16600)));
        Self {
            nes: ::nes::Nes::new(
                Box::new(NesScreen::new(window.clone())),
                Box::new(NesAudio::default()),
            ),
            window,
        }
    }

    pub fn clock(&mut self) -> Result<()> {
        self.nes.clock()
    }

    pub fn step(&mut self) -> Result<()> {
        let inst = self.nes.step()?;
        log::info!("{inst}");
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
            if window.is_key_down(Key::Z) {
                self.nes.press_btn(Button::START, true)?;
            } else {
                self.nes.release_btn(Button::START, true)?;
            }
            if window.is_key_down(Key::X) {
                self.nes.press_btn(Button::SELECT, true)?;
            } else {
                self.nes.release_btn(Button::SELECT, true)?;
            }
        }
        Ok(())
    }
}
