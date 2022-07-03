use anyhow::Result;

pub trait NesScreen {
    fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) -> Result<()>;
}

pub struct NoScreen;

impl NesScreen for NoScreen {
    fn draw_pixel(&mut self, _: u8, _: u8, _: (u8, u8, u8)) -> Result<()> {
        Ok(())
    }
}
