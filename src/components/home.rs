use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        div { class: "flex-grow mx-0 px-6 py-6 bg-gray-50 dark:bg-gray-800",
            h1 { class: "text-5xl font-bold text-purple-700 dark:text-purple-300 text-center mb-6",
                "Welcome to my blog!"
            }
            Outlet::<Route> {}
        }
    }
}
