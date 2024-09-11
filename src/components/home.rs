use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        h1 { class: "text-4xl font-bold text-gray-800 dark:text-gray-100 text-center",
            "Welcome to the Dioxus Blog!"
        }
    }
}
