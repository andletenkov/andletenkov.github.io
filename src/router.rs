use dioxus::prelude::*;

use crate::components::{
    blog::{Blog, BlogList, BlogPost},
    home::Home,
    nav_bar::NavBar,
    not_found::PageNotFound,
};

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[nest("/blog")]
    #[layout(Blog)]
    #[route("/")]
    BlogList {},
    #[route("/post/:title")]
    BlogPost { title: String },
    #[end_layout]
    #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
