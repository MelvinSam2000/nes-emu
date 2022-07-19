use std::time::Duration;

use ::nes::joypad::Button;
use anyhow::anyhow;
use anyhow::Result;
use fluvio_wasm_timer::Delay;
use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::SinkExt;
use gloo_file::callbacks::FileReader;
use gloo_file::Blob;
use wasm_bindgen_futures::spawn_local;
use wasm_logger::Config;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::nes::Nes;

const CHANNEL_LEN: usize = 50;

struct CNes {
    nes_channel: mpsc::Sender<NesMessage>,
    load_signal: Option<oneshot::Sender<()>>,
    file_reader: Option<FileReader>,
}

pub enum NesMessage {
    Load(Vec<u8>),
    Reset,
    ButtonPress(Button),
    ButtonRelease(Button),
    UtilsLoadingFile(Blob),
}

async fn nes_thread(
    mut rx: mpsc::Receiver<NesMessage>,
    load_signal: oneshot::Receiver<()>,
) -> Result<()> {
    use self::NesMessage::*;

    let mut nes = Nes::new()?;
    load_signal.await?;
    let ctx = nes.audio_ctx.clone();
    let promise = ctx
        .resume()
        .map_err(|_| anyhow!("Cannot resume audio context"))?;
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| anyhow!("Cannot resume audio context"))?;

    loop {
        if let Ok(Some(msg)) = rx.try_next() {
            match msg {
                Load(rom_bytes) => {
                    nes.load(&rom_bytes)?;
                    nes.reset()?;
                }
                Reset => nes.reset()?,
                ButtonPress(btn) => nes.press_btn(btn)?,
                ButtonRelease(btn) => nes.release_btn(btn)?,
                _ => unreachable!()
            }
        } else {
            for _ in 0..400 {
                nes.clock()?;
            }
        }
        Delay::new(Duration::from_nanos(10)).await?;
    }
}

impl Component for CNes {
    type Message = NesMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (tx, rx) = mpsc::channel::<NesMessage>(CHANNEL_LEN);
        let (load_signal_tx, load_signal_rx) = oneshot::channel::<()>();

        spawn_local(async move {
            if let Err(err) = nes_thread(rx, load_signal_rx).await {
                log::error!("NES crashed due to err: {}", err);
                gloo_dialogs::alert("NES crashed :c (Please refresh page)");
            }
        });

        Self {
            nes_channel: tx,
            load_signal: Some(load_signal_tx),
            file_reader: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            NesMessage::UtilsLoadingFile(file) => {
                let load_signal = self.load_signal.take();
                let link = ctx.link().clone();
                let file_reader = gloo_file::callbacks::read_as_bytes(&file, move |res| {
                    let wrapper = || {
                        let rom_bytes: Vec<u8> = res.ok()?;
                        link.send_message(NesMessage::Load(rom_bytes));
                        let _ = load_signal.map(|load_signal| {
                            load_signal.send(()).ok()
                        });
                        Some(())
                    };
                    if let None = wrapper() {
                        log::error!("Failed to load ROM file");
                        gloo_dialogs::alert("Failed to load ROM file");
                    }
                });
                self.file_reader = Some(file_reader);
            }
            _ => {
                let mut nes_channel = self.nes_channel.clone();

                spawn_local(async move {
                    let status = nes_channel.send(msg).await;
                    if let Err(err) = status {
                        log::error!("NES channel communication error: {}", err);
                    }
                });
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        // button callbacks
        // mobile
        let btn_press = |btn| link.callback(move |_| NesMessage::ButtonPress(btn));
        let btn_release = |btn| link.callback(move |_| NesMessage::ButtonRelease(btn));
        // desktop
        let onkeydown = link.batch_callback(move |e: KeyboardEvent| match e.key().as_str() {
            "ArrowUp" => Some(NesMessage::ButtonPress(Button::Up)),
            "ArrowDown" => Some(NesMessage::ButtonPress(Button::Down)),
            "ArrowRight" => Some(NesMessage::ButtonPress(Button::Right)),
            "ArrowLeft" => Some(NesMessage::ButtonPress(Button::Left)),
            "a" | "A" => Some(NesMessage::ButtonPress(Button::A)),
            "s" | "S" => Some(NesMessage::ButtonPress(Button::B)),
            "Shift" => Some(NesMessage::ButtonPress(Button::Select)),
            "Enter" => Some(NesMessage::ButtonPress(Button::Start)),
            _ => None,
        });
        let onkeyup = link.batch_callback(move |e: KeyboardEvent| match e.key().as_str() {
            "ArrowUp" => Some(NesMessage::ButtonRelease(Button::Up)),
            "ArrowDown" => Some(NesMessage::ButtonRelease(Button::Down)),
            "ArrowRight" => Some(NesMessage::ButtonRelease(Button::Right)),
            "ArrowLeft" => Some(NesMessage::ButtonRelease(Button::Left)),
            "a" | "A" => Some(NesMessage::ButtonRelease(Button::A)),
            "s" | "S" => Some(NesMessage::ButtonRelease(Button::B)),
            "Shift" => Some(NesMessage::ButtonRelease(Button::Select)),
            "Enter" => Some(NesMessage::ButtonRelease(Button::Start)),
            _ => None,
        });

        // nes file reader callback
        let load_rom = link.batch_callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files()?.get(0)?;
            Some(NesMessage::UtilsLoadingFile(file.into()))
        });

        html! {
            <div class="nes" tabindex="0" {onkeydown} {onkeyup}>
                <input class="nes-rom-file"
                    type="file"
                    accept="*.nes"
                    onchange={load_rom}/>
                <canvas id="nes-canvas" width=256 height=240>
                </canvas>
                // NES Buttons
                <div class="nes-joypad">
                    <div class="nes-joypad-left">
                        <button id="up"
                            onpointerenter={btn_press(Button::Up)}
                            onpointerup={btn_release(Button::Up)}
                            onpointerleave={btn_release(Button::Up)}>
                        </button>
                        <button id="down"
                            onpointerenter={btn_press(Button::Down)}
                            onpointerup={btn_release(Button::Down)}
                            onpointerleave={btn_release(Button::Down)} >
                        </button>
                        <button id="right"
                            onpointerenter={btn_press(Button::Right)}
                            onpointerup={btn_release(Button::Right)}
                            onpointerleave={btn_release(Button::Right)} >
                        </button>
                        <button id="left"
                            onpointerenter={btn_press(Button::Left)}
                            onpointerup={btn_release(Button::Left)}
                            onpointerleave={btn_release(Button::Left)} >
                        </button>
                    </div>
                    <div class="nes-joypad-right-up">
                        <button id="b"
                            onpointerenter={btn_press(Button::B)}
                            onpointerup={btn_release(Button::B)}
                            onpointerleave={btn_release(Button::B)} >
                            { "B" }
                        </button>
                        <button id="a"
                            onpointerenter={btn_press(Button::A)}
                            onpointerup={btn_release(Button::A)}
                            onpointerleave={btn_release(Button::A)} >
                            { "A" }
                        </button>
                    </div>
                    <div class="nes-joypad-right-down">
                        <button id="start"
                            onpointerenter={btn_press(Button::Start)}
                            onpointerup={btn_release(Button::Start)}
                            onpointerleave={btn_release(Button::Start)} >
                            { "START" }
                        </button>
                        <button id="select"
                            onpointerenter={btn_press(Button::Select)}
                            onpointerup={btn_release(Button::Select)}
                            onpointerleave={btn_release(Button::Select)} >
                            { "SELECT" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

pub fn main() {
    wasm_logger::init(Config::new(log::Level::Debug));
    log::debug!("Debug Logging enabled");
    yew::start_app::<CNes>();
}

pub mod nes;
