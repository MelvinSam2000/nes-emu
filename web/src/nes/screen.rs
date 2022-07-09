use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;

use super::NES_HEIGHT;
use super::NES_WIDTH;

pub struct Screen {
    ctx: CanvasRenderingContext2d,
    buffer: Box<[u8; NES_HEIGHT * NES_WIDTH * 4]>,
}

impl Screen {
    pub fn new() -> Result<Self> {
        let document = web_sys::window()
            .context("Cannot get window")?
            .document()
            .context("Cannot get document")?;
        let canvas: HtmlCanvasElement = document
            .get_element_by_id("nes-canvas")
            .context("Cannot get canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|err| anyhow!("Cannot cast canvas to html canvas due to: {:?}", err))?;

        let ctx = canvas
            .get_context("2d")
            .map_err(|err| anyhow!("Cannot get canvas 2d context due to: {:?}", err))?
            .context("Could not get 2d context")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|err| anyhow!("Cannot get canvas 2d context due to: {:?}", err))?;

        let buffer = Box::new([0u8; NES_HEIGHT * NES_WIDTH * 4]);

        Ok(Self { ctx, buffer })
    }
}

impl ::nes::nesscreen::NesScreen for Screen {
    fn draw_pixel(&mut self, x: u8, y: u8, rgb: (u8, u8, u8)) -> Result<()> {
        let offset = (y as usize * NES_WIDTH + x as usize) * 4;
        self.buffer[offset] = rgb.0;
        self.buffer[offset + 1] = rgb.1;
        self.buffer[offset + 2] = rgb.2;
        self.buffer[offset + 3] = 255;
        Ok(())
    }

    fn vblank(&mut self) -> Result<()> {
        let img_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(self.buffer.as_ref()),
            NES_WIDTH as u32,
            NES_HEIGHT as u32,
        )
        .map_err(|err| anyhow!("Could not create image data buffer due to err: {:?}", err))?;
        self.ctx
            .put_image_data(&img_data, 0., 0.)
            .map_err(|err| anyhow!("Cannot render image data due to: {:?}", err))?;
        Ok(())
    }
}
