#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::cnes::CNes;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "app",
            CNes {}
        }
    })
}

pub mod components;
pub mod dk;
pub mod nes;
