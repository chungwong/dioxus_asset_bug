#![allow(non_snake_case)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::all, clippy::pedantic)]

pub mod datepicker;
pub mod drawer;
pub mod tabs;

use dioxus_html::document::head;
use dioxus_lib::prelude::*;
use manganis::mg;

pub const FILE: &'static str = file!();

#[component]
pub(crate) fn Styled(children: Element) -> Element {
    rsx! {
       head::Link { rel: "stylesheet", href: mg!("./assets/tailwind.css") }
       {children}
    }
}
