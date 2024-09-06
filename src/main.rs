#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use router::Route;

mod components;
mod router;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            class: "dark:bg-gray-300",
            Router::<Route> {}
        }
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    // Run application
    launch(App);
}
