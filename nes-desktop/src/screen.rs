use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use minifb::Window;

use crate::HEIGHT;
use crate::WIDTH;

pub struct NesScreen {
    buffer: [u32; WIDTH * HEIGHT],
    window: Rc<RefCell<Window>>,
    i: usize,
}

impl NesScreen {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        // Limit to max ~30 fps update rate
        window
            .borrow_mut()
            .limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        Self {
            buffer: [0u32; WIDTH * HEIGHT],
            window,
            i: 0,
        }
    }
}

impl ::nes::nesscreen::NesScreen for NesScreen {
    fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) -> Result<()> {
        self.buffer[x as usize + (y as usize) * WIDTH] =
            ((rgb.0 as u32) << 16) | ((rgb.1 as u32) << 8) | rgb.0 as u32;
        if self.i == WIDTH * HEIGHT {
            self.window
                .borrow_mut()
                .update_with_buffer(&self.buffer, WIDTH, HEIGHT)?;
            self.i = 0;
        }
        self.i += 1;
        Ok(())
    }
}
