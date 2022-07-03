#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::nes::Nes;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus::web::launch(App);
}

const CANVAS: &str = r#"<canvas id="nes-canvas" width="200" height="100"></canvas>"#;

fn App(cx: Scope) -> Element {

    cx.use_hook(|_| {
        cx.spawn(async {
            log::info!("Starting NES Emulator...");
            let mut nes = Nes::default();
            while let Ok(_) = nes.0.clock() {}
            log::error!("Nes Emulator crashed...");
        });
    });

    cx.render(rsx! {
        div {
            class: "app",
            dangerous_inner_html: "{CANVAS}",
            "Hello World!"
        }
    })
}

pub mod nes;
