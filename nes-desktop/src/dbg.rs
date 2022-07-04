use anyhow::anyhow;
use anyhow::Result;
use minifb::Scale;
use minifb::Window;
use minifb::WindowOptions;

const CHR_WIDTH: usize = 128;
const CHR_HEIGHT: usize = 128;

pub struct ChrScreen {
    buffer: [u32; CHR_WIDTH * CHR_HEIGHT],
    window: Window,
    i: usize,
}

impl ChrScreen {
    pub fn new(bank_one: bool) -> Result<Self> {
        let mut window = Window::new("CHR screen", 256, 256, Default::default())?;
        window.set_position(
            20 + if bank_one {
                0
            } else {
                CHR_WIDTH as isize * 2 + 5
            },
            25 + 256 * 2,
        );
        Ok(Self {
            buffer: [0u32; CHR_WIDTH * CHR_HEIGHT],
            window,
            i: 0,
        })
    }
}

impl ::nes::nesscreen::NesScreen for ChrScreen {
    fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) -> Result<()> {
        if x >= CHR_WIDTH as u8 || y >= CHR_HEIGHT as u8 {
            return Err(anyhow!("Invalid index for ChrScreen: x={} y={}", x, y));
        }
        self.buffer[x as usize + (y as usize) * CHR_WIDTH] =
            ((rgb.0 as u32) << 16) | ((rgb.1 as u32) << 8) | rgb.2 as u32;
        if self.i == CHR_WIDTH * CHR_HEIGHT {
            self.window
                .update_with_buffer(&self.buffer, CHR_WIDTH, CHR_HEIGHT)?;
            self.i = 0;
        }
        self.i += 1;
        Ok(())
    }
}

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
}
