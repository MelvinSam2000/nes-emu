use std::cmp::max;
use std::cmp::min;

use anyhow::anyhow;
use anyhow::Result;

pub struct PulseChannel {
    pub volume: f32,     // 0 to 1 (percentage)
    pub duty_cycle: f32, // 12.5, 25, 50 or 75 (%)
    pub period: u16,     // in Hz, max 11 bits or 2^11 Hz = 2kHz
    pub enabled: bool,
}

impl Default for PulseChannel {
    fn default() -> Self {
        Self {
            volume: 0.,
            duty_cycle: 0.125,
            period: 0,
            enabled: false,
        }
    }
}

impl PulseChannel {
    pub fn set_volume(&mut self, bits: u8) -> Result<()> {
        self.volume = match bits {
            0..=15 => bits as f32 / 15.,
            _ => Err(anyhow!("Invalid volume bits: {:#x}", bits))?,
        };
        Ok(())
    }

    pub fn set_dutycycle(&mut self, bits: u8) -> Result<()> {
        self.duty_cycle = match bits {
            0 => 0.125,
            1 => 0.25,
            2 => 0.5,
            3 => 0.75,
            _ => Err(anyhow!("Invalid duty cycle bits: {:#x}", bits))?,
        };
        Ok(())
    }

    pub fn set_period(&mut self, bits: u8, high: bool) {
        if !high {
            self.period = (self.period & 0xff00) | bits as u16;
        } else {
            self.period = (self.period & 0x00ff) | (((bits & 0b111) as u16) << 8);
        }
    }
}

pub fn get_frequency(period: u16) -> u16 {
    let raw_freq = (111860.8 / (1 + period) as f32) as u16;
    let mut safe_freq = min(raw_freq, 2000);
    safe_freq = max(100, safe_freq);
    safe_freq
}
