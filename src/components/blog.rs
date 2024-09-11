use dioxus::prelude::*;

use pulldown_cmark::{html, Parser};
use serde::Deserialize;

use crate::router::Route;

#[derive(Debug, Deserialize)]
pub(crate) struct PostMetadata {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) tags: Vec<String>,
    pub(crate) creation_date: String,
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

async fn fetch_metadata() -> anyhow::Result<Vec<PostMetadata>> {
    let metadata = gloo_net::http::Request::get("/index.json")
        .send()
        .await?
        .json::<Vec<PostMetadata>>()
        .await?;

    Ok(metadata)
}

async fn fetch_from_github(
    repo_owner: &str,
    repo_name: &str,
    branch: &str,
    path: &str,
) -> anyhow::Result<String> {
    let content = gloo_net::http::Request::get(&format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        repo_owner, repo_name, branch, path
    ))
    .send()
    .await?
    .text()
    .await?;

    Ok(content)
}

#[component]
pub(crate) fn Blog() -> Element {
    rsx! {
        div { class: "flex-grow mx-0 px-6 py-6 bg-gray-50 dark:bg-gray-800",
            h1 { class: "text-5xl font-bold text-purple-700 dark:text-purple-300 text-center mb-6",
                "Blog"
            }
            Outlet::<Route> {}
        }
    }
}

#[component]
pub(crate) fn BlogList() -> Element {
    let future = use_resource(fetch_metadata);

    match &*future.read_unchecked() {
        Some(Ok(metadata)) => rsx! {
            div { class: "mx-auto bg-gray-50 dark:bg-gray-800 rounded-lg shadow-lg",
                ul { class: "space-y-4",
                    for post in metadata {
                        li { class: "p-6 bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-teal-100 dark:hover:bg-teal-900 transition duration-200",
                            Link {
                                class: "text-xl text-teal-500 dark:text-teal-400 font-medium hover:underline",
                                to: Route::BlogPost {
                                    id: post.id.clone(),
                                },
                                "{post.title}"
                            }
                            br {}
                            span { class: "text-sm text-gray-700 dark:text-gray-400",
                                "{post.creation_date}"
                            }
                        }
                    }
                }
            }
        },
        Some(Err(_)) => rsx! {
            div { class: "text-coral-600 dark:text-coral-400 text-center", "Loading posts failed" }
        },
        None => rsx! {
            div { class: "text-amber-600 dark:text-amber-400 text-center", "Loading posts..." }
        },
    }
}

#[component]
pub(crate) fn BlogPost(id: ReadOnlySignal<String>) -> Element {
    let post = use_resource(move || async move {
        fetch_from_github(
            "andletenkov",
            "andletenkov.github.io",
            "main",
            &format!("posts/{}.md", id()),
        )
        .await
    });

    match &*post.read_unchecked() {
        Some(Ok(content)) => {
            let html = markdown_to_html(&content);
            rsx! {
                div { class: "container prose dark:prose-invert mx-auto px-6 py-10 bg-gray-50 dark:bg-gray-900 rounded-lg shadow-lg w-full max-w-6xl",
                    p {
                        dangerous_inner_html: html,
                        onmounted: move |_| {
                            js_sys::eval("hljs.highlightAll();").expect("Error highlighting code");
                        }
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! {
                div { class: "text-coral-600 dark:text-coral-400 text-center py-10",
                    "Loading post failed: {err}"
                }
            }
        }
        None => {
            rsx! {
                div { class: "text-amber-600 dark:text-amber-400 text-center py-10",
                    "Loading post..."
                }
            }
        }
    }
}
