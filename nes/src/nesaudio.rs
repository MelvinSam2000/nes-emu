use anyhow::Result;

use crate::apu::AudioChannel;

pub trait NesAudio {
    fn enable_channel(&mut self, channel: AudioChannel, enabled: bool) -> Result<()>;
    fn update_pulse(
        &mut self,
        pulse: AudioChannel,
        duty_cycle: Option<f32>,
        volume: Option<f32>,
        freq: Option<u16>,
    ) -> Result<()>;
    fn update_triangle(&mut self, freq: Option<u16>, mute: Option<bool>) -> Result<()>;
}

pub struct NoAudio;

impl NesAudio for NoAudio {
    fn enable_channel(&mut self, _channel: AudioChannel, _enabled: bool) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn update_pulse(
        &mut self,
        _pulse: AudioChannel,
        _duty_cycle: Option<f32>,
        _volume: Option<f32>,
        _freq: Option<u16>,
    ) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn update_triangle(&mut self, _freq: Option<u16>, _mute: Option<bool>) -> Result<()> {
        // Do nothing
        Ok(())
    }
}
