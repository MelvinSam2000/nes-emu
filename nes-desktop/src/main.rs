use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use anyhow::Context;
use anyhow::Result;
use minifb::Window;
use minifb::WindowOptions;

use crate::nes::Nes;

const WIDTH: usize = 240;
const HEIGHT: usize = 256;

fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let window = Rc::new(RefCell::new(Window::new(
        "Nes Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?));

    log::info!("Starting NES Emulator...");
    let mut nes = Nes::new(window.clone());

    let nes_rom_path = std::env::args().collect::<Vec<String>>();
    let nes_rom_path = nes_rom_path
        .get(1)
        .context("Cannot get file from CLI arguments")?;
    let game_rom = fs::read(&nes_rom_path)?;
    nes.load(&game_rom)?;

    log::info!("Loaded game {:?}", &nes_rom_path);

    while window.borrow().is_open() {
        nes.poll_key_press()?;
        if let Err(err) = nes.clock() {
            log::error!("Game crahed due to err: {}", err);
            break;
        }
    }
    Ok(())
}

pub mod audio;
pub mod nes;
pub mod screen;
