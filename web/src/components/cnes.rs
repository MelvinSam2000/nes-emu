use std::time::Duration;

use anyhow::Result;
use dioxus::prelude::*;
use fluvio_wasm_timer::Delay;

use crate::nes::Nes;
use crate::nes::NesEvent;

const CANVAS: &str = r#"<canvas id="nes-canvas" width=256 height=240></canvas>"#;

pub async fn nes_loop(mut rx: UnboundedReceiver<NesEvent>) -> Result<()> {
    let mut nes = Nes::new()?;
    nes.load(&[])?;
    nes.reset()?;
    loop {
        match rx.try_next() {
            Ok(event) => match event {
                Some(NesEvent::Load(rom_bytes)) => {
                    nes.load(&rom_bytes)?;
                    nes.reset()?;
                }
                Some(NesEvent::Reset) => {
                    nes.reset()?;
                }
                None => {}
            },
            Err(_) => {
                for _ in 0..1000 {
                    nes.clock()?;
                }
            }
        }

        Delay::new(Duration::from_nanos(10)).await?;
    }
}

pub fn CNes(cx: Scope) -> Element {
    let _ = use_coroutine(&cx, |rx: UnboundedReceiver<NesEvent>| async move {
        log::info!("Starting NES thread");
        if let Err(err) = nes_loop(rx).await {
            log::error!("NES crashed due to error: {:?}", err);
        }
    });

    cx.render(rsx! {
        div {
            class: "nes",
            dangerous_inner_html: "{CANVAS}",
        }
    })
}
