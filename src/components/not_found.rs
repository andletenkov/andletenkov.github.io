use dioxus::prelude::*;
#[component]
pub(crate) fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "attempted to navigate to: {route:?}" }
    }
}