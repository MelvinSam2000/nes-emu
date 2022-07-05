use anyhow::anyhow;
use anyhow::Result;
use minifb::Window;

const VRAM_WIDTH: usize = 256;
const VRAM_HEIGHT: usize = 240;

pub struct VramScreen {
    buffer: [u32; VRAM_WIDTH * VRAM_HEIGHT],
    window: Window,
    i: usize,
}

/*
        |
   TL   |   TR
        |
        |
-----------------
        |
   BL   |   BR
        |
        |

*/
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

impl VramScreen {
    pub fn new(corner: Corner) -> Result<Self> {
        let mut window = Window::new("VRAM screen", VRAM_WIDTH, VRAM_HEIGHT, Default::default())?;
        match corner {
            Corner::TopLeft => {
                window.set_position(20 + 512, 20);
            },
            Corner::TopRight => {
                window.set_position(20 + 512 + VRAM_WIDTH as isize, 20);
            },
            Corner::BottomLeft => {
                window.set_position(20 + 512, 20 + VRAM_HEIGHT as isize);
            },
            Corner::BottomRight => {
                window.set_position(20 + 512 + VRAM_WIDTH as isize, 20 + VRAM_HEIGHT as isize);
            },
        }
        Ok(Self {
            buffer: [0u32; VRAM_WIDTH * VRAM_HEIGHT],
            window,
            i: 0,
        })
    }
}

impl ::nes::nesscreen::NesScreen for VramScreen {
    fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) -> Result<()> {
        if x as usize >= VRAM_WIDTH || y as usize >= VRAM_HEIGHT {
            return Err(anyhow!("Invalid index for VRAM Screen: x={} y={}", x, y));
        }
        self.buffer[x as usize + (y as usize) * VRAM_WIDTH] =
            ((rgb.0 as u32) << 16) | ((rgb.1 as u32) << 8) | rgb.2 as u32;
        if self.i == VRAM_WIDTH * VRAM_HEIGHT {
            self.window
                .update_with_buffer(&self.buffer, VRAM_WIDTH, VRAM_HEIGHT)?;
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
