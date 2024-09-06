use dioxus::prelude::*;

use dioxus_logger::tracing::info;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
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
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}

#[component]
pub(crate) fn BlogList() -> Element {
    let future = use_resource(fetch_metadata);

    match &*future.read_unchecked() {
        Some(Ok(metadata)) => rsx! {
            h2 { "Choose a post" }
            ul {
                for post in metadata {
                    li {
                        Link {
                            to: Route::BlogPost {
                                id: post.id.clone(),
                            },
                            "{post.title}"
                        }
                        br {}
                        "{post.creation_date}"
                    }
                }
            }
        },
        Some(Err(_)) => rsx! {
            div { "Loading posts failed" }
        },
        None => rsx! {
            div { "Loading posts..." }
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
                p {
                    dangerous_inner_html: html,
                    onmounted: move |_| {
                        js_sys::eval("hljs.highlightAll();").expect("Error highlighting code");
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! {
                div { "Loading post failed {err}" }
            }
        }
        None => {
            rsx! {
                div { "Loading post..." }
            }
        }
    }
}
