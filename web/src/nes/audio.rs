use std::f32::consts::PI;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;
use nes::apu::AudioChannel;
use wasm_bindgen::JsValue;
use web_sys::AudioContext;
use web_sys::GainNode;
use web_sys::OscillatorNode;
use web_sys::OscillatorType;
use web_sys::PeriodicWave;
use web_sys::PeriodicWaveOptions;

const MAX_OSC_VOLUME: f32 = 0.05;

pub struct NesAudio {
    context: Rc<AudioContext>,
    p1_osc: OscillatorNode,
    p2_osc: OscillatorNode,
    tri_osc: OscillatorNode,
    p1_gain: GainNode,
    p2_gain: GainNode,
    tri_gain: GainNode,
}

impl NesAudio {
    pub fn new() -> Result<Self, JsValue> {
        let context = Rc::new(AudioContext::new()?);

        // Create audio nodes
        let p1_osc = context.create_oscillator()?;
        let p2_osc = context.create_oscillator()?;
        let tri_osc = context.create_oscillator()?;

        let p1_gain = context.create_gain()?;
        let p2_gain = context.create_gain()?;
        let tri_gain = context.create_gain()?;
        let output_gain = context.create_gain()?;

        // Connect audio nodes
        output_gain.connect_with_audio_node(&context.destination())?;
        p1_gain.connect_with_audio_node(&output_gain)?;
        p2_gain.connect_with_audio_node(&output_gain)?;
        tri_gain.connect_with_audio_node(&output_gain)?;
        p1_osc.connect_with_audio_node(&p1_gain)?;
        p2_osc.connect_with_audio_node(&p2_gain)?;
        tri_osc.connect_with_audio_node(&tri_gain)?;

        // Initialize types
        set_duty_cycle(&context, &p1_osc, 0.5)
            .map_err(|err| JsValue::from_str(&err.to_string()))?;
        set_duty_cycle(&context, &p2_osc, 0.5)
            .map_err(|err| JsValue::from_str(&err.to_string()))?;
        tri_osc.set_type(OscillatorType::Triangle);

        // Initialize parameters
        output_gain.gain().set_value(1.);
        p1_gain.gain().set_value(0.);
        p2_gain.gain().set_value(0.);
        tri_gain.gain().set_value(0.);

        // Start oscillators
        p1_osc.start()?;
        p2_osc.start()?;
        tri_osc.start()?;

        Ok(Self {
            context,
            p1_osc,
            p2_osc,
            tri_osc,
            p1_gain,
            p2_gain,
            tri_gain,
        })
    }

    pub fn get_audio_ctx(&self) -> Rc<AudioContext> {
        self.context.clone()
    }
}

impl ::nes::nesaudio::NesAudio for NesAudio {
    fn enable_channel(&mut self, channel: ::nes::apu::AudioChannel, enabled: bool) -> Result<()> {
        match channel {
            AudioChannel::Pulse1 => {
                self.p1_gain
                    .gain()
                    .set_value(if enabled { MAX_OSC_VOLUME } else { 0. })
            }
            AudioChannel::Pulse2 => {
                self.p2_gain
                    .gain()
                    .set_value(if enabled { MAX_OSC_VOLUME } else { 0. })
            }
            AudioChannel::Triangle => {
                self.tri_gain
                    .gain()
                    .set_value(if enabled { MAX_OSC_VOLUME } else { 0. })
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
                    set_duty_cycle(&self.context, &self.p1_osc, duty_cycle)?;
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
                    set_duty_cycle(&self.context, &self.p1_osc, duty_cycle)?;
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
                .set_value(if mute { 0. } else { MAX_OSC_VOLUME });
        }
        Ok(())
    }
}

fn set_duty_cycle(cx: &AudioContext, pulse: &OscillatorNode, dc: f32) -> Result<()> {
    const SAMPLES: usize = 32;
    let mut real = vec![0f32; SAMPLES];
    real[0] = dc;
    real.iter_mut().enumerate().for_each(|(i, real_i)| {
        let npi = (i as f32) * PI;
        *real_i = (4. / npi) * (((npi * dc) as f32).sin());
    });

    let mut options = PeriodicWaveOptions::new();
    options.disable_normalization(false);
    let real = JsValue::from(
        &real
            .into_iter()
            .enumerate()
            .map(|(i, elem)| JsValue::from_f64(if i == 0 { 0. } else { elem as f64 }))
            .collect::<js_sys::Array>(),
    );

    options.real(&real);
    let wave = PeriodicWave::new_with_options(cx, &options)
        .map_err(|err| anyhow!("Failed to set duty cycle to {} due to err {:?}", dc, err))?;
    pulse.set_periodic_wave(&wave);
    Ok(())
}

impl Drop for NesAudio {
    fn drop(&mut self) {
        let _ = self.context.close();
    }
}
