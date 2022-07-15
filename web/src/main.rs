use std::time::Duration;

use anyhow::Result;
use fluvio_wasm_timer::Delay;
use futures::channel::mpsc::channel;
use futures::channel::mpsc::Receiver;
use futures::channel::mpsc::Sender;
use futures::SinkExt;
use wasm_bindgen_futures::spawn_local;
use wasm_logger::Config;
use yew::prelude::*;

use crate::nes::Nes;

const CHANNEL_LEN: usize = 50;

struct CNes {
    nes_channel: Sender<NesMessage>,
}

pub enum NesMessage {
    Load(Vec<u8>),
    Reset,
}

async fn nes_thread(mut rx: Receiver<NesMessage>) -> Result<()> {
    use self::NesMessage::*;

    let mut nes = Nes::new()?;
    nes.load(&[])?;
    nes.reset()?;

    loop {
        if let Ok(Some(msg)) = rx.try_next() {
            match msg {
                Load(_) => todo!(),
                Reset => nes.reset()?,
            }
        } else {
            for _ in 0..1000 {
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
        let (tx, rx) = channel::<NesMessage>(CHANNEL_LEN);

        spawn_local(async move {
            if let Err(err) = nes_thread(rx).await {
                log::error!("NES crashed due to err: {}", err);
            }
        });

        Self { nes_channel: tx }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use self::NesMessage::*;

        let mut nes_channel = self.nes_channel.clone();

        spawn_local(async move {
            let status = match &msg {
                Load(_) => {
                    nes_channel.send(msg).await
                }
                Reset => {
                    nes_channel.send(msg).await
                }
            };
            if let Err(err) = status {
                log::error!("NES channel communication error: {}", err);
            }
        });

        
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="nes">
                <canvas id="nes-canvas" width=256 height=240>
                </canvas>
            </div>
        }
    }
}

pub fn main() {
    wasm_logger::init(Config::new(log::Level::Debug));
    log::info!("Logging enabled");
    yew::start_app::<CNes>();
}

pub mod dk;
pub mod nes;
