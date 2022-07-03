use std::cmp::max;
use std::cmp::min;

#[derive(Default)]
pub struct TriangleChannel {
    pub muted: bool,
    pub period: u16, // in Hz, max 11 bits or 2^11 Hz = 2kHz
    pub enabled: bool,
}

impl TriangleChannel {
    pub fn set_period(&mut self, bits: u8, high: bool) {
        if !high {
            self.period = (self.period & 0xff00) | bits as u16;
        } else {
            self.period = (self.period & 0x00ff) | (((bits & 0b111) as u16) << 8);
        }
    }

    pub fn get_frequency(&mut self) -> u16 {
        let raw_freq = (55930.4 / (1 + self.period) as f32) as u16;
        let mut safe_freq = min(raw_freq, 2000);
        safe_freq = max(100, safe_freq);
        safe_freq
    }
}
