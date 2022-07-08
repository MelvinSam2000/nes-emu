use anyhow::anyhow;
use anyhow::Result;
use minifb::Scale;
use minifb::Window;
use minifb::WindowOptions;

pub struct PaletteScreen {
    buffer: [u32; 32],
    window: Window,
    i: usize,
}

impl PaletteScreen {
    pub fn new() -> Result<Self> {
        let win_options = WindowOptions {
            scale: Scale::X32,
            ..Default::default()
        };
        let mut window = Window::new("PALETTE screen", 16, 2, win_options)?;
        window.set_position(20, 25 + 256 * 2 + 128 * 2);
        Ok(Self {
            buffer: [0u32; 32],
            window,
            i: 0,
        })
    }
}

impl ::nes::nesscreen::NesScreen for PaletteScreen {
    fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) -> Result<()> {
        if x >= 16 || y >= 2 {
            return Err(anyhow!("Invalid index for PaletteScreen: x={} y={}", x, y));
        }
        self.buffer[x as usize + (y as usize) * 16] =
            ((rgb.0 as u32) << 16) | ((rgb.1 as u32) << 8) | rgb.2 as u32;
        if self.i == 32 {
            self.window.update_with_buffer(&self.buffer, 16, 2)?;
            self.i = 0;
        }
        self.i += 1;
        Ok(())
    }

    fn vblank(&mut self) -> Result<()> {
        // Do nothing
        Ok(())
    }
}
