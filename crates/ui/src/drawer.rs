use dioxus_lib::prelude::*;

use crate::Styled;

pub const FILE: &'static str = file!();
#[component]
fn CloseButton(open: Signal<bool>, on_close: Option<EventHandler<MouseEvent>>) -> Element {
    rsx! {
        div { class: "ml-3 flex h-7 items-center",
            button {
                r#type: "button",
                onclick: move |e| {
                    if let Some(on_close) = on_close {
                        on_close.call(e);
                    }
                },
                class: "relative rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                span { class: "absolute -inset-2.5" }
                span { class: "sr-only", "Close panel" }
                svg {
                    "fill": "none",
                    "stroke": "currentColor",
                    "stroke-width": "1.5",
                    "viewBox": "0 0 24 24",
                    "aria-hidden": "true",
                    class: "h-6 w-6",
                    path {
                        "stroke-linejoin": "round",
                        "d": "M6 18L18 6M6 6l12 12",
                        "stroke-linecap": "round"
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct Props {
    open: Signal<bool>,
    title: Option<Element>,
    children: Element,
    on_ok: Option<EventHandler<MouseEvent>>,
    on_close: Option<EventHandler<MouseEvent>>,
}

#[component]
#[allow(clippy::missing_errors_doc)]
pub fn Drawer(mut props: Props) -> Element {
    let on_close = move |e| {
        if let Some(on_close) = props.on_close {
            on_close.call(e);
        }
        props.open.set(false);
    };

    rsx! {
        if (props.open)() {
            Styled {
                div {
                    "aria-labelledby": "slide-over-title",
                    "aria-modal": "true",
                    role: "dialog",
                    class: "relative z-10",
                    div { class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" }
                    div { class: "fixed inset-0 overflow-hidden",
                        div { class: "absolute inset-0 overflow-hidden",
                            onclick: move |_| {
                                props.open.set(false);
                            },
                            div { class: "pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10 sm:pl-16",
                                onclick: move |e| {
                                    e.stop_propagation();
                                },
                                div { class: "pointer-events-auto w-screen max-w-md",
                                    div { class: "flex h-full flex-col divide-y divide-gray-200 bg-white shadow-xl",
                                        div { class: "h-0 flex-1 overflow-y-auto",
                                            div { class: "px-4 py-6 sm:px-6",
                                                div { class: "flex items-center justify-between",
                                                    h2 {
                                                        class: "text-base font-semibold leading-6",
                                                        { props.title }
                                                    }
                                                    CloseButton {
                                                        open: props.open,
                                                        on_close
                                                    }
                                                }
                                            }
                                            div { class: "flex flex-1 flex-col justify-between",
                                                div { class: "divide-y divide-gray-200 px-4 sm:px-6",
                                                    { props.children }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
