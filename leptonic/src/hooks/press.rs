use educe::Educe;
use leptos_reactive::{
    create_signal,
    Callable, Callback, Signal,
};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent, TouchEvent};

use crate::utils::{current_target_contains_target, props::Attributes, EventTargetExt};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/usePress.ts

#[derive(Debug, Clone, Copy)]
pub struct PressEvent {}

#[derive(Debug, Clone, Copy)]
pub struct UsePressInput {
    pub on_press: Callback<PressEvent>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UsePressProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: Attributes,

    #[educe(Debug(ignore))]
    pub on_key_down: Box<dyn Fn(KeyboardEvent)>,
    /// This handler must be attached to the target element: `<foo on:click=on_click />`
    #[educe(Debug(ignore))]
    pub on_click: Box<dyn Fn(MouseEvent)>,
    #[educe(Debug(ignore))]
    pub on_touch_start: Box<dyn Fn(TouchEvent)>,
    #[educe(Debug(ignore))]
    pub on_touch_move: Box<dyn Fn(TouchEvent)>,
    #[educe(Debug(ignore))]
    pub on_touch_end: Box<dyn Fn(TouchEvent)>,
}

#[derive(Debug)]
pub struct UsePressReturn {
    pub props: UsePressProps,
    pub is_pressed: Signal<bool>,
}

pub fn use_press(input: UsePressInput) -> UsePressReturn {
    let attrs = Attributes::new();

    let (is_pressed, set_is_pressed) = create_signal(false);

    let on_key_down = Box::new(move |e: KeyboardEvent| {
        if !current_target_contains_target(e.current_target().as_ref(), e.target().as_ref())
            .unwrap_or(true)
        {
            tracing::debug!("Aborting on_key_down, as current_target did not contain target.");
            return;
        }

        let key = e.key();

        if e.target()
            .and_then(|t| t.as_element())
            .map(|t| should_prevent_default_keyboard(t, &key))
            .unwrap_or(false)
        {
            e.prevent_default();
        }

        let is_link = e
            .current_target()
            .and_then(|t| t.as_html_element())
            .and_then(|el| el.get_attribute("role"))
            .map_or(false, |r| r == "link");

        let resembles_press = match is_link {
            true => match key.as_str() {
                "Enter" => true,
                _ => false,
            },
            false => match key.as_str() {
                "Enter" | "Spacebar" | " " => true,
                _ => false,
            },
        };

        if resembles_press {
            input.on_press.call(PressEvent {});
        }
    });

    let on_click = Box::new(move |e: MouseEvent| {
        e.prevent_default();
        input.on_press.call(PressEvent {});
    });

    let on_mouse_down = Box::new(move |e: MouseEvent| {
        e.prevent_default();
        input.on_press.call(PressEvent {});
    });
    
    let on_mouse_up = Box::new(move |e: MouseEvent| {
        e.prevent_default();
        input.on_press.call(PressEvent {});
    });

    let on_touchstart = Box::new(move |e: TouchEvent| {
        e.prevent_default();
        if let Some(on_press_start) = input.on_press_start {
            on_press_start.call(PressEvent {});
        }
    });

    let on_touchmove = Box::new(move |e: TouchEvent| {
        e.prevent_default();

        // TODO: detect target leave / enter and submit enter/start.
        if let Some(on_press_end) = input.on_press_end {
            on_press_end.call(PressEvent {});
        }
    });

    let on_touchend = Box::new(move |e: TouchEvent| {
        if !current_target_contains_target(e.current_target().as_ref(), e.target().as_ref())
            .unwrap_or(true)
        {
            tracing::debug!("Aborting on_touchend, as current_target did not contain target.");
            return;
        }
        e.stop_propagation();
        input.on_press.call(PressEvent {});
    });

    UsePressReturn {
        props: UsePressProps {
            attrs,
            on_key_down,
            on_click,
            on_touch_start: on_touchstart,
            on_touch_move: on_touchmove,
            on_touch_end: on_touchend,
        },
        is_pressed: is_pressed.into(),
    }
}

/// Tests for `<a href=[...]>`
fn is_html_anchor(element: web_sys::Element) -> bool {
    element.tag_name().as_str() == "A" && element.has_attribute("href")
}

/// Tests whether a keyboard event's default action should be presented when the given `key` was pressed.
fn should_prevent_default_keyboard(element: web_sys::Element, key: &str) -> bool {
    if element.is_instance_of::<web_sys::HtmlInputElement>() {
        return !is_valid_input_key(element.unchecked_into::<web_sys::HtmlInputElement>(), key);
    }

    if element.is_instance_of::<web_sys::HtmlButtonElement>() {
        return match element.get_attribute("type") {
            Some(ty) => ty != "submit" && ty != "reset",
            None => false,
        };
    }

    if is_html_anchor(element) {
        return false;
    }

    true
}

const NON_TEXT_INPUT_TYPES: [&'static str; 9] = [
    "checkbox", "radio", "range", "color", "file", "image", "button", "submit", "reset",
];

fn is_valid_input_key(element: web_sys::HtmlInputElement, key: &str) -> bool {
    // Checkboxes and radio-buttons should only toggle with space, not enter.
    // TODO: Whats with custom implementations for checkboxes and radio buttons??
    match element.get_attribute("type") {
        Some(ty) => match ty.as_str() {
            "checkbox" | "radio" => key == " " || key == "Spacebar",
            other => NON_TEXT_INPUT_TYPES.contains(&other),
        },
        None => true,
    }
}
