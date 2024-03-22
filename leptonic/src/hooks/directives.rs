use crate::utils::{attributes::Attributes, event_handlers::EventHandlers};

/// Add the provided attributes to the element.
pub fn attrs(el: leptos::HtmlElement<leptos::html::AnyElement>, attrs: Attributes) {
    let _el = el.attrs(attrs);
}

/// Add the provided event handlers to the element.
pub fn handlers(mut el: leptos::HtmlElement<leptos::html::AnyElement>, handlers: EventHandlers) {
    if let Some(handler) = handlers.on_click {
        el = el.on(leptos::ev::click, move |e| handler(e));
    }
    if let Some(handler) = handlers.on_pointer_down {
        el = el.on(leptos::ev::pointerdown, move |e| handler(e));
    }
    if let Some(handler) = handlers.on_key_down {
        el = el.on(leptos::ev::keydown, move |e| handler(e));
    }
}
