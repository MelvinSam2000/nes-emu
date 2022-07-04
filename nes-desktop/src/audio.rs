use std::f32::consts::PI;

use anyhow::anyhow;
use anyhow::Result;
use nes::apu::AudioChannel;
use web_audio_api::context::AudioContext;
use web_audio_api::context::BaseAudioContext;
use web_audio_api::node::AudioNode;
use web_audio_api::node::AudioScheduledSourceNode;
use web_audio_api::node::GainNode;
use web_audio_api::node::OscillatorNode;
use web_audio_api::node::OscillatorType;
use web_audio_api::PeriodicWaveOptions;

const MAX_OSC_VOLUME: f32 = 0.05;

#[allow(dead_code)]
pub struct NesAudio {
    context: AudioContext,
    p1_osc: OscillatorNode,
    p2_osc: OscillatorNode,
    tri_osc: OscillatorNode,
    p1_gain: GainNode,
    p2_gain: GainNode,
    tri_gain: GainNode,
}

impl Default for NesAudio {
    fn default() -> Self {
        let context = AudioContext::default();

        // Create audio nodes
        let p1_osc = context.create_oscillator();
        let p2_osc = context.create_oscillator();
        let tri_osc = context.create_oscillator();

        let p1_gain = context.create_gain();
        let p2_gain = context.create_gain();
        let tri_gain = context.create_gain();
        let output_gain = context.create_gain();

        // Connect audio nodes
        output_gain.connect(&context.destination());
        p1_gain.connect(&output_gain);
        p2_gain.connect(&output_gain);
        tri_gain.connect(&output_gain);
        p1_osc.connect(&p1_gain);
        p2_osc.connect(&p2_gain);
        tri_osc.connect(&tri_gain);

        // Initialize types
        set_duty_cycle(&context, &p1_osc, 0.5);
        set_duty_cycle(&context, &p2_osc, 0.5);
        tri_osc.set_type(OscillatorType::Triangle);

        // Initialize parameters
        output_gain.gain().set_value(1f32);
        p1_gain.gain().set_value(0f32);
        p2_gain.gain().set_value(0f32);
        tri_gain.gain().set_value(0f32);

        // Start oscillators
        p1_osc.start();
        p2_osc.start();
        tri_osc.start();

        Self {
            context,
            p1_osc,
            p2_osc,
            tri_osc,
            p1_gain,
            p2_gain,
            tri_gain,
        }
    }
}

impl ::nes::nesaudio::NesAudio for NesAudio {
    fn enable_channel(&mut self, channel: ::nes::apu::AudioChannel, enabled: bool) -> Result<()> {
        match channel {
            AudioChannel::Pulse1 => {
                self.p1_gain
                    .gain()
                    .set_value(if enabled { MAX_OSC_VOLUME } else { 0f32 })
            }
            AudioChannel::Pulse2 => {
                self.p2_gain
                    .gain()
                    .set_value(if enabled { MAX_OSC_VOLUME } else { 0f32 })
            }
            AudioChannel::Triangle => {
                self.tri_gain
                    .gain()
                    .set_value(if enabled { MAX_OSC_VOLUME } else { 0f32 })
            }
        };
        Ok(())
    }

    fn update_pulse(
        &mut self,
        pulse: ::nes::apu::AudioChannel,
        duty_cycle: Option<f32>,
        volume: Option<f32>,
        freq: Option<u16>,
    ) -> Result<()> {
        match pulse {
            AudioChannel::Pulse1 => {
                if let Some(duty_cycle) = duty_cycle {
                    set_duty_cycle(&self.context, &self.p1_osc, duty_cycle);
                }
                if let Some(volume) = volume {
                    self.p1_gain.gain().set_value(volume);
                }
                if let Some(freq) = freq {
                    self.p1_osc.frequency().set_value(freq.into());
                }
                Ok(())
            }
            AudioChannel::Pulse2 => {
                if let Some(duty_cycle) = duty_cycle {
                    set_duty_cycle(&self.context, &self.p1_osc, duty_cycle);
                }
                if let Some(volume) = volume {
                    self.p2_gain.gain().set_value(volume);
                }
                if let Some(freq) = freq {
                    self.p2_osc.frequency().set_value(freq.into());
                }
                Ok(())
            }
            AudioChannel::Triangle => Err(anyhow!("Invalid argument: triangle pulse")),
        }
    }

    fn update_triangle(&mut self, freq: Option<u16>, mute: Option<bool>) -> Result<()> {
        if let Some(freq) = freq {
            self.tri_osc.frequency().set_value(freq.into());
        }
        if let Some(mute) = mute {
            self.tri_gain
                .gain()
                .set_value(if mute { 0f32 } else { MAX_OSC_VOLUME });
        }
        Ok(())
    }
}

fn set_duty_cycle(cx: &AudioContext, pulse: &OscillatorNode, dc: f32) {
    const SAMPLES: usize = 32;
    let mut real = vec![0f32; SAMPLES];
    let imag = vec![0f32; SAMPLES];
    real[0] = dc;
    for (i, real_i) in real.iter_mut().enumerate() {
        let npi = (i as f32) * PI;
        *real_i = (4. / npi) * (((npi * dc) as f32).sin());
    }
    let wave = cx.create_periodic_wave(PeriodicWaveOptions {
        real: Some(real),
        imag: Some(imag),
        disable_normalization: false,
    });
    pulse.set_periodic_wave(wave);
}
