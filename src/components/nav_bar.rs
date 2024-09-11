use dioxus::prelude::*;

use crate::router::Route;
use crate::theme;

#[component]
pub(crate) fn NavBar() -> Element {
    rsx! {
        nav { class: "flex-no-wrap relative flex w-full items-center justify-between bg-purple-50 py-2 shadow-md dark:bg-gray-800 lg:flex-wrap lg:justify-start lg:py-4",
            div { class: "flex w-full flex-wrap items-center justify-between px-6",
                ul { class: "flex space-x-8 justify-center",
                    li { class: "hover:underline",
                        Link {
                            class: "text-lg font-semibold text-purple-700 dark:text-purple-300 hover:text-purple-500 dark:hover:text-purple-500",
                            to: Route::Home {},
                            "Home"
                        }
                    }
                    li { class: "hover:underline",
                        Link {
                            class: "text-lg font-semibold text-purple-700 dark:text-purple-300 hover:text-purple-500 dark:hover:text-purple-500",
                            to: Route::BlogList {},
                            "Blog"
                        }
                    }
                }
                a {
                    class: "text-gray-800 dark:text-gray-200 hover:bg-purple-700 dark:hover:bg-purple-500 hover:text-white px-3 py-2 rounded-md text-sm font-medium transition duration-300 ease-in-out",
                    onclick: move |_| {
                        let mut is_dark = theme::IS_DARK.cloned();
                        let new = !is_dark.get();
                        is_dark.set(new).expect("Could not set theme");
                        theme::toggle(new);
                    },
                    svg {
                        class: "dark:hidden",
                        width: "16",
                        height: "16",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            class: "fill-purple-300",
                            d: "M7 0h2v2H7zM12.88 1.637l1.414 1.415-1.415 1.413-1.413-1.414zM14 7h2v2h-2zM12.95 14.433l-1.414-1.413 1.413-1.415 1.415 1.414zM7 14h2v2H7zM2.98 14.364l-1.413-1.415 1.414-1.414 1.414 1.415zM0 7h2v2H0zM3.05 1.706 4.463 3.12 3.05 4.535 1.636 3.12z"
                        }
                        path {
                            class: "fill-purple-500",
                            d: "M8 4C5.8 4 4 5.8 4 8s1.8 4 4 4 4-1.8 4-4-1.8-4-4-4Z"
                        }
                    }
                    svg {
                        class: "hidden dark:block",
                        width: "16",
                        height: "16",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            class: "fill-purple-400",
                            d: "M6.2 1C3.2 1.8 1 4.6 1 7.9 1 11.8 4.2 15 8.1 15c3.3 0 6-2.2 6.9-5.2C9.7 11.2 4.8 6.3 6.2 1Z"
                        }
                        path {
                            class: "fill-purple-600",
                            d: "M12.5 5a.625.625 0 0 1-.625-.625 1.252 1.252 0 0 0-1.25-1.25.625.625 0 1 1 0-1.25 1.252 1.252 0 0 0 1.25-1.25.625.625 0 1 1 1.25 0c.001.69.56 1.249 1.25 1.25a.625.625 0 1 1 0 1.25c-.69.001-1.249.56-1.25 1.25A.625.625 0 0 1 12.5 5Z"
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
