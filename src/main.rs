#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use router::Route;

mod components;
mod router;
mod storage;
mod theme;

#[component]
pub fn App() -> Element {
    theme::init();
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    // Run application
    launch(App);
}
