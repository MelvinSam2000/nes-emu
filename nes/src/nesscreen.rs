use anyhow::Result;

pub trait NesScreen {
    fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) -> Result<()>;
    fn vblank(&mut self) -> Result<()>;
}

pub struct NoScreen;

impl NesScreen for NoScreen {
    fn draw_pixel(&mut self, _x: u8, _y: u8, _rgb: (u8, u8, u8)) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn vblank(&mut self) -> Result<()> {
        // Do nothing
        Ok(())
    }
}
