use anyhow::anyhow;
use anyhow::Result;

use self::pulse::PulseChannel;
use self::triangle::TriangleChannel;
use crate::nesaudio::NesAudio;
use crate::Nes;

pub enum AudioChannel {
    Pulse1,
    Pulse2,
    Triangle,
}

#[derive(Default)]
pub struct Apu {
    pulse1: PulseChannel,
    pulse2: PulseChannel,
    triangle: TriangleChannel,
}

pub fn read<S, A>(_nes: &mut Nes<S, A>, _addr: u16) -> Result<u8> {
    Err(anyhow!("Cannot read anything from APU..."))
}

pub fn write<S, A>(nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()>
where
    A: NesAudio,
{
    match addr {
        // PULSE 1
        0x4000 => {
            nes.apu.pulse1.set_dutycycle((data & 0b11000000) >> 6)?;
            nes.apu.pulse1.set_volume(data & 0x0f)?;
            let PulseChannel {
                duty_cycle,
                period: _,
                volume,
                enabled: _,
            } = nes.apu.pulse1;
            nes.audio
                .update_pulse(AudioChannel::Pulse1, Some(duty_cycle), Some(volume), None)?;
        }
        0x4002 => {
            nes.apu.pulse1.set_period(data, false);
            let period = nes.apu.pulse1.period;
            let freq = pulse::get_frequency(period);
            nes.audio
                .update_pulse(AudioChannel::Pulse1, None, None, Some(freq))?;
        }
        0x4003 => {
            nes.apu.pulse1.set_period(data, true);
            let period = nes.apu.pulse1.period;
            let freq = pulse::get_frequency(period);
            nes.audio
                .update_pulse(AudioChannel::Pulse1, None, None, Some(freq))?;
        }
        // PULSE 2
        0x4004 => {
            nes.apu.pulse2.set_dutycycle((data & 0b11000000) >> 6)?;
            nes.apu.pulse2.set_volume(data & 0x0f)?;
            let PulseChannel {
                duty_cycle,
                period: _,
                volume,
                enabled: _,
            } = nes.apu.pulse2;

            nes.audio
                .update_pulse(AudioChannel::Pulse2, Some(duty_cycle), Some(volume), None)?;
        }
        0x4006 => {
            nes.apu.pulse2.set_period(data, false);
            let period = nes.apu.pulse2.period;
            let freq = pulse::get_frequency(period);
            nes.audio
                .update_pulse(AudioChannel::Pulse2, None, None, Some(freq))?;
        }
        0x4007 => {
            nes.apu.pulse2.set_period(data, true);
            let period = nes.apu.pulse2.period;
            let freq = pulse::get_frequency(period);
            nes.audio
                .update_pulse(AudioChannel::Pulse2, None, None, Some(freq))?;
        }
        // TRIANGLE
        0x4008 => {
            let val = data & 0b11000000 != 0;
            nes.apu.triangle.muted = val;
            nes.audio.update_triangle(None, Some(val))?;
        }
        0x400a => {
            nes.apu.triangle.set_period(data, false);
            let freq = nes.apu.triangle.get_frequency();
            nes.audio.update_triangle(Some(freq), None)?;
        }
        0x400b => {
            nes.apu.triangle.set_period(data, true);
            let freq = nes.apu.triangle.get_frequency();
            nes.audio.update_triangle(Some(freq), None)?;
        }
        0x4015 => {
            let p1_enabled = nes.apu.pulse1.enabled;
            let p2_enabled = nes.apu.pulse2.enabled;
            let t_enabled = nes.apu.triangle.enabled;
            nes.apu.pulse1.enabled = data & (1 << 0) != 0;
            nes.apu.pulse2.enabled = data & (1 << 1) != 0;
            nes.apu.triangle.enabled = data & (1 << 2) != 0;
            if p1_enabled != nes.apu.pulse1.enabled {
                nes.audio
                    .enable_channel(AudioChannel::Pulse1, !p1_enabled)?;
            }
            if p2_enabled != nes.apu.pulse2.enabled {
                nes.audio
                    .enable_channel(AudioChannel::Pulse2, !p2_enabled)?;
            }
            if t_enabled != nes.apu.triangle.enabled {
                nes.audio
                    .enable_channel(AudioChannel::Triangle, !t_enabled)?;
            }
        }
        0x4001 | 0x4005 | 0x4009 | 0x400a..=0x401f => {
            log::warn!("Writing address {:#x} of APU is ignored.", addr);
        }
        _ => {
            Err(anyhow!("Cannot write addr {:#x} of APU", addr))?;
        }
    };
    Ok(())
}

pub mod pulse;
pub mod triangle;
