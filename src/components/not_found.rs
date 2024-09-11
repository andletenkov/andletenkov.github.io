use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100",
            h1 { class: "text-4xl font-bold mb-4 text-red-500", "Page Not Found" }
            p { class: "text-lg mb-6 text-center",
                "We are terribly sorry, but the page you requested doesn't exist."
            }
            pre { class: "text-sm bg-red-100 dark:bg-red-800 p-4 rounded text-red-500 dark:text-red-200",
                "Attempted to navigate to: {route:?}"
            }
            Link {
                class: "mt-6 inline-block text-blue-500 dark:text-blue-400 hover:underline",
                to: Route::Home {},
                "Go to home page"
            }
        }
    }
}
