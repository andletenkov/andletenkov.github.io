use dioxus::prelude::*;

#[component]
pub(crate) fn Footer() -> Element {
    rsx! {
        footer { class: "w-full py-4 bg-gray-50 dark:bg-gray-800 text-gray-600 dark:text-gray-400 flex justify-center items-center",
            div { class: "w-full max-w-screen-xl mx-auto p-4 md:py-8",
                hr { class: "my-6 border-gray-200 sm:mx-auto dark:border-gray-700 lg:my-8" }
                div { class: "text-sm text-center",
                    span { "Powered by " }
                    a {
                        href: "https://dioxuslabs.com",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "text-purple-600 dark:text-purple-300 hover:underline font-semibold",
                        "Dioxus"
                    }
                }
            }
        }
    }
}
