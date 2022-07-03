#[derive(Default)]
pub struct RegScroll {
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub latch: bool,
}

impl RegScroll {
    pub fn write(&mut self, data: u8) {
        if !self.latch {
            self.scroll_x = data;
        } else {
            self.scroll_y = data;
        }
        self.latch = !self.latch;
    }
}
