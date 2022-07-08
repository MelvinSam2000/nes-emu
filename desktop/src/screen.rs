use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use anyhow::Result;
use minifb::Window;

use crate::HEIGHT;
use crate::WIDTH;

pub struct NesScreen {
    buffer: [u32; WIDTH * HEIGHT],
    window: Rc<RefCell<Window>>,
}

impl NesScreen {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        window
            .borrow_mut()
            .limit_update_rate(Some(Duration::from_micros(16600)));
        Self {
            buffer: [0u32; WIDTH * HEIGHT],
            window,
        }
    }
}

impl ::nes::nesscreen::NesScreen for NesScreen {
    fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) -> Result<()> {
        self.buffer[x as usize + (y as usize) * WIDTH] =
            ((rgb.0 as u32) << 16) | ((rgb.1 as u32) << 8) | rgb.2 as u32;
        Ok(())
    }

    fn vblank(&mut self) -> Result<()> {
        self.window
            .try_borrow_mut()?
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)?;
        Ok(())
    }
}
