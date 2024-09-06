use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::router::Route;

#[component]
pub(crate) fn NavBar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::BlogList {}, "Blog" }
                }
            }
        },
        input {
            r#type: "checkbox",
            oninput: move |event| {
                let is_enabled = event.value() == "true";
                if is_enabled {
                    js_sys::eval("document.documentElement.classList.add('dark');").expect("Error dark");
                } else {
                    js_sys::eval("document.documentElement.classList.remove('dark');").expect("Error light");
                }
            },
        },
        Outlet::<Route> {}
    }
}
