use dioxus::prelude::*;
use pulldown_cmark::{html, Parser};

use crate::router::Route;

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
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
    rsx! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::BlogPost {
                        title: "go-to-python".to_string(),
                    },
                    "Read the first blog post"
                }
            }
        }
    }
}

#[component]
pub(crate) fn BlogPost(title: String) -> Element {
    let content = "## hihi";
    let inner_html = markdown_to_html(&content);
    rsx! {
        p { dangerous_inner_html: inner_html }
    }
}
