//! Focus utilities for managing element focus without side effects.
//!
//! This module provides utilities for focusing elements without triggering scroll.

use std::fmt::Debug;
use wasm_bindgen::JsCast;

pub fn focus_event_target(target: &web_sys::EventTarget, prevent_scroll: bool) {
    if let Some(html_element) = try_downcast_to_html_element(target) {
        focus_html_element(html_element, prevent_scroll);
    }
}

pub fn focus_element(element: &web_sys::Element, prevent_scroll: bool) {
    if let Some(html_element) = try_downcast_to_html_element(element) {
        focus_html_element(html_element, prevent_scroll);
    }
}

pub fn focus_html_element(html_element: &web_sys::HtmlElement, prevent_scroll: bool) {
    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(prevent_scroll);
    if let Err(err) = html_element.focus_with_options(&options) {
        tracing::warn!(?html_element, "Failed to focus element: {err:?}");
    }
}

fn try_downcast_to_html_element<T: JsCast + Debug>(value: &T) -> Option<&web_sys::HtmlElement> {
    if let Some(html_element) = value.dyn_ref::<web_sys::HtmlElement>() {
        Some(html_element)
    } else {
        tracing::warn!(?value, "Not an HTML element.");
        None
    }
}
