use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use ::nes::joypad::Button;
use anyhow::Result;
use minifb::Key;
use minifb::Window;

use crate::audio::NesAudio;
use crate::commands;
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
    command_recv: Receiver<String>,
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

        // Spawn command thread
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            tx.send(buffer).unwrap();
        });

        Ok(Self {
            nes: ::nes::Nes::new(NesScreen::new(window.clone()), NesAudio::default()),
            window,
            dbg_chr,
            dbg_vram,
            dbg_palette,
            clock: 0,
            command_recv: rx,
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
        let nes = &mut self.nes;
        if let Ok(window) = window {
            Self::poll_single_key(nes, &window, Key::Up, Button::Up)?;
            Self::poll_single_key(nes, &window, Key::Down, Button::Down)?;
            Self::poll_single_key(nes, &window, Key::Right, Button::Right)?;
            Self::poll_single_key(nes, &window, Key::Left, Button::Left)?;
            Self::poll_single_key(nes, &window, Key::A, Button::A)?;
            Self::poll_single_key(nes, &window, Key::S, Button::B)?;
            Self::poll_single_key(nes, &window, Key::Enter, Button::Start)?;
            Self::poll_single_key(nes, &window, Key::Space, Button::Select)?;
        }
        Ok(())
    }

    pub fn poll_command(&mut self) -> Result<()> {
        if let Ok(msg) = self.command_recv.try_recv() {
            match commands::parse(&msg[..msg.len() - 1]) {
                Ok(cmd) => commands::exec(cmd, &mut self.nes)?,
                Err(err) => log::error!("{:?}", err),
            }
        }
        Ok(())
    }

    fn poll_single_key(
        nes: &mut ::nes::Nes<NesScreen, NesAudio>,
        window: &Window,
        key: Key,
        button: Button,
    ) -> Result<()> {
        if window.is_key_down(key) {
            nes.press_btn(button, true)
        } else {
            nes.release_btn(button, true)
        }
    }
}
