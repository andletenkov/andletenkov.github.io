use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}
