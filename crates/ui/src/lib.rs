#![allow(non_snake_case)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::all, clippy::pedantic)]

pub mod tabs;

use dioxus::prelude::*;

#[component]
pub(crate) fn Styled(children: Element) -> Element {
    rsx! {
       head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
       {children}
    }
}
